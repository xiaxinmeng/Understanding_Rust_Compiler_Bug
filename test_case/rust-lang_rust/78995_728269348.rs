rust
enum Void{}
fn foo(x: Void) {
	match x {
		_ => println!("foo")
	}
}
