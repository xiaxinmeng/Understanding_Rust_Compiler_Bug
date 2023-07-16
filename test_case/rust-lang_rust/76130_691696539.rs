plain
    Checking object v0.20.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.13.0
 Documenting std v0.0.0 (/checkout/library/std)
thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13

error: internal compiler error: unexpected panic


error: Unrecognized option: 'markdown-css'

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:961:13
stack backtrace:
   0:     0x7f3f9c321ad0 - std::backtrace_rs::backtrace::libunwind::trace::h14e81b0c16b543a7
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x7f3f9c321ad0 - std::backtrace_rs::backtrace::trace_unsynchronized::hc9e39e172835ff32
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x7f3f9c321ad0 - std::sys_common::backtrace::_print_fmt::h31994df1c8c77b8f
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys_common/backtrace.rs:79
   3:     0x7f3f9c321ad0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he74e98acff43ec99
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys_common/backtrace.rs:58
   4:     0x7f3f9c38fcfc - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/core/src/fmt/mod.rs:1080
   5:     0x7f3f9c313eb7 - std::io::Write::write_fmt::hea6289e6649ff508
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/io/mod.rs:1517
   6:     0x7f3f9c326810 - std::sys_common::backtrace::_print::h8cc65937fea24014
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys_common/backtrace.rs:61
   7:     0x7f3f9c326810 - std::sys_common::backtrace::print::hff6a0ebb96535e34
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys_common/backtrace.rs:48
   8:     0x7f3f9c326810 - std::panicking::default_hook::{{closure}}::h71b9700c5dda1b1d
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:208
   9:     0x7f3f9c32655c - std::panicking::default_hook::h0c46e634c06c69b6
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:227
  10:     0x7f3f9cbbe988 - rustc_driver::report_ice::h6e77b939115cba92
  11:     0x7f3f9c326fb8 - std::panicking::rust_panic_with_hook::he52576a0c2093334
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:581
  12:     0x7f3f9c326b69 - std::panicking::begin_panic_handler::{{closure}}::h411c800b810762b2
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:484
  13:     0x7f3f9c321f5c - std::sys_common::backtrace::__rust_end_short_backtrace::ha657a272019e6aad
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys_common/backtrace.rs:153
  14:     0x7f3f9c326b29 - rust_begin_unwind
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:483
  15:     0x7f3f9c326adb - std::panicking::begin_panic_fmt::h8679424505ca4cf5
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/panicking.rs:437
  16:     0x7f3f9fcd63e1 - rustc_errors::HandlerInner::flush_delayed::h05a1088434136227
  17:     0x7f3f9fcd2d41 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h5f31ac2fc3b0316d
  18:     0x5611b2593c06 - core::ptr::drop_in_place::hc3dc037261968ae2
  19:     0x5611b258cbe1 - core::ptr::drop_in_place::h525f12be64433844
  20:     0x5611b2576cd1 - scoped_tls::ScopedKey<T>::set::hba233388ee0ecc21
  21:     0x5611b25869a7 - std::panicking::try::h3e10ed14228ced02
  22:     0x5611b23e801b - rustc_driver::catch_fatal_errors::h09e1229f08e09da9
  23:     0x5611b249aa60 - rustdoc::doctest::make_test::h17350a0df985b588
  24:     0x5611b24a5177 - <rustdoc::html::markdown::CodeBlocks<I> as core::iter::traits::iterator::Iterator>::next::h93846ad4c55fd073
  25:     0x5611b24ab3e9 - <rustdoc::html::markdown::Footnotes<I> as core::iter::traits::iterator::Iterator>::next::h7ef22dbed47b14ac
  26:     0x5611b26797f0 - pulldown_cmark::html::HtmlWriter<I,W>::run::h0bbd83f4b26c8d45
  27:     0x5611b268ac02 - pulldown_cmark::html::push_html::hb87b419c0ce1d04c
  28:     0x5611b24ae4a2 - rustdoc::html::markdown::Markdown::into_string::h9cf35b8ad42db44e
  29:     0x5611b25eb560 - rustdoc::html::render::document_full::h810f0ebb4f719107
  30:     0x5611b25ea6ff - rustdoc::html::render::document::h8f735b64e17df7f9
  31:     0x5611b25e668a - rustdoc::html::render::print_item::h3f9ef42e5eadfd1f
  32:     0x5611b241ab59 - rustdoc::html::layout::render::h1e1ccf0df8415a32
  33:     0x5611b25de6d8 - rustdoc::html::render::Context::render_item::h345b28833191c4c1
  34:     0x5611b25d2a38 - <rustdoc::html::render::Context as rustdoc::formats::renderer::FormatRenderer>::item::h92a8c69bb685602c
  35:     0x5611b24df9b7 - rustdoc::formats::renderer::run_format::h2b1f6a3f510dc6bf
  36:     0x5611b23acee2 - rustdoc::main_options::hd01215a626cac169
  37:     0x5611b2576364 - scoped_tls::ScopedKey<T>::set::h09c7e2eb0b4bd82b
  38:     0x5611b23e9e05 - std::sys_common::backtrace::__rust_begin_short_backtrace::h8806aaec260767b9
  39:     0x5611b268e4ae - core::ops::function::FnOnce::call_once{{vtable.shim}}::h99e38bf85be8fd83
  40:     0x7f3f9c33672a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h462e787c72080422
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/alloc/src/boxed.rs:1042
  41:     0x7f3f9c33672a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd438caf7b24ea9a1
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/alloc/src/boxed.rs:1042
  42:     0x7f3f9c33672a - std::sys::unix::thread::Thread::new::thread_start::h653bd369f002a164
                               at /rustc/e853b03937d798e4e5b1d6f72b26dc9248b4a88d/library/std/src/sys/unix/thread.rs:87
  43:     0x7f3f9c0798ca - start_thread
  44:     0x7f3f9bbc9abd - clone
error: internal compiler error: unexpected panic


error: Unrecognized option: 'markdown-css'

thread panicked while processing panic. aborting.
error: Could not document `std`.
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.48.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-02845e98e30d645a.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-ca56c52335bd9d26.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-d32ab0a86fb4bc9d.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-df5d70bea9ed06b2.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-4d43f9264df959a3.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-3c5d9f1d5bfd1ead.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-d20e443d60dce0b0.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-c9b0a6d53b4cca7f.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libobject-dede933fe277a3c1.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-fe473151f06a12d7.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-39726c5f0cbaa225.rmeta --extern profiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-7b0ee4bcbd9ae1ee.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-3e5fe049c0785672.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-09b0e00dc9c7762c.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.48.0-nightly
  (e853b0393
  2020-09-13)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.48.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:20:30
Build completed unsuccessfully in 0:20:30
== clock drift check ==
  local time: Sun Sep 13 16:58:52 UTC 2020
  network time: Sun, 13 Sep 2020 16:58:52 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3868) (python)
