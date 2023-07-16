rust
// module_a.rs:

fn my_method(self: MyType, other_param: u8) -> u8 { ... }

// b.rs:

use module_a::my_method;

fn main() {
    let recv = make();
    recv.my_method(42);
}
