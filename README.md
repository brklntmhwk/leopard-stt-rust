# Leopard Speech-to-Text Rust

This is a speech-to-text software powered by Leopard Speech-to-Text from [Picovoice](https://picovoice.ai/).

The code inside this repo mostly refers to
- https://github.com/Picovoice/leopard

## Prerequisites

- Sign up for Picovoice
  - https://console.picovoice.ai/
  - In your dashboard, copy your accesskey

## How to use

1. Clone this repo
```
git clone https://github.com/brklntmhwk/leopard-stt-rust.git
```
2. Open it

3. Create .env file in the project root and paste your accesskey as ACCESS_KEY in .env
  - Refer to .env.example

4. Put an audio file (mp3, etc..) in audio/ directory
  - you could put multiple files, but the first one is supposed to be read

5. Build & run this binary project
```
cargo run
```

5. The resultant text file is created as output.txt
