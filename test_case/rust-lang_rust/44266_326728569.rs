
enum Thing {
    Foo = 0,
    Bar = 1,
}

const THING_FOO: u8 = Thing::Foo as u8;
const THING_BAR: u8 = Thing::Bar as u8;

fn main() {
    // works
    println!("Foo is {}", Thing::Foo as u8);
    
    // doesn't
    match 0u8 {
        THING_FOO => println!("0 is Foo"),
        _ => println!("eh")
    };
}
