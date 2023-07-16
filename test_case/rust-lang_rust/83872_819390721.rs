rust
// resolve.rs
mod sub4 {
    pub const X: usize = 0;
 
    pub mod other {
        pub const X: usize = 1;
    }
    pub mod inner {
        pub use other::*;
        pub use super::*;
    }
}

pub use sub4::inner::*;
// main.rs
println!("{:?}", resolve::X);
