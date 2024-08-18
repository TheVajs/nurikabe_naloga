use std::collections::{HashSet, VecDeque};
use wasm_bindgen::JsValue;

use super::*;

const BLACK: i32 = 0;

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

    /// Checks if there is a a possible connection with a different island.
    ///
    fn is_connecting_islands(&self, x: usize, y: usize, island_id: i32) -> bool {
        for_none_of_neibhbours(self.width, self.height, x, y, |a, b| -> bool {
            let sample = self.cells[a][b];
            sample != BLACK && sample != island_id
        })
    }

    /// Easy test first, which checks for specific situation, when a fragmentation
    /// of the river can occur (river splits in two separate parts). If a situation
    /// is found, than one of the possible start positions for a deapth first search
    /// (DFS) check is returned.
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
    /// - diagonaly touching islands/rivers
    /// - potential island (x, y) placed on grid border.
    ///
    /// All other situation are taken care by the previous steps.
    ///
    fn creates_wall_cut(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let living_on_edge = x == self.height - 1 || x == 0 || y == self.width - 1 || y == 0;
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

        let mut neighbours = vec![];
        for_valid_diagonal_neighbours(self.width, self.height, x, y, |a, b| {
            if self.cells[a][b] != BLACK {
                neighbours.push((a, b))
            }
        });

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
pub struct RandomAntSolver {
    path: String,
    ants: usize,
    grid: Grid,
    solution: Grid,
    solution_num_white: usize,
    islands: Vec<Island>,
    iteration: usize,
    explain: String,
    pub verbose: bool,
}

impl RandomAntSolver {
    pub fn new(ants: usize, nurikabe: Nurikabe) -> Self {
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

        Self {
            path: nurikabe.path,
            ants,
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

impl Solver for RandomAntSolver {
    fn solve(&mut self) -> Step {
        for _ in 0..self.ants {
            self.iteration += 1;

            let mut islands = self.islands.clone();
            let mut k_grid = self.grid.clone();

            k_grid.reached_white = islands.len();

            while !islands.is_empty() {
                let mut island = islands.remove(random_int(0..islands.len()));
                let mut queue = vec![island.pos];
                let mut first = true;

                while !queue.is_empty() {
                    // Random strategy
                    let (x, y) = queue.remove(random_int(0..queue.len()));

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

        // No phermons to update.

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