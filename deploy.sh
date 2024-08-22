
cargo build --release --target wasm32-unknown-unknown --package token-sender-backend

# notes_keeper_backend.wasm ---->>  the name of the wasm file should use underscroes not hifens.
candid-extractor target/wasm32-unknown-unknown/release/token_sender_backend.wasm > src/token-sender-backend/token-sender-backend.did

dfx deploy token-sender-backend
