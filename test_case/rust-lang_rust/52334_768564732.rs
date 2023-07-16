blank
ices/52334.sh: fixed with no errors
=== stdout ===
=== stderr ===
warning: `extern` block uses type `CStr`, which is not FFI-safe
 --> <anon>:3:18
  |
3 |     fn meh(blah: Foo);
  |                  ^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes)]` on by default
  = help: consider adding a `#[repr(C)]` or `#[repr(transparent)]` attribute to this struct
  = note: this struct has unspecified layout

warning: 1 warning emitted
