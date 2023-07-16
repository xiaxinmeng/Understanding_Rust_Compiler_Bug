rust
fn validate_filename<S>(_: S) {}

fn main() {
    let _: Vec<_> = Some(()).into_iter().inspect(validate_filename).collect();
}
