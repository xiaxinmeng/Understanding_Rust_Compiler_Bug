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
   Compiling semver-parser v0.10.1
error[E0658]: custom inner attributes are unstable
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.10.1/src/generated.rs:4:4
  |
4 | #![rustfmt::skip]
  |
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = help: add `#![feature(custom_inner_attributes)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `semver-parser`
---
   Compiling semver-parser v0.10.1
error[E0658]: custom inner attributes are unstable
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.10.1/src/generated.rs:4:4
  |
4 | #![rustfmt::skip]
  |
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = help: add `#![feature(custom_inner_attributes)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `semver-parser`
---
   Compiling semver-parser v0.10.1
error[E0658]: custom inner attributes are unstable
 --> /cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.10.1/src/generated.rs:4:4
  |
4 | #![rustfmt::skip]
  |
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = note: see issue #54726 <https://github.com/rust-lang/rust/issues/54726> for more information
  = help: add `#![feature(custom_inner_attributes)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: could not compile `semver-parser`
error: could not compile `semver-parser`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:532:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/clippy
Build completed unsuccessfully in 0:00:01
