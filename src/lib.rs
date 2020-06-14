mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::std::error::Error;
use core::fmt;
use crate::utils::random_int;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone)]
struct OutOfBoundsError;

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of bounds")
    }
}

#[wasm_bindgen]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[wasm_bindgen]
pub struct Map {
    height: u32,
    width: u32,
    cells: Vec<u32>,
    game_over: bool,
    meat: Option<(u32, u32)>,
}

#[wasm_bindgen]
pub struct MeatPositionTuple(pub u32, pub u32);

impl Map {
    fn get_index(&self, row: u32, column: u32) -> Result<u32, &str> {
        if !(0..self.height).contains(&row) || !(0..self.width).contains(&column) {
            return Result::Err("Out of bounds");
        }
        return Ok((row * self.width) + column);
    }
}

#[wasm_bindgen]
impl Map {
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }

    pub fn meat_position(&self) -> MeatPositionTuple {
        match self.meat {
            Some(row) => {
                MeatPositionTuple(row.0, row.1)
            }
            _ => MeatPositionTuple(0, 0)
        }
    }

    pub fn new() -> Map {
        let height = 50;
        let width = 50;
        let cells: Vec<u32> = (0..height * width)
            .map(|i| -> u32 {
                return if { ((height * width) / 2 + (width / 2)) == i } { 1 } else { 0 };
            })
            .collect();

        return Map {
            height,
            width,
            cells,
            game_over: false,
            meat: None,
        };
    }

    fn set_meat(&self, curr_row: u32, curr_col: u32) -> (u32, u32) {
        return match self.meat {
            Some((row, col)) => {
                if (row == curr_row) & (col == curr_col) {
                    (random_int(self.height), random_int(self.height))
                } else {
                    (row, col)
                }
            }
            None => {
                (random_int(self.height), random_int(self.height))
            }
        };
    }

    pub fn tick(&mut self, direction: Direction) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = match self.get_index(row, col) {
                    Ok(idx) => idx as usize,
                    Err(_error) => {
                        self.game_over = true;
                        panic!("Game over!")
                    }
                };

                let cell = self.cells[idx];
                if cell == 0 { continue; }
                next[idx] = 0;
                match direction {
                    Direction::Left => { self.handle_step(next.as_mut(), row, col - 1) }
                    Direction::Right => { self.handle_step(next.as_mut(), row, col + 1) }
                    Direction::Up => { self.handle_step(next.as_mut(), row - 1, col) }
                    Direction::Down => { self.handle_step(next.as_mut(), row + 1, col) }
                }

                self.meat = Option::from(self.set_meat(row, col));
            }
        }

        self.cells = next;
    }

    fn handle_step(&mut self, next_cells: &mut Vec<u32>, row: u32, column: u32) {
        let idx = match self.get_index(row, column) {
            Ok(idx) => idx as usize,
            Err(_error) => {
                self.game_over = true;
                panic!("Game over!")
            }
        };

        next_cells[idx] = 1
    }
}
