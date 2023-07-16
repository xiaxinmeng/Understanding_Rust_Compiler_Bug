 rust
use std::panic::catch_unwind;
struct A;

let _a = A;
panic!();

impl Drop for A {
    fn drop(&mut self) {
        catch_unwind(|| panic!());
    }
}
