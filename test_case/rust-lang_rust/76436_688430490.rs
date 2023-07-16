
thread 'rustc' panicked at 'assertion failed: num_bytes <= self.shared_state.page_size - PAGE_HEADER_SIZE', /cargo/git/checkouts/measureme-11f27a204054df90/c004094/measureme/src/paged_serialization_sink.rs:133:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (cc857b33d 2020-09-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z self-profile=/tmp/.tmpNwWiWk/self-profile-output -C embed-bitcode=no -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `LLVMing`,
 right: `Codegenning`', /rustc/cc857b33d54a8ee4ee103623dc1de441b8cf6009/compiler/rustc_codegen_ssa/src/back/write.rs:1419:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.48.0-nightly (cc857b33d 2020-09-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z self-profile=/tmp/.tmpNwWiWk/self-profile-output -C embed-bitcode=no -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden


