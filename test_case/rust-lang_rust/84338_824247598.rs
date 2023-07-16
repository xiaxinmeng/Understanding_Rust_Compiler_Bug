plain
    Finished release [optimized] target(s) in 3m 29s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit code: 254)
  --- stderr
  thread 'rustc' panicked at 'The pointer has to be unique', /checkout/library/alloc/src/rc.rs:1138:9

  error: internal compiler error: unexpected panic

  note: the compiler unexpectedly panicked. this is a bug.
  note: the compiler unexpectedly panicked. this is a bug.

  note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

  note: rustc 1.53.0-nightly (f805859ee 2021-04-21) running on x86_64-unknown-linux-gnu

  note: compiler flags: -Z macro-backtrace -Z force-unstable-if-unmarked -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro
  query stack during panic:
  end of query stack
  end of query stack
  thread 'rustc' panicked at 'The pointer has to be unique', /checkout/library/alloc/src/rc.rs:1138:9
  stack backtrace:
     0:     0x7f2ab84754e0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9d3c9e489717f97d
     1:     0x7f2ab84e68fd - core::fmt::write::hb8bdd6c6bccb681f
     2:     0x7f2ab8469855 - std::io::Write::write_fmt::ha35a665cc1270480
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:04:44
     3:     0x7f2ab84798d7 - std::panicking::default_hook::{{closure}}::h7d7fb73fd85d7f76
     4:     0x7f2ab84792db - std::panicking::default_hook::h740070dfec8c35e3
     5:     0x7f2ab8f5559d - rustc_driver::report_ice::haa538df265ab06a3
     6:     0x7f2ab847a07a - std::panicking::rust_panic_with_hook::hc4eb8825e604532b
     7:     0x7f2ab8479bd7 - std::panicking::begin_panic_handler::{{closure}}::hf2766b2bb19805bc
     8:     0x7f2ab847597c - std::sys_common::backtrace::__rust_end_short_backtrace::h4b46cdd04ec9b444
     9:     0x7f2ab8479b69 - rust_begin_unwind
    10:     0x7f2ab843df51 - core::panicking::panic_fmt::h40fd73f0042911f3
    11:     0x7f2ab843de9d - core::panicking::panic::h4b917cec51ba7ec2
    12:     0x7f2ab8f5b9e2 - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::hff0876f84c79fce2
    13:     0x7f2ab8f57510 - rustc_span::with_source_map::h812cf7ddaab85b05
    14:     0x7f2ab8f684c4 - scoped_tls::ScopedKey<T>::set::h9a3e56bcc0e6224e
    15:     0x7f2ab8f579e3 - rustc_span::with_session_globals::h81286970a79c2e3a
    16:     0x7f2ab8f6bb9d - std::sys_common::backtrace::__rust_begin_short_backtrace::hf017091316cc771c
    17:     0x7f2ab8f6be89 - std::panicking::try::hd235c5662b9984b4
    18:     0x7f2ab8fbc073 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h310cc146c3291b26
    19:     0x7f2ab848a788 - std::sys::unix::thread::Thread::new::thread_start::h24c663c639977be7
    20:     0x7f2ab313e6db - start_thread
    21:     0x7f2ab810a71f - __clone
    22:                0x0 - <unknown>
  error: internal compiler error: unexpected panic

  note: the compiler unexpectedly panicked. this is a bug.


  note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

  note: rustc 1.53.0-nightly (f805859ee 2021-04-21) running on x86_64-unknown-linux-gnu

  note: compiler flags: -Z macro-backtrace -Z force-unstable-if-unmarked -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro
  query stack during panic:
  end of query stack
  thread panicked while panicking. aborting.
  thread panicked while panicking. aborting.
  rustc exited with signal: 4 (core dumped)
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
