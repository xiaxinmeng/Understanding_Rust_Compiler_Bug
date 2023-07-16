
fn print_stack_trace(_: libc::c_int) {
    panic!("help");
    // unsafe {
    //     static mut STACK_TRACE: [*mut libc::c_void; 256] = [std::ptr::null_mut(); 256];
    //     let depth = libc::backtrace(STACK_TRACE.as_mut_ptr(), 256);
    //     if depth == 0 {
    //         return;
    //     }
    //     backtrace_symbols_fd(STACK_TRACE.as_ptr(), depth, 2);
    // }
}
