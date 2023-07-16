text
warning: Error finalizing incremental compilation session directory `/tmp/tmp.RK5UN5mIuz/ice_testcase/target/debug/incremental/ice_testcase-xfn1k7c1h8y/s-fi7fvzf5cc-4ag7eb-working`: No such file or directory (os error 2)

error: internal compiler error: broken MIR in DefId(0:35 ~ ice_testcase[1b24]::problematic_function[0]) (Terminator { source_info: SourceInfo { span: src/lib.rs:67:26: 67:80, scope: scope[0] }, kind: _2 = const <nalgebra::base::matrix::Matrix<f64, nalgebra::base::dimension::U2, nalgebra::base::dimension::U1, <nalgebra::base::default_allocator::DefaultAllocator as nalgebra::base::allocator::Allocator<f64, nalgebra::base::dimension::U2>>::Buffer> as std::convert::Into<nalgebra::geometry::point::Point<f64, nalgebra::base::dimension::U2>>>::into(move _3) -> [return: bb3, unwind: bb1] }): bad arg #0 (nalgebra::base::matrix::Matrix<f64, nalgebra::base::dimension::U2, nalgebra::base::dimension::U1, nalgebra::base::array_storage::ArrayStorage<f64, nalgebra::base::dimension::U2, nalgebra::base::dimension::U1>> <- nalgebra::base::matrix::Matrix<f64, nalgebra::base::dimension::U2, nalgebra::base::dimension::U1, <nalgebra::base::default_allocator::DefaultAllocator as nalgebra::base::allocator::Allocator<f64, nalgebra::base::dimension::U2>>::Buffer>): NoSolution
  --> src/lib.rs:67:72
   |
67 |     let _: Point2<f64> = material_surface_element.map_reference_coords().into();
   |                                                                        ^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
stack backtrace:
   0:     0x7f812a9f15b4 - backtrace::backtrace::libunwind::trace::h0fbcca877b5cc7be
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f812a9f15b4 - backtrace::backtrace::trace_unsynchronized::heee54a145e5e242e
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f812a9f15b4 - std::sys_common::backtrace::_print_fmt::hcd55bcb24d288827
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f812a9f15b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcf782761d696ebc5
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f812aa29cec - core::fmt::write::h6d12a02d697eb964
                               at src/libcore/fmt/mod.rs:1030
   5:     0x7f812a9e5947 - std::io::Write::write_fmt::h92b068839d5f88f6
                               at src/libstd/io/mod.rs:1412
   6:     0x7f812a9f5a5e - std::sys_common::backtrace::_print::hd7ae75a7027ca13e
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f812a9f5a5e - std::sys_common::backtrace::print::he36b145a0f885346
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f812a9f5a5e - std::panicking::default_hook::{{closure}}::h78532abf43437641
                               at src/libstd/panicking.rs:190
   9:     0x7f812a9f5751 - std::panicking::default_hook::hb1a613bdd06f1622
                               at src/libstd/panicking.rs:207
  10:     0x7f812af06ac3 - rustc_driver::report_ice::h114705861dab380f
  11:     0x7f812a9f6230 - std::panicking::rust_panic_with_hook::ha051cd69a44ecdee
                               at src/libstd/panicking.rs:470
  12:     0x7f812cd918e5 - std::panicking::begin_panic::h81d44c05963fa168
  13:     0x7f812cdc8cdc - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h25de78a9e6dff3f2
  14:     0x7f812aed34e6 - core::ptr::real_drop_in_place::h9c6508b592c1d638
  15:     0x7f812aed80a3 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h071d376be54d060e
  16:     0x7f812af1f11c - core::ptr::real_drop_in_place::hbbcadcaa3930f1de
  17:     0x7f812af1a809 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hc0c5fb91eaefa48a
  18:     0x7f812aec95a1 - std::thread::local::LocalKey<T>::with::h47f871d4948616e8
  19:     0x7f812aec24de - scoped_tls::ScopedKey<T>::set::h35e3aee7969efb0c
  20:     0x7f812aee8862 - syntax::with_globals::h9899c44fce69c29e
  21:     0x7f812aec382b - std::sys_common::backtrace::__rust_begin_short_backtrace::h7c08dd05be391ff7
  22:     0x7f812aa06d1a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:81
  23:     0x7f812aede219 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hc69113382c8c4814
  24:     0x7f812a9d787f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h21cb5edd552bea91
                               at /rustc/a44774c3a9739b2eea8923e09d67b14312c78ef3/src/liballoc/boxed.rs:942
  25:     0x7f812aa05740 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb548d9c171babe6b
                               at /rustc/a44774c3a9739b2eea8923e09d67b14312c78ef3/src/liballoc/boxed.rs:942
  26:     0x7f812aa05740 - std::sys_common::thread::start_thread::hb5b09c960584ac18
                               at src/libstd/sys_common/thread.rs:13
  27:     0x7f812aa05740 - std::sys::unix::thread::Thread::new::thread_start::h341c42eadd8ca9e6
                               at src/libstd/sys/unix/thread.rs:79
  28:     0x7f812a944669 - start_thread
  29:     0x7f812a859323 - clone
  30:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (a44774c3a 2019-11-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `ice_testcase`.
