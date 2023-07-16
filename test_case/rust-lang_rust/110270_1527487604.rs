rust
#[no_mangle]
extern "C" fn unsafe_main(_argc: i32, _argv: *const *const i8) -> i32 {
    unsafe {
        let bytes = b"HELLO\n\0";
        write(STDOUT_FILENO, bytes.as_ptr() as *const c_void, bytes.len());
        // libc::_exit(0);
        0xdeadface_u32 as i32
    }
}
