use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self(api_key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: Option<i64>,
    pub total_tokens: i64,
}
