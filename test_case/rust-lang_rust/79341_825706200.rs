plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.39
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unknown `doc` attribute `cfg_hide`
   |
   |
61 |       doc(cfg_hide(
   |  _________^
62 | |         not(test),
63 | |         target_pointer_width = "16",
64 | |         target_pointer_width = "32",
80 | |         target_has_atomic_load_store = "ptr",
81 | |     ))
   | |_____^
   |
   |
   = note: `-D invalid-doc-attributes` implied by `-D warnings`
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>

error: aborting due to previous error

