rust
#![deny(warnings)]
struct Foo {
    #[deprecated]
    deprecated: usize,
}
fn main() {
    let foo = Foo {
        #[allow(deprecated)]
        deprecated: 1
     };
     #[allow(deprecated)]
    let tmp = foo.deprecated;
    println!("{}", tmp);
}
