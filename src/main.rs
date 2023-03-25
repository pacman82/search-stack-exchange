use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use aleph_alpha_client::{Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding};
use anyhow::Error;
use clap::Parser;
use search_stack_exchange::{Embedding, Embeddings, Post, PostReader};

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
    Question {
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
    /// Token for the Aleph Alpha API. You can see your token if you go to your profile at
    /// <https://app.aleph-alpha.com>.
    #[clap(long, short = 't', env = "AA_API_TOKEN", hide_env_values = true)]
    token: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let opt = Cli::parse();

    match opt.command {
        Command::Question { title_opt } => {
            let TitleOpt {
                posts_xml,
                question,
                token,
            } = title_opt;

            let client = Client::new(&token)?;
            let titles = extract_titles(&posts_xml)?;

            // Load embeddings if already calculated.
            let mut embedding_path = posts_xml.to_owned();
            embedding_path.set_extension("emb");
            let embedding_cache = open_embedddings_cache(&embedding_path)?;
            let title_embeddings = if let Some(cache) = embedding_cache {
                eprintln!("Use cached embeddings");
                Embeddings::from_reader_n(&mut BufReader::new(cache), titles.len())?
            } else {
                eprintln!("Generate embeddings");
                // Generate embeddings
                let embeddings =
                    Embeddings::from_texts(&client, titles.iter().map(|s| s.as_str())).await?;
                // Save them for the next time
                let mut file = File::create(embedding_path)?;
                embeddings.write(&mut file)?;
                embeddings
            };

            let embed_question = TaskSemanticEmbedding {
                prompt: Prompt::from_text(&question),
                representation: SemanticRepresentation::Symmetric,
                compress_to_size: Some(128),
            };
            let question_embedding = &client
                .execute("luminous-base", &embed_question, &Default::default())
                .await
                .unwrap()
                .embedding;

            let index_title =
                title_embeddings.find_most_similar(&Embedding::try_from_slice(question_embedding)?);

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

fn open_embedddings_cache(path: &Path) -> Result<Option<File>, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(Some(file)),
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(error)
            }
        }
    }
}
