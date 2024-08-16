pub mod aco;
pub mod naive;
pub mod state;

use std::fmt::Debug;

use state::State;
use wasm_bindgen::JsValue;

pub use crate::nurikabe::Nurikabe;
pub use crate::solvers::naive::NaiveSolver;

pub fn for_valid_neighbours(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    mut f: impl FnMut(usize, usize),
) {
    if x + 1 < height {
        f(x + 1, y);
    }

    if y + 1 < width {
        f(x, y + 1);
    }

    if x > 0 {
        f(x - 1, y);
    }

    if y > 0 {
        f(x, y - 1);
    }
}

pub fn for_valid_diagonal_neighbours(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    mut f: impl FnMut(usize, usize),
) {
    if x + 1 < height && y + 1 < width {
        f(x + 1, y + 1);
    }

    if y + 1 < width && x > 0 {
        f(x - 1, y + 1);
    }

    if x > 0 && y > 0 {
        f(x - 1, y - 1);
    }

    if y > 0 && x + 1 < height {
        f(x + 1, y - 1);
    }
}

/// Returns false if one neighbour is invalid.
///
pub fn for_none_of_neibhbours(
    dim: (usize, usize),
    x: usize,
    y: usize,
    mut f: impl FnMut(usize, usize) -> bool,
) -> bool {
    (x + 1 < dim.0 && f(x + 1, y))
        || (y + 1 < dim.1 && f(x, y + 1))
        || (x > 0 && f(x - 1, y))
        || (y > 0 && f(x, y - 1))
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Step {
    Contradiction,
    SolutionFound,
    Proceed,
    CannotProceed,
}

pub trait Solver {
    /// Solving step.
    ///
    fn solve(&mut self) -> Step;

    /// Returns current state of solver for presentation on the JS/view side.
    ///
    fn get_state(&self) -> JsValue;

    /// Current solving iteration.
    ///
    fn get_iteration(&self) -> usize;
}
