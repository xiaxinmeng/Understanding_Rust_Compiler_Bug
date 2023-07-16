rust
// crate foo
#[cfg_attr(
    all(target_os = "linux", not(target_arch = "mips")),
    export_name = "\x01.L_this_is_a_private_symbol"
)]
#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    export_name = "\x01L_this_is_a_private_symbol"
)]
#[cfg_attr(target_arch = "mips", export_name = "\x01$_this_is_a_private_symbol")]
pub static X: usize = 42;

#[inline]
pub fn foo() -> usize {
    unsafe { core::ptr::read_volatile(&X) }
}
