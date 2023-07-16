Rust
struct LoudDrop(u8);

impl Drop for LoudDrop {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}
fn main() {
    {
        let _v = vec!(LoudDrop(0), LoudDrop(1));
    }
    println!("Hi");
    {
        let _v = vec!(LoudDrop(2), LoudDrop(3), LoudDrop(4), panic!());
    }
}
