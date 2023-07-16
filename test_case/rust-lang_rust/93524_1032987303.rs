rust
mod m {
    pub const C: usize = 7;
}

pub mod a {
    pub use super::m::C;
}

pub mod b {
    pub use super::m::C;
}
