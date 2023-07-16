rust
#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(0)]
struct ZeroPad(u8);

impl ZeroPad {
    const ZERO: ZeroPad = unsafe { ZeroPad(0) };
}

struct Test(u8, ZeroPad, u16);
fn main() {
    println!("{}", std::mem::size_of::<Option<Test>>());
}
