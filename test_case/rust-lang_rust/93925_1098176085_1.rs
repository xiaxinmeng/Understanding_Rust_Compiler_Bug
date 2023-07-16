rust
unsafe fn x() -> i32 {
    4
}

fn main() {
    unsafe {
        x()
    }

    // avoid interpreting the unsafe block as `main`'s return expression
    ()
}
