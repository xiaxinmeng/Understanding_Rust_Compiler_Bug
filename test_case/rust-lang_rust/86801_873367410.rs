rust
//- crate_a.rs
// rustflags: -Cpanic=unwind

extern "C-unwind" {
    fn may_throw_foreign_exception();
}

pub fn wrapper() {
    unsafe { may_throw_foreign_exception(); }
}

//- crate_b.rs
// rustflags: -Cpanic=abort
fn main() {
    a::wrapper();
}
