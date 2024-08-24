const P: u128 = 257;
pub fn hash(plain: impl AsRef<str>) -> u128 {
	plain.as_ref().as_bytes().iter().fold(0u128, |hash, &byte| {
		hash.wrapping_mul(P).wrapping_add(byte as u128 + 1)
	})
}
