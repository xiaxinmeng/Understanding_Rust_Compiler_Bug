
#[cfg(target_pointer_width = "32")]
#[export_name = "_AbortCompilerPass"]
pub extern "stdcall" fn abort_compiler_pass_extern(how: winapi::DWORD) { abort_compiler_pass(how)}
#[cfg(target_pointer_width = "64")]
#[export_name = "AbortCompilerPass"]
pub extern "stdcall" fn abort_compiler_pass_extern(how: winapi::DWORD) { abort_compiler_pass(how)}
