rust
use core::panic::PanicInfo;

#[lang = "start"]
#[no_mangle]
fn lang_start<T: 'static>(
    main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
) -> isize {
    main();

    return 0;
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_impl"] extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    panic!("Yeet!");
}

#[lang = "oom"]
fn rust_alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Yeet oom!");
}
