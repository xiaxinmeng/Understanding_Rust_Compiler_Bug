rust
pub static FOO: () = unsafe {
    let illegal_ptr2int: usize = std::mem::transmute(&());
    let _copy = illegal_ptr2int; // <-- error on the RHS read here
};
