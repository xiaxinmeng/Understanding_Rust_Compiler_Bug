plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7f9ab0300cd66f6f616e03ea90b2d71af474bf28 and 2236f9267d06075bc23812075a8e79d92cd9de4c
Executing the job since rustdoc was updated
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   |
45 |     path: String,
   |     ^^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `spans`
  --> src/tools/clippy/clippy_lints/src/regex.rs:54:5
   |
   |
54 |     spans: FxHashSet<Span>,


error: field is never read: `last`
   |
   |
55 |     last: Option<HirId>,

error: aborting due to 3 previous errors

error: could not compile `clippy_lints`
---
   Compiling rls-analysis v0.18.2 (/checkout/src/tools/rls/rls-analysis)
   Compiling futures v0.3.12
   Compiling jsonrpc-derive v17.0.0
   Compiling jsonrpc-core v17.0.0
warning: struct is never constructed: `Signature`
   --> src/tools/rls/rls-analysis/src/analysis.rs:140:12
140 | pub struct Signature {
    |            ^^^^^^^^^
    |
    = note: `#[warn(dead_code)]` on by default
    = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `SigElement`
   --> src/tools/rls/rls-analysis/src/analysis.rs:150:12
150 | pub struct SigElement {
    |            ^^^^^^^^^^

   Compiling parity-tokio-ipc v0.8.0
---

warning: field is never read: `spans`
  --> src/tools/clippy/clippy_lints/src/regex.rs:54:5
   |
54 |     spans: FxHashSet<Span>,


warning: field is never read: `last`
   |
   |
55 |     last: Option<HirId>,

   Compiling jsonrpc-core-client v17.0.0
   Compiling rls-ipc v0.1.0 (/checkout/src/tools/rls/rls-ipc)
   Compiling rustc-ap-rustc_arena v718.0.0
---
   |
45 |     path: String,
   |     ^^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
error: field is never read: `spans`
  --> src/tools/clippy/clippy_lints/src/regex.rs:54:5
   |
   |
54 |     spans: FxHashSet<Span>,


error: field is never read: `last`
   |
   |
55 |     last: Option<HirId>,

error: aborting due to 3 previous errors

error: could not compile `clippy_lints`
error: could not compile `clippy_lints`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit status: 101
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:598:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/clippy
Build completed unsuccessfully in 0:00:16
