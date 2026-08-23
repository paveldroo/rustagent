use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub llm_api_key: String,
    pub llm_url: String,
    pub model_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        Ok(envy::from_env::<Self>()?)
    }
}
