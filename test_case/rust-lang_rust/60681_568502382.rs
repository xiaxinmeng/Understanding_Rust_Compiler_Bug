rust
#![deny(warnings)]

struct Foo {
    #[deprecated]
    deprecated: usize,
}

fn main() {
    let foo = Foo {
        #[allow(deprecated)]
        deprecated: 1,
    };
    
    println!("{}", foo.deprecated)
}
