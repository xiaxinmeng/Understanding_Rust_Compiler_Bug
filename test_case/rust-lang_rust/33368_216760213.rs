 rust
thread_local! { pub static IS_PANICKING: Cell<bool> = Cell::new(false); }

// Here is a function that will be called when panic! happens
// Just like the one in https://github.com/rust-lang/rust/blob/master/src/libstd/panicking.rs#L198
fn on_panic(obj: &(Any+Send), file: &'static str, line: u32) {
    let is_panicking = IS_PANICKING.with(|s| {
        let orig = s.get();
        s.set(true);
        orig
    });

    if is_panicking {
        // Abort right here
        util::dumb_print(format_args!("thread panicked while processing \
                                       panic. aborting.\n"));
        unsafe { intrinsics::abort() }
    }

    // ...
}

// https://github.com/rust-lang/rust/blob/master/src/libstd/sys/common/unwind/mod.rs#L159
pub fn panicking() -> bool {
    IS_PANICKING.with(|s| s.get())
}

// The catch_panic
// https://github.com/rust-lang/rust/blob/master/src/libstd/sys/common/unwind/mod.rs#L131
unsafe fn inner_try(f: fn(*mut u8), data: *mut u8)
                    -> Result<(), Box<Any + Send>> {
    if panicking() {
        // It should not be allowed (to catch panic while panicking)
        unsafe { intrinsics::abort() }
    }

    let mut payload = imp::payload();
    let r = intrinsics::try(f, data, &mut payload as *mut _ as *mut _);

    // Clear the flag because we already caught the panic
    IS_PANICKING.with(|s| s.set(false));

    if r == 0 {
        Ok(())
    } else {
        Err(imp::cleanup(payload))
    }
}
