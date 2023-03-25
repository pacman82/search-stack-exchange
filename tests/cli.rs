use assert_cmd::Command;
use dotenv::dotenv;
use lazy_static::lazy_static;
use predicates::str::contains;

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

#[test]
fn best_question() {
    let assert = Command::cargo_bin("search-stack-exchange")
        .unwrap()
        .args([
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
