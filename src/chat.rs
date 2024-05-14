use log::error;
use serde::{Deserialize, Serialize};

use crate::ApiKey;
use crate::common::Usage;

type Result<T> = anyhow::Result<T>;

/// https://platform.openai.com/docs/models/model-endpoint-compatibility
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Model {
    #[serde(rename = "gpt-4o")]
    Gpt4O,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-0301")]
    Gpt35Turbo0301,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt35Turbo16k0613,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    System,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompletionRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChoice {
    pub index: i64,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

pub async fn completion(api_key: &ApiKey, req: &CompletionRequest) -> Result<CompletionResponse> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key.as_str()))
        .json(req)
        .send()
        .await?;

    if res.status().is_success() {
        let text = res.text().await?;
        let json: CompletionResponse = serde_json::from_str(&text).map_err(|e| {
            error!("OpenAI API error: {}", text);
            anyhow::anyhow!("{}: {}", e, text)
        })?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!("OpenAI API error: {}", res.text().await?))
    }
}
