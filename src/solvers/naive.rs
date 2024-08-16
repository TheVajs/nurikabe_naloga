use std::{
    cell::{Ref, RefCell},
    collections::{BTreeSet, HashSet, VecDeque},
    rc::Rc,
};

use wasm_bindgen::JsValue;

use crate::console_log;

use super::*;

#[derive(Debug)]
struct Region {
    main: (usize, usize),
    state: State,
    known: BTreeSet<(usize, usize)>,
    unknown: BTreeSet<(usize, usize)>,
}

impl Region {
    fn unknown(x: usize, y: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            main: (x, y),
            state: State::Unknown,
            known: BTreeSet::new(),
            unknown: BTreeSet::new(),
        }))
    }

    fn new(
        x: usize,
        y: usize,
        state: State,
        unknown: BTreeSet<(usize, usize)>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            main: (x, y),
            state,
            known: BTreeSet::from_iter([(x, y)]),
            unknown,
        }))
    }

    #[inline]
    fn is_white(&self) -> bool {
        self.state == State::White || self.is_island()
    }

    #[inline]
    fn is_black(&self) -> bool {
        self.state == State::Black
    }

    #[inline]
    fn is_unknow(&self) -> bool {
        self.state == State::Unknown
    }

    #[inline]
    fn is_known(&self) -> bool {
        self.state != State::Unknown
    }

    fn is_island(&self) -> bool {
        matches!(&self.state, State::Island(_))
    }

    #[inline]
    fn size(&self) -> usize {
        self.known.len()
    }

    fn remove_unknown(&mut self, x: usize, y: usize) {
        self.unknown.remove(&(x, y));
        // if let Some(index) = self
        //     .unknown
        //     .iter()
        //     .position(move |&(a, b)| x == a && y == b)
        // {
        //     self.unknown.remove(index);
        // }
    }
}

#[derive(Debug)]
pub struct NaiveSolver {
    width: usize,
    height: usize,
    grid: Vec<Vec<Rc<RefCell<Region>>>>,
    regions: Vec<Rc<RefCell<Region>>>,
    num_black_cells: usize,
    step: Step,
    solved: bool,
    iteration: usize,
    pub explenation: String,
    pub verbose: bool,
}

impl NaiveSolver {
    pub fn new(nurikabe: Nurikabe) -> Self {
        let width = nurikabe.width;
        let height = nurikabe.height;

        let mut num_black_cells = width * height;

        let mut grid = vec![];
        let mut row = Vec::with_capacity(width);
        let mut regions = vec![];

        for (i, val) in nurikabe.data.into_iter().enumerate() {
            let state = State::new(val);

            let x = i / width;
            let y = i % width;

            match state {
                State::Island(size) => {
                    num_black_cells -= size as usize;

                    let mut unknowns = BTreeSet::new();
                    for_valid_neighbours(width, height, x, y, |a, b| {
                        unknowns.insert((a, b));
                    });

                    let region = Region::new(x, y, state, unknowns);
                    row.push(region.clone());
                    regions.push(region.clone());
                }
                _ => {
                    row.push(Region::unknown(x, y));
                }
            };

            if i % width == width - 1 {
                grid.push(row);
                row = Vec::with_capacity(width);
            }
        }

        Self {
            width,
            height,
            grid,
            regions,
            num_black_cells,
            step: Step::Proceed,
            solved: false,
            explenation: String::from(""),
            verbose: false,
            iteration: 0,
        }
    }

    fn add_region(&mut self, state: State, x: usize, y: usize) {
        let mut unknowns = BTreeSet::new();
        for_valid_neighbours(self.width, self.height, x, y, |a, b| {
            if self.sample(a, b).is_unknow() {
                unknowns.insert((a, b));
            }
        });

        let region = Region::new(x, y, state, unknowns);

        self.grid[x][y] = region.clone();
        self.regions.push(region.clone());
    }

    /// Returns total number of known cells.
    ///
    fn known(&self) -> usize {
        let mut num = 0;

        for x in 0..self.height {
            for y in 0..self.width {
                if self.sample(x, y).is_known() {
                    num += 1;
                }
            }
        }

        num
    }

