`
warning: variant is never constructed: `Some`
 --> src/main.rs:5:5
  |
5 |     Some(*const T), // Can also use PhantomData<T>
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: Error finalizing incremental compilation session directory `/tmp/a/target/debug/incremental/a-3ibewktm5f4s7/s-frdnn34p4t-5wozmt-working`: No such file or directory (os error 2)

warning: 2 warnings emitted

error: internal compiler error: broken MIR in DefId(0:6 ~ a[e6bb]::foo[0]) (NoSolution): could not prove TraitPredicate(<[O<B>] as std::cmp::PartialEq>)
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:258:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13
stack backtrace:
   0:     0x7fb253922b30 - std::backtrace_rs::backtrace::libunwind::trace::he85dfb3ae4206056
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x7fb253922b30 - std::backtrace_rs::backtrace::trace_unsynchronized::h1ad28094d7b00c21
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x7fb253922b30 - std::sys_common::backtrace::_print_fmt::h901b54610713cd21
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys_common/backtrace.rs:79
   3:     0x7fb253922b30 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb0ad78ee1571f7e0
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys_common/backtrace.rs:58
   4:     0x7fb253990e7c - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/core/src/fmt/mod.rs:1080
   5:     0x7fb253914f07 - std::io::Write::write_fmt::hf7b7d7b243f84a36
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/io/mod.rs:1517
   6:     0x7fb253927880 - std::sys_common::backtrace::_print::hd093978a5287b8ff
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys_common/backtrace.rs:61
   7:     0x7fb253927880 - std::sys_common::backtrace::print::h20f46787581d56d7
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys_common/backtrace.rs:48
   8:     0x7fb253927880 - std::panicking::default_hook::{{closure}}::h486cbb4b82ffc357
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:208
   9:     0x7fb253927538 - std::panicking::default_hook::h4190c9e3edd4d591
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:227
  10:     0x7fb2541bb8d4 - rustc_driver::report_ice::h0baa69c9fa359ad1
  11:     0x7fb2539280b6 - std::panicking::rust_panic_with_hook::h72e78719cdda225c
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:581
  12:     0x7fb253927c39 - std::panicking::begin_panic_handler::{{closure}}::h8bd07dbd34150a96
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:484
  13:     0x7fb253922fbc - std::sys_common::backtrace::__rust_end_short_backtrace::hdb6b3066ad29028a
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys_common/backtrace.rs:153
  14:     0x7fb253927bf9 - rust_begin_unwind
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:483
  15:     0x7fb253927bab - std::panicking::begin_panic_fmt::hc19e75fd2b7bc39b
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/panicking.rs:437
  16:     0x7fb2572fd471 - rustc_errors::HandlerInner::flush_delayed::hc01570dc9a0301e8
  17:     0x7fb2572f9dd1 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h2fa2449267c97abc
  18:     0x7fb2541df306 - core::ptr::drop_in_place::h3ccf454d686c35e3
  19:     0x7fb2541e9d36 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h9871ec79020d3d08
  20:     0x7fb2541cdb4d - core::ptr::drop_in_place::h37be4f3688653901
  21:     0x7fb2541bfd31 - rustc_span::with_source_map::he9a81669cd160b82
  22:     0x7fb2541eebd8 - rustc_interface::interface::create_compiler_and_run::h06c2389085f2a79e
  23:     0x7fb2541c02aa - rustc_span::with_session_globals::h3a06e50e1db00721
  24:     0x7fb2541f148d - std::sys_common::backtrace::__rust_begin_short_backtrace::h6f73648415d0d9ce
  25:     0x7fb2541621ce - core::ops::function::FnOnce::call_once{{vtable.shim}}::h213be4bbd07cfb62
  26:     0x7fb25393796a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h1080dfe0ef616bdf
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/alloc/src/boxed.rs:1042
  27:     0x7fb25393796a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd2747e1f2d5cec32
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/alloc/src/boxed.rs:1042
  28:     0x7fb25393796a - std::sys::unix::thread::Thread::new::thread_start::hd0f336b4ef6808a7
                               at /rustc/f68e08933d8f519a9655934fedebbc509661b219/library/std/src/sys/unix/thread.rs:87
  29:     0x7fb2538343e9 - start_thread
  30:     0x7fb253751293 - __GI___clone
  31:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (f68e08933 2020-09-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `a`
