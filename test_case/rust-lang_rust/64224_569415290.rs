rust
#![feature(unwind_attributes)]

extern "C" {
    // can unwind:
    #[unwind(allow)] fn foo(); 
}

pub unsafe fn bar() -> i32 {
    std::panic::catch_unwind(|| { foo(); 42 }).unwrap_or(13)
}
