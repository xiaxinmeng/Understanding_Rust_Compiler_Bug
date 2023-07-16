rust
#![feature(unsized_locals)]

trait Foo {
    fn you_move_me(self, g: &dyn FnMut());
}

impl Foo for () {
    fn you_move_me(self, g: &dyn FnMut()) { }
}

fn call_me(mut f: Box<dyn Foo>) {
    f.you_move_me(&|| {
        f = Box::new(());
    })
}

fn main() { }
