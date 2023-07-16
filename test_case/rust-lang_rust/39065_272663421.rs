
failures:

---- sys::imp::ext::ffi::OsStrExt::as_bytes_0 stdout ----
	error[E0432]: unresolved import `std::os::ffi::OsStrExt`
 --> <anon>:5:5
  |
5 | use std::os::ffi::OsStrExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^ Could not find `ffi` in `os`

error: no method named `as_bytes` found for type `&std::ffi::OsStr` in the current scope
 --> <anon>:8:20
  |
8 | let bytes = os_str.as_bytes();
  |                    ^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::os::ext::ffi::OsStrExt;`

error: aborting due to previous error(s)

thread 'sys::imp::ext::ffi::OsStrExt::as_bytes_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- sys::imp::ext::ffi::OsStrExt::from_bytes_0 stdout ----
	error[E0432]: unresolved import `std::os::ffi::OsStrExt`
 --> <anon>:5:5
  |
5 | use std::os::ffi::OsStrExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^ Could not find `ffi` in `os`

error: no associated item named `from_bytes` found for type `std::ffi::OsStr` in the current scope
 --> <anon>:8:14
  |
8 | let os_str = OsStr::from_bytes(bytes);
  |              ^^^^^^^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::os::ext::ffi::OsStrExt;`

error: aborting due to previous error(s)

thread 'sys::imp::ext::ffi::OsStrExt::from_bytes_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203

---- sys::imp::ext::ffi::OsStringExt::from_vec_0 stdout ----
	error[E0432]: unresolved import `std::os::ffi::OsStringExt`
 --> <anon>:5:5
  |
5 | use std::os::ffi::OsStringExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ Could not find `ffi` in `os`

error: no associated item named `from_vec` found for type `std::ffi::OsString` in the current scope
 --> <anon>:8:17
  |
8 | let os_string = OsString::from_vec(bytes);
  |                 ^^^^^^^^^^^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::os::ext::ffi::OsStringExt;`

error: aborting due to previous error(s)

thread 'sys::imp::ext::ffi::OsStringExt::from_vec_0' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:203

---- sys::imp::ext::ffi::OsStringExt::into_vec_0 stdout ----
	error[E0432]: unresolved import `std::os::ffi::OsStringExt`
 --> <anon>:5:5
  |
5 | use std::os::ffi::OsStringExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^ Could not find `ffi` in `os`

error: no method named `into_vec` found for type `std::ffi::OsString` in the current scope
 --> <anon>:9:23
  |
9 | let bytes = os_string.into_vec();
  |                       ^^^^^^^^
  |
  = help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
  = help: candidate #1: `use std::os::ext::ffi::OsStringExt;`

error: aborting due to previous error(s)
