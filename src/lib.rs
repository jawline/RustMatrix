use std::vec::Vec;

pub struct Matrix<T> {
	data: Vec<T>,
	pub cols: usize,
	pub rows: usize
}

impl<T: Copy> Matrix<T> {
	pub fn new(cols: usize, rows: usize, initial: T) -> Matrix<T> {
		let data = (0..rows*cols).map(|_| initial.clone()).collect();
		
		Matrix {
			cols: cols,
			rows: rows,
			data: data
		}
	}

	pub fn get(&self, col: usize, row: usize) -> T {
		self.data[(row * self.cols) + col]
	}

	pub fn set(&mut self, col: usize, row: usize, val: T) {
		self.data[(row * self.cols) + col] = val;
	}
}