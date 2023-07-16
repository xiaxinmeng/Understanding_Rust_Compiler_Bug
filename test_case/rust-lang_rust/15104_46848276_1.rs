 rust
fn main() {
    match &[1,2,3] {
        [] => (),
        [_, ..] => ()  // works fine
    }
}
