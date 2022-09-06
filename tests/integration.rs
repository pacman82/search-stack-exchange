use aleph_alpha_client::{
    cosine_similarity, Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding,
};
use lazy_static::lazy_static;
use ordered_float::NotNan;
use search_stack_exchange::{Embeddings, Post, PostReader};

lazy_static! {
    static ref AA_API_TOKEN: String = std::env::var("AA_API_TOKEN")
        .expect("AA_API_TOKEN environment variable must be specified to run tests.");
}

/// Path to full 3D printing posts.xml
// const THREE_D_PRINTING_POSTS: &str = "./tests/3dprinting.Posts.xml";
/// Smaller sample for quicker tests
const SMALL_POSTS: &str = "./tests/small-posts.xml";

#[test]
fn count_all_questions_in_3d_printing() {
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new(SMALL_POSTS).unwrap();
    let mut num_questions = 0;
    while let Some(post) = reader.next_post().unwrap() {
        if matches!(post, Post::Question { .. }) {
            num_questions += 1;
        }
    }
    assert_eq!(3, num_questions);
}

#[tokio::test]
async fn find_best_question() {
    // Given
    let client = Client::new(&AA_API_TOKEN).unwrap();
    let posts = SMALL_POSTS;
    let question = "Is 3D Printing dangereous?";

    // When
    let mut titles = Vec::new();
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new(posts).unwrap();
    while let Some(post) = reader.next_post().unwrap() {
        if let Post::Question { title, .. } = post {
            titles.push(title);
        }
    }
    let mut title_embeddings = Embeddings::new();
    for title in &titles {
        let embedding_task = TaskSemanticEmbedding {
            prompt: Prompt::from_text(title),
            representation: SemanticRepresentation::Symmetric,
            compress_to_size: Some(128),
        };
        let task_output = client
            .execute("luminous-base", &embedding_task)
            .await
            .unwrap();
        title_embeddings
            .embeddings
            .push(task_output.embedding.try_into().unwrap());
    }
    let embed_question = TaskSemanticEmbedding {
        prompt: Prompt::from_text(question),
        representation: SemanticRepresentation::Symmetric,
        compress_to_size: Some(128),
    };
    let question = &client
        .execute("luminous-base", &embed_question)
        .await
        .unwrap()
        .embedding;

    let (pos_answer, _similarity) = title_embeddings
        .embeddings
        .iter()
        .map(|embedding| NotNan::new(cosine_similarity(embedding, question)).unwrap())
        .enumerate()
        .max_by_key(|(_index, similarity)| *similarity)
        .unwrap();
    let best_question = &titles[pos_answer];

    // Then
    assert_eq!("Is 3D printing safe for your health?", best_question);
}
