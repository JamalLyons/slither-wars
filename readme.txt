a remake of the popular game http://slither.com but my own version

The current tech stack for the game is below.

Client: 

- Bevy
- wasm-bindgen
- serde
- serde_json

Server:

- Tokio
- Tungstenite
- serde
- serde_json
- uuid
- rand
- tracing
- tracing-subscriber
- futures
- futures-util
- futures-channel

Database:

- sqlite