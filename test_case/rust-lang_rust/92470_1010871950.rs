
> rustc +fc922e0fe35a7aaa4bf5dbbe516ffd78dde591c2 a.rs
thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', /cargo/registry/src/github.com-1ecc6299db9ec823/ena-0.14.0/src/snapshot_vec.rs:199:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (fc922e0fe 2022-01-02) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `Writer` fulfills its obligations
#1 [resolve_instance] resolving instance `<_ as Writer>::write`
end of query stack
