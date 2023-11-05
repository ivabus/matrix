#![recursion_limit = "32768"]

use rand::{thread_rng, Rng};
use std::cmp::Ord;
use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Debug)]
pub struct Matrix<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(Vec<Vec<T>>);

impl<
		T: Mul<Output = T>
			+ Copy
			+ AddAssign
			+ From<u8>
			+ PartialEq
			+ Add
			+ PartialOrd
			+ Ord
			+ Copy
			+ Neg<Output = T>,
	> Matrix<T>
{
	fn len(&self) -> usize {
		self.0.len()
	}

	fn push(&mut self, element: Vec<T>) {
		self.0.push(element)
	}

	fn new() -> Matrix<T> {
		Matrix(Vec::new())
	}

	pub fn dot(&self, other: &Matrix<T>) -> Option<Matrix<T>> {
		if !(check_valid(self) && check_valid(other)) {
			return None;
		}

		if self.0[0].len() != other.0.len() {
			return None;
		}

		let m = self.0[0].len();
		let mut c: Matrix<T> = Matrix::new();

		for i in 0..self.len() {
			c.push(vec![]);
			for j in 0..other.0[0].len() {
				let mut s: T = 0_u8.into();
				for r in 0..m {
					s += self.0[i][r] * other.0[r][j];
				}
				c.0[i].push(s as T);
			}
		}

		Some(c)
	}
}

