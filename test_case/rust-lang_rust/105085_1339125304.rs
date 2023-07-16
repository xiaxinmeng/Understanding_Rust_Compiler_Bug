rust
struct NoisyDrop(&'static str);
impl Drop for NoisyDrop {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}
fn main() {
    let _ = &&(NoisyDrop("drop!"), 0).1;
    // NosiyDrop is not dropped ...
}
