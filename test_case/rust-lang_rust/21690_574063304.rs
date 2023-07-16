rust
mod core::intrinsics { // or somewhere else
    // bitflags for fast-math:
    const NonNan: u32 = 0b1;
    const NoSignedZero: u32 = 0b10;
    const Associative: u32 = 0b100;
    ...

    // fp arithmetic intrinsics taking a const bitset of fast-math flags
    fn fp_add<T>(T, T, const fast_math_flags: u32) -> T;
    fn fp_sub<T>(T, T, const fast_math_flags: u32) -> T;  
   ...
   fn fp_sqrt<T>(T, T, const fast_math_flags: u32) -> T;
   ...
}
