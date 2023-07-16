plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/76260/merge:refs/remotes/pull/76260/merge
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
   Compiling cargo_metadata v0.11.1
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 6.64s
tidy check
Empty file with UI testing output: "/checkout/src/test/ui/error-codes/E0774.stderr"
Empty file with UI testing output: "/checkout/src/test/ui/error-codes/E0775.stderr"
tidy error: duplicate error code: 774
Checking which error codes lack tests...
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:458: E0774: include_str!("./error_codes/E0774.md"),
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes.rs:459: E0774: include_str!("./error_codes/E0775.md"),
Found 0 error codes with no tests
Done!
Done!
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0775.md:11: trailing whitespace
tidy error: /checkout/compiler/rustc_error_codes/src/error_codes/E0775.md:26: line longer than 80 chars
tidy error: /checkout/compiler/rustc_feature/src/active.rs:589: feature isa_attribute is not sorted by "since" (version number)
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
expected success, got: exit code: 1



failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:10
== clock drift check ==
  local time: Mon Sep 14 20:20:28 UTC 2020
  network time: Mon, 14 Sep 2020 20:20:28 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (3961) (node)
Terminate orphan process: pid (3970) (python)
