 Rust
#![feature(tuple_indexing)]

pub struct Bar<'a>(i32, &'a i32);

impl<'a> Bar<'a> {
    pub fn set(&'a mut self) {
        self.1 = &self.0;
    }
}

// if I use this instead of bar.set I get a borrowck error
//pub fn set<'a>(b: &'a mut Bar<'a>) {
//    b.1 = &b.0;
//}

pub fn unsound<'f>(bar: &'f mut Bar<'f>) -> &'f mut Bar<'f> {
    bar.set();
    bar
}

fn main() {
    let r = &0i32;
    let b = &mut Bar(0, r);
    let b = unsound(b);
    let new_r = &b.1;
    b.0 = 1;
    println!("{}", new_r);
}

