rust
fn funny(v: Vec<i32>) -> Vec<i32> {
        let mut t1 = 0;
        let mut t2 = 1;
	v.iter()
		.map(|x| {t1 = t1 + x; t1})
		.map(|x| {t2 = t2 * x; t2})
		.collect()
}
