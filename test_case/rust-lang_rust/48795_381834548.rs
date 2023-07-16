rust
use std::path::Path;

pub trait Block { }

struct Inner {
    data: i32,
}

impl Block for Inner { }

impl Inner {
    fn new_box(data: i32) -> Box<Inner> {
        Box::new(Inner {
            data: data,
        })
    }
}

pub struct Outer {
    inner: Box<Inner>,
    block: *mut Block,
}

impl Outer {
    pub fn new_box<P: AsRef<Path>>() -> Box<Outer> {
        let mut inner = Inner::new_box(123);
        let block = &mut *inner as *mut _;

        Box::new(Outer {
            inner: inner,
            block: block,
        })
    }

    pub fn get_inner(&mut self) -> &mut Block {
        &mut *self.inner
    }
}

fn main() {
    let mut outer = Outer::new_box::<&str>();
    let b = outer.block;
    let a = outer.get_inner() as *mut Block;
    println!("{:p} == {:p}: {}", a, b, a == b);
}
