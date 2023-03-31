use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::Stream;
use reqwest_eventsource::{Event, EventSource};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::chat::{CompletionRequest, Message};
use crate::common::Model;

type Result<T> = anyhow::Result<T>;

#[derive(Debug, Serialize)]
struct StreamCompletionRequest {
    model: Model,
    messages: Vec<Message>,
    temperature: Option<f32>,
    stream: bool,
}

impl StreamCompletionRequest {
    fn from(req: &CompletionRequest) -> Self {
        Self {
            model: req.model.clone(),
            messages: req.messages.clone(),
            temperature: req.temperature,
            stream: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChunkChoice {
    pub delta: CompletionChunkDelta,
    pub index: usize,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChunkDelta {
    pub content: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompletionChunkResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<CompletionChunkChoice>,
}

pub struct StreamCompletionResponse {
    es: EventSource,
}

impl StreamCompletionResponse {
    pub fn new(es: EventSource) -> Self {
        Self { es }
    }
}

impl Stream for StreamCompletionResponse {
    type Item = Result<CompletionChunkResponse>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let inner_poll = Pin::new(&mut self.es).poll_next(cx);

        match inner_poll {
            Poll::Ready(Some(Ok(Event::Open))) => {
                debug!("Connection opened");
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Ok(Event::Message(message)))) if message.data == "[DONE]" => {
                debug!("Message: {:#?}", message);
                self.get_mut().es.close();
                Poll::Ready(None)
            }
            Poll::Ready(Some(Ok(Event::Message(message)))) => {
                debug!("Message: {:#?}", message);
                let chunk = serde_json::from_str::<CompletionChunkResponse>(&message.data).map_err(|e| {
                    self.get_mut().es.close();
                    anyhow::anyhow!("OpenAI API error: {}", e.to_string())
                })?;
                debug!("Chunk: {:#?}", chunk);
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(error))) => {
                self.get_mut().es.close();
                Poll::Ready(Some(Err(anyhow::anyhow!("OpenAI API error: {}", error.to_string()))))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub async fn completion<ApiKey: AsRef<str>>(
    api_key: ApiKey,
    req: &CompletionRequest,
) -> Result<StreamCompletionResponse> {
    let client = reqwest::Client::new();
    let req = StreamCompletionRequest::from(req);
    let es = EventSource::new(client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key.as_ref()))
        .json(&req))?;

    Ok(StreamCompletionResponse::new(es))
}
