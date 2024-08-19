import init, {
  startup,
  load,
  // sum_of_squares,
  // sum_of_squares_simple,
} from "./pkg/nurikabe.js";

async function run_wasm() {
  console.log("Run startup!");

  await init();

  document.getElementById("file_selector").onchange = async (_e) => {
    let path = document.getElementById("file_selector").value;
    path = path.split("\\");
    path = "./data/" + path[path.length - 1];

    await update_grid(path);
  };

  async function update_grid(path) {
    let nurikabe = await from_file(path);
    window.nurikabe = nurikabe;
    window.previous = null;
    restart_grid();
  }

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

  async function restart_grid() {
    let grid = document.getElementById("nurikabe");
    grid.innerHTML = "";
    view_nurikabe(nurikabe);
    enable_last_modifiable();
  }

  window.enable_clicking = (id) => {
    let obj = document.getElementById(id);
    if (!obj) {
      return;
    }

    obj.onmousemove = (e) => (window.mouse_pos = on_mouse_move(e, obj));
    obj.onclick = (e) => on_board_click(e, id);
  };

  // ===========================
  // Default state.
  // ===========================

  let path = "./data/nurikabe10x10v2.csv";
  await update_grid(path);

  window.method = "ants";
  let method = document.getElementById("method");
  method.value = "ants";
  method.onchange = async () => {
    window.method = document.getElementById("method").value;
    restart_grid();
  };

  document.getElementById("ants").value = 10;
  document.getElementById("max_iter").value = 5000;
  document.getElementById("local").value = 0.1;
  document.getElementById("global").value = 0.2;
  document.getElementById("greedy").value = 0.9;
  document.getElementById("bve").value = 0.001;

  // ===========================
  // Call WASM.
  // ===========================

  // console.log(navigator.hardwareConcurrency);

  startup();

  // Testing

  // let start_time = performance.now();

  // let list = [];
  // for (let i = 0; i < 50000000; i++) {
  //   list.push(1);
  // }
  // console.log(sum_of_squares_simple(list));

  // let duration = performance.now() - start_time;
  // console.log(duration);

  // start_time = performance.now();

  // list = [];
  // for (let i = 0; i < 50000000; i++) {
  //   list.push(1);
  // }
  // console.log(sum_of_squares(list));

  // duration = performance.now() - start_time;
  // console.log(duration);
}

run_wasm();
