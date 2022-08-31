use search_stack_exchange::{PostReader, Post};

#[test]
fn find_similar_question() {
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new("./tests/3dprinting.Posts.xml").unwrap();
    let mut num_questions = 0;
    while let Some(post) = reader.next_post().unwrap() {
        if matches!(post, Post::Question { .. }) {
            num_questions += 1;
        }
    }
    assert_eq!(4800, num_questions);
}
