
pub mod a {
    pub use self::Priv as Pub;
    struct Priv;
}

pub fn main() {}
