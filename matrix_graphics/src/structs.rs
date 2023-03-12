#[derive(Copy, Clone)]
pub struct Line {
	pub start_x: f64,
	pub start_y: f64,
	pub end_x: f64,
	pub end_y: f64,
	pub rotate_angle: f64,
	pub rotate_center_x: f64,
	pub rotate_center_y: f64,
}

impl Line {
	pub fn new(
		start_x: f64,
		start_y: f64,
		end_x: f64,
		end_y: f64,
		rotate_angle: f64,
		rotate_center_x: f64,
		rotate_center_y: f64,
	) -> Line {
		Line {
			start_x,
			start_y,
			end_x,
			end_y,
			rotate_angle,
			rotate_center_x,
			rotate_center_y,
		}
	}
	pub fn get_points(&self) -> bresenham::Bresenham {
		bresenham::Bresenham::new(
			(self.start_x as isize, self.start_y as isize),
			(self.end_x as isize, self.end_y as isize),
		)
	}
	fn rotate(&mut self) {
		let matrix_rotate = vec![
			vec![self.rotate_angle.cos(), -self.rotate_angle.sin()],
			vec![self.rotate_angle.sin(), self.rotate_angle.cos()],
		];
		let matrix_start = vec![
			vec![(self.start_x - self.rotate_center_x)],
			vec![(self.start_y - self.rotate_center_y)],
		];
		let matrix_end = vec![
			vec![(self.end_x - self.rotate_center_x)],
			vec![(self.end_y - self.rotate_center_y)],
		];
		let res_start = matrix::mult(&matrix_rotate, &matrix_start).unwrap();
		let res_end = matrix::mult(&matrix_rotate, &matrix_end).unwrap();
		(self.start_x, self.start_y) =
			(res_start[0][0] + self.rotate_center_x, res_start[1][0] + self.rotate_center_y);
		(self.end_x, self.end_y) =
			(res_end[0][0] + self.rotate_center_x, res_end[1][0] + self.rotate_center_y);
	}

	pub fn update(&mut self) {
		if self.rotate_angle != 0. {
			self.rotate();
		}
	}
}
