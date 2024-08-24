mod hash {
	use crate::hash::hash;

	#[test]
	fn empty() {
		let h1 = hash("");
		assert_eq!(h1, 0);
	}

	#[test]
	fn null() {
		let h2 = hash("\x00");
		assert_ne!(h2, 0);
	}

	#[test]
	fn a() {
		let h1 = hash("a");
		assert_eq!(h1, 98);
	}

	#[test]
	fn equal() {
		let s1 = "abc";
		assert_eq!(hash(s1), hash("abc"));
	}

	#[test]
	fn order() {
		assert_ne!(hash("acb"), hash("abc"));
	}
}

mod min_edit {
	use crate::min_edit::{min_edit, Transform::*};
	#[test]
	fn empty() {
		let a1: &[i32] = &[];
		let a2: &[i32] = &[];
		assert!(min_edit(a1, a2).is_empty());
	}

	#[test]
	fn keep() {
		let a1 = &[10, 20, 30, 40];
		let a2 = &[10, 20, 30, 40];
		assert_eq!(min_edit(a1, a2)[..], [Keep(0), Keep(1), Keep(2), Keep(3)]);
	}

	#[test]
	fn insert_delete() {
		let a1 = &[];
		let a2 = &[10, 20];
		assert_eq!(min_edit(a1, a2)[..], [Insert(0), Insert(1)]);
		assert_eq!(min_edit(a2, a1)[..], [Delete(0), Delete(1)]);
	}

	#[test]
	fn mixed() {
		// consecutive `Insert`s should always be prioritized over `Delete`s
		let a1 = &[1, 2, 3, 4];
		let a2 = &[1, 5, 6, 4];
		assert_eq!(
			min_edit(a1, a2)[..],
			[Keep(0), Insert(1), Insert(2), Delete(1), Delete(2), Keep(3)]
		);

		// but if separated, they should have a `Keep` in between the two sets
		let a1 = &[1, 2, 42, 3, 4];
		let a2 = &[1, 5, 42, 6, 4];
		assert_eq!(
			min_edit(a1, a2)[..],
			[
				Keep(0),
				Insert(1),
				Delete(1),
				Keep(2),
				Insert(3),
				Delete(3),
				Keep(4)
			]
		);
	}

	#[test]
	fn strings() {
		// from https://web.stanford.edu/class/cs124/lec/med.pdf
		let s1 = "intention".chars().collect::<Box<[_]>>();
		let s2 = "execution".chars().collect::<Box<[_]>>();
		assert_eq!(
			min_edit(s1.as_ref(), s2.as_ref())[..],
			[
				Insert(0),
				Insert(1),
				Delete(0),
				Delete(1),
				Delete(2),
				Keep(3),
				Insert(3),
				Insert(4),
				Delete(4),
				Keep(5),
				Keep(6),
				Keep(7),
				Keep(8)
			]
		)
	}

	#[test]
	fn index_order() {
		// indices of `Keep`s and `Delete`s should be a sequence 0..n (n is length of start string)
		let s1 = "33tjqa9g jt jterithbi krdps[".chars().collect::<Box<[_]>>();
		let s2 = "4eujtj3rjbt r3 %#$#s 4sfhjsr".chars().collect::<Box<[_]>>();

		let mut next_expected: usize = 0;
		for step in min_edit(s1.as_ref(), s2.as_ref()) {
			let curr = match step {
				Keep(idx) => idx,
				Delete(idx) => idx,
				Insert(_) => continue,
			};
			assert_eq!(curr, next_expected);
			next_expected += 1;
		}
	}
}
