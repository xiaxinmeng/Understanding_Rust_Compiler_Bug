 rust
#![crate_type="rlib"]

pub struct Struct {
    pub field: u32,
}

#[link_section = ".my_section"]
pub static VAR: Struct = Struct { field: 1 };
