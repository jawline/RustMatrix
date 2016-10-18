use std::vec::Vec;

pub struct Matrix<T> {
	data: Vec<T>,
	pub cols: usize,
	pub rows: usize
}

impl<T: Copy> Matrix<T> {

	pub fn new(rows: usize, cols: usize, initial: T) -> Matrix<T> {
		let data = (0..rows*cols).map(|_| initial.clone()).collect();
		
		Matrix {
			cols: cols,
			rows: rows,
			data: data
		}
	}

	pub fn with_data(rows: usize, cols: usize, initial: T, old_data: &Vec<T>, data_rows: usize, data_cols: usize) -> Matrix<T> {
		let mut data: Vec<T> = (0..rows * cols).map(|_| initial.clone()).collect();

		for row in 0..data_rows {
			for col in 0..data_cols {
				data[(row * cols) + col] = old_data[(row * data_cols) + col];
			}
		}

		return Matrix {
			data: data,
			rows: rows,
			cols: cols
		}
	}

	pub fn get(&self, row: usize, col: usize) -> T {
		self.data[(row * self.cols) + col]
	}

	pub fn set(&mut self, row: usize, col: usize, val: T) {
		self.data[(row * self.cols) + col] = val;
	}

	/**
	 * Expand or contract the matrix, filling any new slots with initial
	 */
	pub fn resize(&self, new_rows: usize, new_cols: usize, initial: T) -> Matrix<T> {
		Matrix::with_data(new_rows, new_cols, initial, &self.data, self.rows, self.cols)
	}
}