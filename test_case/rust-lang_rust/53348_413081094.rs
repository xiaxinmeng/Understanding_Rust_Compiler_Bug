rust
fn main() {
	let v = vec!["hello", "this", "is", "a", "test"];

	let mut v2: Vec<String> = Vec::new();
	
	let owned = v.into_iter().map(|s|s.to_owned()).collect::<Vec<_>>();
	
	for i in owned {
		v2.push(i);
	}
}
