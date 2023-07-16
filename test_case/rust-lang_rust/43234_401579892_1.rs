rust
#![feature(nll)]

struct Baz {
    x: usize,
    y: usize,
}

struct Bar {
    baz: Baz,
}

impl Bar {
    fn get_baz_mut(&mut self) -> &mut Baz {
        &mut self.baz
    }
}

struct Foo {
    bar: Bar,
}

impl Foo {
    fn foo(&mut self) -> Option<&mut Baz> {
        for _i in 0..4 {
            let baz = self.bar.get_baz_mut();
            if baz.x == 0 {
                return Some(baz);
            }
        }
        None
    }
}

// Just here to get it to compile as a binary
fn main() {
    println!("Hello world");
}
