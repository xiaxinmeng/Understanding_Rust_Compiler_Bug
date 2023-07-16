 rust
use Foo::Bar;

mod Foo {
    pub struct Bar {
        pub x: int,
        y: int
    }

    impl Bar {
        pub fn new(new_x: int, new_y: int) -> Bar {
            Bar { x: new_x, y: new_y }
        }
    }
}

impl Bar {
    fn set_x(&self, new_x: int) {
        self.x = new_x;
    }
}

fn main() {
    let bar = Bar::new(1, 2);  // error: unresolved name `Bar::new`.
    println!("{}", bar.x);
}
