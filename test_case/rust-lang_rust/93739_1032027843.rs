rust
#[repr(u8)]
pub enum Bar {A = 0, B = 1}

#[repr(u8)]
enum AlwaysTwo { Two = 2 }

enum Foo {
    Two(Bar, Bar),
    One(AlwaysTwo, Bar),
}

fn main() {
    println!("{}", std::mem::size_of::<Foo>());
}
