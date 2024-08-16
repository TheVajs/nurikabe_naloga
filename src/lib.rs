use nurikabe::{load_nurikabe, Nurikabe};
use solvers::{aco::AntSolver, NaiveSolver, Solver, Step};
use wasm_bindgen::prelude::*;

pub mod log;
pub mod nurikabe;
pub mod solvers;
pub mod test;

#[wasm_bindgen]
pub fn load(input: &str) -> Result<JsValue, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)
        .map_err(|report| format!("Make shour the data is correctly formatted!\n {}", report))?;

    let result = serde_wasm_bindgen::to_value(&nurikabe).map_err(|error| format!("{}", error))?;
    Ok(result)
}

#[wasm_bindgen(module = "/js/view.js")]
extern "C" {
    fn view_nurikabe(nurikabe: JsValue);

    fn on_begin();

    fn on_finished();
}

#[wasm_bindgen]
pub fn solve(input: &str) -> Result<bool, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)
        .map_err(|report| format!("Make shour the data is correctly formatted!\n {}", report))?;

    let mut solver = NaiveSolver::new(nurikabe);
    solver.verbose = true;

    let refresh_rate = 1;
    let max_iter = 30;
    let mut step = Step::CannotProceed;

    on_begin();

    while solver.get_iteration() < max_iter {
        step = solver.solve();

        if solver.get_iteration() % refresh_rate == (refresh_rate - 1) {
            view_nurikabe(solver.get_state());
        }

        if step != Step::Proceed {
            break;
        }
    }

    on_finished();

    Ok(step == Step::SolutionFound)
}

#[wasm_bindgen]
pub async fn ant_colony_optimization(
    input: &str,
    ants: usize,
    l_evap: f64,
    g_evap: f64,
	greedines: f64,
	max_iter: usize,
    start_evap: f64,
) -> Result<bool, String> {
    let nurikabe: Nurikabe = load_nurikabe(input)
        .map_err(|report| format!("Make shour the data is correctly formatted!\n {}", report))?;

    let mut solver = AntSolver::new(ants, l_evap, g_evap, start_evap, greedines, nurikabe);
    solver.verbose = true;

    // let refresh_rate = 100;
    let mut step = Step::CannotProceed;

    on_begin();

    while solver.get_iteration() < max_iter {
        step = solver.solve();

        // if solver.get_iteration() % refresh_rate == 0{
        //     view_nurikabe(solver.get_state());
        // }

        if step != Step::Proceed {
            break;
        }
    }

    view_nurikabe(solver.get_state());

    on_finished();

    Ok(step == Step::SolutionFound)
}
