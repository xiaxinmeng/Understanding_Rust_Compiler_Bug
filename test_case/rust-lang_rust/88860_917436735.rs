plain
    Checking std v0.0.0 (/checkout/library/std)
error: function is never used: `begin_panic_handler`
   --> library/std/src/panicking.rs:442:8
    |
442 | pub fn begin_panic_handler(info: &PanicInfo<'_>) -> ! {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:38
