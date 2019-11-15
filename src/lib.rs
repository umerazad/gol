mod utils;
use std::fmt;

use wasm_bindgen::prelude::*;
mod timer;

use timer::Timer;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            _ => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

const HEIGHT: u32 = 128;
const WIDTH: u32 = 128;

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        // Just for debugging purposes.
        utils::set_panic_hook();

        let height = HEIGHT;
        let width = WIDTH;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 9 == 0 {
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

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    /// Drives the life evolution in the game.
    pub fn tick(&mut self) {
        let mut board = self.cells.clone();

        let _timer = Timer::new("Universe::tick");

        for i in 0..self.height {
            for j in 0..self.width {
                let live_neighbors = self.live_neighbor_count(i, j);
                let idx = self.get_index(i, j);
                let cell = self.cells[idx];

                // Commenting out this logging for now as it makes the game
                // very slow.

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     i,
                //     j,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    // Any live cell with fewer than 2 live neighbors dies due
                    // to under population.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Any live cell with exact 2 or three live neighbors lives on.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Any live cell with more than 3 live neighbors dies due to
                    // over population.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Any dead cell with exactly 3 live neighbors comes to life.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells maintain their state.
                    (state, _) => state,
                };

                // log!("       it becomes: {:?}", next_cell);

                board[idx] = next_cell;
            }
        }

        self.cells = board;
    }
}

// Non bindgen implementation stuff
impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (r, c) in cells.iter().cloned() {
            let idx = self.get_index(r, c);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn set_cell(&mut self, row: u32, col: u32, state: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx] = state;
    }

    /// Returns the index of the cell offset in our universe's linear memory.
    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Returns the count of alive cells in this particular cell's neighborhood.
    /// Note: We wrap around the edges.
    pub fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut result = 0;

        for dr in [self.height - 1, 0, 1].iter().cloned() {
            for dc in [self.width - 1, 0, 1].iter().cloned() {
                // ignore the source cell.
                if dr == 0 && dc == 0 {
                    continue;
                }

                let neighbor_row = (row + dr) % self.height;
                let neighbor_col = (col + dc) % self.width;
                result += self.get_cell(neighbor_row, neighbor_col) as u8;
            }
        }

        result
    }

    fn get_cell(&self, row: u32, col: u32) -> Cell {
        let idx = self.get_index(row, col);
        self.cells[idx]
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '.' } else { '+' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        let mut u = Universe::new();
        u.set_width(64);
        u.set_height(64);

        assert_eq!(u.get_index(0, 0), 0);
        assert_eq!(u.get_index(1, 0), 64);
        assert_eq!(u.get_index(1, 1), 65);
        assert_eq!(u.get_index(7, 7), 64 * 7 + 7);
    }
}
