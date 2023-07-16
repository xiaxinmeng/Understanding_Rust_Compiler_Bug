
#[cfg(target_arch = "x86")]
pub extern "thiscall" fn foo(a: usize, b: usize) {}

#[cfg(target_arch = "x86_64")]
pub extern "fastcall" fn foo(a: usize, b: usize) {}
