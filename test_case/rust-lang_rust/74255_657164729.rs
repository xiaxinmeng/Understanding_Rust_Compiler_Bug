Rust
#![feature(const_generics)]
#![allow(dead_code, incomplete_features)]

struct IceStruct;

impl IceStruct {
	fn ice_struct_fn<const I: usize>() {}
}

fn ice_fn() {
	IceStruct::ice_struct_fn::<1>();
}
