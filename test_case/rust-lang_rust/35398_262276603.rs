rust
fn main() {
    fn function() {
        let c = |a: u32| a + 4;
        let _ = c(2);
    }
    let _ = function();
}
