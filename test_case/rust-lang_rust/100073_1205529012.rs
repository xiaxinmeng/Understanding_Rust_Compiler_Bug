rust
#![feature(raw_dylib)]

#[link(name = "exporter", kind = "raw-dylib")]
extern {
    #[link_ordinal(1, 2)]
    fn imported_function();
}

fn main() {}
