rust
// -Cinstrument-coverage -Cdebug-assertions=off
fn main() {
    Some(Some(0)).map(|x| {
        debug_assert!(x.is_some())
    });
}
