
$ RUST_BACKTRACE=full cargo install xargo
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:323
stack backtrace:
   0:     0x556c4239fac3 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::hf9ed9ccfd9f14c2b
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x556c4239bdf4 - std::sys_common::backtrace::_print::hd8a1b72dcf3955ef
                               at /checkout/src/libstd/sys_common/backtrace.rs:71
   2:     0x556c423a76bc - std::panicking::default_hook::{{closure}}::h5ff605bba7612658
                               at /checkout/src/libstd/sys_common/backtrace.rs:60
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x556c423a7284 - std::panicking::default_hook::h9bc4f6dfee57d6bd
                               at /checkout/src/libstd/panicking.rs:371
   4:     0x556c423a7b0b - std::panicking::rust_panic_with_hook::hdc01585dc2bf7122
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x556c423a79e4 - std::panicking::begin_panic::hf84f4975d9f9b642
                               at /checkout/src/libstd/panicking.rs:511
   6:     0x556c423a7919 - std::panicking::begin_panic_fmt::hcc3f360b2ba80419
                               at /checkout/src/libstd/panicking.rs:495
   7:     0x556c423a78a7 - rust_begin_unwind
                               at /checkout/src/libstd/panicking.rs:471
   8:     0x556c423d401d - core::panicking::panic_fmt::h795d9a9608ddc2bb
                               at /checkout/src/libcore/panicking.rs:69
   9:     0x556c423d3f54 - core::panicking::panic::hcab3e0dfa81beee9
                               at /checkout/src/libcore/panicking.rs:49
  10:     0x556c420566de - cargo::version::haea7e89a3b191e3c
                               at /checkout/src/libcore/macros.rs:21
                               at /checkout/cargo/src/cargo/lib.rs:263
  11:     0x556c41feb8cf - cargo::ops::registry::http_handle::h3e076e6b4e3144ec
                               at /checkout/cargo/src/cargo/ops/registry.rs:227
  12:     0x556c42014e19 - <cargo::sources::registry::remote::RemoteRegistry<'cfg> as cargo::sources::registry::RegistryData>::update_index::h282902cfa6380402
                               at /checkout/cargo/src/cargo/sources/registry/remote.rs:63
  13:     0x556c4201a6c7 - cargo::sources::registry::RegistrySource::do_update::h07f4b56389efd3cc
                               at /checkout/cargo/src/cargo/sources/registry/mod.rs:309
  14:     0x556c4201a986 - <cargo::sources::registry::RegistrySource<'cfg> as cargo::core::source::Source>::update::h790227a7871fecb4
                               at /checkout/cargo/src/cargo/sources/registry/mod.rs:353
  15:     0x556c41f74ff2 - cargo::ops::cargo_install::install::ha102628dcbc73b48
                               at /checkout/cargo/src/cargo/core/source.rs:64
                               at /checkout/cargo/src/cargo/ops/cargo_install.rs:273
                               at /checkout/cargo/src/cargo/ops/cargo_install.rs:80
  16:     0x556c41e1d96e - cargo::try_execute_builtin_command::h117b1963a08549be
                               at /checkout/cargo/src/bin/install.rs:148
                               at /checkout/cargo/src/cargo/lib.rs:128
                               at /checkout/cargo/src/bin/cargo.rs:259
  17:     0x556c41e09714 - cargo::execute::h38e6c27236d82ec3
                               at /checkout/cargo/src/bin/cargo.rs:223
  18:     0x556c41e00701 - cargo::call_main_without_stdin::hde98a839d8ab6c0f
                               at /checkout/cargo/src/cargo/lib.rs:128
  19:     0x556c41e082e1 - cargo::main::h2cff0d0817bf7ef6
                               at /checkout/cargo/src/bin/cargo.rs:91
                               at /checkout/cargo/src/bin/cargo.rs:84
  20:     0x556c423a7805 - std::panicking::try::do_call::h689a21caeeef92aa
                               at /checkout/src/libstd/panicking.rs:454
  21:     0x556c423aeaca - __rust_maybe_catch_panic
                               at /checkout/src/libpanic_unwind/lib.rs:98
  22:     0x556c423a82a6 - std::rt::lang_start::hf63d494cb7dd034c
                               at /checkout/src/libstd/panicking.rs:433
                               at /checkout/src/libstd/panic.rs:361
                               at /checkout/src/libstd/rt.rs:57
  23:     0x7f87aa25f510 - __libc_start_main
  24:     0x556c41defb4c - <unknown>


$ rustup update
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'

   stable-x86_64-unknown-linux-gnu unchanged - rustc 1.16.0 (30cf806ef 2017-03-10)
  nightly-x86_64-unknown-linux-gnu unchanged - rustc 1.17.0-nightly (a559452b0 2017-03-17)
