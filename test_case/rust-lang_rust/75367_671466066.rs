rust
fn funny(v: Vec<i32>) -> Vec<i32> {
	v.iter()
		.foldmap(0, |acc, x| acc + x)
                .foldmap(1, |acc, x| acc * x)
		.collect()
}
