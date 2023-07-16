
enum Foo{
   Bar,
   Baz
}


use std::mem;

fn main() {
    unsafe{
        let ordinal1: u8 = mem::transmute(Foo::Bar);
        let ordinal2: u8 = mem::transmute(Foo::Baz);
        println!("{} {}", ordinal1, ordinal2);
    }
}
