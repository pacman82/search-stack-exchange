mod reader;

pub use reader::{Error,PostReader, Post};

pub struct Embeddings {
    /// Store all 128 sized embeddings in contigious memory
    pub embeddings: Vec<[f32; 128]>
}

impl Embeddings {
    pub fn new() -> Self {
        Self {
            embeddings: Vec::new()
        }
    }
}