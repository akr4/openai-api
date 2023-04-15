use log::error;
use serde::{Deserialize, Serialize};

use crate::common::Usage;

type Result<T> = anyhow::Result<T>;

/// https://platform.openai.com/docs/models/model-endpoint-compatibility
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Model {
    #[serde(rename = "text-davinci-003")]
    TextDavinci003,
    #[serde(rename = "text-davinci-002")]
    TextDavinci002,
    #[serde(rename = "text-curie-001")]
    TextCurie001,
    #[serde(rename = "text-babbage-001")]
    TextBabbage001,
    #[serde(rename = "text-ada-001")]
    TextAda001,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompletionRequest {
    pub model: Model,
    pub prompt: String,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChoice {
    pub index: i64,
    pub text: String,
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
        .post("https://api.openai.com/v1/completions")
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
