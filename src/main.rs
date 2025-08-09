use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tokio;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    message: Option<Message>,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Streaming Chat (Ctrl+C to exit)");
    println!("Make sure Ollama is running: `ollama serve`");
    println!("And you have a model installed: `ollama pull llama3.2:1b`");
    println!();

    let client = Client::new();
    let mut conversation: Vec<Message> = vec![];

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "exit" {
            break;
        }

        // Add user message to conversation
        conversation.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        // Prepare request
        let request = ChatRequest {
            model: "llama3.2:1b".to_string(),
            messages: conversation.clone(),
            stream: true,
        };

        print!("🤖 ");
        io::stdout().flush()?;

        // Stream response
        match stream_chat(&client, &request).await {
            Ok(response_content) => {
                // Add assistant response to conversation
                conversation.push(Message {
                    role: "assistant".to_string(),
                    content: response_content,
                });
                println!(); // New line after streaming
            }
            Err(e) => {
                println!("Error: {}", e);
                println!("Make sure Ollama is running and the model is available.");
            }
        }
    }

    println!("Goodbye! 👋");
    Ok(())
}

async fn stream_chat(
    client: &Client,
    request: &ChatRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = client
        .post("http://localhost:11434/api/chat")
        .json(request)
        .send()
        .await?;

    let mut full_response = String::new();

    while let Some(chunk) = response.chunk().await? {
        let chunk_str = String::from_utf8_lossy(&chunk);
        
        for line in chunk_str.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<ChatResponse>(line) {
                Ok(chat_response) => {
                    if let Some(message) = chat_response.message {
                        print!("{}", message.content);
                        io::stdout().flush().unwrap();
                        full_response.push_str(&message.content);
                    }
                    
                    if chat_response.done {
                        break;
                    }
                }
                Err(_) => {
                    // Skip malformed JSON lines
                    continue;
                }
            }
        }
    }

    Ok(full_response)
}
