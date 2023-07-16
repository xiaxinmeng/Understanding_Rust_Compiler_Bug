
[-@- rust]$ cat foo.rs
#![feature(specialization)]
trait Foo {
    fn foo(&self);
}
trait Bar {
    fn bar(&self);
}
impl<T> Bar for T where T: Foo {
    fn bar(&self) {}
}
impl<T> Foo for T where T: Bar {
    fn foo(&self) {}
}
impl Foo for u64 {}
fn main() {
}
[-@- rust]$ rustc foo.rs
[-@- rust]$ rustc --version --verbose
rustc 1.28.0-nightly (5d0631a64 2018-05-30)
binary: rustc
commit-hash: 5d0631a6438cf30cac236b7176371663e35c8d07
commit-date: 2018-05-30
host: x86_64-unknown-linux-gnu
release: 1.28.0-nightly
LLVM version: 6.0
