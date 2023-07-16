rust
#![feature(rustc_attrs)]

#[rustc_args_required_const(1)]
fn intrinsic(a: u8){}

fn main() {
    intrinsic(1);
}
