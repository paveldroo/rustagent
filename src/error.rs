use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse envs: {0}")]
    EnvConfig(#[from] envy::Error),

    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("json encoding error: {0}")]
    DecodeEncode(#[from] serde_json::Error),

    #[error("unexpected response status {status}: {body}")]
    Api {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("write to stdout error: {0}")]
    Io(#[from] std::io::Error),
}
