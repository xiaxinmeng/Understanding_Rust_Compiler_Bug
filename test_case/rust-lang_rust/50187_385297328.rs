rust
#![feature(use_extern_macros)]

mod type_ns {
    pub type A = u8;
}
mod value_ns {
    pub const A: u8 = 0;
}
mod merge {
    pub use type_ns::A;
    pub use value_ns::A;
}

use merge::A; //~ ERROR unresolved import
