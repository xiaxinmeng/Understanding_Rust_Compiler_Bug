
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-ordinary-web-framework src/main.rs`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/libcore/slice/mod.rs:2716:10
stack backtrace:
   0:        0x10786db35 - backtrace::backtrace::libunwind::trace::h6ba66c89851a4029
                               at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1:        0x10786db35 - backtrace::backtrace::trace_unsynchronized::h0247c0aa34f5e559
                               at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2:        0x10786db35 - std::sys_common::backtrace::_print_fmt::h189d0eab826d8a40
                               at src/libstd/sys_common/backtrace.rs:77
   3:        0x10786db35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd44c270fa289156b
                               at src/libstd/sys_common/backtrace.rs:61
   4:        0x107887750 - core::fmt::write::h0bf6a2859f20c912
                               at src/libcore/fmt/mod.rs:1028
   5:        0x107869dab - std::io::Write::write_fmt::h8dfb96299973a9de
                               at src/libstd/io/mod.rs:1412
   6:        0x10786f6e3 - std::sys_common::backtrace::_print::hb893b433353020bb
                               at src/libstd/sys_common/backtrace.rs:65
   7:        0x10786f6e3 - std::sys_common::backtrace::print::he2bf38c193655d04
                               at src/libstd/sys_common/backtrace.rs:50
   8:        0x10786f6e3 - std::panicking::default_hook::{{closure}}::hd90c9bc4019b5a15
                               at src/libstd/panicking.rs:189
   9:        0x10786f3fa - std::panicking::default_hook::h75bf8748eabc8283
                               at src/libstd/panicking.rs:206
  10:        0x10786fd87 - std::panicking::rust_panic_with_hook::h8ae59d5211838c52
                               at src/libstd/panicking.rs:469
  11:        0x10786f8cd - std::panicking::continue_panic_fmt::h4da5ca5042844a5b
                               at src/libstd/panicking.rs:376
  12:        0x10786f7c9 - rust_begin_unwind
                               at src/libstd/panicking.rs:303
  13:        0x10788506f - core::panicking::panic_fmt::h409f036532c7f608
                               at src/libcore/panicking.rs:84
  14:        0x107885029 - core::panicking::panic_bounds_check::h212ac8f7472ac878
                               at src/libcore/panicking.rs:61
  15:        0x1078446c9 - <usize as core::slice::SliceIndex<[T]>>::index::h115411ca8fab9edb
                               at /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/libcore/slice/mod.rs:2716
  16:        0x107844375 - core::slice::<impl core::ops::index::Index<I> for [T]>::index::hfd309f2ad4342967
                               at /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/libcore/slice/mod.rs:2567
  17:        0x107863cbb - <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index::ha80ba6dadce203ca
                               at /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/liballoc/vec.rs:1816
  18:        0x107845d73 - rust_ordinary_web_framework::parser::divide_request::h2cf22e46df1eaa18
                               at src/parser.rs:46
  19:        0x10784590e - rust_ordinary_web_framework::parser::arrange_request::h438cbf238e793327
                               at src/parser.rs:34
  20:        0x10784588a - rust_ordinary_web_framework::parser::parse_request::h7d1f392e8187e5ef
                               at src/parser.rs:22
  21:        0x107862959 - rust_ordinary_web_framework::server::instance_listen::h7559f108dd08f826
                               at src/server.rs:22
  22:        0x10784359f - rust_ordinary_web_framework::main::h8888a4b960d98f89
                               at src/main.rs:10
  23:        0x107861492 - std::rt::lang_start::{{closure}}::hf0d8424e7f5557e5
                               at /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/libstd/rt.rs:61
  24:        0x10786f7a8 - std::rt::lang_start_internal::{{closure}}::h5e52d19b097f7f73
                               at src/libstd/rt.rs:48
  25:        0x10786f7a8 - std::panicking::try::do_call::h3f6781fb79ce8d48
                               at src/libstd/panicking.rs:288
  26:        0x1078721df - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  27:        0x10787012e - std::panicking::try::ha9dc40392366df8f
                               at src/libstd/panicking.rs:267
  28:        0x10787012e - std::panic::catch_unwind::h6798d3615d8ee27d
                               at src/libstd/panic.rs:396
  29:        0x10787012e - std::rt::lang_start_internal::h2ed9f769258b4275
                               at src/libstd/rt.rs:47
  30:        0x107861472 - std::rt::lang_start::hc6e3b5b1be52f9c1
                               at /rustc/c27f7568bc74c418996892028a629eed5a7f5f00/src/libstd/rt.rs:61
  31:        0x1078435d2 - rust_ordinary_web_framework::main::h8888a4b960d98f89
make: *** [run] Error 101
