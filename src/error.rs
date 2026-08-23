use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse envs: {0}")]
    EnvConfig(#[from] envy::Error),

    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("json encoding error: {0}")]
    Encode(#[from] serde_json::Error),
}
