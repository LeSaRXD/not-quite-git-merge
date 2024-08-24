use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transform {
	Keep(usize),
	Insert(usize),
	Delete(usize),
}

pub fn min_edit<'a, T: PartialEq>(a: &'a [T], b: &'a [T]) -> Vec<Transform> {
	use Transform::*;

	type Matrix = [Box<[(usize, Transform)]>];

	let mut matrix: Box<Matrix> = (0..=a.len())
		.map(|y| {
			(0..=b.len())
				.map(|x| match (x, y) {
					(0, 0) => (0, Keep(0)),
					(0, y) => (y, Delete(y - 1)),
					(x, 0) => (x, Insert(x - 1)),
					(_, y) => (0, Keep(y - 1)),
				})
				.collect()
		})
		.collect();

	for j in 0..a.len() {
		(0..b.len()).for_each(|i| {
			let px = matrix[j][i + 1].0 + 1;
			let py = matrix[j + 1][i].0 + 1;
			let pxy = matrix[j][i].0;
			matrix[j + 1][i + 1] = match (a[j] == b[i], min(px, py)) {
				(true, _) => (pxy, Keep(j)),
				(false, prev) => (
					prev,
					// if prev == pxy + 2 {
					// 	Substitute(&a[j], &b[i])
					// } else
					if prev == px { Delete(j) } else { Insert(i) },
				),
			};
		});
	}

	let mut transforms = vec![];
	let (mut y, mut x) = (a.len(), b.len());
	while x > 0 || y > 0 {
		let curr = matrix[y][x].1.clone();
		(x, y) = match curr {
			Keep(_) => (x - 1, y - 1),
			Delete(_) => (x, y - 1),
			Insert(_) => (x - 1, y),
		};
		transforms.push(curr);
	}
	transforms.reverse();
	transforms
}
