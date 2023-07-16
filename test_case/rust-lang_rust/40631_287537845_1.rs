
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:323
stack backtrace:
   0:     0x55f0d4951ac3 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::hf9ed9ccfd9f14c2b
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x55f0d494ddf4 - std::sys_common::backtrace::_print::hd8a1b72dcf3955ef
                               at /checkout/src/libstd/sys_common/backtrace.rs:71
   2:     0x55f0d49596bc - std::panicking::default_hook::{{closure}}::h5ff605bba7612658
                               at /checkout/src/libstd/sys_common/backtrace.rs:60
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x55f0d4959284 - std::panicking::default_hook::h9bc4f6dfee57d6bd
                               at /checkout/src/libstd/panicking.rs:371
   4:     0x55f0d4959b0b - std::panicking::rust_panic_with_hook::hdc01585dc2bf7122
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x55f0d49599e4 - std::panicking::begin_panic::hf84f4975d9f9b642
                               at /checkout/src/libstd/panicking.rs:511
   6:     0x55f0d4959919 - std::panicking::begin_panic_fmt::hcc3f360b2ba80419
                               at /checkout/src/libstd/panicking.rs:495
   7:     0x55f0d49598a7 - rust_begin_unwind
                               at /checkout/src/libstd/panicking.rs:471
   8:     0x55f0d498601d - core::panicking::panic_fmt::h795d9a9608ddc2bb
                               at /checkout/src/libcore/panicking.rs:69
   9:     0x55f0d4985f54 - core::panicking::panic::hcab3e0dfa81beee9
                               at /checkout/src/libcore/panicking.rs:49
  10:     0x55f0d46086de - cargo::version::haea7e89a3b191e3c
                               at /checkout/src/libcore/macros.rs:21
                               at /checkout/cargo/src/cargo/lib.rs:263
  11:     0x55f0d459d8cf - cargo::ops::registry::http_handle::h3e076e6b4e3144ec
                               at /checkout/cargo/src/cargo/ops/registry.rs:227
  12:     0x55f0d45c6e19 - <cargo::sources::registry::remote::RemoteRegistry<'cfg> as cargo::sources::registry::RegistryData>::update_index::h282902cfa6380402
                               at /checkout/cargo/src/cargo/sources/registry/remote.rs:63
  13:     0x55f0d45cc6c7 - cargo::sources::registry::RegistrySource::do_update::h07f4b56389efd3cc
                               at /checkout/cargo/src/cargo/sources/registry/mod.rs:309
  14:     0x55f0d45cc986 - <cargo::sources::registry::RegistrySource<'cfg> as cargo::core::source::Source>::update::h790227a7871fecb4
                               at /checkout/cargo/src/cargo/sources/registry/mod.rs:353
  15:     0x55f0d4526ff2 - cargo::ops::cargo_install::install::ha102628dcbc73b48
                               at /checkout/cargo/src/cargo/core/source.rs:64
                               at /checkout/cargo/src/cargo/ops/cargo_install.rs:273
                               at /checkout/cargo/src/cargo/ops/cargo_install.rs:80
  16:     0x55f0d43cf96e - cargo::try_execute_builtin_command::h117b1963a08549be
                               at /checkout/cargo/src/bin/install.rs:148
                               at /checkout/cargo/src/cargo/lib.rs:128
                               at /checkout/cargo/src/bin/cargo.rs:259
  17:     0x55f0d43bb714 - cargo::execute::h38e6c27236d82ec3
                               at /checkout/cargo/src/bin/cargo.rs:223
  18:     0x55f0d43b2701 - cargo::call_main_without_stdin::hde98a839d8ab6c0f
                               at /checkout/cargo/src/cargo/lib.rs:128
  19:     0x55f0d43ba2e1 - cargo::main::h2cff0d0817bf7ef6
                               at /checkout/cargo/src/bin/cargo.rs:91
                               at /checkout/cargo/src/bin/cargo.rs:84
  20:     0x55f0d4959805 - std::panicking::try::do_call::h689a21caeeef92aa
                               at /checkout/src/libstd/panicking.rs:454
  21:     0x55f0d4960aca - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:98
  22:     0x55f0d495a2a6 - std::rt::lang_start::hf63d494cb7dd034c
                               at /checkout/src/libstd/panicking.rs:433
                               at /checkout/src/libstd/panic.rs:361
                               at /checkout/src/libstd/rt.rs:57
  23:     0x7f8ea52457ec - __libc_start_main
  24:     0x55f0d43a1b4c - <unknown>
