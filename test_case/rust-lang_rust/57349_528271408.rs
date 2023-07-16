
pub const fn context_hash(context: &'static [u8]) -> Sha256 {
	let mut h =	Sha256::new();
	h.input(context);
	let res = h.result_reset();
	h.input(&res);
	h.input(&res);
	h
}
