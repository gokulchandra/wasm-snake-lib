mod utils;

use crate::utils::random_int;
use core::fmt;
use wasm_bindgen::prelude::*;

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
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

#[wasm_bindgen]
pub struct Map {
    height: u32,
    width: u32,
    cells: Vec<u32>,
    game_over: bool,
    meat: Option<(u32, u32)>,
    snake: Vec<SnakeCell>,
    score: u32
}

#[wasm_bindgen]
pub struct SnakeCell {
    position: (u32, u32),
    direction: Option<Direction>,
    turn: Option<Direction>,
}

impl fmt::Debug for SnakeCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Row: {:?} \n  Col: {:?} \n direction: {:?} \n Turn: {:?} \n\n", self.position.0, self.position.1, self.direction, self.turn)
    }
}


#[wasm_bindgen]
pub struct MeatPositionTuple(pub u32, pub u32);

impl Map {
    fn get_index(&self, row: u32, column: u32) -> Result<u32, &str> {
        if !(0..self.height).contains(&row) || !(0..self.width).contains(&column) {
            log!("row {} col {}", row, column);
            return Result::Err("Out of bounds");
        }
        return Ok((row * self.width) + column);
    }
}

#[wasm_bindgen]
impl Map {
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_ptr()
    }

    pub fn snake(&self) -> *const SnakeCell {
        self.snake.as_ptr()
    }

    pub fn meat_position(&self) -> MeatPositionTuple {
        match self.meat {
            Some(row) => MeatPositionTuple(row.0, row.1),
            _ => MeatPositionTuple(0, 0),
        }
    }

    pub fn new() -> Map {
        let height = 50;
        let width = 50;
        let mut snake_cell = SnakeCell {
            position: (26, 25),
            direction: None,
            turn: None,
        };
        let cells: Vec<u32> = (0..height * width)
            .map(|i| -> u32 {
                return match i {
                    1325 => 1,
                    _ => 0
                };
            })
            .collect();

        return Map {
            height,
            width,
            cells,
            game_over: false,
            meat: None,
            snake: vec![snake_cell],
            score: 0
        };
    }

    fn should_turn(&self, current_cell: &SnakeCell, next_snake_cell: Option<&SnakeCell>) -> Option<Direction> {
        return match next_snake_cell {
            None => None,
            Some(next_cell) => {
                match (current_cell.direction, next_cell.direction) {
                    (Some(a), Some(b)) => {
                        if a != b {
                            log!("Turning from {:?} -> {:?}", a, b);
                            next_cell.direction
                        } else { None }
                    }
                    _ => None
                }
            }
        };
    }

    pub fn tick(&mut self, direction: Direction) {
        let mut next_cells = self.cells.clone();
        let mut next_snake: Vec<SnakeCell> = vec![];

        log!("{:?}", self.snake);

        for snake_cell in self.snake.as_slice() {
            let row = snake_cell.position.0;
            let column = snake_cell.position.1;
            let idx = match self.get_index(row, column) {
                Ok(idx) => idx as usize,
                Err(_error) => {
                    panic!("Game over!")
                }
            };

            next_cells[idx] = 0;

            let dir = match next_snake.len() > 0 {
                true => snake_cell.direction.unwrap(),
                false => direction
            };

            let (next_row, next_column) = self.handle_step(next_cells.as_mut(), row, column, dir);

            let next_direction = match snake_cell.turn {
                Some(turn) => turn,
                None => match next_snake.len() > 0 {
                    true => snake_cell.direction.unwrap(),
                    false => direction
                }
            };

            next_snake.push(SnakeCell {
                position: (next_row, next_column),
                direction: Some(next_direction),
                turn: self.should_turn(snake_cell, next_snake.last()),
            });

            match self.meat {
                Some((meat_row, meat_column)) => {
                    if (meat_row == row) & (meat_column == column) & next_snake.len().eq(&(1 as usize)) {
                        self.score = self.score + 1;
                        self.meat = Option::from((random_int(self.height), random_int(self.height)));
                        next_snake.push(SnakeCell {
                            position: (row, column),
                            direction: Some(direction),
                            turn: None,
                        });
                        next_cells[idx] = 1;
                    }
                }
                None => {
                    self.meat = Option::from((random_int(self.height), random_int(self.width)));
                }
            }
        }
        self.cells = next_cells;
        self.snake = next_snake;
    }

    fn handle_step(&self, next_cells: &mut Vec<u32>, row: u32, column: u32, direction: Direction) -> (u32, u32) {
        let (next_row, next_col) = match direction {
            Direction::Left => (row, column - 1),
            Direction::Right => (row, column + 1),
            Direction::Up => (row - 1, column),
            Direction::Down => (row + 1, column),
        };

        let idx = match self.get_index(next_row, next_col) {
            Ok(idx) => idx as usize,
            Err(_error) => {
                panic!("Game over!")
            }
        };

        next_cells[idx] = 1;
        return (next_row, next_col);
    }
}
