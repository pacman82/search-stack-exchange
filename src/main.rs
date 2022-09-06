use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;
use search_stack_exchange::{PostReader, Post};

/// Semantic Search on top of stack overflow
#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// The question with the title which fits your query best
    Title {
        #[clap(flatten)]
        title_opt: TitleOpt,
    },
}

#[derive(Parser)]
struct TitleOpt {
    /// Input Posts.xml for the stack exchange community you want to search
    posts_xml: PathBuf,
    /// Your question you want to ask
    question: String,
}

fn main() -> Result<(), Error> {
    let opt = Cli::parse();

    match opt.command {
        Command::Title { title_opt } => {
            let TitleOpt {
                posts_xml,
                question: _,
            } = title_opt;

            let titles = extract_titles(posts_xml)?;
            
        }
    }
    Ok(())
}

fn extract_titles(posts_xml: PathBuf) -> Result<Vec<String>, Error> {
    let mut titles = Vec::new();
    let mut reader = PostReader::new(posts_xml)?;
    while let Some(post) = reader.next_post()? {
        if let Post::Question { title, .. } = post {
            titles.push(title);
        }
    }
    Ok(titles)
}