impl<
		T: Mul<Output = T>
			+ Copy
			+ AddAssign
			+ From<u8>
			+ PartialEq
			+ Add
			+ PartialOrd
			+ Ord
			+ Copy
			+ Neg<Output = T>,
	> PartialEq for Matrix<T>
{
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

fn check_valid<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(
	a: &Matrix<T>,
) -> bool {
	let len = a.0[0].len();

	for i in &a.0 {
		if i.len() != len {
			return false;
		}
	}

	true
}

pub fn sum<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(
	a: &Matrix<T>,
	b: &Matrix<T>,
) -> Option<Matrix<T>> {
	if !(check_valid(a) && check_valid(b)) {
		return None;
	}

	if a.len() != b.len() {
		return None;
	}

	let mut c: Matrix<T> = Matrix::new();
	for i in 0..a.len() {
		c.push(vec![]);
		for j in 0..a.0[0].len() {
			c.0[i].push(a.0[i][j] + b.0[i][j]);
		}
	}

	Some(c)
}

fn get_minor<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(
	base: &&Matrix<T>,
	mi: &usize,
	mj: &usize,
) -> Matrix<T> {
	let mut minor: Matrix<T> = Matrix::new();
	let mut offset: usize = 0;
	for i in 0..base.len() {
		if i == *mi {
			offset += 1;
			continue;
		}
		minor.push(vec![]);
		for j in 0..base.0[0].len() {
			if j == *mj {
				continue;
			}
			minor.0[i - offset].push(base.0[i][j])
		}
	}
	minor
}

pub fn det<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(
	a: &Matrix<T>,
) -> Option<T> {
	if !check_valid(a) {
		return None;
	}
	if a.0[0].len() != a.0.len() {
		return None;
	}
	if a.0[0].len() == 1 {
		return Some(a.0[0][0]);
	}
	let j = 1;
	let mut determinant: T = 0.into();
	for i in 0..a.len() {
		let minor: T = det(&get_minor(&a, &i, &j)).unwrap();
		determinant += if (i + 1) % 2 == 1 {
			-Into::<T>::into(1i8)
		} else {
			Into::<T>::into(1i8)
		} * a.0[i][1]
			* minor;
	}
	Some(determinant)
}
#[allow(dead_code)]
fn gen_matrix<
	T: Mul<Output = T>
		+ Copy
		+ AddAssign
		+ From<u8>
		+ PartialEq
		+ Add
		+ PartialOrd
		+ Ord
		+ Copy
		+ Neg<Output = T>,
>(
	i: usize,
	j: usize,
) -> Option<Matrix<T>> {
	if !(i > 0 && j > 0) {
		return None;
	}

	let mut rng = thread_rng();
	let mut m: Matrix<T> = Matrix::new();

	for a in 0..i {
		m.push(vec![]);
		for _ in 0..j {
			m.0[a].push(T::from(rng.gen_range(0..100)));
		}
	}
	Some(m)
}

#[cfg(test)]
mod test {
	use rand::{thread_rng, Rng};

	use super::*;

	#[test]
	fn basic_sum() {
		let a = vec![vec![1., 2.], vec![3., 4.]];
		let b = vec![vec![5., 6.], vec![7., 8.]];
		assert_eq!(sum(&Matrix(a), &Matrix(b)).unwrap(), Matrix(vec![vec![6., 8.], vec![10., 12.]]))
	}

	#[test]
	fn basic_mul() {
		let a = vec![vec![3., -1., 2.], vec![4., 2., 0.], vec![-5., 6., 1.]];
		let b = vec![vec![8., 1.], vec![7., 2.], vec![2., -3.]];
		assert_eq!(
			mul(&Matrix(a), &Matrix(b)).unwrap(),
			Matrix(vec![vec![21., -5.], vec![46., 8.], vec![4., 4.]])
		)
	}

	#[test]
	fn mv_sum() {
		let mut rng = thread_rng();
		let (s1, s2) = (rng.gen_range(2..500), rng.gen_range(2..500));
		let a: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		let b: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		assert_eq!(sum(&a, &b).unwrap(), sum(&b, &a).unwrap())
	}

	#[test]
	#[should_panic]
	fn mv_mul() {
		let a: Matrix<f64> = gen_matrix(10, 1).unwrap();
		let b: Matrix<f64> = gen_matrix(1, 10).unwrap();
		let ab = mul(&a, &b);
		let ba = mul(&b, &a);
		if ab.is_some() && ba.is_some() {
			assert_eq!(ab.unwrap(), ba.unwrap())
		} else {
			assert!(false)
		}
	}

	#[test]
	fn comb_sum() {
		let mut rng = thread_rng();
		let (s1, s2) = (rng.gen_range(2..500), rng.gen_range(2..500));
		let a: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		let b: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		let c: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		assert_eq!(sum(&sum(&a, &b).unwrap(), &c).unwrap(), sum(&a, &sum(&b, &c).unwrap()).unwrap())
	}

	#[test]
	fn comb_mul() {
		let mut rng = thread_rng();
		let (s1, s2, s3, s4) = (
			rng.gen_range(2..500),
			rng.gen_range(2..500),
			rng.gen_range(2..500),
			rng.gen_range(2..500),
		);
		let a: Matrix<f64> = gen_matrix(s1, s2).unwrap();
		let b: Matrix<f64> = gen_matrix(s2, s3).unwrap();
		let c: Matrix<f64> = gen_matrix(s3, s4).unwrap();
		assert_eq!(mul(&mul(&a, &b).unwrap(), &c).unwrap(), mul(&a, &mul(&b, &c).unwrap()).unwrap())
	}

	#[test]
	fn basic_det_1() {
		let matrix: Matrix<i32> = Matrix(vec![vec![1, -2, -2], vec![5, 1, 3], vec![8, 0, 4]]);
		assert_eq!(det(&matrix).unwrap(), 12i32)
	}
	#[test]
	fn basic_det_2() {
		let matrix: Matrix<i32> =
			Matrix(vec![vec![1, 0, 0, 0], vec![0, 3, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]]);
		assert_eq!(det(&matrix).unwrap(), 3i32)
	}
	#[test]
	fn basic_det_3() {
		let matrix: Matrix<i32> =
			Matrix(vec![vec![0, 0, 0, 1], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![1, 0, 0, 0]]);
		assert_eq!(det(&matrix).unwrap(), -1i32)
	}
	#[test]
	fn basic_det_4() {
		let matrix: Matrix<i32> =
			Matrix(vec![vec![1, 0, 0, 7], vec![0, 1, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 0, 1]]);
		assert_eq!(det(&matrix).unwrap(), 1i32)
	}
	#[test]
	fn basic_det_5() {
		let matrix: Matrix<i32> =
			Matrix(vec![vec![3, 0, 2, -1], vec![1, 2, 0, -2], vec![4, 0, 6, -3], vec![5, 0, 2, 0]]);
		assert_eq!(det(&matrix).unwrap(), 20i32)
	}
}
