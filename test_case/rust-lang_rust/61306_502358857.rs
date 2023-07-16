rust
#![feature(rustc_private)]
extern crate libc;
#[macro_use]
extern crate bitflags;

use libc::uint32_t;
use libc::uint64_t;
bitflags! {
    #[repr(C)]
    struct E1 : uint32_t {
        const V1 = 1;
        const V2 = 2;
        const V3 = 4;
    }
}
bitflags! {
    #[repr(C)]
    struct E2 : uint64_t {
        const V1 = 1;
        const V2 = 2;
        const V3 = 4;
    }
}

#[repr(u32)]
enum E3 { V1 = 1, V2 = 2, V3 = 4, }

extern {
    fn test(val_e1: E1, val_e2: E2, val_e3: E3, val_u: uint32_t);
}
fn main() {
    unsafe {
        test(E1::V1, E2::V1,E3::V1, 1);
        test(E1::V3, E2::V3,E3::V3, 4);
    }
}
