rust
fn before(_: &char, _: &char) -> bool {
    false
}

fn after(_: &char, _: impl std::borrow::Borrow<char>) -> bool {
    false
}

fn example(_: fn(&char, &char) -> bool) {}

fn main() {
    example(before);
    example(after);
}
