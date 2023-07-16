Rust
#![feature(let_else)]

struct Foo<'a>(&'a mut u32);

impl<'a> Drop for Foo<'a> {
    fn drop(&mut self) {
        *self.0 = 0;
    }
}

fn main() {
    let mut foo = 0;
    let Foo(0) = Foo(&mut foo) else {
        *&mut foo = 1;
        todo!()
    };
}
