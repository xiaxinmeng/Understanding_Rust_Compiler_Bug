 rust
// foo.rs
local_data_key!(foo: int);

fn set_foo() { foo.replace(Some(3)); }
fn get_foo() { *foo.get().unwrap() }

// bar.rs
extern crate foo;
fn main() {
    foo::set_foo();
    assert_eq!(foo::get_foo(), 3);
    assert_eq!(*foo::foo.get().unwrap(), 3);
}
