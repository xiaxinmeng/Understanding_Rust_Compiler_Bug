plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
---- src/ffi/c_str.rs - ffi::c_str::CStr::from_ptr (line 236) stdout ----
error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
  --> src/ffi/c_str.rs:245:22
   |
12 | const HELLO: &CStr = CStr::from_ptr(HELLO_PTR);
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error: aborting due to previous error
---
    src/ffi/c_str.rs - ffi::c_str::CStr::from_ptr (line 236)

test result: FAILED. 3913 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 45.64s

error: doctest failed, to rerun pass `-p core --doc`
