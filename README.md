# Chatbot Styles Collection

A collection of different chatbot implementations in Rust, showcasing various approaches to LLM interaction.

## Available Chatbot Styles

### 1. Ollama API Chatbot (Default)
- Uses the Ollama API for model inference
- Streaming responses via HTTP requests
- Easy to set up with any Ollama-compatible model

### 2. Direct Model Inference (Candle)
- Direct model loading and inference using Candle
- No external API dependencies
- Pass a model path as a command line argument

## Usage

**Ollama API Mode:**
```bash
cargo run ollama-chatbot
```

**Direct Inference Mode:**
```bash
# With specific model path
cargo run candle-chat /path/to/your/model

# Or let it prompt you to choose from available models
cargo run candle-chat
```

## Setup

### For Ollama Mode:
1. Install Ollama: Download from https://ollama.ai
2. Start Ollama service: `ollama serve`  
3. Download a model: `ollama pull llama3.2:1b` (or `qwen2.5:0.5b` for fastest)
4. Run: `cargo run`

### For Direct Inference Mode:
1. **Auto-discovery (Recommended)**: Run the setup script to find your Ollama models:
   ```bash
   python setup_models.py
   cargo run candle-chat
   ```
   
2. **Manual path**: Obtain a compatible model file and run:
   ```bash
   cargo run candle-chat /path/to/model
   ```

## Development

- `cargo check` - Check code without building
- `cargo run ollama-chatbot` - Run with Ollama API
- `cargo run candle-chat` - Run with direct inference (prompts for model selection)
- `cargo run candle-chat <model_path>` - Run with specific model path
- `cargo build --release` - Build optimized binary
- `python setup_models.py` - Refresh available models list