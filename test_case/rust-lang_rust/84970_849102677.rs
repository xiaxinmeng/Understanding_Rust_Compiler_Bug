
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(Fingerprint(8537439170242672706, 4648092694241280842))`,
 right: `Some(Fingerprint(212923312148573672, 18320173992410164289))`: found unstable fingerprints for evaluate_obligation(b0fb4a538915940f-122ed1eb1e3c4fae): Ok(EvaluatedToOk)', /rustc/82b86216422e1ee696e20511180fada7a7a87949/compiler/rustc_query_system/src/query/plumbing.rs:588:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-beta.3 (82b862164 2021-05-22) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock, read_buffer::chunk::TableData>: std::marker::Sync`
#1 [typeck] type-checking `myproject::http::write`
end of query stack
error: could not compile `myproject`
