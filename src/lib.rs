mod reader;
mod error;

use aleph_alpha_client::{Client, TaskSemanticEmbedding, Prompt, SemanticRepresentation};
pub use reader::{PostReader, Post};
pub use error::Error;

pub struct Embeddings {
    /// Store all 128 sized embeddings in contigious memory
    pub embeddings: Vec<[f32; 128]>
}

impl Embeddings {
    const MODEL: &'static str = "luminous-base";

    pub fn new() -> Self {
        Self {
            embeddings: Vec::new()
        }
    }

    pub async fn from_texts(client: &Client, facts: impl IntoIterator<Item = &'_ str>) -> Result<Self, Error> {
        let mut embeddings = Vec::new();
        for fact in facts {
            let task = TaskSemanticEmbedding {
                prompt: Prompt::from_text(fact),
                representation: SemanticRepresentation::Symmetric,
                compress_to_size: Some(128),
            };
            let embedding = client
                .execute(Self::MODEL, &task)
                .await
                .map_err(|e| Error::Embedding(e.to_string()))?
                .embedding;
            embeddings.push(embedding.try_into().expect("Slice size must be 128"))
        }
        Ok(Self {
            embeddings,
        })
    }
}

impl Default for Embeddings {
    fn default() -> Self {
        Self::new()
    }
}