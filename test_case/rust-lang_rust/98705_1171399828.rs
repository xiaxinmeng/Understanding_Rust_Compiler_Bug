rust
// `Expr` is used a lot. Make sure it doesn't unintentionally get bigger.
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
//rustc_data_structures::static_assert_size!(Expr, 104); // FIXME

impl Expr {
