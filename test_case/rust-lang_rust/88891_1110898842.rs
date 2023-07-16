
❯ cargo fix --edition
    Checking regex v1.5.5
    Checking num-traits v0.2.14
   Compiling gumdrop_derive v0.8.1
    Checking thiserror v1.0.30
    Checking halo2_gadgets v0.1.0-beta.3 (/home/str4d/dev/rust/zcash/halo2/halo2_gadgets)
    Checking halo2 v0.1.0-beta.2 (/home/str4d/dev/rust/zcash/halo2/halo2)
   Migrating halo2_gadgets/src/lib.rs from 2018 edition to 2021
   Migrating halo2/src/lib.rs from 2018 edition to 2021
    Checking plotters v0.3.1
    Checking proptest v1.0.0
    Checking criterion v0.3.5
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:560:17: expected const for `K/#1` (Const { ty: usize, val: Param(K/#1) }/1) but found Type(std::alloc::Global) when substituting substs=[sinsemilla::message::MessagePiece<F, K>, std::alloc::Global]

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/59eed8a2aac0230a8b53e89d4e99d55912ba6b35/compiler/rustc_errors/src/lib.rs:1093:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.1 (59eed8a2a 2021-11-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [adt_significant_drop_tys] computing when `sinsemilla::message::Message` has a significant destructor
#1 [has_significant_drop_raw] computing whether `sinsemilla::message::Message<pasta_curves::Fp, 10_usize, 253_usize>` has a significant drop
end of query stack
error: could not compile `halo2_gadgets`
warning: build failed, waiting for other jobs to finish...
error: build failed
