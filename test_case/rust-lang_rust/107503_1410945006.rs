rust
#[inline(always)]
fn falser() -> bool {
    false
}

pub fn has_late_error<T>() {
    struct S;
    impl S {
        const C: () = panic!();
    }

    S::C;
}

fn main() {
    if falser() {
        has_late_error::<()>();
    }
}
