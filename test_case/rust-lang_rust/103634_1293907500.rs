rust
#![feature(rustc_attrs)]

#[rustc_layout(debug)]
pub union U {
    a: [u16; 0],
    b: u8,
}
