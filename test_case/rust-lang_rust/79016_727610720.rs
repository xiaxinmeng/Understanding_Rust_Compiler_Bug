rust
#![feature(destructuring_assignment)]

struct Bomb(&'static str);
impl Drop for Bomb {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn main() {
    let _val;
    let x;
    _ = Bomb("bomb 1");
    _val = Bomb("bomb 2");
    (x, _) = (Bomb("bomb 3"), Bomb("bomb 4"));
    drop(Bomb("bomb 5"));
}
