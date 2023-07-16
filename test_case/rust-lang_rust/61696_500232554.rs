rust
#![feature(never_type, core_intrinsics)]

pub enum E1 {
    V1 { f: bool },
    V2 { f: ! },
    V3,
    V4,
}

fn main() {
    match (E1::V1 { f: true }) {
        E1::V2 { .. } => unsafe { ::core::intrinsics::unreachable() },
        _ => {},
    }
}
