mod candle_chat;
mod ollama_chatbot;

// use std::env;
use clap::{Parser, Subcommand};
// use serde::Deserialize;
use inquire::Select;
use std::fs;

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
    CandleChat {
        /// Path to the model file
        #[arg(long, short = 'm')]
        model_path_opt: Option<String>,
    },
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
        Commands::CandleChat { model_path_opt } => {
            let model_path: String;
            if let Some(ref path_str) = model_path_opt.as_ref() {
                let path = std::path::Path::new(path_str);
                if !path.exists() {
                    eprintln!("Model path does not exist: {}", path_str);
                    return;
                }
                model_path = path.to_str().unwrap_or("").to_string();
                println!("Using specified model path: {}", model_path);
            } else {
                let models_json = match fs::read_to_string("models.json") {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Failed to read models.json: {}", e);
                        return;
                    }
                };
                #[derive(serde::Deserialize)]
                struct ModelsConfig {
                    models: Vec<Model>,
                }

                #[derive(serde::Deserialize)]
                struct Model {
                    name: String,
                    path: String,
                    size: String,
                }

                let models_config: ModelsConfig = match serde_json::from_str(&models_json) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Failed to parse models.json: {}", e);
                        return;
                    }
                };

                let model_options: Vec<String> = models_config
                    .models
                    .iter()
                    .map(|model| format!("{} ({})", model.name, model.size))
                    .collect();

                let selection = match Select::new("Select a model:", model_options).prompt() {
                    Ok(choice) => choice,
                    Err(_) => {
                        eprintln!("Selection cancelled");
                        return;
                    }
                };

                // Find the selected model and get its path
                let selected_index = models_config
                    .models
                    .iter()
                    .position(|model| format!("{} ({})", model.name, model.size) == selection)
                    .unwrap();

                model_path = models_config.models[selected_index].path.clone();
                println!("Selected model path: {}", model_path);
            }

            if let Err(e) = candle_chat::candle_chat(model_path.to_string()) {
                eprintln!("Error: {}", e);
            }
        }
    }
}
