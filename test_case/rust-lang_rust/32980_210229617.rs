 rust
pub fn foo() -> i32 {
    let mut a = 0;
    while a < 100 {
        a += 1;

        a = a * 2;
    }

    a * a
}
