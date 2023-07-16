plain
Successfully built 8a3c8a7d04a5
Successfully tagged rust-ci:latest
Built container sha256:8a3c8a7d04a57627a3d436d085956ff5e3c1cc128e4191fc4b1ac2df41eedeec
Uploading finished image to https://ci-caches.rust-lang.org/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724
upload failed: - to s3://rust-lang-ci-sccache2/docker/55e0616340de9767316ac8a2060e4b2402700344c201ac04865af7e91102302ab3c0bf7f297a771d5ca396e00e9848a4e4279034a0339b65793190c46eafc724 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
   Compiling hir-ty v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-ty)
error: unreachable expression
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:276:5
    |
274 | #[derive(Copy, Clone, Debug, PartialEq, Eq)]
275 | pub(super) struct Slice {
276 |     _unimplemented: Void,
    |     ^^^^^^^^^^^^^^^^^^^^
    |     |
---
    |
698 |     pub macro Debug($item:item) {
    |     --------------- in this expansion of `#[derive(Debug)]`
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable expression
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:281:15
    |
281 |         match self._unimplemented {}
---

error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:348:29
    |
348 |             Slice(slice) => Some(*slice),
    |                             ^^^^ ------ any code following this expression is unreachable
    |                             unreachable call

error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:470:67
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:470:67
    |
470 |             (Slice(self_slice), Slice(other_slice)) => self_slice.is_covered_by(*other_slice),
    |                                                                   ^^^^^^^^^^^^^ ------------ any code following this expression is unreachable
    |                                                                   unreachable call

error: unreachable call
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:505:36
   --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:505:36
    |
505 |                 .any(|other| slice.is_covered_by(other)),
    |                                    ^^^^^^^^^^^^^ ----- any code following this expression is unreachable
    |                                    unreachable call

error: unreachable expression
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1020:36
    --> crates/hir-ty/src/diagnostics/match_check/deconstruct_pat.rs:1020:36
     |
1020 |             &Slice(slice) => match slice._unimplemented {},
     |                                    |
     |                                    unreachable expression
     |                                    any code following this expression is unreachable


error: could not compile `hir-ty` due to 7 previous errors
Build completed unsuccessfully in 0:26:01
