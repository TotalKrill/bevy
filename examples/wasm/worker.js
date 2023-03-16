import wasm_bindgen from "./target/wasm_example.js";

self.onmessage = async event => {
  const initialized = await wasm_bindgen(
    event.data[0], // Module
    event.data[1]  // Memoryx
  );

  console.log("INITIALIZED: ", initialized);
  initialized.bevy_worker_entry_point(Number(event.data[2]));
}
