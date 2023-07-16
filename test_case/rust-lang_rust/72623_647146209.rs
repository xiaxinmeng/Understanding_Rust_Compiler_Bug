rust
mod foo {
    mod bar {
        pub struct X;
    }
    pub use self::bar::X;
}

fn main() { X; }
