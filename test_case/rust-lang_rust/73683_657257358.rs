rust
#![feature(const_generics)]
#![allow(dead_code, incomplete_features)]

pub struct ChannelUsage;

impl ChannelUsage {
    fn collect_arr<const N: i32>() {}
}

fn foo() {
    ChannelUsage::collect_arr::<16i32>();
}
