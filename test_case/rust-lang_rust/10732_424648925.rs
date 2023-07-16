rust

fn main() {
    let _ = abc::Foo { a: 3 };
}

mod abc {
    pub struct Foo {
        pub a: i32
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("{}", self.a);
        }
    }
}
