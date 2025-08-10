# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Chatbot Styles Collection** - A collection of different chatbot implementations in Rust showcasing various approaches to LLM interaction.

**Goal**: Demonstrate different architectural patterns for chatbot implementations, from API-based to direct model inference.

**Tech Stack**: 
- Rust command line application
- Tokio async runtime
- Multiple LLM backends (Ollama API, Candle direct inference)

## Architecture Overview

The project uses a modular architecture with two distinct chatbot implementations:

### Main Entry Point (`src/main.rs`)
- Command-line argument parsing determines which chatbot style to use
- No arguments: defaults to Ollama API chatbot
- With model path argument: uses direct inference chatbot

### Ollama API Chatbot (`src/ollama_chatbot.rs`)
- HTTP client using reqwest for Ollama API communication
- Streaming response handling via chunked HTTP responses
- Conversation state management with message history
- JSON serialization/deserialization for API protocol

### Direct Inference Chatbot (`src/candle_chat.rs`)
- Placeholder for direct GGUF model loading (currently simulated)
- File validation for model paths
- Designed for eventual Candle framework integration

## Development Commands

- `cargo check` - Check code without building
- `cargo run` - Run Ollama API chatbot (default mode)
- `cargo run /path/to/model` - Run direct inference chatbot with specified model
- `cargo build --release` - Build optimized binary

## Implementation Details

### Streaming Architecture
Both chatbot implementations use different streaming approaches:
- **Ollama**: HTTP chunked responses parsed line-by-line as JSON
- **Candle**: Token-by-token simulation (placeholder for real inference)

### Conversation Management
- Ollama chatbot maintains full conversation history in `Vec<Message>`
- Messages follow OpenAI-compatible format with `role` and `content` fields
- Default model is `llama3.2:1b` but can be modified in the source

### Error Handling
- Graceful degradation when Ollama service is unavailable
- File validation for direct inference model paths
- JSON parsing errors are silently skipped to handle malformed API responses

## Setup Requirements

### For Ollama Mode (Default):
1. Install Ollama from https://ollama.ai
2. Start service: `ollama serve`
3. Pull a model: `ollama pull llama3.2:1b`
4. Run: `cargo run`

### For Direct Inference Mode:
1. Obtain compatible model file
2. Run: `cargo run /path/to/model` (currently simulated)