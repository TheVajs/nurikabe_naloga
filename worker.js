// Great source: https://github.com/sgasse/wasm_worker_interaction/tree/main

// This worker.js is called from WASM.

importScripts('./pkg/nurikabe.js');

const { NurikabeApp } = wasm_bindgen;

async function init_wasm_in_worker() {
  await wasm_bindgen("./pkg/nurikabe_bg.wasm");

  var app = NurikabeApp.new(0);
  // Perserve state between calls.

  self.onmessage = async (event) => {
    // Do heavy work in separate thread.

    var worker_result = app.is_even(event.data);

    // Send back to main thread to update view.
    self.postMessage(worker_result);
  };
}

init_wasm_in_worker();
