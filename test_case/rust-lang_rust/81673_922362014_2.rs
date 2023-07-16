Rust
struct Movable(i32);

impl Movable {
    fn into_inner(self) -> i32 {
        self.0
    }
}

fn main() {
    let mut foo = Movable(42);

    loop {
        foo.0 += 1;
        println!("{}", foo.into_inner());
    }
}
