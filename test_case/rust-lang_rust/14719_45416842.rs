 rust
// foo.rs
pub fn foo<T>() -> T { fail!() }
pub fn get_foo() -> fn() -> int { foo::<int> }

// bar.rs
extern crate foo;
fn main() {
    println!("{}", foo::foo::<int> == foo::get_foo());
}
