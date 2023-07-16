
$ rustc.exe ./src/test/run-pass/reexport-star.rs --pretty normal
// FIXME #3654

mod a {
    pub fn f() { }
    pub fn g() { }
}

mod b {
    pub use a::*;
}

fn main() { b::f(); b::g(); }
