// #![allow(clippy::empty_docs)]

// use std::{collections::HashSet, iter::zip};

// use wasm_bindgen::prelude::*;

// use crate::console_log;

// #[wasm_bindgen(module = "/js/view.js")]
// extern "C" {
//     type Nurikabe;

//     #[wasm_bindgen(constructor)]
//     fn new() -> Nurikabe;

//     #[wasm_bindgen(method, getter)]
//     fn width(this: &Nurikabe) -> u32;

//     #[wasm_bindgen(method, setter)]
//     fn set_width(this: &Nurikabe, width: u32) -> Nurikabe;

//     #[wasm_bindgen(method, getter)]
//     fn height(this: &Nurikabe) -> u32;

//     #[wasm_bindgen(method, setter)]
//     fn set_height(this: &Nurikabe, height: u32) -> Nurikabe;

//     #[wasm_bindgen(method)]
//     fn render(this: &Nurikabe) -> String;
// }


// struct Map {
// 	pub possible: Vec<Vec<u8>>,
// 	pub grid: Vec<Vec<usize>>,
// }

// impl Map {
// 	pub fn all_possible(&self, x: usize, y: usize) -> [u8; 16] {
// 		let mut all: [u8; 16] = [0; 16];

// 		for bitmap in 0..16 {
// 			// for i in 0..max {
// 			// 	let bit = bitmap >> i & 1;
// 			// }	
// 			all[bitmap] = bitmap as u8;		
// 		}
		
// 		all
// 	}

// 	pub fn solve(&mut self) -> bool {

// 		for v in self.all_possible(0, 0) {
// 			console_log!("{:#06b}", v);
// 		}

// 		false
// 	}
// }

// #[wasm_bindgen(start)]
// fn run() {
// 	let mut test = Map {
// 		possible: vec![vec![0, 2], vec![0, 0]],
// 		grid: vec![vec![3]],
// 	};

// 	console_log!("{}", test.solve());	
// }

// #[wasm_bindgen(start)]
// fn run() {
//     let nurikabe = Nurikabe::new();
//     assert_eq!(nurikabe.height(), 0);

//     nurikabe.set_height(10);
//     nurikabe.set_width(10);

//     console_log!("{}", &nurikabe.render());
// }

// #[repr(i32)]
// #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
// enum State {
//     Unknown = -3,
//     White = -2,
//     Black = -1,
//     Island(i32),
// }

// Order invariant has struct
// https://users.rust-lang.org/t/make-hash-return-same-value-whather-the-order-of-element-of-a-tuple/69932

// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};

// #[derive(Debug)]
// struct MyTuple(i32, i32);

// impl PartialEq<Self> for MyTuple {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0 && self.1 == other.1 || 
//             self.0 == other.1 && self.1 == other.0
//     }
// }

// impl Eq for MyTuple {}

// impl Hash for MyTuple {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         if self.0 <= self.1 {
//             self.0.hash(state);
//             self.1.hash(state);
//         } else {
//             self.1.hash(state);
//             self.0.hash(state);
//         }
//     }
// }

// fn main() {
//     let hm: HashSet<_> = [
//         MyTuple(1, 0),
//         MyTuple(0, 2),
//         MyTuple(1, 0),
//         MyTuple(2, 0),
//         MyTuple(0, 1),
//     ].into_iter().collect();

//     println!("{:#?}", hm);
// }

// Test aco

// #[wasm_bindgen(start)]
// fn run() {
//     let width = 5;
//     let height = 5;
//     let data = vec![
//         1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
//     ];

//     let nurikabe = Nurikabe::new(width, height, data);

//     console_log!("Test");

//     let mut aco = AntSolver::new(nurikabe);
//     aco.solve();

//     console_log!("Done!");
// }

// Debug.

// for grid in ant_grids.iter() {
//     for i in 0..grid.height {
//         console_log!("{}= {:?}", i, grid.cells[i]);
//     }
// }
