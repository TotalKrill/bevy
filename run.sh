RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' \
    cargo build --example breakout --target wasm32-unknown-unknown -Z build-std=std,panic_abort
wasm-bindgen --out-name wasm_example \
  --out-dir examples/wasm/target \
  --target web target/wasm32-unknown-unknown/debug/examples/breakout.wasm
devserver --header Cross-Origin-Opener-Policy='same-origin' --header Cross-Origin-Embedder-Policy='require-corp' --path examples/wasm
