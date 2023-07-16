
use foo::Bar;

fn main() {
    let b = Bar {y: 8};
    println!("{}", b.y);
}

mod foo {
    pub struct Bar {
        pub y: int,
    }
}
