Rust
#![feature(const_mut_refs)]
#![feature(adt_const_params)]

struct T<const B: &'static u64>;

impl <const B: &'static u64> T<B> {
    const fn set_false(&self) {
        unsafe {
            *(B as *const u64 as *mut u64) -= 1;
        }
    }
}

const _: () = {
    let x = T::<{&1}>;
    x.set_false();
};

fn main() {
    
}
