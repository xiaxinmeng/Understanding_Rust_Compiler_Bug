rust
#[lang = "start"]
fn lang_start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        core::intrinsics::transmute::<_, fn()>(main)();
    }
    panic!("Root server shouldn't ever return from main!");
}
