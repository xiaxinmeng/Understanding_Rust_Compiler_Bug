
struct Cat {
    x: int
}

trait Meow {
    fn mew();
}

impl Cat: Meow {
    fn mew() {}
}

fn main() {
    let kitty = Cat { x:5 };
    assert kitty.mew == 1;
}

