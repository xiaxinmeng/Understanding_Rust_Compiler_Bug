plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/76467/merge:refs/remotes/pull/76467/merge
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Checking which error codes lack tests...
Found 430 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:307: TODO is deprecated; use FIXME
tidy error: /checkout/src/librustdoc/passes/collect_intra_doc_links.rs:806: TODO is deprecated; use FIXME
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
expected success, got: exit code: 1



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:10
== clock drift check ==
  local time: Mon Sep 28 01:11:49 UTC 2020
  network time: Mon, 28 Sep 2020 01:11:49 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6337) (node)
Terminate orphan process: pid (6346) (python)
