rust
fn sized_ref<T>(_: &T, _: &T) {}

fn main() {
    sized_ref("First", "Second");
}
