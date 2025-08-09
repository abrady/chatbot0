# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Streaming Chat UI** - A minimal chat application focused on streaming tokens as they arrive.

**Goal**: Build a clean, minimal chat app that streams responses in real-time without unnecessary complexity.

**Tech Stack**: 
- rust command line
- local model

## Core Features to Build

1. **API Route** (`/api/chat`):
   - Proxy requests to external model
   - Stream responses using Server-Sent Events (SSE)

2. **Chat Interface**:
   - React chat component
   - Real-time streaming text rendering
   - Incremental token display

3. **Stretch Features**:
   - System vs user prompt differentiation
   - Temperature control slider
   - Token usage meter

## Architecture Notes

- Uses Next.js App Router (not Pages Router)
- Focus on streaming UX over feature completeness
- Minimal UI approach - "no bells, just vibes"

## Development Commands

- `cargo check` - Check code without building
- `cargo run` - Build and run the chat application
- `cargo build --release` - Build optimized binary

## Model Setup

1. Install Ollama: Download from https://ollama.ai
2. Start Ollama service: `ollama serve`
3. Download a model: `ollama pull llama3.2:1b` (or `qwen2.5:0.5b` for fastest)
4. Run the chat: `cargo run`

## Claude Code Configuration

The repository has Claude Code permissions configured in `.claude/settings.local.json` allowing directory operations.