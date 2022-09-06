use std::{
    convert::TryInto,
    io::{self, BufRead, Write},
    mem::size_of,
};

use crate::Error;
use aleph_alpha_client::{
    cosine_similarity, Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding,
};
use ordered_float::NotNan;

pub const EMBEDDING_SIZE: usize = 128;

/// Embeddings encode meaning. They are high dimensional vectors those angles are used to determine
/// similarity of different prompts.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Embedding(pub [f32; EMBEDDING_SIZE]);

impl Embedding {
    pub fn new() -> Self {
        Self([0f32; EMBEDDING_SIZE])
    }

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

    /// Write the embedding into a binary buffer.
    pub fn write_to_bytes(&self, buf: &mut [u8; EMBEDDING_SIZE * size_of::<f32>()]) {
        for (bytes, float) in buf.chunks_exact_mut(size_of::<f32>()).zip(self.0) {
            bytes.copy_from_slice(&float.to_le_bytes())
        }
    }

    /// Load embedding from a binary buffer.
    pub fn read_from_bytes(&mut self, buf: &[u8; EMBEDDING_SIZE * size_of::<f32>()]) {
        for (bytes, float) in buf.chunks_exact(size_of::<f32>()).zip(&mut self.0) {
            *float = f32::from_le_bytes(bytes.try_into().unwrap());
        }
    }
}

impl Default for Embedding {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
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

    pub fn from_reader_n(read: &mut impl BufRead, n: usize) -> Result<Self, io::Error> {
        let mut embeddings = Self::new();
        embeddings.read_n(read, n)?;
        Ok(embeddings)
    }

    pub fn from_vec(embeddings: Vec<Embedding>) -> Self {
        Self { embeddings }
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
            let mut embedding = None;
            while embedding.is_none() {
                embedding = match client.execute(Self::MODEL, &task).await {
                    Ok(output) => Some(output.embedding),
                    Err(error) => match error {
                        aleph_alpha_client::Error::TooManyRequests
                        | aleph_alpha_client::Error::Busy => None,
                        _ => return Err(Error::Embedding(error.to_string())),
                    },
                };
            }
            embeddings.push(Embedding::try_from_slice(&embedding.unwrap())?)
        }
        Ok(Self::from_vec(embeddings))
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

    pub fn write(&self, write: &mut impl Write) -> Result<(), io::Error> {
        let mut buf = [0u8; EMBEDDING_SIZE * size_of::<f32>()];
        for embedding in &self.embeddings {
            embedding.write_to_bytes(&mut buf);
            write.write_all(&buf)?;
        }
        write.flush()?;
        Ok(())
    }

    /// Read n embeddings from reader
    pub fn read_n(&mut self, read: &mut impl BufRead, n: usize) -> Result<(), io::Error> {
        self.embeddings.clear();
        let mut buf = [0u8; EMBEDDING_SIZE * size_of::<f32>()];
        for _ in 0..n {
            read.read_exact(&mut buf)?;
            let mut embedding = Embedding::new();
            embedding.read_from_bytes(&buf);
            self.embeddings.push(embedding)
        }
        Ok(())
    }
}

impl Default for Embeddings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn embedding_to_and_fro_bytes() {
        let embedding =
            Embedding::try_from_slice(&(0..128).map(|i| i as f32).collect::<Vec<_>>()).unwrap();
        let mut buf = [0u8; EMBEDDING_SIZE * size_of::<f32>()];
        embedding.write_to_bytes(&mut buf);
        let mut loaded = Embedding::new();
        loaded.read_from_bytes(&buf);

        assert_eq!(embedding, loaded)
    }

    #[test]
    fn multiple_embedding_to_and_fro_bytes() {
        let embedding =
            Embedding::try_from_slice(&(0..128).map(|i| i as f32).collect::<Vec<_>>()).unwrap();

        let embeddings = Embeddings::from_vec(vec![embedding, embedding]);
        let mut buf = Vec::new();
        embeddings.write(&mut buf).unwrap();
        let loaded = Embeddings::from_reader_n(&mut Cursor::new(buf), 2).unwrap();

        assert_eq!(embeddings, loaded)
    }
}
