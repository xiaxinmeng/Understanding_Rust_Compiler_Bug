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
   0:     0x7f72d9726ad0 - std::backtrace_rs::backtrace::libunwind::trace::h14e81b0c16b543a7
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
   1:     0x7f72d9726ad0 - std::backtrace_rs::backtrace::trace_unsynchronized::hc9e39e172835ff32
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/../../backtrace/src/backtrace/mod.rs:66
   2:     0x7f72d9726ad0 - std::sys_common::backtrace::_print_fmt::h31994df1c8c77b8f
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys_common/backtrace.rs:79
   3:     0x7f72d9726ad0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he74e98acff43ec99
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys_common/backtrace.rs:58
   4:     0x7f72d9794cfc - core::fmt::write::h1857a60b204f1b6a
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/core/src/fmt/mod.rs:1080
   5:     0x7f72d9718eb7 - std::io::Write::write_fmt::hea6289e6649ff508
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/io/mod.rs:1517
   6:     0x7f72d972b810 - std::sys_common::backtrace::_print::h8cc65937fea24014
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys_common/backtrace.rs:61
   7:     0x7f72d972b810 - std::sys_common::backtrace::print::hff6a0ebb96535e34
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys_common/backtrace.rs:48
   8:     0x7f72d972b810 - std::panicking::default_hook::{{closure}}::h71b9700c5dda1b1d
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:208
   9:     0x7f72d972b55c - std::panicking::default_hook::h0c46e634c06c69b6
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:227
  10:     0x7f72d9fc2ad8 - rustc_driver::report_ice::h9c03cd00e6aa04cf
  11:     0x7f72d972bfb8 - std::panicking::rust_panic_with_hook::he52576a0c2093334
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:581
  12:     0x7f72d972bb69 - std::panicking::begin_panic_handler::{{closure}}::h411c800b810762b2
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:484
  13:     0x7f72d9726f5c - std::sys_common::backtrace::__rust_end_short_backtrace::ha657a272019e6aad
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys_common/backtrace.rs:153
  14:     0x7f72d972bb29 - rust_begin_unwind
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:483
  15:     0x7f72d972badb - std::panicking::begin_panic_fmt::h8679424505ca4cf5
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/panicking.rs:437
  16:     0x7f72dd0e3441 - rustc_errors::HandlerInner::flush_delayed::h222db002ce0037b9
  17:     0x7f72dd0dfda1 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h1b33bc2a4c0650c5
  18:     0x564f83351f16 - core::ptr::drop_in_place::he2cea8e87623108c
  19:     0x564f83346b61 - core::ptr::drop_in_place::h2f73dcef4259a1cf
  20:     0x564f833348f1 - scoped_tls::ScopedKey<T>::set::h70126529b2de0635
  21:     0x564f833445c7 - std::panicking::try::h323fe65c7727ff59
  22:     0x564f831a643b - rustc_driver::catch_fatal_errors::h4ea612d9b9275ddc
  23:     0x564f832585d0 - rustdoc::doctest::make_test::haed5da7583bac7e0
  24:     0x564f83262ce7 - <rustdoc::html::markdown::CodeBlocks<I> as core::iter::traits::iterator::Iterator>::next::h93846ad4c55fd073
  25:     0x564f83268f59 - <rustdoc::html::markdown::Footnotes<I> as core::iter::traits::iterator::Iterator>::next::h7ef22dbed47b14ac
  26:     0x564f83436c60 - pulldown_cmark::html::HtmlWriter<I,W>::run::h0bbd83f4b26c8d45
  27:     0x564f83448072 - pulldown_cmark::html::push_html::hb87b419c0ce1d04c
  28:     0x564f8326c0a2 - rustdoc::html::markdown::Markdown::into_string::h9cf35b8ad42db44e
  29:     0x564f833a8c00 - rustdoc::html::render::document_full::h810f0ebb4f719107
  30:     0x564f833a7d9f - rustdoc::html::render::document::h8f735b64e17df7f9
  31:     0x564f833a5cbb - rustdoc::html::render::print_item::h3f9ef42e5eadfd1f
  32:     0x564f831d8929 - rustdoc::html::layout::render::h1e1ccf0df8415a32
  33:     0x564f8339bb38 - rustdoc::html::render::Context::render_item::h345b28833191c4c1
  34:     0x564f8338fe98 - <rustdoc::html::render::Context as rustdoc::formats::renderer::FormatRenderer>::item::h92a8c69bb685602c
  35:     0x564f8329d5b7 - rustdoc::formats::renderer::run_format::h22530c0425b532bd
  36:     0x564f8314d831 - rustdoc::run_renderer::h4aab4ec0e93fc609
  37:     0x564f8315035b - rustdoc::main_options::hefffd2f80fce74f5
  38:     0x564f83334144 - scoped_tls::ScopedKey<T>::set::h6e9e2033b8cd5a9f
  39:     0x564f831a85e5 - std::sys_common::backtrace::__rust_begin_short_backtrace::h725d48ec23a65176
  40:     0x564f8344b87e - core::ops::function::FnOnce::call_once{{vtable.shim}}::h1a55041abc1ed589
  41:     0x7f72d973b72a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h462e787c72080422
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/alloc/src/boxed.rs:1042
  42:     0x7f72d973b72a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd438caf7b24ea9a1
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/alloc/src/boxed.rs:1042
  43:     0x7f72d973b72a - std::sys::unix::thread::Thread::new::thread_start::h653bd369f002a164
                               at /rustc/ad8dc9fdc815d1f01fcd2911ef4cbc5ddfdc4ed3/library/std/src/sys/unix/thread.rs:87
  44:     0x7f72d947e8ca - start_thread
  45:     0x7f72d8fceabd - clone
error: internal compiler error: unexpected panic


error: Unrecognized option: 'markdown-css'

thread panicked while processing panic. aborting.
error: Could not document `std`.
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library/std/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="addr2line"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="gimli-symbolize"' --cfg 'feature="miniz_oxide"' --cfg 'feature="object"' --cfg 'feature="panic_unwind"' --cfg 'feature="profiler"' --cfg 'feature="profiler_builtins"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.48.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern addr2line=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libaddr2line-02845e98e30d645a.rmeta --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liballoc-ca56c52335bd9d26.rmeta --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcfg_if-d32ab0a86fb4bc9d.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-df5d70bea9ed06b2.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libcore-4d43f9264df959a3.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-3c5d9f1d5bfd1ead.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/liblibc-d20e443d60dce0b0.rmeta --extern miniz_oxide=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libminiz_oxide-c9b0a6d53b4cca7f.rmeta --extern object=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libobject-dede933fe277a3c1.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-fe473151f06a12d7.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-39726c5f0cbaa225.rmeta --extern profiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libprofiler_builtins-7b0ee4bcbd9ae1ee.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-3e5fe049c0785672.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/release/deps/libunwind-09b0e00dc9c7762c.rmeta -Dwarnings -Winvalid_codeblock_attributes --crate-version '1.48.0-nightly
  (ad8dc9fdc
  2020-09-13)' --cfg backtrace_in_libstd` (exit code: 1)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.48.0" "--index-page" "/checkout/src/doc/index.md"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:20:46
Build completed unsuccessfully in 0:20:46
== clock drift check ==
  local time: Sun Sep 13 02:02:03 UTC 2020
  network time: Sun, 13 Sep 2020 02:02:03 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (4566) (python)
