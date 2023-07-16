 rust
#[inline(never)]
fn foo(n: usize) -> Vec<Option<(*mut (), &'static ())>> {
    (0..n).map(|_| None).collect()
}

fn main() {
    let _ = (foo(10), foo(32));
}
