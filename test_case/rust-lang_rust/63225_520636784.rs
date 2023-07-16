rust
#![feature(in_band_lifetimes)]

struct Foo<'a> {
    x: &'a u32
    
}

impl Foo<'a> {
    fn method(&self) {
        fn blah(f: Foo<'a>) { }
    }
}
