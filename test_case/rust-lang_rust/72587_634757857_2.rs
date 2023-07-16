rust
#![warn(unused_lifetimes)]
#![warn(clippy::extra_unused_lifetimes)]
fn unused_lifetime<'a>() {}
fn main() {
	unused_lifetime();
}
