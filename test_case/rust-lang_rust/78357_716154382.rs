plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/78357/merge:refs/remotes/pull/78357/merge
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
Found 433 error codes
Found 0 error codes with no tests
Done!
* 304 features
some tidy checks failed
invalid source: "git+https://github.com/lzutao/rust-libc?branch=i78184#b58a839d02b8d0dcc6a5a78cfd88f67a8af19808"

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:10
== clock drift check ==
  local time: Sun Oct 25 14:09:36 UTC 2020
  network time: Sat, 24 Oct 2020 20:07:48 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6728) (python)
