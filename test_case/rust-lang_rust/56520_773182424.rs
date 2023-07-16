
error: internal compiler error: failed to process buffered lint here
  --> /home/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/fluence-fork-libp2p-secio-0.26.0/src/codec/len_prefix.rs:73:25
   |
73 |                         log::error!("data length {} exceeds allowed maximum {}", data.len(), max_len)
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at /rustc/04caa632dd10c2bf64b69524c7f9c4c30a436877/compiler/rustc_lint/src/early.rs:384:18
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (04caa632d 2021-01-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `fluence-fork-libp2p-secio`
