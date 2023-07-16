rust
#![feature(raw_ref_op)]
macro_rules! offset {
    ($ty: tt, $field: ident) => {
        unsafe { &raw const ((*(0 as *const $ty)).$field) } as usize
    };
}

struct Emm {
    a: i32,
    b: u64,
}

fn main() {
    let tmp = offset!(Emm, b);
    let tmp = unsafe { &raw const ((*(0 as *const Emm)).b) } as usize;
}
