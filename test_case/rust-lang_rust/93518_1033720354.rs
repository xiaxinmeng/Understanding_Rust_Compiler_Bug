rust
mod depth1 {
    mod depth2 {
        pub const C: usize = 7;
    }

    pub use self::depth2::C;
}

pub use depth1::C; // Direct link to `pub const C` instead of `pub use self::depth2::C`
