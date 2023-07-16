rust
#![feature(const_mut_refs)]
pub mod a {
    #[no_mangle]
    pub static mut FOO: &mut [i32] = &mut [42];
}

pub mod b {
    #[no_mangle]
    pub static mut BAR: &mut [i32] = unsafe { &mut *crate::a::FOO };
}

fn main() {
    unsafe {
        assert_eq!(a::FOO.as_ptr(), b::BAR.as_ptr());
    }
}
