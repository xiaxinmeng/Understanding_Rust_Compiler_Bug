 rust
trait Common {}

impl<'t, T> Common for (T, &'t T) {}

impl<'t, T> Common for (&'t T, T) {}

fn main() {}
