 rust
struct AutoBuilder<'a> {
    context: &'a int
}

impl<'a> Drop for AutoBuilder<'a> {
    fn drop(&mut self) {
    }
}

fn main() {
}