    #[inline]
    fn sample(&self, x: usize, y: usize) -> Ref<Region> {
        (*self.grid[x][y]).borrow()
    }

    #[inline]
    fn sample_reg(&self, a: (usize, usize)) -> Rc<RefCell<Region>> {
        self.grid[a.0][a.1].clone()
    }

    #[inline]
    fn sample_value(&self, x: usize, y: usize) -> i32 {
        (*self.grid[x][y]).borrow().state.into()
    }

    /// Check if white region can be connected to some island.
    ///
    fn is_white_region_to_big(&self, size: usize) -> bool {
        for region in self.regions.iter() {
            let region = region.as_ref().borrow();

            if let State::Island(max_size) = region.state {
                let island_size = region.size();
                if (island_size + size + 1) <= max_size as usize {
                    return false;
                }
            }
        }

        true
    }

    fn contradictions(&mut self) -> bool {
        self.step = Step::Contradiction;

        // Check if there is a pool
        for x in 0..self.height - 1 {
            for y in 0..self.width - 1 {
                if self.grid[x][y].as_ref().borrow().is_black()
                    && self.grid[x + 1][y].as_ref().borrow().is_black()
                    && self.grid[x][y + 1].as_ref().borrow().is_black()
                    && self.grid[x + 1][y + 1].as_ref().borrow().is_black()
                {
                    console_log!("Contradiction: Pool detected!");
                    return true;
                }
            }
        }

        // Check if white and black cells don't match
        let mut num_black = 0;
        let mut num_white = 0;

        for region in self.regions.iter() {
            let region = region.as_ref().borrow();

            match region.state {
                State::Island(size) => {
                    if region.size() > size as usize {
                        console_log!("Contradiction: Island with to many white cells.");
                        return true;
                    }
                }
                State::White => {
                    // Region is marked white but not an island. Can they still connect?
                    if self.is_white_region_to_big(region.size()) {
                        console_log!(
                            "Contradiction: White region that can't be connected to any remaining island."
                        );
                        return true;
                    }
                }
                _ => (),
            }

            if region.is_black() {
                num_black += region.size();
            } else {
                num_white += region.size();
            }
        }

        if num_black > self.num_black_cells {
            console_log!("Contradiction: To many black cells.");
            return true;
        }

        if num_white > (self.width * self.height) - self.num_black_cells {
            console_log!("Contradiction: To many white cells.");
            return true;
        }

        self.step = Step::Proceed;
        false
    }

    fn solve_completed_islands(&mut self) -> bool {
        let mut mark_black = BTreeSet::new();

        for region in self.regions.iter() {
            let mut region = region.borrow_mut();

            if let State::Island(size) = region.state {
                if size as usize == region.size() {
                    mark_black.append(&mut region.unknown)
                }
            }
        }

        self.update_grid(BTreeSet::new(), mark_black, "Complete island found.")
    }

    fn solve_single_unknown(&mut self) -> bool {
        let mut mark_white = BTreeSet::new();
        let mut mark_black = BTreeSet::new();

        for region in self.regions.iter() {
            let mut region = region.borrow_mut();

            if region.unknown.len() != 1 {
                continue;
            }

            if region.is_black() && region.size() < self.num_black_cells {
                mark_black.append(&mut region.unknown);
            } else if region.is_white() {
                mark_white.append(&mut region.unknown);
            } else if let State::Island(size) = region.state {
                if region.size() < size as usize {
                    mark_white.append(&mut region.unknown);
                }
            }
        }

        self.update_grid(mark_white, mark_black, "Found single uknown.")
    }

    // TODO
    fn solve_two_unknown(&mut self) -> bool {
        // let mut mark_black = BTreeSet::new();

        // for region in self.regions.iter() {
        //     let mut region = region.borrow_mut();

        //     if region.unknown.len() != 2 {
        //         continue;
        //     }
        // }

        // self.update_grid(
        //     BTreeSet::new(),
        //     mark_black,
        //     "Found two uknown for region with size N - 1.",
        // )
        false
    }

