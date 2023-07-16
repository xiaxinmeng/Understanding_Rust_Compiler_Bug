 rust
/// Common out-of-memory routine
#[cold]
#[inline(never)]
static OOM_MESSAGE = "failed allocation: aborting\n"
pub fn oom() -> ! {
    // FIXME(#14674): This really needs to do something other than just abort
    //                here, but any printing done must be *guaranteed* to not
    //                allocate.

    unsafe {    
        with_task_stdout(|io| {
            io.write(OOM_MESSAGE.as_bytes())
        })
        core::intrinsics::abort()
    }
}
