rust
fn main() {
    let mut a = Some(3);
    while let Some(ref mut v) = {a} {
        a.as_mut().map(|a| std::mem::swap(a, v));
    }
}
