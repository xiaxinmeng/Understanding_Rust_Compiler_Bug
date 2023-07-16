rust
mod foo {
    pub trait Foo {
        fn foo();
    }
    
    pub mod bar {
        pub trait Foo {
            fn foobar();
        }
    }
}

/*pub trait Foo {
    fn bar();
}*/

pub use foo::bar::*;
pub use foo::*;

fn _f<T: Foo>() {}

fn main() {}
