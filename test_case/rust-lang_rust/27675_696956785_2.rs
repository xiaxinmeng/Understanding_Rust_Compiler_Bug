rust
// ICE
fn main() {
    safe_transmute::<usize, &'static str>(0usize);
}
