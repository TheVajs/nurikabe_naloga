// Great source: https://github.com/sgasse/wasm_worker_interaction/tree/main

// This worker.js is called from WASM.

importScripts("./pkg/nurikabe.js");

const { NurikabeApp } = wasm_bindgen;

async function init_wasm_in_worker() {
  await wasm_bindgen("./pkg/nurikabe_bg.wasm");

  var app = NurikabeApp.new();

  // Do heavy work in separate thread.
  self.onmessage = async (event) => {
    let properties = event.data;

    let start_time = performance.now();
    let nurikabe = app.start_solver(properties);
    nurikabe.duration = parseInt(performance.now() - start_time);

    self.postMessage(nurikabe);
  };
}

init_wasm_in_worker();
