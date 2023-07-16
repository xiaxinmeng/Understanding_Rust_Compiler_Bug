rust
fn cause_late_error<T>() {
    cause_late_error::<((), T)>();
}

fn main() {
    let (a, b) = (1, 2);
    if a + b == 5 {
        cause_late_error::<()>();
    }
}
