rust
#![deny(warnings)]
struct Foo {
    #[deprecated]
    deprecated: usize,
}
fn main() {
    #[allow(deprecated)]
    let foo = Foo {deprecated: 1};
     #[allow(deprecated)]
    let tmp = foo.deprecated;
    println!("{}", tmp);
}
