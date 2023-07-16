rust
#[inline]
default fn to_string(&self) -> String {
    let mut buf = String::new();
    let mut formatter = core::fmt::Formatter::new(&mut buf);
    // Bypass format_args!() to avoid write_str with zero-length strs
    fmt::Display::fmt(self, &mut formatter)
        .expect("a Display implementation returned an error unexpectedly");
    buf
}
