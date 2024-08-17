use std::{cell::RefCell, rc::Rc};

use gloo_utils::format::JsValueSerdeExt;
use nurikabe::{load_nurikabe, Nurikabe};
use serde::{Deserialize, Serialize};
use solvers::{aco::AntSolver, NaiveSolver, Solver, Step};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, MessageEvent, Worker};

pub mod log;
pub mod nurikabe;
pub mod solvers;
pub mod test;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct Properties {
    nurikabe: Nurikabe,
    method: String,
    ants: usize,
    l_evap: f64,
    g_evap: f64,
    greedines: f64,
    max_iter: usize,
    start_evap: f64,
}

#[wasm_bindgen]
pub fn load(input: &str) -> Result<JsValue, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)?;

    let result = serde_wasm_bindgen::to_value(&nurikabe).map_err(|error| format!("{}", error))?;
    Ok(result)
}

#[wasm_bindgen]
pub struct NurikabeApp {
    previous: Option<Nurikabe>,
}

#[wasm_bindgen]
impl NurikabeApp {
    pub fn new() -> Self {
        Self { previous: None }
    }

    /// Do work in separate thread.
    ///
    pub fn start_solver(&mut self, properties: JsValue) -> Result<JsValue, String> {
        let properties = JsValue::into_serde::<Properties>(&properties)
            .map_err(|_| "Expects properties objects")?;

        let result = match &properties.method[..] {
            "rules" => Ok(NurikabeApp::rule_solver(properties)),
            "ants" => Ok(NurikabeApp::ant_colony_optimization(properties)),
            method => Err(format!("Not implemented method: {}", &method)),
        };

        self.previous = result
            .clone()
            .ok()
            .map(|v| v.into_serde::<Nurikabe>().unwrap());

		result
    }

    fn rule_solver(properties: Properties) -> JsValue {
        let mut solver = NaiveSolver::new(properties.nurikabe);
        solver.verbose = true;

        let max_iter = 50;

        while solver.get_iteration() < max_iter {
            let step = solver.solve();

            if step != Step::Proceed {
                break;
            }
        }

        solver.get_state()
    }

    fn ant_colony_optimization(properties: Properties) -> JsValue {
        let Properties {
            nurikabe,
            ants,
            l_evap,
            g_evap,
            greedines,
            start_evap,
            ..
        } = properties;

        let mut solver = AntSolver::new(ants, l_evap, g_evap, start_evap, greedines, nurikabe);
        solver.verbose = true;

        while solver.get_iteration() < properties.max_iter {
            let step = solver.solve();

            if step != Step::Proceed {
                break;
            }
        }

        solver.get_state()
    }
}

impl Default for NurikabeApp {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn startup() {
    set_panic_hook();

    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));

    setup_callbacks(worker_handle);
}

fn setup_callbacks(worker: Rc<RefCell<web_sys::Worker>>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let on_click_callback = Closure::wrap(Box::new(move || {
        let document = window.document().unwrap();

        let mut properties = Properties::default();

        let get_element_by_id = document.get_element_by_id("ants");
        properties.ants = get_element_by_id
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<usize>()
            .unwrap();

        properties.max_iter = document
            .get_element_by_id("max_iter")
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<usize>()
            .unwrap();

        properties.l_evap = document
            .get_element_by_id("local")
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap();

        properties.g_evap = document
            .get_element_by_id("global")
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap();

        properties.greedines = document
            .get_element_by_id("greedy")
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap();

        let nurikabe = window.get("nurikabe").unwrap();
        let method = window.get("method").unwrap();
        properties.nurikabe = JsValue::into_serde::<Nurikabe>(&nurikabe).expect("Nurikabe!");
        properties.method = JsValue::into_serde::<String>(&method).expect("Method!");

        // Send to worker.

        let worker_handle = &*worker.borrow();
        let _ = worker_handle.post_message(&serde_wasm_bindgen::to_value(&properties).unwrap());
        persistent_callback_handle = get_on_msg_callback();

        // Since the worker returns the message asynchronously, we attach a callback to be
        // triggered when the worker returns.
        worker_handle.set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("solve")
        .expect("#solve should exist")
        .dyn_ref::<HtmlElement>()
        .expect("#solve should be a HtmlElement")
        .set_onclick(Some(on_click_callback.as_ref().unchecked_ref()));

    on_click_callback.forget();
}

/// Create a closure to act on the message returned by the worker
///
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::wrap(Box::new(move |event: MessageEvent| {
        let solver_result: Nurikabe = event.data().into_serde().expect("Nurikabe Result.");

        view_nurikabe(solver_result);
    }) as Box<dyn FnMut(_)>)
}

/// Update nurikabe grid.
fn view_nurikabe(nurikabe: Nurikabe) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    // let previous = window
    //     .get("previous")
    //     .map(|p| JsValue::into_serde::<Nurikabe>(&p).unwrap());
    let parent = document.get_element_by_id("nurikabe").unwrap();
    parent.set_inner_html("");

    let width = nurikabe.width;
    let height = nurikabe.height;

    let grid_parent = document.create_element("div").unwrap();

    if nurikabe.iteration > 0 {
        let step_text = document.create_element("p").unwrap();
        step_text.set_inner_html(&format!("{}. {}", nurikabe.iteration, nurikabe.verbose));
        let _ = grid_parent.append_child(&step_text);
    }

    let grid = document.create_element("div").unwrap();
    grid.set_class_name("grid");

    for i in 0..height {
        let row = document.create_element("div").unwrap();
        row.set_class_name("row");

        for j in 0..width {
            let value = nurikabe.data[i * width + j];
            // let previous_value = match &previous {
            //     Some(p) => p.data[i * width + j],
            //     None => value,
            // };

            let cell = document.create_element("div").unwrap();

            if value > 0 {
                cell.set_inner_html(&format!("{}", value));
            } else {
                cell.set_inner_html(" ");
            }

            if false {
                // window.get("previous_coloring").unwrap().as_bool().unwrap()
                // && previous_value != value
                if value == -1 {
                    cell.set_class_name("new_black");
                } else {
                    cell.set_class_name("new");
                }
            } else {
                match value {
                    -3 => cell.set_class_name("unknown"),
                    -2 => cell.set_class_name("white"),
                    -1 => cell.set_class_name("black"),
                    _ => cell.set_class_name("white"),
                }
            }
            let _ = row.append_child(&cell);
        }

        let _ = grid.append_child(&row);
    }

    let _ = grid_parent.append_child(&grid);
    let _ = parent.append_child(&grid_parent);

    // Update properties
    let properties = document.get_element_by_id("properties").unwrap();
    properties.set_inner_html(&format!(
        "<i>File:</i> <br/>
		<i>Dims:</i> {} x {} <br/>
		<i>Solved:</i> <b>{}</b> <br/>
		<i>Iteration:</i> <b>{}</b> <br/>
		<i>Time:</i> {} ms<br/>
		<br/>",
        nurikabe.width, nurikabe.height, nurikabe.solved, nurikabe.iteration, nurikabe.duration
    ));
    // todo, add to window.
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
