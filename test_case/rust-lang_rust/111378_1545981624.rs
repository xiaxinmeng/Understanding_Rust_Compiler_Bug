rust
#![deny(local_binding_shadows_glob_reexport)]

pub mod upstream_a {
    mod inner {
        pub struct Foo {}
    }

    pub use self::inner::*;

    struct Foo;
    //~^ ERROR local binding shadows glob re-export
}

pub mod upstream_b {
    mod inner {
        pub struct Foo {}
    }

    mod other {
        pub struct Foo;
    }

    pub use self::inner::*;

    use self::other::Foo;
    //~^ ERROR local binding shadows glob re-export
}

// Downstream crate
// mod downstream {
//     fn proof() {
//         let _ = crate::upstream_a::Foo;
//         let _ = crate::upstream_b::Foo;
//     }
// }

pub fn main() {}
