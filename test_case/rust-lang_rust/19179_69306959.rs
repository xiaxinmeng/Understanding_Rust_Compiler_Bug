 rust
pub mod a {
    pub use self::aa::*;

    pub mod aa {
        use b::YY; // comment out this...

        pub struct XX;
    }
}

pub mod b {
    pub use self::bb::*;

    pub mod bb {
        use a::XX; // ...or this

        pub struct YY;
    }
}
