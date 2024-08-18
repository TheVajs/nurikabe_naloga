import init, { startup, load } from "./pkg/nurikabe.js";

async function run_wasm() {
  await init();

  console.log("Run startup!");

  async function from_file(path) {
    let nurikabe = {};

    await fetch(path)
      .then((res) => res.text())
      .then((raw_input) => {
        nurikabe = load(raw_input);
        nurikabe.raw_input = raw_input;
        nurikabe.path = path;
        nurikabe.duration = 1;
        nurikabe.iteration = 0;
        nurikabe.solved = false;
      })
      .catch((e) => console.error(e));

    return nurikabe;
  }

  window.enable_clicking = (id) => {
    let obj = document.getElementById(id);
    if (obj) {
      function on_mousemove(e) {
        window.mouse_pos = on_mouse_move(e, obj);
      }

      function on_click(e) {
        on_board_click(e, id);
      }

      obj.onmousemove = on_mousemove;
      obj.onclick = on_click;
    }
  };

  async function update(path) {
    let grid = document.getElementById("nurikabe");
    grid.innerHTML = "";

    // Update state.
    let nurikabe = await from_file(path);
    window.nurikabe = nurikabe;
    window.previous = null;
    view_nurikabe(nurikabe);

    window.enable_clicking("grid");
    enable_last_modifiable();
  }

  async function restart_board(_e) {
    window.method = document.getElementById("method").value;
    document.getElementById("nurikabe").innerHTML = "";
    view_nurikabe(window.nurikabe);
  }

  // ===========================
  // Default nurikabe grid.
  // ===========================

  let path = "./data/nurikabe10x10v2.csv";
  await update(path);

  // ===========================
  // Set event listeners.
  // ===========================

  let file_selector = document.getElementById("file_selector");
  file_selector.onchange = async () => {
    let path = document.getElementById("file_selector").value;
    path = path.split("\\");
    path = "./data/" + path[path.length - 1];

    await update(path);
  };

  window.method = "ants";
  document.getElementById("method").value = "ants";
  document.getElementById("method").onchange = restart_board;
  document.getElementById("ants").value = 10;
  document.getElementById("max_iter").value = 5000;
  document.getElementById("local").value = 0.1;
  document.getElementById("global").value = 0.2;
  document.getElementById("greedy").value = 0.9;

  // ===========================
  // Call WASM.
  // ===========================

  startup();
}

run_wasm();
