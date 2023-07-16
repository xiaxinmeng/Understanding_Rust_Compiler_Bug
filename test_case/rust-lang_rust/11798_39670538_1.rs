
use foo::Bar;

fn main() {
    impl Bar {
        fn boop(&self) -> bool {
            true
        }
    }

    let b = Bar {y: 8};
    println!("{}", b.y);
}

mod foo {
    pub struct Bar {
        pub y: int,
    }
}
