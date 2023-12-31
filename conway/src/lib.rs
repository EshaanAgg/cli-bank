mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// extern crate web_sys;

// // A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
// It is a special tiny allocator designed for WebAssembly.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // External JavaScript window.alert function
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}! Nice to meet you!", name));
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    // The cells are represented as a one-dimensional array in the memory
    cells: Vec<Cell>,
}

// Public methods are exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    // Get the index of the cell in the memory given it's row and column
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    // Returns the live neighbour count of a cell, assuming that the top-bottom and left-right connected
    fn live_neighbor_count(&self, row: usize, column: usize) -> u8 {
        let mut count = 0;

        // Instead of using -1 in delta, we use (+x -1) to avoid negative's while taking the modulo
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u8;
            }
        }

        // log!(
        //     "cell[{}, {}] is initially {:?} and has {} live neighbors.",
        //     row,
        //     column,
        //     self.cells[self.get_index(row, column)],
        //     count
        // );

        count
    }

    // Simulate one tick of the time of the Universe
    pub fn tick(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (self.cells[idx], live_neighbors) {
                    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Any live cell with two or three live neighbours lives on to the next generation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Any live cell with more than three live neighbours dies, as if by overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state
                    (org, _) => org,
                };

                self.cells[idx] = next_cell;
            }
        }
    }

    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn pattern_new(width: usize, height: usize) -> Universe {
        // Used for debugging purposes; set at a common path
        utils::set_panic_hook();

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn new(width: usize, height: usize) -> Universe {
        // Used for debugging purposes; set at a common path
        utils::set_panic_hook();

        let cells = vec![Cell::Dead; width * height];

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for &(row, col) in cells {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '0',
                    Cell::Alive => '1',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
