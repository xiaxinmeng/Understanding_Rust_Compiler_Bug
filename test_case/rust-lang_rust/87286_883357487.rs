
Compiling tcargo v0.1.0 (/home/pi/src/tcargo)
thread 'rustc' panicked at 'called `set` on feature `` which is not `active`', compiler/rustc_feature/src/active.rs:100:18
stack backtrace:
   0: rust_begin_unwind
             at /rustc/59216858a323978a97593cba22b5ed84350a3783/library/std/src/panicking.rs:515:5
   1: std::panicking::begin_panic_fmt
             at /rustc/59216858a323978a97593cba22b5ed84350a3783/library/std/src/panicking.rs:457:5
   2: rustc_feature::active::<impl rustc_feature::Feature>::set
   3: rustc_expand::config::features
   4: rustc_interface::passes::register_plugins
   5: rustc_interface::queries::Queries::register_plugins
   6: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
   7: rustc_span::with_source_map
   8: rustc_interface::interface::create_compiler_and_run
   9: scoped_tls::ScopedKey<T>::set
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (59216858a 2021-07-18) running on armv7-unknown-linux-gnueabihf

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `tcargo