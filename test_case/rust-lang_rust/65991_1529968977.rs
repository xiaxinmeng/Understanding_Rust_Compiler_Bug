rust
#![feature(arbitrary_self_types)]
pub trait Tr {
    fn f(self: *mut Self);
}

pub fn g(v: *mut dyn Tr) {
    Tr::f(v)
}
