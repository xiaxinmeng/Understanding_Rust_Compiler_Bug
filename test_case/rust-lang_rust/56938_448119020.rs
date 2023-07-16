rust
#[repr(align(32))]
struct Foo(u8);

fn main() {
    println!("{}", std::mem::size_of::<Foo>());
}
