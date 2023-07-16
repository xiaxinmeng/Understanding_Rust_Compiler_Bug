rust
pub mod core_arch {
    pub mod arm_shared {
        pub mod neon {
            pub fn vcvtq_u32_f32() {
                println!("Hemlo");
            }
        }

        pub use neon::*;
    }
}


pub mod neon {
    pub fn vcvtq_u32_f32() {
        println!("Good bye");
    }
}

pub use core_arch::arm_shared::*;
pub use neon::*;

pub fn main() {
    vcvtq_u32_f32();
}
