
thread \'rustc\' panicked at \'assertion failed: pos.checked_add(num_bytes).unwrap() <= self.mapped_file.len()\', /cargo/registry/src/github.com-1ecc6299db9ec823/measureme-0.3.0/src/mmap_serialization_sink.rs:38:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (ed8b708c1 2019-09-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z self-profile=/tmp/.tmp1D64N5/self-profile-output -Z self-profile-events=all -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

thread \'rustc\' panicked at \'index 1073741840 out of range for slice of length 1073741824\', src/libcore/slice/mod.rs:2583:5
stack backtrace:
   0:     0x7fd1648a2b04 - backtrace::backtrace::libunwind::trace::h61b15987b9420dc8
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1:     0x7fd1648a2b04 - backtrace::backtrace::trace_unsynchronized::h944547918bca7d09
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2:     0x7fd1648a2b04 - std::sys_common::backtrace::_print_fmt::hf631db7a19c7ecfe
                               at src/libstd/sys_common/backtrace.rs:76
   3:     0x7fd1648a2b04 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h356b821d79ead967
                               at src/libstd/sys_common/backtrace.rs:60
   4:     0x7fd1648db14c - core::fmt::write::haa7725ecee710b81
                               at src/libcore/fmt/mod.rs:1030
   5:     0x7fd164896d27 - std::io::Write::write_fmt::h677b5c4b9e48abad
                               at src/libstd/io/mod.rs:1412
   6:     0x7fd1648a7335 - std::sys_common::backtrace::_print::hdc27c79deedd181f
                               at src/libstd/sys_common/backtrace.rs:64
   7:     0x7fd1648a7335 - std::sys_common::backtrace::print::h0bb3a218c68a1b38
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7fd1648a7335 - std::panicking::default_hook::{{closure}}::h9160d687734b4c2f
                               at src/libstd/panicking.rs:196
   9:     0x7fd1648a7026 - std::panicking::default_hook::h298b832ea14df44f
                               at src/libstd/panicking.rs:210
  10:     0x7fd164ddcf63 - rustc_driver::report_ice::hce2a6b74528a3743
  11:     0x7fd1648a7b1c - std::panicking::rust_panic_with_hook::h7c6406c2637b219f
                               at src/libstd/panicking.rs:477
  12:     0x7fd1648a75d2 - std::panicking::continue_panic_fmt::h8e5e175fd262b206
                               at src/libstd/panicking.rs:380
  13:     0x7fd1648a74c6 - rust_begin_unwind
                               at src/libstd/panicking.rs:307
  14:     0x7fd1648d4ada - core::panicking::panic_fmt::h864c751c34920017
                               at src/libcore/panicking.rs:85
  15:     0x7fd1648d5216 - core::slice::slice_index_len_fail::h5579666bd5db7c44
                               at src/libcore/slice/mod.rs:2583
  16:     0x7fd166856244 - <measureme::mmap_serialization_sink::MmapSerializationSink as core::ops::drop::Drop>::drop::hfdb5ca3a69a780c5
  17:     0x7fd164de1948 - alloc::sync::Arc<T>::drop_slow::h1e9b9b3bcf38be1c
  18:     0x7fd164de1c3e - alloc::sync::Arc<T>::drop_slow::h2d5c08f8d08065e0
  19:     0x7fd164df4f15 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h226d778706bf29d2
  20:     0x7fd164db01bc - core::ptr::real_drop_in_place::h78ba5d2f1d1836e8
  21:     0x7fd164daa59c - rustc_interface::interface::run_compiler_in_existing_thread_pool::hf9ebc0f8adadba07
[...]
