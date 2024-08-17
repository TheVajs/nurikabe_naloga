const { startup, load } = wasm_bindgen;

async function run_wasm() {
  await wasm_bindgen("./pkg/nurikabe_bg.wasm");

  console.log("Run startup!");

  class nurikabeapp {
    static async from_file(path) {
      let nurikabe = {};

      await fetch(path)
        .then((res) => res.text())
        .then((raw_input) => {
          nurikabe = load(raw_input);
          nurikabe.raw_input = raw_input;
          nurikabe.path = path;
          nurikabe.time = 0;
          nurikabe.iteration = 0;
          nurikabe.solved = false;
        })
        .catch((e) => console.error(e));

      return nurikabe;
    }

    static async start_solver(raw_input) {
      console.clear();
      window.previous = null;

      let properties = get_properties();
      console.log(properties);
      // if (properties.method == "ants") {
      //   let start_evap = 1.0 / (nurikabe.width * nurikabe.height);

      //   ant_colony_optimization(
      //     raw_input,
      //     properties.ant,
      //     properties.local,
      //     properties.global,
      //     properties.greedines,
      //     properties.max_iter,
      //     start_evap
      //   );
      // } else if (properties.method == "rules") {
      //   window.previous_coloring = false;
      //   solve(raw_input);
      // }
    }
  }

  window.nurikabe_app = nurikabeapp;

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
    // Clear grid.
    let grid = document.getElementById("nurikabe");
    grid.innerHTML = "";

    // Update state.
    let nurikabe = await window.nurikabe_app.from_file(path);
    window.nurikabe = nurikabe;
    window.previous = null;
    view_nurikabe(nurikabe);

    window.enable_clicking("grid");
    enable_last_modifiable();
  }

  // ===========================
  // render
  // ===========================

  let path = "./data/nurikabe10x10v1.csv";
  let nurikabe = await window.nurikabe_app.from_file(path);
  await update(path);

  // ===========================
  // set event listeners
  // ===========================

  let file_selector = document.getElementById("file_selector");
  file_selector.onchange = async () => {
    let path = document.getElementById("file_selector").value;
    path = path.split("\\");
    path = "./data/" + path[path.length - 1];

    await update(path);
  };

  let btn_solve = document.getElementById("solve");
  btn_solve.onclick = async () => {
    if (window.nurikabe && !window.solving) {
      window.nurikabe_app.start_solver(window.nurikabe.raw_input);
    }
  };

  // ===========================
  // set event listeners
  // ===========================

  function set_method(e) {
    console.clear();
    document.getElementById("nurikabe").innerHTML = "";
    view_nurikabe(window.nurikabe);
  }

  function get_properties() {
    var e = document.getElementById("method");
    var value = e.value;
    // var text = e.options[e.selectedIndex].text;

    return {
      ant: parseInt(document.getElementById("ants").value),
      max_iter: parseInt(document.getElementById("max_iter").value),
      local: parseFloat(document.getElementById("local").value),
      global: parseFloat(document.getElementById("global").value),
      greedines: parseFloat(document.getElementById("greedy").value),
      method: value,
    };
  }

  document.getElementById("method").onchange = set_method;
  document.getElementById("ants").value = 10;
  document.getElementById("max_iter").value = 1000;
  document.getElementById("local").value = 0.1;
  document.getElementById("global").value = 0.2;

  startup();
}

run_wasm();
