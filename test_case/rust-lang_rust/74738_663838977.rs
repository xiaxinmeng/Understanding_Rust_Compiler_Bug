plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/74738/merge:refs/remotes/pull/74738/merge
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
tidy check
* 615 error codes
* highest error code: E0772
* 295 features
invalid license `CC0-1.0` in `dunce 1.0.1 (registry+https://github.com/rust-lang/crates.io-index)`
Dependencies not explicitly permitted:
* yansi-term 0.1.2 (registry+https://github.com/rust-lang/crates.io-index)
some tidy checks failed
Found 509 error codes
Found 0 error codes with no tests
Done!
Done!


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
Build completed unsuccessfully in 0:00:16
Build completed unsuccessfully in 0:00:16
== clock drift check ==
  local time: Sat Jul 25 10:25:27 UTC 2020
  network time: Sat, 25 Jul 2020 10:25:27 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (12863) (python)
