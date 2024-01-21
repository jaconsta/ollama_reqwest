# Ollama Run my LLaMa .

Run your GPT prompts locally with a Rust abstraction and session memory.

Runs in sync mode. No async ATM,

## Requirements

A Linux or Mac (M1+) with at least 16GB of RAM.

## Install.

Download [Ollama](https://ollama.ai/) and be sure the server is running.

This implementation expects the Mistral model is running.

```
ollama serve
ollama run mistral
```

## Run

```
cargo run
```

And start typing your prompts.

Note the response may take a while, until it is processed, due to the sync nature of the app.