    fn solve_bordering(&mut self) -> bool {
        let mut mark_black = BTreeSet::new();

        for x in 0..self.height {
            for y in 0..self.width {
                if !self.grid[x][y].as_ref().borrow().is_unknow() {
                    continue;
                }

                let mut island_count: usize = 0;
                for_valid_neighbours(self.width, self.height, x, y, |a, b| {
                    if self.sample(a, b).is_island() {
                        island_count += 1;
                    }
                });

                if island_count >= 2 {
                    mark_black.insert((x, y));
                }
            }
        }

        self.update_grid(BTreeSet::new(), mark_black, "Found bordering islands.")
    }

    fn solve_potential_pools(&mut self) -> bool {
        let mut mark_white = BTreeSet::new();

        console_log!("Start solving potential pools");

        for x in 0..self.height - 1 {
            for y in 0..self.width - 1 {
                let mut list = [
                    self.sample(x, y),
                    self.sample(x + 1, y),
                    self.sample(x, y + 1),
                    self.sample(x + 1, y + 1),
                ];

                list.sort_by(|a, b| a.state.cmp(&b.state));

                let mut black = 0;
                let mut unknown = 0;
                for r in list.iter() {
                    match r.state {
                        State::Unknown => unknown += 1,
                        State::Black => black += 1,
                        State::White | State::Island(_) => (),
                    }
                }

                if black == 3 && unknown == 1 {
                    if let Some(r) = list.first() {
                        if mark_white.contains(&r.main) {
                            continue;
                        }
                        mark_white.insert(r.main);
                    }
                }
            }
        }

        self.update_grid(mark_white, BTreeSet::new(), "Found potential pool.")
    }

    fn solve_unrechable(&mut self) -> bool {
        let mut mark_black = BTreeSet::new();

        for x in 0..self.height {
            for y in 0..self.width {
                if self.unreachable(x, y) {
                    mark_black.insert((x, y));
                }
            }
        }

        self.update_grid(BTreeSet::new(), mark_black, "Solve unreachable.")
    }

    /// Breath first search(BFS) to first viable island. Island is viable only
    /// if it can stil reach the current cell in question.
    ///
    /// ref: https://www.redblobgames.com/pathfinding/a-star/introduction.html
    ///
    fn unreachable(&self, x: usize, y: usize) -> bool {
        if self.sample(x, y).is_known() {
            return false;
        }

        let mut frontier = VecDeque::new();
        let mut reached = HashSet::new();
        const MAX_DIST: i32 = 15;

        frontier.push_back((x, y, 1));
        reached.insert((x, y));

        while let Some((x, y, cur_dist)) = frontier.pop_front() {
            if cur_dist > MAX_DIST {
                continue;
            }

            let mut white_regions = BTreeSet::new();
            let mut islands = BTreeSet::new();

            for_valid_neighbours(self.width, self.height, x, y, |a, b| {
                let r = self.sample(a, b);
                match r.state {
                    State::White => white_regions.insert((a, b)),
                    State::Island(_) => islands.insert((a, b)),
                    _ => false,
                };
            });

            let mut cur_size = 0;

            for &(x, y) in white_regions.iter() {
                cur_size += self.sample(x, y).size();
            }

            for &(x, y) in islands.iter() {
                cur_size += self.sample(x, y).size();
            }

            if islands.len() > 1 {
                continue;
            }

            if !white_regions.is_empty() {
                if self.is_white_region_to_big(cur_size + cur_dist as usize) {
                    continue;
                } else {
                    return false;
                }
            }

            // if size - r.known.len() as i32 >= cur_dist {
            if islands.len() == 1 {
                let pos = *islands.first().unwrap();
                let r = self.sample(pos.0, pos.1);
                if let State::Island(size) = r.state {
                    if cur_dist + cur_size as i32 <= size {
                        return false;
                    } else {
                        continue;
                    }
                }
            }

            for_valid_neighbours(self.width, self.height, x, y, |a, b| {
                let r = self.sample(a, b);

                // Add unkown neighbours to queue if not already known.

                match r.state {
                    State::Unknown if !reached.contains(&(a, b)) => {
                        frontier.push_front((a, b, cur_dist + 1));
                        reached.insert((a, b));
                    }
                    _ => (),
                }
            });
        }

        true
    }

