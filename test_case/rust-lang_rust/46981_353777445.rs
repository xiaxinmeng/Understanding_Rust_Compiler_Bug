
    Updating registry `https://github.com/rust-lang/crates.io-index`
warning: manifest has no documentation, homepage or repository.
See http://doc.crates.io/manifest.html#package-metadata for more info.
   Packaging libimagentryannotation v0.5.0 (file:///home/m/archive/development/rust/imag/lib/entry/libimagentryannotation)
   Verifying libimagentryannotation v0.5.0 (file:///home/m/archive/development/rust/imag/lib/entry/libimagentryannotation)
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading libimagentrylink v0.5.0
   Compiling num-traits v0.1.41
   Compiling cfg-if v0.1.2
   Compiling unicode-xid v0.0.4
   Compiling rustc-demangle v0.1.5
   Compiling glob v0.2.11
   Compiling itoa v0.3.4
   Compiling boolinator v2.4.0
   Compiling dtoa v0.4.2
   Compiling semver-parser v0.7.0
   Compiling lazy_static v0.2.11
   Compiling cc v1.0.3
   Compiling is-match v0.1.0
   Compiling libc v0.2.34
   Compiling serde v1.0.25
   Compiling lazy_static v1.0.0
   Compiling same-file v0.1.3
   Compiling percent-encoding v1.0.1
   Compiling gcc v0.3.54
   Compiling quote v0.3.15
   Compiling rustc-serialize v0.3.24
   Compiling either v1.4.0
   Compiling version v2.0.1
   Compiling utf8-ranges v1.0.0
   Compiling log v0.3.8
   Compiling regex-syntax v0.4.1
   Compiling unicode-normalization v0.1.5
   Compiling ansi_term v0.10.2
   Compiling void v1.0.2
   Compiling matches v0.1.6
   Compiling synom v0.11.3
   Compiling walkdir v1.0.7
   Compiling semver v0.8.0
   Compiling memchr v2.0.1
   Compiling time v0.1.38
   Compiling rand v0.3.18
   Compiling itertools v0.7.4
   Compiling unreachable v1.0.0
   Compiling unicode-bidi v0.3.4
   Compiling libimagerror v0.5.0
   Compiling syn v0.11.11
   Compiling backtrace-sys v0.1.16
   Compiling aho-corasick v0.6.4
   Compiling rust-crypto v0.2.36
   Compiling thread_local v0.3.5
   Compiling tempfile v2.2.0
   Compiling idna v0.1.4
   Compiling regex v0.2.3
   Compiling url v1.6.0
   Compiling toml v0.4.5
   Compiling serde_json v1.0.8
   Compiling serde_derive_internals v0.18.1
   Compiling backtrace v0.3.4
   Compiling error-chain v0.11.0
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0 (f3d6973f4 2017-08-27) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'failed to acquire jobserver token: Error { repr: Os { code: 11, message: "Resource temporarily unavailable" } }', /checkout/src/libcore/result.rs:860:4
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic_new
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: rustc_trans::back::write::execute_work
  11: rustc_trans::back::write::run_passes
  12: rustc_driver::driver::phase_5_run_llvm_passes
  13: rustc_driver::driver::compile_input
  14: rustc_driver::run_compiler

error: Could not compile `serde_derive_internals`.
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.20.0 (f3d6973f4 2017-08-27) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'failed to acquire jobserver token: Error { repr: Os { code: 11, message: "Resource temporarily unavailable" } }', /checkout/src/libcore/result.rs:860:4
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
   1: std::sys_common::backtrace::_print
   2: std::panicking::default_hook::{{closure}}
   3: std::panicking::default_hook
   4: std::panicking::rust_panic_with_hook
   5: std::panicking::begin_panic_new
   6: std::panicking::begin_panic_fmt
   7: rust_begin_unwind
   8: core::panicking::panic_fmt
   9: core::result::unwrap_failed
  10: rustc_trans::back::write::execute_work
  11: rustc_trans::back::write::run_passes
  12: rustc_driver::driver::phase_5_run_llvm_passes
  13: rustc_driver::driver::compile_input
  14: rustc_driver::run_compiler

error: Could not compile `toml`.
warning: build failed, waiting for other jobs to finish...
error: failed to verify package tarball

Caused by:
  build failed

