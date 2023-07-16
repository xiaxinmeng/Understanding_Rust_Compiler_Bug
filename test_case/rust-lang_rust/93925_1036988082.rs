rs
#[must_use]
unsafe fn foo() -> i64 {
    4
}

fn main() {
    unsafe { foo() };
}
