rust
mod one {
    #[repr(C)] struct Banana { weight: u64 }
    extern "C" { fn weigh_banana(count: *const Banana) -> u64; }
}

mod two {
    #[repr(C)] struct Banana { weight: u64 } // note: distinct type
    // For a weirder corner case (may still be valid depending on how C code is written):
    // #[repr(C)] struct Banana { weight: u64, some_optional_field: u64 }
    extern "C" { fn weigh_banana(count: *const Banana) -> u64; }
}
