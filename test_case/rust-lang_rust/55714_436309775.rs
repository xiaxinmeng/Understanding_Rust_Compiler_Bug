
command: "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/vext01/source/rust/src/test/ui-fulldeps/invalid-punct-ident-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/invalid-punct-ident-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/vext01/source/rust/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/invalid-punct-ident-3/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'procedural macro API is used outside of a procedural macro', libproc_macro/lib.rs:1314:13
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:476
   5: std::panicking::begin_panic
             at ./src/libstd/panicking.rs:410
   6: proc_macro::Span::call_site
             at libproc_macro/lib.rs:1314
             at libproc_macro/lib.rs:297
   7: invalid_punct_ident::invalid_raw_ident
   8: std::panicking::try::do_call
             at libsyntax_ext/proc_macro_impl.rs:73
             at ./src/libstd/panic.rs:319
             at ./src/libstd/panicking.rs:310
   9: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  10: <std::thread::local::LocalKey<T>>::with
             at ./src/libstd/panicking.rs:289
             at ./src/libstd/panic.rs:398
             at libsyntax_ext/proc_macro_impl.rs:73
             at ./src/libproc_macro/lib.rs:1300
             at ./src/libstd/thread/local.rs:309
             at ./src/libstd/thread/local.rs:255
  11: <syntax_ext::proc_macro_impl::BangProcMacro as syntax::ext::base::ProcMacro>::expand
             at ./src/libproc_macro/lib.rs:1284
             at libsyntax_ext/proc_macro_impl.rs:72
  12: syntax::ext::expand::MacroExpander::expand_invoc
             at libsyntax/ext/expand.rs:857
             at libsyntax/ext/expand.rs:525
  13: syntax::ext::expand::MacroExpander::expand_fragment
             at libsyntax/ext/expand.rs:361
  14: syntax::ext::expand::MacroExpander::expand_crate
             at libsyntax/ext/expand.rs:288
  15: rustc_driver::driver::phase_2_configure_and_expand_inner::{{closure}}
             at librustc_driver/driver.rs:1011
             at ./src/librustc/util/common.rs:163
             at ./src/librustc/util/common.rs:157
             at librustc_driver/driver.rs:1010
  16: rustc_driver::driver::phase_2_configure_and_expand_inner
             at ./src/librustc/util/common.rs:163
             at ./src/librustc/util/common.rs:157
             at librustc_driver/driver.rs:963
  17: rustc_driver::driver::compile_input
             at librustc_driver/driver.rs:766
             at librustc_driver/driver.rs:186
  18: rustc_driver::run_compiler
             at librustc_driver/lib.rs:552
             at librustc_driver/lib.rs:474
             at librustc_driver/driver.rs:76
             at /home/vext01/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at librustc_driver/driver.rs:75
             at librustc_driver/lib.rs:473
  19: <scoped_tls::ScopedKey<T>>::set
             at librustc_driver/lib.rs:1725
             at librustc_driver/lib.rs:189
             at /home/vext01/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at ./src/libsyntax/lib.rs:123
             at /home/vext01/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  20: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at ./src/libsyntax/lib.rs:122
             at librustc_driver/lib.rs:188
             at librustc_driver/lib.rs:1640
             at ./src/libstd/panic.rs:319
  21: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  22: rustc_driver::run
             at ./src/libstd/panicking.rs:289
             at ./src/libstd/panic.rs:398
             at librustc_driver/lib.rs:1554
             at librustc_driver/lib.rs:1565
             at librustc_driver/lib.rs:1639
             at librustc_driver/lib.rs:187
  23: rustc_driver::main
             at librustc_driver/lib.rs:1718
  24: std::rt::lang_start::{{closure}}
             at ./src/libstd/rt.rs:74
  25: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  26: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  27: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:398
             at libstd/rt.rs:58
  28: main
  29: __libc_start_main
  30: _start
{"message":"proc macro panicked","code":null,"level":"error","spans":[{"file_name":"/home/vext01/source/rust/src/test/ui-fulldeps/invalid-punct-ident-3.rs","byte_start":552,"byte_end":573,"line_start":16,"line_end":16,"column_start":1,"column_end":22,"is_primary":true,"text":[{"text":"invalid_raw_ident!(); //~ ERROR proc macro panicked","highlight_start":1,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"message: procedural macro API is used outside of a procedural macro","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: proc macro panicked\n  --> /home/vext01/source/rust/src/test/ui-fulldeps/invalid-punct-ident-3.rs:16:1\n   |\nLL | invalid_raw_ident!(); //~ ERROR proc macro panicked\n   | ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: message: procedural macro API is used outside of a procedural macro\n\n"}
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
