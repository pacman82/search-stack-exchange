use assert_cli::Assert;

#[test]
fn best_question() {
    Assert::command(&["search-stack-exchange", "title", "small-posts.xml"]).unwrap()
}