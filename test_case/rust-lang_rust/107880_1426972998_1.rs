rust
pub mod core_arch {
    pub mod arm_shared {
        pub mod neon {
            pub fn vcvtq_u32_f32() {}
        }

        pub use neon::*;
    }
}

pub mod neon {
    pub fn vcvtq_u32_f32() {}
}

pub use core_arch::arm_shared::*;
pub use neon::*;
