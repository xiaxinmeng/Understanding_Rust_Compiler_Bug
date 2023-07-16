plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check
---
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0432]: unresolved import `rustc_data_structures::stable_hasher::NodeIdHashingMode`
  --> compiler/rustc_middle/src/ty/mod.rs:35:56
   |
35 | use rustc_data_structures::stable_hasher::{HashStable, NodeIdHashingMode, StableHasher};
   |                                                        ^^^^^^^^^^^^^^^^^ no `NodeIdHashingMode` in `stable_hasher`
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `with_node_id_hashing_mode` found for mutable reference `&mut StableHashingContext<'_>` in the current scope
   --> compiler/rustc_middle/src/ty/mod.rs:488:25
    |
488 |                     hcx.with_node_id_hashing_mode(NodeIdHashingMode::HashDefPath, |hcx| {
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&mut StableHashingContext<'_>`
Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
