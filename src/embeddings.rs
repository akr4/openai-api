use log::error;
use serde::{Deserialize, Serialize};

use crate::ApiKey;

type Result<T> = anyhow::Result<T>;

/// https://platform.openai.com/docs/models/model-endpoint-compatibility
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Model {
    #[serde(rename = "text-embedding-ada-002")]
    TextEmbeddingAda002,
    #[serde(rename = "text-search-ada-doc-001")]
    TextSearchAdaDoc001,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingsRequest {
    pub model: Model,
    pub input: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Data {
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbeddingsResponse {
    pub data: Vec<Data>,
}

pub async fn embedding(api_key: &ApiKey, request: EmbeddingsRequest) -> Result<EmbeddingsResponse> {
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/embeddings")
        .header("Authorization", format!("Bearer {}", api_key.as_str()))
        .json(&request)
        .send()
        .await?;

    if res.status().is_success() {
        let text = res.text().await?;
        let json: EmbeddingsResponse = serde_json::from_str(&text).map_err(|e| {
            error!("OpenAI API error: {}", text);
            anyhow::anyhow!("{}: {}", e, text)
        })?;
        Ok(json)
    } else {
        Err(anyhow::anyhow!("OpenAI API error: {}", res.text().await?))
    }
}
