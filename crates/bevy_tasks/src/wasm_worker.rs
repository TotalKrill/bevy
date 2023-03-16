use wasm_bindgen::prelude::*;

pub struct WasmWorkerBuilder {
    stack_size: usize,
}

impl WasmWorkerBuilder {
    pub fn new() -> Self {
        Self { stack_size: 65536 }
    }

    pub fn name(self, _name: String) -> Self {
        self
    }

    pub fn stack_size(self, size: usize) -> Self {
        Self { stack_size: size }
    }

    pub fn spawn<F: FnOnce() + Send + 'static>(self, f: F) -> Result<WebWorkerJoinHandle, JsValue> {
        let worker = web_sys::Worker::new_with_options(
            "./worker.js",
            web_sys::WorkerOptions::new().type_(web_sys::WorkerType::Module),
        )?;
        // Double-boxing because `dyn FnOnce` is unsized and so `Box<dyn FnOnce()>` is a fat pointer.
        // But `Box<Box<dyn FnOnce()>>` is just a plain pointer, and since wasm has 32-bit pointers,
        // we can cast it to a `u32` and back.
        let ptr = Box::into_raw(Box::new(Box::new(f) as Box<dyn FnOnce()>));
        let msg = js_sys::Array::new();

        msg.push(&wasm_bindgen::module());
        msg.push(&wasm_bindgen::memory());

        // Send the address of the closure to execute.
        msg.push(&JsValue::from(ptr as u32));
        worker.post_message(&msg);
        Ok(WebWorkerJoinHandle {})
    }
}

#[derive(Debug)]
pub struct WebWorkerJoinHandle {}

impl WebWorkerJoinHandle {
    pub fn join(self) -> Result<(), ()> {
        Ok(())
    }
}

#[wasm_bindgen]
/// This function is here for `worker.js` to call.
pub fn bevy_worker_entry_point(addr: u32) {
    // Interpret the address we were given as a pointer to a closure to call.
    let closure = unsafe { Box::from_raw(addr as *mut Box<dyn FnOnce()>) };
    (*closure)();
}
