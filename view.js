/**
 * TODO
 *
 * @param {*} id
 * @returns
 */
function get_board_squares(id) {
  let rows = document.getElementById(id).children;

  let rects = [];
  for (let i = 0; i < rows.length; i++) {
    Array.prototype.push.apply(rects, rows[i].children);
  }

  return rects;
}

/**
 * Process click event to mark rectangle on nurikabe board.
 *
 * @param {Event} e
 * @returns none
 */
function on_board_click(_e, id) {
  if (!window.mouse_pos) {
    return;
  }

  let parent = document.getElementById(id);
  let top = parent.offsetTop;
  let left = parent.offsetLeft;

  let squares = get_board_squares(id);
  let [x, y] = window.mouse_pos;

  if (!squares || squares.length <= 0) {
    console.warn("No boards squares");
    return;
  }

  let width = 30;
  let height = 30;
  for (let i = 0; i < squares.length; i++) {
    let { offsetTop, offsetLeft } = squares[i];
    offsetTop -= top;
    offsetLeft -= left;

    if (
      x > offsetLeft &&
      x < offsetLeft + height &&
      y > offsetTop &&
      y < offsetTop + width &&
      squares[i].innerHTML == " "
    ) {
      if (squares[i].className == "white") {
        squares[i].className = "black";
      } else if (squares[i].className == "black") {
        squares[i].className = "unknown";
      } else {
        squares[i].className = "white";
      }
    }
  }
}

// ===========================
// Update HTML
// ===========================

/**
 * Updates HTML elements to represent the current state of nurikabe object
 *
 * @param {} nurikabe
 * @param {} option
 * @returns
 */
function view_nurikabe(nurikabe) {
  let previous = window.previous ? window.previous : nurikabe;
  let parent = document.getElementById("nurikabe");

  let width = nurikabe.width;
  let height = nurikabe.height;

  let grid_parent = document.createElement("div");

  if (nurikabe.iteration > 0) {
    let step_text = document.createElement("p");
    step_text.innerHTML = `${nurikabe.iteration}. ${nurikabe.verbose}`;
    grid_parent.appendChild(step_text);
  }

  let grid = document.createElement("div");
  grid.className = "grid";

  for (let i = 0; i < height; i++) {
    let row = document.createElement("div");
    row.className = "row";

    for (let j = 0; j < width; j++) {
      let value = nurikabe.data[i * width + j];
      let previous_value = previous.data[i * width + j];
      let cell = document.createElement("div");

      cell.innerHTML = value > 0 ? value : " ";

      if (window.previous_coloring && previous_value != value) {
        if (value == -1) {
          cell.className = "new_black";
        } else {
          cell.className = "new";
        }
      } else {
        switch (value) {
          case -3:
            cell.className = "unknown";
            break;
          case -2:
            cell.className = "white";
            break;
          case -1:
            cell.className = "black";
            break;
          default:
            cell.className = "white";
        }
      }
      row.appendChild(cell);
    }

    grid.appendChild(row);
  }

  grid_parent.appendChild(grid);
  parent.appendChild(grid_parent);

  // Update properties
  let properties = document.getElementById("properties");
  properties.innerHTML = `
				<i>File:</i> ${window.nurikabe.path} <br/>
				<i>Dims:</i> ${nurikabe.width} x ${nurikabe.height} <br/>
				<i>Solved:</i> <b>${nurikabe.solved}</b> <br/>
				<i>Iteration:</i> <b>${nurikabe.iteration}</b> <br/>
				<i>Time:</i> ${0} ms`;

  window.previous = nurikabe;
}

function on_begin() {
  window.solving = true;

  window.start_time = performance.now();

  document.getElementById("nurikabe").innerHTML = "";
}

function on_finished() {
  window.solving = false;

  enable_last_modifiable();
}

function scroll_down() {
  window.scrollTo(0, document.body.scrollHeight);
}

function enable_last_modifiable() {
  let last = document.getElementById("nurikabe").lastChild.lastChild;
  last.id = "grid";
  window.enable_clicking("grid");
}
