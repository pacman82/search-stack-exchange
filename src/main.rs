use std::path::{PathBuf, Path};

use aleph_alpha_client::{Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding};
use anyhow::Error;
use clap::Parser;
use search_stack_exchange::{Embeddings, Embedding, Post, PostReader};

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let opt = Cli::parse();

    match opt.command {
        Command::Title { title_opt } => {
            let TitleOpt {
                posts_xml,
                question,
            } = title_opt;

            let api_token = std::env::var("AA_API_TOKEN")?;
            let client = Client::new(&api_token)?;
            let titles = extract_titles(&posts_xml)?;
            let title_embeddings =
                Embeddings::from_texts(&client, titles.iter().map(|s| s.as_str())).await?;

            let embed_question = TaskSemanticEmbedding {
                prompt: Prompt::from_text(&question),
                representation: SemanticRepresentation::Symmetric,
                compress_to_size: Some(128),
            };
            let question_embedding = &client
                .execute("luminous-base", &embed_question)
                .await
                .unwrap()
                .embedding;

            let index_title = title_embeddings
                .find_most_similar(&Embedding::try_from_slice(question_embedding)?);

            let best_title = &titles[index_title];

            println!("{best_title}")
        }
    }
    Ok(())
}

fn extract_titles(posts_xml: &Path) -> Result<Vec<String>, Error> {
    let mut titles = Vec::new();
    let mut reader = PostReader::new(posts_xml)?;
    while let Some(post) = reader.next_post()? {
        if let Post::Question { title, .. } = post {
            titles.push(title);
        }
    }
    Ok(titles)
}
