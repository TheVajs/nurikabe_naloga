// use rayon::iter::IntoParallelIterator;
// use rayon::iter::ParallelIterator;
use std::collections::{HashSet, VecDeque};
use wasm_bindgen::JsValue;
use web_sys::js_sys::Math::sqrt;

use super::*;

const BLACK: i32 = 0;

#[derive(Debug, Clone)]
struct Island {
    id: i32,
    enclosed: bool,
    pos: (usize, usize),
    size: usize,
    final_size: usize,
}

impl Island {
    fn new(id: i32, pos: (usize, usize), size: usize, final_size: usize) -> Self {
        Self {
            id,
            enclosed: false,
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

    /// Checks if there is a a possible connection with a different island.
    ///
    fn is_connecting_islands(&self, x: usize, y: usize, island: &Island) -> bool {
        for_none_of_neibhbours(self.width, self.height, x, y, |a, b| -> bool {
            let sample = self.cells[a][b];
            sample != BLACK && sample != island.id
        })
    }

    /// Easy test first, which checks for specific situation, when a fragmentation
    /// of the river can occur (river splits in two separate parts). If a situation
    /// is found, than one of the possible start positions for a deapth first search
    /// (DFS) check is returned.
    ///
    fn is_river_frgmented(&mut self, x: usize, y: usize, island: &mut Island) -> bool {
        if let Some(start) = self.cut_creates_frgments(x, y, island) {
            assert_eq!(self.cells[start.0][start.1], BLACK, "Problem!");

            // Set white.
            self.cells[x][y] = island.id;
            self.reached_white += 1;

            let reached_black_cells = self.dfs(start);
            let num_black_cells = self.height * self.width - self.reached_white;

            let in_fragments = reached_black_cells != num_black_cells;

            // Restore.
            self.cells[x][y] = BLACK;
            self.reached_white -= 1;

            in_fragments
        } else {
            false
        }
    }

    /// Checks for ossible situation when cuts in the river can make isolated
    /// rivers.
    ///
    fn cut_creates_frgments(
        &self,
        x: usize,
        y: usize,
        island: &mut Island,
    ) -> Option<(usize, usize)> {
        let mut neighbours: [bool; 8] = [false; 8];
        let mut num = 0;

        for_valid_neighbours_with_outside(self.width, self.height, x, y, |a, b| {
            neighbours[num] = self.cells[a][b] > 0;
            num += 1;
        });

        // Corner case. Literally. :)
        if num == 3 {
            return None;
        }

        let mut prev = None;
        let skips = neighbours[..num].iter().fold(0, |mut s, &cell| {
            if let Some(pre) = prev {
                if pre != cell {
                    s += 1;
                }
            }
            prev = Some(cell);
            s
        });

        if skips > 2 || island.enclosed {
            island.enclosed = true;

            let mut diagonal = vec![];

            for_valid_diagonal_neighbours(self.width, self.height, x, y, |a, b| {
                diagonal.push((a, b));
            });

            for (a, b) in diagonal.into_iter() {
                if self.cells[a][y] == BLACK {
                    return Some((a, y));
                } else if self.cells[x][b] == BLACK {
                    return Some((x, b));
                }
            }
        }

        None
    }

    fn dfs(&self, start: (usize, usize)) -> usize {
        let mut queue = VecDeque::new();
        let mut reached = HashSet::new();
        let mut num_black = 0;

        queue.push_front(start);
        reached.insert(start);

        while let Some((x1, y1)) = queue.pop_front() {
            num_black += 1;

            for_valid_neighbours(self.width, self.height, x1, y1, |a, b| {
                if self.cells[a][b] == BLACK && !reached.contains(&(a, b)) {
                    queue.push_front((a, b));
                    reached.insert((a, b));
                }
            });
        }

        num_black
    }

    /// Adds valid neighbour to N quee, where based on phermon value the next
    /// cell is selected.
    ///
    fn add_neighbours(
        &self,
        x: usize,
        y: usize,
        n: &mut Vec<(usize, usize)>,
        set: &mut HashSet<(usize, usize)>,
    ) {
        let list = [
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
            (x + 1, y),
        ];

        let start = random_int(0..4);
        for i in 0..4 {
            let (a, b) = list[(start + i) % 4];
            if a < self.height
                && b < self.width
                && self.cells[a][b] == BLACK
                && !set.contains(&(a, b))
            {
                n.push((a, b));
            }
        }
    }

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
    path: String,
    ants: usize,
    l_evap: f64,
    g_evap: f64,
    evap: f64,
    greedines: f64,
    bve: f64,
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
        bve: f64,
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
            path: nurikabe.path,
            ants,
            evap: evap.clamp(0.0, 1.0),
            l_evap: local_evap.clamp(0.0, 1.0),
            g_evap: global_evap.clamp(0.0, 1.0),
            greedines: greedines.clamp(0.0, 1.0),
            bve: bve.clamp(0.001, 1.0),
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
        let mut results = (0..self.ants)
            .map(|_| {
                let mut islands = self.islands.clone();
                let mut k_grid = self.grid.clone();
                let mut k_phermons = self.phermons.clone();
                let mut k_set = HashSet::new();

                k_grid.reached_white = islands.len();

                // islands.sort_by_key(|island| island.final_size);

                while !islands.is_empty() {
                    let mut island = islands.remove(random_int(0..islands.len()));
                    let mut queue = vec![island.pos];
                    let mut first = true;

                    while !queue.is_empty() {
                        // Phermon strategy.

                        let (x, y) = if first {
                            queue.remove(0)
                        } else {
                            // Pick base on greedines.

                            let s = if random_float() < self.greedines {
                                // Pick max phermon position.

                                let mut pick = 0.0;
                                let mut index = 0;
                                for (i, &(a, b)) in queue.iter().enumerate() {
                                    if k_phermons[a][b] > pick {
                                        pick = k_phermons[a][b];
                                        index = i;
                                    }
                                }
                                index
                            } else {
                                // Pick roulette.

                                let r = random_float();
                                let sum: f64 = queue.iter().map(|&(a, b)| k_phermons[a][b]).sum();

                                let mut acc = 0.0;
                                let accumilate = queue
                                    .iter()
                                    .map(|&(a, b)| {
                                        acc += k_phermons[a][b] / (sum + 1e-10);
                                        acc
                                    })
                                    .collect::<Vec<f64>>();

                                accumilate
                                    .into_iter()
                                    .position(|prob| r < prob)
                                    .unwrap_or(queue.len() - 1)
                            };

                            let (x, y) = queue.remove(s);
                            if k_grid.cells[x][y] != BLACK {
                                continue;
                            }

                            if k_grid.is_connecting_islands(x, y, &island)
                                || k_grid.is_river_frgmented(x, y, &mut island)
                            {
                                continue;
                            }

                            k_grid.reached_white += 1;

                            (x, y)
                        };

                        // Cell is valid. Update the current ant grid.

                        let dist = {
                            let xdis = island.pos.0 - x;
                            let ydis = island.pos.1 - y;
                            let dist = sqrt((xdis * xdis + ydis * ydis) as f64);
                            1.0 - 1.0 / (dist - 0.8).exp()
                        };

                        let p = self.l_evap;

                        k_phermons[x][y] = (1.0 - p) * (k_phermons[x][y]) + p * (self.evap * dist);
                        k_grid.cells[x][y] = island.id;

                        island.size += 1;
                        if island.size >= island.final_size {
                            queue.clear();
                            break;
                        }

                        k_grid.add_neighbours(x, y, &mut queue, &mut k_set);

                        for_valid_diagonal_neighbours(k_grid.width, k_grid.height, x, y, |a, b| {
                            if k_grid.cells[a][b] != BLACK {
                                island.enclosed = true;
                            }
                        });

                        first = false;
                    }
                }
                (k_grid, k_phermons)
            })
            .collect::<Vec<_>>();

        for (k_grid, k_phermons) in results.iter_mut() {
            self.iteration += 1;

            if k_grid.evaluate(self.solution_num_white) > self.solution.best_p {
                self.solution.clone_from(k_grid);
                self.phermons.clone_from(k_phermons);

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

        let h = random_int(0..self.solution.height);
        let w = random_int(0..self.solution.width);
        self.phermons[h][w] = 1.0 / (self.solution.height * self.solution.width) as f64;

        // Best value evaporation.
        self.solution.best_p *= 1.0 - self.bve;

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
                        .find(|r| i == r.pos.0 * self.solution.width + r.pos.1)
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
            path: self.path.clone(),
            width: self.solution.width,
            height: self.solution.height,
            solved: self.solution.is_solved(),
            iteration: self.iteration,
            data,
            duration: 0,
            verbose,
        })
        .unwrap()
    }

    fn get_iteration(&self) -> usize {
        self.iteration
    }
}
