plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: unused import: `crate::ffi::c_void`
 --> library/std/src/os/./windows/io/handle.rs:7:5
7 | use crate::ffi::c_void;
  |     ^^^^^^^^^^^^^^^^^^
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:136:32
    |
136 |         if owned_handle.handle.as_ptr().is_null() { Err(()) } else { Ok(owned_handle) }
    |                                ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:146:32
    |
146 |         if owned_handle.handle.as_ptr() == c::INVALID_HANDLE_VALUE {
    |                                ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:157:21
    |
157 |         self.handle.as_ptr()
    |                     ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:164:21
    |
164 |         self.handle.as_ptr()
    |                     ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:171:34
    |
171 |         let handle = self.handle.as_ptr();
    |                                  ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior

error[E0599]: no method named `as_ptr` found for raw pointer `*mut libc::c_void` in the current scope
   --> library/std/src/os/./windows/io/handle.rs:247:48
    |
247 |             let _ = c::CloseHandle(self.handle.as_ptr());
    |                                                ^^^^^^ method not found in `*mut libc::c_void`
    |
    = note: try using `<*const T>::as_ref()` to get a reference to the type behind the pointer: https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref
    = note: using `<*const T>::as_ref()` on a pointer which is unaligned or points to invalid or uninitialized memory is undefined behavior
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to 7 previous errors
Build completed unsuccessfully in 0:00:19
