 Rust
#![crate_type = "lib"]

pub fn f<T>(_v: T) {
    let nil = ();
    let _t = nil as uint; // this should be an error here
}
