 rust
#![repr(u8)]
enum Foo {
    A = 0,
    B = 1,
    C = 2
}

fn main() {
    let x = 0u8;
    const A: u8 = Foo::A as u8;
    const B: u8 = Foo::B as u8;
    const C: u8 = Foo::C as u8;

    match x {
        A => println!("A"),
        B => println!("B"),
        C => println!("C"),
        _ => println!("No match!"),
    };
}
