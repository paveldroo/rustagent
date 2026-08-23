use std::time::Duration;

use crate::{config, error::Error};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
enum Role {
    System,
    User,
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    role: Role,
    pub content: String,
}

impl Message {
    #[must_use]
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_string(),
        }
    }

    #[must_use]
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_string(),
        }
    }

    #[must_use]
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatTemplateKwargs {
    enable_thinking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    stream: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    chat_template_kwargs: Option<ChatTemplateKwargs>,
}

pub struct Client {
    pub http: reqwest::Client,
    cfg: config::Config,
}

impl Client {
    pub fn new(cfg: config::Config) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(30))
            .build()?;
        let c = Self { http: client, cfg };
        Ok(c)
    }

    pub async fn llm_request(self, task: String) -> Result<String, Error> {
        let message = Message::user(&task);

        let req = ChatRequest {
            model: self.cfg.model_name,
            messages: vec![message],
            stream: false,
            chat_template_kwargs: Some(ChatTemplateKwargs {
                enable_thinking: false,
            }),
        };

        let res = self
            .http
            .post(self.cfg.llm_url)
            .bearer_auth(self.cfg.llm_api_key)
            .json(&req)
            .send()
            .await?;

        dbg!(res);

        Ok(String::new())
    }
}
