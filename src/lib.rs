mod error;
mod reader;

use std::convert::TryInto;

use aleph_alpha_client::{
    cosine_similarity, Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding,
};
pub use error::Error;
use ordered_float::NotNan;
pub use reader::{Post, PostReader};

pub const EMBEDDING_SIZE: usize = 128;

/// Embeddings encode meaning. They are high dimensional vectors those angles are used to determine
/// similarity of different prompts.
pub struct Embedding(pub [f32; EMBEDDING_SIZE]);

impl Embedding {
    /// Constructs an embedding of [`EMBEDDING_SIZE`] from a slice.
    pub fn try_from_slice(slice: &[f32]) -> Result<Self, Error> {
        let array: [f32; EMBEDDING_SIZE] = slice.try_into().map_err(|_| {
            Error::Embedding("API returned embeddings with wrong dimensions.".to_owned())
        })?;
        Ok(Self(array))
    }

    pub fn similarity(&self, other: &Embedding) -> f32 {
        cosine_similarity(&self.0, &other.0)
    }
}

pub struct Embeddings {
    /// Store all 128 sized embeddings in contigious memory
    embeddings: Vec<Embedding>,
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
                compress_to_size: Some(EMBEDDING_SIZE as u32),
            };
            let embedding = client
                .execute(Self::MODEL, &task)
                .await
                .map_err(|e| Error::Embedding(e.to_string()))?
                .embedding;
            embeddings.push(Embedding::try_from_slice(&embedding)?)
        }
        Ok(Self { embeddings })
    }

    pub fn find_most_similar(&self, needle: &Embedding) -> usize {
        let (pos_answer, _similarity) = self
            .embeddings
            .iter()
            .map(|embedding| NotNan::new(embedding.similarity(needle)).unwrap())
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
