 rust
struct Cat {
    x: i32
}

trait Meow {
    fn mew(&self) -> i32;
}

impl Meow for Cat {
    fn mew(&self) -> i32 {
        self.x
    }
}

fn main() {
    let kitty = Cat { x:5 };
    assert!(kitty.x() == 5);
}
