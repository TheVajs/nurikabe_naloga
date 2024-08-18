pub mod aco;
pub mod naive;
pub mod random_ant;
pub mod state;

use std::fmt::Debug;
use std::ops::{Bound, Range, RangeBounds};

use state::State;
use wasm_bindgen::JsValue;

pub use crate::nurikabe::Nurikabe;
pub use crate::solvers::naive::NaiveSolver;

fn random_int(range: Range<usize>) -> usize {
    let mut dest: [u8; 1] = [0];
    getrandom::getrandom(&mut dest).expect("Random");
    let lower = match range.start_bound() {
        Bound::Included(&lower) => lower,
        _ => 0,
    };
    let dist = match range.end_bound() {
        Bound::Excluded(&end) => end - lower,
        _ => 0,
    };

    lower + ((dest[0] as f32 / 255.1) * dist as f32).floor() as usize
}

fn random_float() -> f64 {
    get_random_buf().expect("Random")[0] as f64 / (255.1)
}

fn get_random_buf() -> Result<[u8; 1], getrandom::Error> {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

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
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    mut f: impl FnMut(usize, usize) -> bool,
) -> bool {
    (x + 1 < height && f(x + 1, y))
        || (y + 1 < width && f(x, y + 1))
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
