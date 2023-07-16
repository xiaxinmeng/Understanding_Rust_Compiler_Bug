 rust
mod bar {
    pub use self::middle::*;

    mod middle {
        pub use self::baz::Baz;

        mod baz {
            pub enum Baz {
                Baz1,
                Baz2
            }
        }
    }
}

mod foo {
    use bar::Baz::{Baz1, Baz2};
}
