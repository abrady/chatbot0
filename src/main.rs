mod ollama_chatbot;
mod candle_chat;

// use std::env;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Use Ollama API for chat
    OllamaChatbot,
    /// Use direct Candle inference with a specified model path
    CandleChat { model_path: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::OllamaChatbot => {
            println!("Using Ollama API for chat");
            if let Err(e) = ollama_chatbot::chatbot() {
                eprintln!("Error: {}", e);
            }
        }
        Commands::CandleChat { model_path } => {
            println!("Using direct Candle inference with model: {}", model_path);
            let path = std::path::Path::new(&model_path);
            if !path.exists() {
                eprintln!("Model path does not exist: {}", model_path);
                return;
            }
            if let Err(e) = candle_chat::candle_chat(model_path) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
