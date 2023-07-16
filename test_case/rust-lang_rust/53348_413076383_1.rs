rust
fn main() {
	let hello = String::from("hello");
	let mut vec: Vec<String> = Vec::new();
	vec.push((*hello).to_string());
}
