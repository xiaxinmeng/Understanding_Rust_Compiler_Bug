 rust
#![feature(relaxed_adts)]

struct Foo(u32);

fn main() {
    let Foo { 0: x } = Foo(22);
    println!("{}", x);
}
