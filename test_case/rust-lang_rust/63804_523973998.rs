rust
struct Bomb;
impl Drop for Bomb {
    fn drop(&mut self) {
        panic!("bomb dropped");
    }
}

thread_local!(static BOMB: Bomb = Bomb);

fn main() {
    BOMB.with(|_| {}); // arm the bomb
}
