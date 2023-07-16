rust
#[lang = "panic_fmt"]
#[linkage = "weak"]
unsafe extern "C" fn panic_fmt(
    _args: ::core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _col: u32,
) -> ! {
