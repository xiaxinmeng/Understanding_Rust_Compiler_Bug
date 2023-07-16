
// Test what happens when we link to the same function name with
// different types. This situation can easily arise when linking two
// crates that make use of the same C library, for example.

mod A {
    pub struct TwoU32s {
        one: u32, two: u32
    }
    extern {
        pub fn rust_dbg_extern_return_TwoU32s() -> TwoU32s;
    }
}

mod B {
    pub struct TwoU32s {
        one: u32, two: u32
    }
    extern {
        pub fn rust_dbg_extern_return_TwoU32s() -> TwoU32s;
    }
}

fn wrapper() {
}

#[fixed_stack_segment]
fn main() {
    unsafe {
        let a = A::rust_dbg_extern_return_TwoU32s();
    }

    unsafe {
        let b = B::rust_dbg_extern_return_TwoU32s();
    }
}
