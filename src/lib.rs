use std::{cell::RefCell, rc::Rc};

use nurikabe::{load_nurikabe, Nurikabe};
use solvers::{aco::AntSolver, NaiveSolver, Solver, Step};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, MessageEvent, Worker};

pub mod log;
pub mod nurikabe;
pub mod solvers;
pub mod test;

#[wasm_bindgen]
pub struct NurikabeApp {
    method: i32,
}

#[wasm_bindgen]
impl NurikabeApp {
    pub fn new(method: i32) -> Self {
        Self { method }
    }

    pub fn get_last_method(&self) -> i32 {
        self.method
    }

    /// Check if a number is even and store it as last processed number.
    pub fn is_even(&mut self, method: i32) -> bool {
        self.method = method;
        matches!(self.method % 2, 0)
    }
}

#[wasm_bindgen]
pub fn startup() {
    set_panic_hook();

    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console_log!("Created a new worker from within Wasm");

    setup_callbacks(worker_handle);
}

#[wasm_bindgen]
pub fn on_solve() {
    todo!()
}

fn setup_callbacks(worker: Rc<RefCell<web_sys::Worker>>) {
   let document = web_sys::window().unwrap().document().unwrap();

    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let on_change_callback = Closure::wrap(Box::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");

        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        match input_field.value().parse::<i32>() {
            Ok(number) => {
				// Send to worker.

                // Access worker behind shared handle, following the interior mutability pattern.
                let worker_handle = &*worker.borrow();
                let _ = worker_handle.post_message(&number.into());
                persistent_callback_handle = get_on_msg_callback();

                // Since the worker returns the message asynchronously, we attach a callback to be
                // triggered when the worker returns.
                worker_handle
                    .set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));
            }
            Err(_) => {
				// Clear result.
                document
                    .get_element_by_id("resultField")
                    .expect("#resultField should exist")
                    .dyn_ref::<HtmlElement>()
                    .expect("#resultField should be a HtmlInputElement")
                    .set_inner_text("");
            }
        }
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(on_change_callback.as_ref().unchecked_ref()));

    on_change_callback.forget();
}

/// Create a closure to act on the message returned by the worker
///
fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    Closure::wrap(Box::new(move |event: MessageEvent| {
        // Return worker data back to main thread with this call.
        // So update view.

        // console_log!("Recieved response: {:?}", &event.data());

        let result = match event.data().as_bool().unwrap() {
            true => "even",
            false => "odd",
        };

        let document = web_sys::window().unwrap().document().unwrap();
        document
            .get_element_by_id("resultField")
            .expect("#resultField should exist")
            .dyn_ref::<HtmlElement>()
            .expect("#resultField should be a HtmlInputElement")
            .set_inner_text(result);
    }) as Box<dyn FnMut(_)>)
}

#[wasm_bindgen]
pub fn load(input: &str) -> Result<JsValue, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)?;

    let result = serde_wasm_bindgen::to_value(&nurikabe).map_err(|error| format!("{}", error))?;
    Ok(result)
}

#[wasm_bindgen]
pub fn solve(input: &str) -> Result<bool, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)?;

    let mut solver = NaiveSolver::new(nurikabe);
    solver.verbose = true;

    // let refresh_rate = 1;
    let max_iter = 30;
    let mut step = Step::CannotProceed;

    while solver.get_iteration() < max_iter {
        step = solver.solve();

        // if solver.get_iteration() % refresh_rate == (refresh_rate - 1) {
        //     view_nurikabe(solver.get_state());
        // }

        if step != Step::Proceed {
            break;
        }
    }

    Ok(step == Step::SolutionFound)
}

#[wasm_bindgen]
pub fn ant_colony_optimization(
    input: &str,
    ants: usize,
    l_evap: f64,
    g_evap: f64,
    greedines: f64,
    max_iter: usize,
    start_evap: f64,
) -> Result<bool, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)?;

    let mut solver = AntSolver::new(ants, l_evap, g_evap, start_evap, greedines, nurikabe);
    solver.verbose = true;

    let mut step = Step::CannotProceed;

    while solver.get_iteration() < max_iter {
        step = solver.solve();

        if step != Step::Proceed {
            break;
        }
    }

    Ok(step == Step::SolutionFound)
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
