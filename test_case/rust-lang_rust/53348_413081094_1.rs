rust
fn main() {
	let v = vec!["hello", "this", "is", "a", "test"];

	let mut v2: Vec<String> = Vec::new();
	
	for i in v {
		v2.push(i.to_string());
	}
}
