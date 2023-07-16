rust
#![feature(core_intrinsics)]

fn main() {
    let _unused = if true {
        std::intrinsics::likely
    } else {
        std::intrinsics::unlikely
    };
}
