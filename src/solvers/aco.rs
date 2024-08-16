use std::{
    collections::{HashSet, VecDeque},
    ops::{Bound, Range, RangeBounds},
};

const BLACK: i32 = 0;

use super::{
    for_none_of_neibhbours, for_valid_diagonal_neighbours, for_valid_neighbours, Nurikabe, Solver,
    Step,
};
use wasm_bindgen::JsValue;

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

    lower + ((dest[0] as f32 / 255.0001) * dist as f32).floor() as usize
}

fn random_float() -> f64 {
    get_random_buf().expect("Random")[0] as f64 / (255.0 + 1e-13)
}

fn get_random_buf() -> Result<[u8; 1], getrandom::Error> {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf)?;
    Ok(buf)
}

#[derive(Debug, Clone)]
struct Island {
    id: i32,
    pos: (usize, usize),
    size: usize,
    final_size: usize,
}

impl Island {
    fn new(id: i32, pos: (usize, usize), size: usize, final_size: usize) -> Self {
        Self {
            id,
            pos,
            size,
            final_size,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<i32>>,
    reached_white: usize,
    eval: usize,
    best_p: f64,
}

impl Grid {
    fn new(width: usize, height: usize, cells: Vec<Vec<i32>>) -> Self {
        Self {
            width,
            height,
            cells,
            reached_white: 0,
            eval: usize::MAX,
            best_p: 0.0,
        }
    }

    fn flat(&self) -> Vec<i32> {
        self.cells.iter().flat_map(|row| row.clone()).collect()
    }

    fn is_connecting_islands(&self, x: usize, y: usize, island_id: i32) -> bool {
        for_none_of_neibhbours((self.height, self.width), x, y, |a, b| -> bool {
            let sample = self.cells[a][b];
            sample != BLACK && sample != island_id
        })
    }

    /// Easy test first, which checks for specific situation, when a disruption
    /// of the river flows can occur. If situation is found, than one of the possible
    /// start positions is returned, where with a DFS search checks if every black cell
    /// is still reachable from that position.
    ///
    fn is_river_frgmented(&mut self, x: usize, y: usize, island_id: i32) -> bool {
        if let Some(start) = self.creates_wall_cut(x, y) {
            // Set white.
            self.cells[x][y] = island_id;

            let num_black_cells = self.height * self.width - self.reached_white - 1;
            let reached_black_cells = self.dfs(start);

            // console_log!("{}, {}, {}", reached_black_cells, num_black_cells, self.reached_white);
            let in_fragments = reached_black_cells != num_black_cells;

            // console_log!("test");
            // for (i, row) in self.cells.iter().enumerate() {
            //     console_log!("{}= {:.4?}", i, row);
            // }

            // Restore.
            self.cells[x][y] = BLACK;

            return in_fragments;
        }

        false
    }

    /// Depth first search with counting.
    ///
    fn dfs(&self, start: (usize, usize)) -> usize {
        let mut queue = VecDeque::new();
        let mut reached = HashSet::new();
        let mut num_black = 0;

        queue.push_front(start);
        reached.insert(start);

        while let Some((x1, y1)) = queue.pop_front() {
            num_black += 1;

            for_valid_neighbours(self.width, self.height, x1, y1, |a, b| {
                if self.cells[a][b] == 0 && !reached.contains(&(a, b)) {
                    queue.push_front((a, b));
                    reached.insert((a, b));
                }
            });
        }

        num_black
    }

    /// Possible situation when cuts in the river can cause problems.
    /// Only two situations are possible:
    ///     when valid diagonal neighbour (I) of island O (x, y) is a island
    ///     w | I        
    ///     O | w
    ///  or
    ///     (x, y) is on border.
    ///
    fn creates_wall_cut(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        // Touching border.

        let living_on_edge = x == self.width - 1 || x == 0 || y == self.width - 1 || y == 0;
        if living_on_edge {
            let mut black_cells = vec![];
            for_valid_neighbours(self.width, self.height, x, y, |a, b| {
                if self.cells[a][b] == BLACK {
                    black_cells.push((a, b));
                }
            });

            if let Some(&(a, b)) = black_cells.first() {
                return Some((a, b));
            }
        }

        // Touching diagonaly test.

        let mut neighbours = vec![];
        for_valid_diagonal_neighbours(self.width, self.height, x, y, |a, b| {
            if self.cells[a][b] != BLACK {
                neighbours.push((a, b))
            }
        });

        // Needs to choose correct black spot!
        match neighbours.len() {
            0 => (),
            1..=4 => {
                for &(a, b) in neighbours.iter() {
                    if self.cells[a][y] == BLACK {
                        return Some((a, y));
                    } else if self.cells[x][b] == BLACK {
                        return Some((x, b));
                    }
                }
            }
            _ => (),
        }

        None
    }

    /// Get number of pools, which are 2x2 black cells.
    ///
    fn get_num_pools(&self) -> usize {
        let mut num = 0;

        for x in 0..self.height - 1 {
            for y in 0..self.width - 1 {
                if self.cells[x][y] == BLACK
                    && self.cells[x + 1][y] == BLACK
                    && self.cells[x][y + 1] == BLACK
                    && self.cells[x + 1][y + 1] == BLACK
                {
                    num += 1;
                }
            }
        }

        num
    }

    fn evaluate(&mut self, white_cells: usize) -> f64 {
        self.eval = white_cells - self.reached_white + self.get_num_pools();
        self.best_p = 1.0 / self.eval as f64;
        self.best_p
    }

    fn is_solved(&self) -> bool {
        self.eval == 0
    }
}

#[derive(Debug)]
pub struct AntSolver {
    ants: usize,
    l_evap: f64,
    g_evap: f64,
    evap: f64,
	greedines: f64,
    phermons: Vec<Vec<f64>>,

    grid: Grid,
    solution: Grid,
    solution_num_white: usize,
    islands: Vec<Island>,
    iteration: usize,
    explain: String,
    pub verbose: bool,
}

impl AntSolver {
    pub fn new(
        ants: usize,
        local_evap: f64,
        global_evap: f64,
        evap: f64,
		greedines: f64,
        nurikabe: Nurikabe,
    ) -> Self {
        let width = nurikabe.width;
        let height = nurikabe.height;

        let mut islands = vec![];
        let mut ids: i32 = 0;
        let mut num_white = 0;

        let mut cells = vec![];
        let mut row = Vec::with_capacity(width);

        for (i, val) in nurikabe.data.into_iter().enumerate() {
            let x = i / width;
            let y = i % width;

            let island = if val > 0 {
                let island_size = val as usize;
                num_white += island_size;
                ids += 1;

                islands.push(Island::new(ids, (x, y), 0, island_size));

                ids
            } else {
                0
            };
            row.push(island);

            if i % width == width - 1 {
                cells.push(row);
                row = Vec::with_capacity(width);
            }
        }

        let phermons = (0..height)
            .map(|_| (0..width).map(|_| evap).collect())
            .collect::<Vec<Vec<f64>>>();

        Self {
            ants,
			evap: evap.clamp(0.0, 1.0), 
			l_evap: local_evap.clamp(0.0, 1.0), 
			g_evap: global_evap.clamp(0.0, 1.0), 
			greedines: greedines.clamp(0.0, 1.0), 
            phermons,
            grid: Grid::new(width, height, cells),
            solution: Grid::new(0, 0, vec![vec![]]),
            solution_num_white: num_white,
            islands,
            iteration: 0,
            explain: String::new(),
            verbose: false,
        }
    }
}

impl Solver for AntSolver {
    fn solve(&mut self) -> Step {
        for _ in 0..self.ants {
            self.iteration += 1;

            let mut islands = self.islands.clone();
            let mut k_grid = self.grid.clone();
            let mut k_phermons = self.phermons.clone();

            k_grid.reached_white = islands.len();

            while !islands.is_empty() {
                let mut island = islands.remove(random_int(0..islands.len()));
                let mut queue = vec![island.pos];
                let mut first = true;

                while !queue.is_empty() {
                    // Phermon strategy.

                    let r = random_float();
                    let s = if r < self.greedines {
                        let mut pick = -1.0;
                        let mut index = 0;

                        for (i, &(a, b)) in queue.iter().enumerate() {
                            if k_phermons[a][b] > pick {
                                pick = k_phermons[a][b];
                                index = i;
                            }
                        }
                        index
                    } else {
                        let r = random_float();

                        let mut acc = 0.0;
                        let sum: f64 = queue.iter().map(|&(a, b)| k_phermons[a][b]).sum();

                        queue.iter()
                            .map(|&(a, b)| {
                                acc += k_phermons[a][b] / sum;
                                acc
                            })
                            .collect::<Vec<_>>()
                            .into_iter()
                            .position(|prob| r < prob)
							.unwrap_or(0)
                    };

                    let (x, y) = queue.remove(s);

                    if !first {
                        // Check validity of island, skip when:
                        // 1. Connects with another island.
                        // 2. Breaks continuation with other black/river cells.

                        if k_grid.is_connecting_islands(x, y, island.id) {
                            continue;
                        }

                        if k_grid.is_river_frgmented(x, y, island.id) {
                            continue;
                        }

                        k_grid.reached_white += 1;
                    }

                    // Local update of phermon.

                    let p = self.l_evap;
                    k_phermons[x][y] = (1.0 - p) * k_phermons[x][y] + p * self.evap;

                    // Cell is valid. Update the current ant grid.

                    k_grid.cells[x][y] = island.id;
                    island.size += 1;

                    if island.size >= island.final_size {
                        break;
                    }

                    // Add neighbours to N list.

                    for_valid_neighbours(k_grid.width, k_grid.height, x, y, |a, b| {
                        if k_grid.cells[a][b] == BLACK {
                            queue.push((a, b));
                        }
                    });

                    first = false;
                }
            }

            if k_grid.evaluate(self.solution_num_white) > self.solution.best_p {
                self.solution.clone_from(&k_grid);
                self.phermons.clone_from(&k_phermons);

                self.explain = format!("Found current best solution is {}", self.solution.eval);

                if self.solution.is_solved() {
                    self.explain = format!(
                        "Puzzle solved! ({}/{})",
                        k_grid.reached_white, self.solution_num_white
                    );
                    return Step::SolutionFound;
                }
            }
        }

        // Global phermon update.

        let p = self.g_evap;
        let phermon = self.solution.best_p;

        for x in 0..self.grid.height {
            for y in 0..self.grid.width {
                if self.solution.cells[x][y] != BLACK {
                    self.phermons[x][y] = (1.0 - p) * self.phermons[x][y] + p * phermon;
                }
            }
        }

        // Best value evaporation.
        const BVE: f64 = 0.001;
        self.solution.best_p *= 1.0 - BVE;

        // console_log!("test");
        // for (i, row) in self.phermons.iter().enumerate() {
        //     console_log!("{}= {:.4?}", i, row);
        // }

        Step::Proceed
    }

    fn get_state(&self) -> JsValue {
        let verbose = if self.verbose {
            self.explain.clone()
        } else {
            String::from("")
        };

        let mut data = self.solution.flat();
        const WHITE: i32 = -2;
        const BLACK: i32 = -1;

        data = data
            .into_iter()
            .enumerate()
            .map(|(i, v)| match v {
                1.. => {
                    match self
                        .islands
                        .iter()
                        .find(|r| i == r.pos.0 * self.solution.height + r.pos.1)
                    {
                        Some(island) => island.final_size as i32,
                        None => WHITE,
                    }
                }
                0 => BLACK,
                _ => WHITE,
            })
            .collect();

        serde_wasm_bindgen::to_value(&Nurikabe {
            width: self.solution.width,
            height: self.solution.height,
            solved: self.solution.is_solved(),
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
