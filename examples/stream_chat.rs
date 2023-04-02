use std::env;

use futures_util::StreamExt;

use openai_api::{chat, chat_stream};

#[tokio::main]
async fn main() {
    // create request
    let request = chat::CompletionRequest {
        model: openai_api::Model::Gpt35Turbo,
        temperature: Some(1.0),
        messages: vec![
            chat::Message {
                role: chat::MessageRole::User,
                content: "Hello".to_string(),
            }
        ],
    };

    // call completion endpoint
    let mut response = chat_stream::completion(
        env::var("OPENAI_API_KEY").expect("environment variable OPENAI_API_KEY is not found."),
        &request).await.unwrap();

    // receive response
    let mut response_text = String::new();
    while let Some(response) = response.next().await {
        match response {
            Ok(response) => {
                if let Some(text_chunk) = response.choices[0].delta.content.clone() {
                    print!("{text_chunk}");
                    response_text.push_str(&text_chunk);
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    // show response text
    println!("\nResponse Text: {response_text}");
}
