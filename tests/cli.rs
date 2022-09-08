use assert_cmd::Command;
use lazy_static::lazy_static;
use predicates::str::contains;

lazy_static! {
    static ref AA_API_TOKEN: String = std::env::var("AA_API_TOKEN")
        .expect("AA_API_TOKEN environment variable must be specified to run tests.");
}

#[test]
fn best_question() {
    let assert = Command::cargo_bin("search-stack-exchange")
        .unwrap()
        .args(&[
            "question",
            "--token",
            &AA_API_TOKEN,
            "tests/small-posts.xml",
            "Is 3D Printing dangereous?",
        ])
        .assert();

    assert
        .success()
        .stdout(contains("Is 3D printing safe for your health?"));
}
