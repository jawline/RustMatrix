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

	/**
	 * Expand or contract the matrix, filling any new slots with initial
	 */
	pub fn resize(&self, new_cols: usize, new_rows: usize, initial: T) -> Matrix<T> {
		let mut data: Vec<T> = (0..new_rows*new_cols).map(|_| initial.clone()).collect();

		for x in 0..self.cols {
			for y in 0..self.rows {
				data[(y * new_cols) + x] = self.get(x, y);
			}
		}

		Matrix {
			cols: new_cols,
			rows: new_rows,
			data: data
		}
	}
}