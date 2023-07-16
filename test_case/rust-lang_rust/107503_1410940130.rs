rust
pub fn cause_late_error<T>() {
    struct S;
    impl S {
        const C: () = panic!();
    }

    S::C;
}

fn main() {
    let a = 1;
    let b = 2;
    if a + b == 5 { // changing 5 to 3 makes compilation fail
        cause_late_error::<()>();
    }
}
