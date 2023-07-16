 rust
struct Cat {
    x: int
}

trait Meow {
    fn mew(&self) -> int;
}

impl Meow for Cat {
    fn mew(&self) -> int {
        self.x
    }
}

fn main() {
    let kitty = Cat { x:5 };
    assert!(kitty.mew == 5);
}
