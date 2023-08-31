RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
    cargo build --example breakout --target wasm32-unknown-unknown -Z build-std=std,panic_abort
echo running bindgen
wasm-bindgen --out-name wasm_example \
  --out-dir examples/wasm/target \
  --target web target/wasm32-unknown-unknown/debug/examples/breakout.wasm
echo starting server
# To install this: 
# cargo install --git https://github.com/TotalKrill/devserv.git
devserv --header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp' --path examples/wasm
