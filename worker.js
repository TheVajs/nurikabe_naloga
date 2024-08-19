// Great source: https://github.com/sgasse/wasm_worker_interaction/tree/main

// This worker.js is called from WASM.

import init, { NurikabeApp } from "./pkg/nurikabe.js";

async function init_wasm_in_worker() {
  await init();

  console.log("Hello from worker!");

  var app = NurikabeApp.new();

  // Do heavy work in separate thread.
  self.onmessage = async (event) => {
    let start_time = performance.now();
    let properties = event.data;
    let nurikabe = app.start_solver(properties);
    nurikabe.duration = parseInt(performance.now() - start_time);
    
    console.log(`Completed in: ${nurikabe.duration}`);

    self.postMessage(nurikabe);
  };
}

init_wasm_in_worker();
