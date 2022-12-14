mod utils;

extern crate web_sys;

use wasm_bindgen::prelude::*;
use std::fmt;
use web_sys::console;

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

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

pub struct Timer<'a> {
	name: &'a str,
}

impl<'a> Timer<'a> {
	pub fn new(name: &'a str) -> Timer<'a> {
		console::time_with_label(name);
		Timer { name }
	}
}

impl<'a> Drop for Timer<'a> {
	fn drop(&mut self) {
		console::time_end_with_label(self.name);
	}
}

impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	// XXX: 何してるかよくわかってない
	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);

				count += self.cells[idx] as u8;
			}
		}
		
		count
	}
	
	pub fn get_cells(&self) -> &[Cell] {
		&self.cells
	}
	
	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
		for (row, col) in cells.iter().cloned() {
			let idx = self.get_index(row, col);
			self.cells[idx] = Cell::Alive;
		}
	}
}

#[wasm_bindgen]
impl Universe {
	pub fn new() -> Universe {
		utils::set_panic_hook();

		let width = 128;
		let height = 128;

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
	
	pub fn width(&self) -> u32 {
		self.width
	}
	
	pub fn set_width(&mut self, width: u32) {
		self.width = width;
		self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
	}

	pub fn set_height(&mut self, height: u32) {
		self.height = height;
		self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
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

	pub fn tick(&mut self) {
		let _timer = Timer::new("Universe::tick");
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_neighbors = self.live_neighbor_count(row, col);

				let next_cell = match (cell, live_neighbors) {
					// 過疎
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					// 生存
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					// 過密
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					// 誕生
					(Cell::Dead, 3) => Cell::Alive,
					// その他のセルはそのままの状態になる
					(otherwise, _) => otherwise,
				};

				next[idx] = next_cell;
			}
		}

		self.cells = next;
	}
}
 impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}

		Ok(())
	}
 }
