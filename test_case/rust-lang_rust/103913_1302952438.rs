rs
fn samples() -> Vec<(i32, i32)> {
	let n = 8000;
    let mut all = (-n..=n)
        .flat_map(|n| (-n..=n).map(move |d| (n, d)))
        .filter(|(n, d)| n.checked_rem(*d).is_some())
        .collect::<Vec<_>>();
    let mut r = SmallRng::from_seed(SEED);
    all.shuffle(&mut r);
    all
}
