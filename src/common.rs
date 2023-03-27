use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "text-davinci-003")]
    TextDavinci003,
    #[serde(rename = "text-curie-001")]
    TextQurie001,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: Option<i64>,
    pub total_tokens: i64,
}
