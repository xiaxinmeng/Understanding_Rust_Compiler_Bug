 rust
#![crate_type="lib"]
extern crate cgu;
pub mod a {
    pub fn a() {
        ::cgu::id(());
    }
}
pub mod b {
    pub fn a() {
        ::cgu::id(());
    }
}
