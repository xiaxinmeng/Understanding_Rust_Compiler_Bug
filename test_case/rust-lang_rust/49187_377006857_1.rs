
# RUST_BACKTRACE=1 rustup update --force
info: syncing channel updates for 'stable-x86_64-unknown-freebsd'
info: syncing channel updates for 'nightly-x86_64-unknown-freebsd'
info: latest update on 2018-03-28, rust version 1.26.0-nightly (9c9424de5 2018-03-27)
thread 'main' panicked at 'components available', /checkout/src/libcore/option.rs:874:4
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:68
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:57
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:577
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:538
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::option::expect_failed
             at /checkout/src/libcore/option.rs:874
  10: rustup_dist::manifestation::Manifestation::update
  11: rustup_dist::dist::update_from_dist_
fatal runtime error: failed to initiate panic, error 5
zsh: abort (core dumped)  RUST_BACKTRACE=1 rustup update --force
