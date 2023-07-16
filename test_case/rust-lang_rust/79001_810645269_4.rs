
pub fn panic_fmt_assert(
    fmt: fmt::Arguments<'_>,
    macro_name: &'static str,
    op: &'static str,
    left_expr: &'static str,
    right_expr: &'static str,
    left_val: &'_ dyn crate::fmt::Debug,
    right_val: &'_ dyn crate::fmt::Debug,
    message: Option<crate::fmt::Arguments<'_>>,
) -> ! {
    panic_fmt(...)
}
