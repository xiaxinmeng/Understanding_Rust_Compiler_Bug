rust
#![deny(local_binding_shadows_glob_reexport)]

pub mod arm_shared {
    pub mod foo {}
    pub(crate) mod neon {}
}

pub mod aarch64 {
    pub use super::arm_shared::*;

    mod neon {}
}

pub fn main() {}
