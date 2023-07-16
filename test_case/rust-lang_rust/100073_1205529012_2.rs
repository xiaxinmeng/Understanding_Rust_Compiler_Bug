rust
#![feature(raw_dylib)]

#[link(name = "exporter", kind = "raw-dylib")]
extern {
    #[link_ordinal(1, 2)]
    static mut imported_variable: i32;
}

fn main() {}
