rust
pub mod a {
    pub mod b {
        pub mod c {
            pub struct Foo;

            pub enum Bar {
                First,
                Second,
            }

            pub use Bar::*;
        }

        pub use c::*;
    }

    pub use b::*;
}

pub use a::*;
