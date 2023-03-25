use aleph_alpha_client::{Client, Prompt, SemanticRepresentation, TaskSemanticEmbedding};
use dotenv::dotenv;
use lazy_static::lazy_static;
use search_stack_exchange::{Embedding, Embeddings, Post, PostReader};

lazy_static! {
    static ref AA_API_TOKEN: String = {
        // Use `.env` file if it exists
        let _ = dotenv();
        std::env::var("AA_API_TOKEN")
            .expect(
                "AA_API_TOKEN environment variable must be specified to run tests. You may also \
                create a .env file containing the AA_API_TOKEN.
            ")
    };
}

/// Smaller sample for quicker tests
const SMALL_POSTS: &str = "./tests/small-posts.xml";

#[test]
fn count_all_questions_in_small_posts() {
    let mut reader = PostReader::new(SMALL_POSTS).unwrap();
    let mut num_questions = 0;
    while let Some(post) = reader.next_post().unwrap() {
        if matches!(post, Post::Question { .. }) {
            num_questions += 1;
        }
    }
    assert_eq!(3, num_questions);
}

#[test]
fn list_all_questions_in_small_posts() {
    let mut reader = PostReader::new(SMALL_POSTS).unwrap();
    let mut questions = Vec::new();
    while let Some(post) = reader.next_post().unwrap() {
        if let Post::Question { body: question, .. } = post {
            questions.push(question)
        }
    }

    assert_eq!([
        "<p>When I've printed an object I've had to choose between high resolution and quick prints.  What techniques or technologies can I use or deploy to speed up my high resolution prints?</p>\n",
        "<p>I would like to buy a 3D printer, but I'm concerned about the health risks that are associated with its operation. Some groups of scientists say it can be <a href=\"http://www.techworld.com/news/personal-tech/scientists-warn-of-3d-printing-health-effects-as-tech-hits-high-street-3460992/\">harmful</a> for humans.</p>\n\n<p>What do I need to consider before buying a 3D printer if I care about my health? Are there any safe printers?</p>\n",
        "<p>I know the minimum layer height will effect how detailed of an item you can print and the amount of time it takes to print something, but is it necessary to have an extremely low minimum layer height if you plan to print only larger objects?</p>\n"].as_slice(), &questions)
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
    let title_embeddings = Embeddings::from_texts(&client, titles.iter().map(|s| s.as_str()))
        .await
        .unwrap();
    let embed_question = TaskSemanticEmbedding {
        prompt: Prompt::from_text(question),
        representation: SemanticRepresentation::Symmetric,
        compress_to_size: Some(128),
    };
    let question = &client
        .execute("luminous-base", &embed_question, &Default::default())
        .await
        .unwrap()
        .embedding;
    let question = Embedding::try_from_slice(question).unwrap();

    let pos_answer = title_embeddings.find_most_similar(&question);
    let best_question = &titles[pos_answer];

    // Then
    assert_eq!("Is 3D printing safe for your health?", best_question);
}

#[test]
fn count_all_answers_in_small_posts() {
    let mut reader = PostReader::new(SMALL_POSTS).unwrap();
    let mut num_answers = 0;
    while let Some(post) = reader.next_post().unwrap() {
        if matches!(post, Post::Answer { .. }) {
            num_answers += 1;
        }
    }
    assert_eq!(7, num_answers);
}
