
thread 'rustc' panicked at 'Diagnostic { level: Fatal, message: "Expected one of: `before_each, after_each, before, after, it, bench, failing, describe, ignore`, but found: `make_test`", code: None, span: MultiSpan { primary_spans: [Span { lo: BytePos(3788), hi: BytePos(3789), expn_id: ExpnId(4294967295) }], span_labels: [] }, children: [] }', /home/ethanbro/.cargo/registry/src/github.com-1ecc6299db9ec823/stainless-0.1.10/src/parse.rs:192
stack backtrace:
   1:     0x7fd10d13637a - std::sys::imp::backtrace::tracing::imp::write::h944c02ac40aee2d7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7fd10d14522f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:247
   3:     0x7fd10d144dcd - std::panicking::default_hook::h88ffbc5922643264
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:257
   4:     0x7fd10d1456d7 - std::panicking::rust_panic_with_hook::ha5aed1dfc0e220e3
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:451
   5:     0x7fd10d145564 - std::panicking::begin_panic::hc066339e2fdc17d1
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:413
   6:     0x7fd10d145489 - std::panicking::begin_panic_fmt::h5912b2d2df332044
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:397
   7:     0x7fd1023d1ec9 - stainless::parse::illegal::h78d6c0bb3700bfc2
                        at /home/ethanbro/.cargo/registry/src/github.com-1ecc6299db9ec823/stainless-0.1.10/src/parse.rs:192
   8:     0x7fd1023d071d - <stainless::describe::DescribeState as stainless::parse::Parse<(syntax_pos::Span, &'a mut syntax::ext::base::ExtCtxt<'b>, core::option::Option<syntax::ast::Ident>)>>::parse::h3918a9d515a047ed
                        at /home/ethanbro/.cargo/registry/src/github.com-1ecc6299db9ec823/stainless-0.1.10/src/parse.rs:174
   9:     0x7fd1023cf2d0 - stainless::describe::describe::h83ec5748187cf358
                        at /home/ethanbro/.cargo/registry/src/github.com-1ecc6299db9ec823/stainless-0.1.10/src/describe.rs:68
  10:     0x7fd1023d6c5c - core::ops::Fn::call::h22c5bd8d81d657c4
  11:     0x7fd10238acd8 - <F as syntax::ext::base::IdentMacroExpander>::expand::h164c94139d109097
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libsyntax/ext/base.rs:241
  12:     0x7fd106234cdb - syntax::ext::expand::MacroExpander::expand_invoc::hde3927bf760b3842
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libsyntax/ext/expand.rs:448
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libsyntax/ext/expand.rs:332
  13:     0x7fd1062326ec - syntax::ext::expand::MacroExpander::expand::hd3e73837bd4e019c
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libsyntax/ext/expand.rs:270
  14:     0x7fd1062318ea - syntax::ext::expand::MacroExpander::expand_crate::hdb654461100f1c42
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libsyntax/ext/expand.rs:209
  15:     0x7fd10d4cf62f - rustc_driver::driver::phase_2_configure_and_expand::{{closure}}::h7e995ed56b5e4ea9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:684
  16:     0x7fd10d4c8e4d - rustc_driver::driver::phase_2_configure_and_expand::hf89d8ed289448ca7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc/util/common.rs:34
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:647
  17:     0x7fd10d4c22bf - rustc_driver::driver::compile_input::h8e119234b60571d5
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/driver.rs:123
  18:     0x7fd10d509cd8 - rustc_driver::run_compiler::hbdfc4f84e2e0f4b9
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:221
  19:     0x7fd10d425158 - std::panicking::try::do_call::hf679f17bf3b43b0b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1116
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:137
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/librustc_driver/lib.rs:1050
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:295
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:356
  20:     0x7fd10d14fc1a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
  21:     0x7fd10d4471b8 - <F as alloc::boxed::FnBox<A>>::call_box::h506fb5d7b8891cd4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/thread/mod.rs:287
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:595
  22:     0x7fd10d144094 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/liballoc/boxed.rs:605
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/sys/unix/thread.rs:84
  23:     0x7fd10537d183 - start_thread
  24:     0x7fd10cdf637c - clone
  25:                0x0 - <unknown>
