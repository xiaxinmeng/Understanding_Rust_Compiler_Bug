rust
union PtrOrInt {
    ptr: *mut (),
    int: usize,
}
pub static FOO: () = unsafe {
    let illegal_ptr2int = PtrOrInt { ptr: std::mem::transmute(&()) };
    let _copy = illegal_ptr2int.int;
};
