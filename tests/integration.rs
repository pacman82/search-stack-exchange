use search_stack_exchange::{PostReader, Post};
use lazy_static::lazy_static;

lazy_static! {
    static ref AA_API_TOKEN: String = std::env::var("AA_API_TOKEN")
        .expect("AA_API_TOKEN environment variable must be specified to run tests.");
}

/// Path to full 3D printing posts.xml
const THREE_D_PRINTING_POSTS: &str = "./tests/3dprinting.Posts.xml";
/// Smaller sample for quicker tests
const SMALL_POSTS: &str = "./tests/small-posts.xml";

#[test]
fn count_all_questions_in_3d_printing() {
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new(THREE_D_PRINTING_POSTS).unwrap();
    let mut num_questions = 0;
    while let Some(post) = reader.next_post().unwrap() {
        if matches!(post, Post::Question { .. }) {
            num_questions += 1;
        }
    }
    assert_eq!(4800, num_questions);
}

#[test]
fn embed_all_questions_in_small_posts() {
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new(SMALL_POSTS).unwrap();
    while let Some(post) = reader.next_post().unwrap() {
        if let Post::Question { title, .. } = post {
            println!("{title}");
        }
    }
}