plain
Successfully built 13c0a576e618
Successfully tagged rust-ci:latest
Built container sha256:13c0a576e61831fcfc02cfc850285830876ebf0ed8cd4991d8007ff13f3e786c
Uploading finished image to https://ci-caches.rust-lang.org/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b
upload failed: - to s3://rust-lang-ci-sccache2/docker/48b9c0d410ba8aaf9ec787312e809393a5b600ad19e558a85cc70a70e4d9391b2b47a21b476f1ec744eb9b8ea65cefd83ee805b20655ccbdca7049f004bce26b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.11.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error: internal compiler error: stable const functions must have either `rustc_const_stable` or `rustc_const_unstable` attribute
    |
    |
704 |     pub const fn code_point(self) -> u32 {
    |
    |
    = note: delayed at compiler/rustc_mir/src/const_eval/fn_queries.rs:53:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:974:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-beta.2 (14b27555a 2021-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
