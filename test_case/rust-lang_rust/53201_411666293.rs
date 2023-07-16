rust
#![feature(const_fn, rustc_attrs, attr_literals)] 

#[rustc_args_required_const(0)]
pub const fn new(value: u8) -> u8 {
    value
}

#[rustc_args_required_const(0)]
pub unsafe fn const_(_: u8) {
    unimplemented!()
}

fn main() {
    let _a = const_(new(0));
}
