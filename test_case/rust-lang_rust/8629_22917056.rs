 rust
fn foo<A: Iterator<int>>(xs: A) -> int {
    xs.map(|x| x * 2).fold(0, |a, b| a + b)
}

fn bar<A, B: Iterator<A>>(mut xs: B) -> bool {
    xs.len() != 0
}
