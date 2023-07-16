plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4045ce641a9eede71cc12031a2cd71692b273890 and 9a4aac0bc9a230f5c0ba6305f222a22ec2f6f4b9
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling openssl v0.10.38
   Compiling rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
    Finished release [optimized] target(s) in 1m 53s
duplicate artifacts found when compiling a tool, this typically means that something was recompiled because a transitive dependency has different features activated than in a previous build:

the following dependencies are duplicated although they have the same features enabled:
the following dependencies have different features:
  regex-syntax 0.6.25 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex_syntax-58cfc058958766c7.rlib"
    `clippy-driver` additionally enabled features {"unicode-case", "unicode-perl", "default", "unicode", "unicode-bool", "unicode-segment", "unicode-script", "unicode-gencat", "unicode-age"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex_syntax-85b94989e8115749.rlib"
  regex 1.5.5 (registry+https://github.com/rust-lang/crates.io-index)
    `miri` additionally enabled features {} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-4d78bd3efd208579.rlib"
    `rls` additionally enabled features {"unicode-case", "unicode-script", "unicode-segment", "unicode-gencat", "unicode", "default", "unicode-bool", "unicode-age", "unicode-perl"} at "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7d13fa063f867ef0.rlib"

to fix this you will probably want to edit the local src/tools/rustc-workspace-hack/Cargo.toml crate, as that will update the dependency graph to ensure that these crates all share the same feature set
thread 'main' panicked at 'tools should not compile multiple copies of the same crate', tool.rs:197:13
Build completed unsuccessfully in 0:28:18
{"embedded-book":"test-pass","reference":"test-pass","rls":"test-pass","edition-guide":"test-pass","nomicon":"test-pass","rustbook":"test-fail","book":"test-pass","rust-by-example":"test-pass"}Building rustbuild
    Finished dev [unoptimized] target(s) in 0.07s
    Finished dev [unoptimized] target(s) in 0.07s
error: Tool `miri` was not recorded in tool state.
