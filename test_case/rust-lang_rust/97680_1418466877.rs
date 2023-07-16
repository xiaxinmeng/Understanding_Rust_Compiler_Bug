rust
// Trait blanket implemented for Fn
trait StrFn<'a>: Fn(&'a str) -> Self::SOutput {
    type SOutput;
}
impl<'a, O, F: Fn(&'a str) -> O> StrFn<'a> for F {
    type SOutput = O;
}

// Usage of the trait with HRTB
fn go<F: for<'a> StrFn<'a>>(f: F) {
    f("");
}

fn main() {
    go(str::chars); // no error
    go(|line: &str| line.chars()); // error
}
