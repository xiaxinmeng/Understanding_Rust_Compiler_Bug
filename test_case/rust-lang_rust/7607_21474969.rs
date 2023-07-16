
pub mod a {
    pub struct Foo { a: uint }
}

pub mod b {
    use a::Foo;
    impl Foo { // ERROR: found value name used as type
        fn bar(&self) { }
    }
}

pub fn main() { }
