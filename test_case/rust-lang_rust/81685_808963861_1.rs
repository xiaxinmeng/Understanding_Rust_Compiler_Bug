rust
fn validate_filename<S>(_: S) {}

fn should_be_iterator<I: Iterator>(_: I) {}

fn main() {
    should_be_iterator(Some(()).into_iter().inspect(validate_filename));
}
