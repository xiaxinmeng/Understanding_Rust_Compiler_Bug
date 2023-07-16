
    // Give the listeners plenty of time to get at least one update.
    // Note that this fails in the same way even if we sleep a lot longer.
    libc::funcs::posix88::unistd::sleep((5*num_listeners + 3) as libc::types::os::arch::c95::c_uint);
