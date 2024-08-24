pub mod hash;
mod min_edit;

use hash::hash_with_len;
use min_edit::{min_edit, Transform};

#[cfg(test)]
mod tests;

fn main() {
	let w1: Box<[_]> = include_str!("hello.md").split("\n").collect();
	let w2: Box<[_]> = include_str!("world.md").split("\n").collect();
	let a1: Box<[_]> = w1.iter().map(hash_with_len).collect();
	let a2: Box<[_]> = w2.iter().map(hash_with_len).collect();

	let path = min_edit(a1.as_ref(), a2.as_ref());

	use Transform::*;
	path.into_iter().for_each(|step| match step {
		Keep(idx) => println!("{}", w1[idx]),
		Insert(idx) => println!("{}", w2[idx]),
		Delete(_) => (),
	});
}
