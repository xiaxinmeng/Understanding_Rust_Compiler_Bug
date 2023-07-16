plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/75159/merge:refs/remotes/pull/75159/merge
---
test build::close_output999 ... ok

failures:

---- build::close_output631 stdout ----
thread 'build::close_output631' panicked at 'assertion failed: !status.success()', src/tools/cargo/tests/testsuite/build.rs:9028:9

---- build::close_output856 stdout ----
---- build::close_output856 stdout ----
thread 'build::close_output856' panicked at 'assertion failed: !status.success()', src/tools/cargo/tests/testsuite/build.rs:9028:9

failures:
    build::close_output631
    build::close_output856
---
== clock drift check ==
  local time: Tue Aug  4 19:55:40 UTC 2020
  network time: Tue, 04 Aug 2020 19:55:40 GMT
== end clock drift check ==
##[error]Process completed with exit code 101.
Terminate orphan process: pid (3438) (node)
Terminate orphan process: pid (3467) (python)
