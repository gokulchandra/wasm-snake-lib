mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Map {
    height: u32,
    width: u32,
    cells: Vec<u32>,
}

impl Map {
    fn get_index(&self, row: u32, column: u32) -> u32 {
        return (row * self.width) + column;
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

    pub fn new() -> Map {
        let height = 50;
        let width = 50;
        let mut cells: Vec<u32> = (0..height * width)
            .map(|i| -> u32 {
                return if { ((height * width) / 2 + (width / 2)) == i } { 1 } else { 0 }
            })
            .collect();

        return Map {
            height,
            width,
            cells,
        };
    }
}
