

Standard Error

   Compiling playground v0.0.1 (file:///playground)
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', /checkout/src/libcore/slice/mod.rs:2091:14
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: internal compiler error: cat_expr Errd
 --> src/main.rs:6:11
  |
6 |   fn main() {
  |  ___________^
7 | |     println!("{}", Red.paint("hi"));
8 | | }
  | |_^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:5
  |
7 |     println!("{}", Red.paint("hi"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:5
  |
7 |     println!("{}", Red.paint("hi"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:14
  |
7 |     println!("{}", Red.paint("hi"));
  |              ^^^^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:20
  |
7 |     println!("{}", Red.paint("hi"));
  |                    ^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:20
  |
7 |     println!("{}", Red.paint("hi"));
  |                    ^^^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:5
  |
7 |     println!("{}", Red.paint("hi"));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:20
  |
7 |     println!("{}", Red.paint("hi"));
  |                    ^^^^^^^^^^^^^^^

error: internal compiler error: no type-dependent def for method call
 --> src/main.rs:7:20
  |
7 |     println!("{}", Red.paint("hi"));
  |                    ^^^^^^^^^^^^^^^

thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', librustc_errors/lib.rs:313:17
stack backtrace:
   0:     0x7f45b95cfe2e - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h15bce005031e9a7c
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f45b95ab926 - std::sys_common::backtrace::print::hf0b8b2cf8ad2a47b
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f45b95a52fd - std::panicking::default_hook::{{closure}}::h58dc056830b721f7
                               at libstd/panicking.rs:211
   3:     0x7f45b95a5070 - std::panicking::default_hook::hb878f870c1cec2ff
                               at libstd/panicking.rs:227
   4:     0x7f45b5d1c12d - rustc::util::common::panic_hook::h64f8a4de5b3a8520
   5:     0x7f45b95a5a83 - std::panicking::rust_panic_with_hook::h02321e9e1ca483ac
                               at libstd/panicking.rs:479
   6:     0x7f45b49aed46 - std::panicking::begin_panic::hc43586f8888489d6
   7:     0x7f45b49c8184 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::hff27d42cab603f90
   8:     0x7f45b99df880 - core::ptr::drop_in_place::hd0ef6e768cbd0c2a
   9:     0x7f45b99f1cd0 - rustc_driver::run_compiler_with_pool::hd8965e4cdc9cedac
  10:     0x7f45b990c64c - <scoped_tls::ScopedKey<T>>::set::h1fbe1c919632854e
  11:     0x7f45b990ca21 - <scoped_tls::ScopedKey<T>>::set::he75885b75a4d67da
  12:     0x7f45b99a8a9a - syntax::with_globals::h0c05f3662565e425
  13:     0x7f45b990b262 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hca986925b703c77e
  14:     0x7f45b95e5db9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:105
  15:     0x7f45b99ee57e - rustc_driver::run::h066cf77e58d574b5
  16:     0x7f45b99fbeba - rustc_driver::main::h0cdb670f01f51016
  17:     0x565139070b52 - std::rt::lang_start::{{closure}}::h862e39fa4f519bc4
  18:     0x7f45b95a54c2 - std::panicking::try::do_call::hb3d83e218e62b9bd
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:310
  19:     0x7f45b95e5db9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:105
  20:     0x7f45b95aa1e5 - std::rt::lang_start_internal::h6d9c0aaba46310ef
                               at libstd/panicking.rs:289
                               at libstd/panic.rs:392
                               at libstd/rt.rs:58
  21:     0x565139070bb3 - main
  22:     0x7f45b91a582f - __libc_start_main
  23:     0x565139070a38 - <unknown>
thread panicked while panicking. aborting.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.


