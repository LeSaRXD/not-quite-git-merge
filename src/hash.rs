const P: u128 = 257;
pub fn hash(plain: impl AsRef<str>) -> u128 {
	plain.as_ref().as_bytes().iter().fold(0u128, |hash, &byte| {
		hash.wrapping_mul(P).wrapping_add(byte as u128 + 1)
	})
}

// since `len()` is constant time, we can add it to avoid intersections without a big performance hit
pub fn hash_with_len(plain: impl AsRef<str>) -> (u128, usize) {
	let len = plain.as_ref().len();
	(hash(plain), len)
}
