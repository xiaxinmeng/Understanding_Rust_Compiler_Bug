 rust
pub fn panic_fmt(fmt: fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    #[allow(improper_ctypes)]
    extern {
        #[lang = "panic_fmt"]
        fn panic_impl(fmt: fmt::Arguments, file: &'static str, line: uint) -> !;
    }
    let (file, line) = *file_line;
    unsafe { panic_impl(fmt, file, line as uint) }
}
