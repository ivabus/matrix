use std::ops::{Add, AddAssign, Mul};

use rand::{thread_rng, Rng};

pub(crate) type Matrix<T> = Vec<Vec<T>>;

fn check_valid<T>(a: &Matrix<T>) -> bool {
	let len = a[0].len();

	for i in a {
		if i.len() != len {
			return false;
		}
	}

	true
}

pub fn sum<T: Add<Output = T> + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Option<Matrix<T>> {
	if !(check_valid(a) && check_valid(b)) {
		return None;
	}

	if a.len() != b.len() {
		return None;
	}

	let mut c: Matrix<T> = Vec::new();
	for i in 0..a.len() {
		c.push(vec![]);
		for j in 0..a[0].len() {
			c[i].push(a[i][j] + b[i][j]);
		}
	}

	Some(c)
}

pub fn mul<T: Mul<Output = T> + Copy + AddAssign + From<u8>>(
	a: &Matrix<T>,
	b: &Matrix<T>,
) -> Option<Matrix<T>> {
	if !(check_valid(a) && check_valid(b)) {
		return None;
	}

	if a[0].len() != b.len() {
		return None;
	}

	let m = a[0].len();
	let mut c: Matrix<T> = Vec::new();

	for i in 0..a.len() {
		c.push(vec![]);
		for j in 0..b[0].len() {
			let mut s: T = 0_u8.into();
			for r in 0..m {
				s += a[i][r] * b[r][j];
			}
			c[i].push(s as T);
		}
	}

	Some(c)
}

pub fn det<T: Mul<Output = T> + Copy + Add>(a: &Matrix<T>) -> Option<Matrix<T>> {
	if ! check_valid(a) {
		return None;
	}
	if a[0].len() != a.len() {
		return None;
	}
	let j = 0;
	for i in 1..n {
		let _pre_minor: Matrix<T> = vec![&a[0..i], &a[i+1..]];
		let mut pre_minor: Metrix<T> = vec![vec![]];
		for m in _pre_minor {
			if m == j {
				continue;
			}
			pre_minor[m].push
		}
		let minor = det(&pre_minor);

	}
}
#[allow(dead_code)]
fn gen_matrix<T: From<i32>>(i: usize, j: usize) -> Option<Matrix<T>> {
	if !(i > 0 && j > 0) {
		return None;
	}

	let mut rng = thread_rng();
	let mut m: Matrix<T> = Vec::new();

	for a in 0..i {
		m.push(vec![]);
		for _ in 0..j {
			m[a].push(T::from(rng.gen_range(0..100)));
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
		assert_eq!(sum(&a, &b).unwrap(), vec![vec![6., 8.], vec![10., 12.]])
	}

	#[test]
	fn basic_mul() {
		let a = vec![vec![3., -1., 2.], vec![4., 2., 0.], vec![-5., 6., 1.]];
		let b = vec![vec![8., 1.], vec![7., 2.], vec![2., -3.]];
		assert_eq!(mul(&a, &b).unwrap(), vec![vec![21., -5.], vec![46., 8.], vec![4., 4.]])
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
	#[ignore]
	fn mv_mul() {
		let a: Matrix<f64> = gen_matrix(10, 1).unwrap();
		let b: Matrix<f64> = gen_matrix(1, 10).unwrap();
		let ab = mul(&a, &b);
		let ba = mul(&b, &a);
		if ab != None && ba != None {
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
}
