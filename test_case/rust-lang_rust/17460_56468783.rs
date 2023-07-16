 rust
#[repr(uint)]
enum A {
    B,
    C,
}

static X: uint = C as uint;

//static Y: [uint, ..X] = [0, ..X];

fn main() {
    println!("{}", X);
}
