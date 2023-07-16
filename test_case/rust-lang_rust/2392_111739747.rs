 rust
struct Cat<F> where F: FnMut() -> u32 {
    func: F,
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
    let kitty = Cat { func: || 5, x: 5 };
    assert!(kitty.x() == 5);
    let x = kitty.func();
    assert_eq!(x, 5);
}
