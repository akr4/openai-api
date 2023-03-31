use crate::common::{Model, Usage};
use serde::{Deserialize, Serialize};
use tracing::error;

type Result<T> = anyhow::Result<T>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

pub async fn completion<ApiKey: AsRef<str>>(
    api_key: ApiKey,
    req: &CompletionRequest,
) -> Result<CompletionResponse> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key.as_ref()))
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
