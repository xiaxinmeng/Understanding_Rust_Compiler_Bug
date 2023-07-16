 rust
// foo.rs
#[inline]
pub fn foo() -> int { 0 }
pub fn get_foo() -> fn() -> int { foo }

// bar.rs
extern crate foo;
fn main() {
    println!("{}", foo::foo == foo::get_foo());
}
