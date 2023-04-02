use std::env;

use openai_api::chat;

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
    let response = chat::completion(
        env::var("OPENAI_API_KEY").expect("environment variable OPENAI_API_KEY is not found."),
        &request).await.unwrap();

    // show response text
    println!("{}", response.choices[0].message.content);
}
