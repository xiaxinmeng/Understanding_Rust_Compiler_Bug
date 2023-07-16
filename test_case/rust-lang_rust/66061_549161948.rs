rust
#![feature(core_intrinsics)]

const fn hello()->u32{
    // The intrinsic is not a const fn,yet it can be called inside a const fn
    // pub fn saturating_sub<T>(a: T, b: T) -> T;
    std::intrinsics::saturating_sub::<u32>(0,10)
}

const HI:u32=hello();

fn main(){
    dbg!(HI);
}
