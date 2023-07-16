rust
fn f(x: Option<()>) {
	(|| x.take())();
}
fn main() {}
