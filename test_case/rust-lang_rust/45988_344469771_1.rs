rust
#[derive(Debug)]
struct Wrap<'p> { p: &'p mut i32 }

impl<'p> Drop for Wrap<'p> {
    fn drop(&mut self) {
        *self.p += 1;
    }
}

#[derive(Debug)]
struct Foo<'p> { a: String, b: Wrap<'p> }

fn main() {
    let mut x = 0;
    let wrap = Wrap { p: &mut x };
    let s = String::from("str");
    let foo = Foo { a: s, b: wrap };
    x = 1; //~ ERROR
    println!("{:?}", foo);
}
