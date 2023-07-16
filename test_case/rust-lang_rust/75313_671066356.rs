rust
fn iter_slice<'a, T>(xs: &'a [T]) -> impl Iterator<Item = &'a T> {
    xs.iter()
}

fn main() {
    iter_slice::<()> as fn(_) -> _;
}
