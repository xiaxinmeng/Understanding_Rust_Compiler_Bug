rust
fn main() {
    struct Ref<'a>(&'a ());
    let closure = |x| Ref(x);

    while let Some(x) = None {
        closure(&x);
    }
}
