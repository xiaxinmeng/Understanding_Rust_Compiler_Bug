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
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
Executing the job since clippy subtree was updated
with:
  github_token: ***
  check_every_seconds: 60
env:
---
   Compiling memchr v2.3.3
   Compiling serde_json v1.0.59
   Compiling itoa v0.4.6
   Compiling pulldown-cmark v0.8.0
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error: could not compile `unicode-xid`
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: aborting due to previous error

---
   Compiling smallvec v1.4.2
   Compiling if_chain v1.0.0
   Compiling ryu v1.0.5
   Compiling bitflags v1.2.1
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
For more information about this error, try `rustc --explain E0602`.
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: could not compile `matches`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
For more information about this error, try `rustc --explain E0602`.
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`

error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
error[E0602]: unknown lint: `rustc::internals`
  |
  = note: requested on the command line with `-W rustc::internals`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0602`.
error: aborting due to previous error
---
For more information about this error, try `rustc --explain E0602`.
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
thread 'main' panicked at 'in-tree tool', src/bootstrap/test.rs:532:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/clippy
Build completed unsuccessfully in 0:00:03
