
    // request kernel to allocate larger signal-stack sizes, so the
    // amx state can be saved (via XSAVE) when a signal arrives
    unsafe {
        // or can get needed size from C++ getauxval(AT_MINSIGSTKSZ) + SIGSTKZ?
        let size = 1024 * 1024;
        let st_mem = libc::malloc(size);
        let new_sig_stack = libc::stack_t {
            ss_flags: 0,
            ss_size: size,
            ss_sp: st_mem,
        };
        let res = libc::sigaltstack(&new_sig_stack, std::ptr::null_mut());
        println!("sigaltstack res = {res:?}, stack addr = {:?}", st_mem);
        if res != 0 {
            panic!("ERROR: Failed to change sigaltstack size");
        }
    }

