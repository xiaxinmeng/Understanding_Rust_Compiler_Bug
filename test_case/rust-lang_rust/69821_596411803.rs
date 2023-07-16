rust
#![feature(core_intrinsics, const_discriminant)]

const ARR: [u32; std::intrinsics::discriminant_value(&Some(7)) as usize] = [42];
