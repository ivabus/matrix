// AUGGGGHHHHHHHHHHHHH
// I WANT TO USE GENERICS
// BUT I DON'T KNOW HOW TO USE THEM PROPERLY

use rand::{Rng, thread_rng};

fn check_valid(a: &Vec<Vec<f64>>) -> bool {
	let len = a[0].len();

	for i in a {
		if i.len() != len {
			return false;
		}
	}

	true
}

pub fn sum(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
	if !(check_valid(&a) && check_valid(&b)) {
		return None;
	}

	if a.len() != b.len() {
		return None;
	}

	let mut c: Vec<Vec<f64>> = Vec::new();
	for i in 0..a.len() {
		c.push(vec![]);
		for j in 0..a[0].len() {
			c[i].push(a[i][j] + b[i][j]);
		}
	}

	Some(c)
}

pub fn mult(a: &Vec<Vec<f64>>, b: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
	if !(check_valid(&a) && check_valid(&b)) {
		return None;
	}

	if a[0].len() != b.len() {
		return None;
	}

	let m = a[0].len();
	let mut c: Vec<Vec<f64>> = Vec::new();

	for i in 0..a.len() {
		c.push(vec![]);
		for j in 0..b[0].len() {
			let mut s = 0.;
			for r in 0..m {
				s += a[i][r] * b[r][j];
			}
			c[i].push(s as f64);
		}
	}

	Some(c)
}

#[allow(dead_code)]
fn gen_matrix(i: usize, j: usize) -> Option<Vec<Vec<f64>>> {
	if !(i > 0 && j > 0) {
		return None;
	}

	let mut rng = thread_rng();
	let mut m: Vec<Vec<f64>> = Vec::new();

	for a in 0..i {
		m.push(vec![]);
		for _ in 0..j {
			m[a].push(rng.gen_range(-100..100) as f64);
		}
	}
	Some(m)
}

#[cfg(test)]
mod test {
	use rand::{Rng, thread_rng};

	#[test]
	fn basic_sum() {
		let a = vec![vec![1., 2.], vec![3., 4.]];
		let b = vec![vec![5., 6.], vec![7., 8.]];
		assert_eq!(crate::sum(&a, &b).unwrap(), vec![vec![6., 8.], vec![10., 12.]])
	}

	#[test]
	fn basic_mult() {
		let a = vec![vec![3., -1., 2.], vec![4., 2., 0.], vec![-5., 6., 1.]];
		let b = vec![vec![8., 1.], vec![7., 2.], vec![2., -3.]];
		assert_eq!(crate::mult(&a, &b).unwrap(), vec![vec![21., -5.], vec![46., 8.], vec![4., 4.]])
	}

	#[test]
	fn mv_sum() {
		let mut rng = thread_rng();
		let (s1, s2) = (rng.gen_range(2..500), rng.gen_range(2..500));
		let a = crate::gen_matrix(s1, s2).unwrap();
		let b = crate::gen_matrix(s1, s2).unwrap();
		assert_eq!(crate::sum(&a, &b).unwrap(), crate::sum(&b, &a).unwrap())
	}

	#[test]
	#[ignore]
	fn mv_mult() {
		let a = crate::gen_matrix(10, 1).unwrap();
		let b = crate::gen_matrix(1, 10).unwrap();
		let ab = crate::mult(&a, &b);
		let ba = crate::mult(&b, &a);
		if ab != None && ba != None {
			assert!(ab.unwrap() == ba.unwrap())
		} else {
			assert!(false)
		}
	}

	#[test]
	fn comb_sum() {
		let mut rng = thread_rng();
		let (s1, s2) = (rng.gen_range(2..500), rng.gen_range(2..500));
		let a = crate::gen_matrix(s1, s2).unwrap();
		let b = crate::gen_matrix(s1, s2).unwrap();
		let c = crate::gen_matrix(s1, s2).unwrap();
		assert_eq!(
			crate::sum(&crate::sum(&a, &b).unwrap(), &c).unwrap(),
			crate::sum(&a, &crate::sum(&b, &c).unwrap()).unwrap()
		)
	}

	#[test]
	fn comb_mult() {
		let mut rng = thread_rng();
		let (s1, s2, s3, s4) = (
			rng.gen_range(2..500),
			rng.gen_range(2..500),
			rng.gen_range(2..500),
			rng.gen_range(2..500),
		);
		let a = crate::gen_matrix(s1, s2).unwrap();
		let b = crate::gen_matrix(s2, s3).unwrap();
		let c = crate::gen_matrix(s3, s4).unwrap();
		assert_eq!(
			crate::mult(&crate::mult(&a, &b).unwrap(), &c).unwrap(),
			crate::mult(&a, &crate::mult(&b, &c).unwrap()).unwrap()
		)
	}
}
