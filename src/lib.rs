mod error;
mod reader;

use aleph_alpha_client::{
    cosine_similarity, Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding,
};
pub use error::Error;
use ordered_float::NotNan;
pub use reader::{Post, PostReader};

pub struct Embeddings {
    /// Store all 128 sized embeddings in contigious memory
    pub embeddings: Vec<[f32; 128]>,
}

impl Embeddings {
    const MODEL: &'static str = "luminous-base";

    pub fn new() -> Self {
        Self {
            embeddings: Vec::new(),
        }
    }

    pub async fn from_texts(
        client: &Client,
        facts: impl IntoIterator<Item = &'_ str>,
    ) -> Result<Self, Error> {
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
        Ok(Self { embeddings })
    }

    pub fn find_most_similar(&self, needle: &[f32; 128]) -> usize {
        let (pos_answer, _similarity) = self
            .embeddings
            .iter()
            .map(|embedding| NotNan::new(cosine_similarity(embedding, needle)).unwrap())
            .enumerate()
            .max_by_key(|(_index, similarity)| *similarity)
            .unwrap();
        pos_answer
    }
}

impl Default for Embeddings {
    fn default() -> Self {
        Self::new()
    }
}
