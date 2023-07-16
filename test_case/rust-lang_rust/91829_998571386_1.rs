rust
fn takes_composed<'a>(_: &dyn FnMut(&'a usize) -> bool) {}

fn main() {
    let b = compose_mut(predicate, bool::not);
    takes_composed(&b);
}
