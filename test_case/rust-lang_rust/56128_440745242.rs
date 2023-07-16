rust
mod bar {
    pub(super) use self::baz::{x, y};
//      ^^^^^ assert has to do with this path
//
// the other thing you need is the `{x, y}` part, which I think causes this to expand into
// two copies of the same `pub use` internally

    mod baz {
        pub fn x() { }
        pub fn y() { }
    }
}

fn main() { }
