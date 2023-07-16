plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/75894/merge:refs/remotes/pull/75894/merge
---
Checking which error codes lack tests...
Found 430 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/ci/scripts/run-build-from-ci.sh:19: line longer than 100 chars
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
expected success, got: exit code: 1



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:00:27
== clock drift check ==
  local time: Mon Sep 28 04:20:30 UTC 2020
  network time: Mon, 28 Sep 2020 04:20:30 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (8131) (python)
