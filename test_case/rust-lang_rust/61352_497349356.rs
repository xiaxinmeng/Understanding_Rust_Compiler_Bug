rust
#![feature(rustc_attrs)]

#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0x2)]
struct LowestBitUnused(u32);

enum S {
    A(LowestBitUnused),
    B(LowestBitUnused),
}
