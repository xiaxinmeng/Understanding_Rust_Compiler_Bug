rust
#[allow(unused_macros)]
macro_rules! simd_shuffle8_param {
    ($x:expr, $y:expr, <const $imm:ident : $ty:ty> $idx:expr $(,)?) => {{
        struct ConstParam<const $imm: $ty>;
        impl<const $imm: $ty> ConstParam<$imm> {
            const IDX: [u32; 8] = $idx;
        }

        simd_shuffle8($x, $y, ConstParam::<$imm>::IDX)
    }}
}
