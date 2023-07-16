rust
pub fn cleanup() { 
    // only perform cleanup if network functionality was actually initialized
    if INIT.is_completed() { 
        _cleanup();
    }
}

// avoid linking to `WSACleanup` if it is never used by letting this function to be optimized away
#[inline(never)]
fn _cleanup() {
    unsafe { 
        // close the socket interface
        c::WSACleanup();
    }
}
