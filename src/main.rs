use std::path::PathBuf;

use anyhow::Error;
use clap::Parser;

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
                posts_xml: _,
                question:_ ,
            } = title_opt;
        }
    }
    Ok(())
}
