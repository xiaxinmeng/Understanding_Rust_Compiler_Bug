rust
macro_rules! empty { () => () }

fn main() {
    // The error below is reported despite the inline attribute disappearing during expansion.
    #[inline = "nonsense"] // error: attribute must be of the form `#[inline]` or `#[inline(always|never)]`
    empty!();
}
