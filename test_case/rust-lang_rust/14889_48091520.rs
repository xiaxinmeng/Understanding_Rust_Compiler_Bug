 rust
fn main() {
}

struct OnExitSentinel<'a> {
    block: ||: 'a,
}

impl<'a> Drop for OnExitSentinel<'a> {
    fn drop(&mut self) {
    }
}
