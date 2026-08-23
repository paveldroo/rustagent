use assert_cmd::{Command, cargo::cargo_bin_cmd};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test(flavor = "multi_thread")]
async fn success() {
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("--task=\"tell the joke\"");

    let json_response = std::fs::read_to_string("tests/fixtures/response.txt").unwrap_or_default();
    let mock_response = ResponseTemplate::new(200).set_body_string(json_response);

    let mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .respond_with(mock_response)
        .mount(&mock)
        .await;

    set_all_envs(&mut cmd);
    cmd.env("LLM_URL", mock.uri());
    cmd.assert().success().stdout(predicates::str::contains(
        "Because they make up everything!",
    ));
}

fn set_all_envs(cmd: &mut Command) {
    cmd.env("LLM_API_KEY", "test");
    cmd.env("LLM_URL", "test");
    cmd.env("MODEL_NAME", "test");
}
