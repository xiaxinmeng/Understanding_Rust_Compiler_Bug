rust
let v = vec![1,2,3];

// Will result in `7` because .iter() is called twice
let mut vi = v.iter();
v.iter().fold(vi.next().cloned(), |a, i| a.map(|a| a + i));

// Correct version
let mut vi = v.iter();
vi.next().cloned().map(|a| vi.fold(a, Add::add));
