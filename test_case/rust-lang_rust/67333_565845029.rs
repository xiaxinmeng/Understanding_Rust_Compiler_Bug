rust
#[inline]
fn bar() -> i32 {
    3
}

fn main() {
    let mut foo = (1, 2);
    foo.1 = bar();
}
