use assert_cli::Assert;

#[test]
fn best_question() {
    Assert::command(&[
        "search-stack-exchange",
        "title",
        "tests/small-posts.xml",
        "Is 3D Printing dangereous?",
    ])
    .unwrap()
}
