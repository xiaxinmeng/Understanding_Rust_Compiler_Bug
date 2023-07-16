rust
enum Void {}
fn foo(x: Void) {
	match x {
		_ => println!("foo") // used not to warn, but now triggers the `unreachable_patterns` lint
	}
}