    fn update_grid(
        &mut self,
        mark_white: BTreeSet<(usize, usize)>,
        mark_black: BTreeSet<(usize, usize)>,
        explenation: &str,
    ) -> bool {
        if mark_white.is_empty() && mark_black.is_empty() {
            return false;
        }

        for (x, y) in mark_white {
            self.mark(x, y, State::White);
        }

        for (x, y) in mark_black {
            self.mark(x, y, State::Black);
        }

        if self.verbose {
            console_log!("{}", explenation);
            self.explenation = String::from(explenation);
        }

        true
    }

    fn mark(&mut self, x: usize, y: usize, state: State) {
        match state {
            State::Black | State::White => {
                if self.sample(x, y).is_known() {
                    console_log!("Contradiction: can only mark unknown cells.");
                    self.step = Step::Contradiction;
                    return;
                }

                for region in self.regions.iter_mut() {
                    region.borrow_mut().remove_unknown(x, y);
                }

                self.add_region(state, x, y);

                // Fuse neighbouring regions.

                for_valid_neighbours(self.width, self.height, x, y, |a, b| {
                    self.fuse_region((x, y), (a, b))
                });
            }
            _ => console_log!("Mark: Logical error, must be white or black"),
        }
    }

    fn fuse_region(&mut self, a: (usize, usize), b: (usize, usize)) {
        {
            let main_region = self.sample_reg(a);
            let region = self.sample_reg(b);

            if Rc::ptr_eq(&main_region, &region) {
                return;
            }

            let mut main_region = main_region.borrow_mut();
            let mut region = region.borrow_mut();

            // if region.is_unknow() {
            //     return;
            // }

            if main_region.is_island() && region.is_island() {
                self.step = Step::Contradiction;
                return;
            }

            if main_region.is_black() != region.is_black() {
                return;
            }

            // Replace data

            if region.is_island() {
                main_region.state = region.state;
            };

            main_region.unknown.append(&mut region.unknown);
            main_region.known.append(&mut region.known);
        }

        // Replace pointer to point at the same region.

        let main = self.sample_reg(a);

        for &(x, y) in main.as_ref().borrow().known.iter() {
            self.grid[x][y] = main.clone();
        }

        // Remove previous region from all region list.

        let pointer = self.sample_reg(b);

        if let Some(index) = self.regions.iter().position(|r| Rc::ptr_eq(r, &pointer)) {
            self.regions.remove(index);
        }

        if !self.regions.iter().any(|r| Rc::ptr_eq(r, &main)) {
            self.regions.push(main);
        }
    }
}

impl Solver for NaiveSolver {
    fn solve(&mut self) -> Step {
        self.iteration += 1;

        if self.known() == self.width * self.height {
            if self.contradictions() {
                console_log!("Contradiction in final result");
                return Step::Contradiction;
            }

            if self.verbose {
                self.explenation = format!("Known: {}/{}", self.known(), self.width * self.height);
            }

            return Step::SolutionFound;
        }

        if self.solve_completed_islands()
            || self.solve_single_unknown()
            || self.solve_two_unknown()
            || self.solve_bordering()
            || self.solve_potential_pools()
            || self.contradictions()
            || self.solve_unrechable()
        {
            return self.step;
        }

        if self.verbose {
            console_log!("Known: {}", self.known());
            console_log!("size: {}", self.height * self.width);
            console_log!("num black: {}", self.num_black_cells);
            self.explenation = format!("Known: {}/{}", self.known(), self.width * self.height);
        }

        Step::CannotProceed
    }

    fn get_state(&self) -> JsValue {
        let mut data = Vec::with_capacity(self.width * self.height);
        for x in 0..self.height {
            for y in 0..self.width {
                // console_log!("{:?}", &self.grid[x][y]);
                data.push(self.sample_value(x, y));
            }
        }

        let verbose = if self.verbose {
            self.explenation.clone()
        } else {
            String::from("")
        };

        serde_wasm_bindgen::to_value(&Nurikabe {
            width: self.width,
            height: self.height,
            solved: self.solved,
            iteration: self.iteration,
            data,
            verbose,
        })
        .unwrap()
    }

    fn get_iteration(&self) -> usize {
        self.iteration
    }
}
