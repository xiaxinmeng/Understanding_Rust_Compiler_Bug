rust
#[track_caller]
fn another() {
    panic!("line 3, right?")
}

#[inline]
fn nested() {
    another();
}

fn main() {
    nested();
}
