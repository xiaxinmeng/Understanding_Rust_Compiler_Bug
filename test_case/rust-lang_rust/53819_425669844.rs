rust
union SignedToUnsigned {
    signed: i32,
    unsigned: u32,
}

// allowed on stable
const UNSIGNED: u32 = unsafe { SignedToUnsigned { signed: 1i32 }.unsigned };

// requires `#![feature(const_fn_union)]`
const fn signedToUnsigned(signed: i32) -> u32 {
    unsafe { SignedToUnsigned { signed }.unsigned }
}
