 Rust
#![feature(tuple_indexing)]

pub struct Bar<'bar>(&'bar i32);

impl<'bari> Bar<'bari> {
    pub fn get_ref(&'bari mut self) -> &'bari mut &'bari i32 {
        &mut self.0
    }
}

pub fn unsound_2<'f, 'g>(bar: &'g mut Bar<'f>) -> &'g mut &'g i32 {
    bar.get_ref()
}

fn main() {
    let r = &0i32;
    let bar = &mut Bar(r);
    let mut v = 0;
    {
        let ir = unsound_2(bar);
        *ir = &v;
    }
    v = 1;
    println!("{}", bar.0);
}

