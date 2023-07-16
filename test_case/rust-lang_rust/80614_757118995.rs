plain
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
extracting /checkout/obj/build/cache/2020-12-30/rustc-beta-x86_64-unknown-linux-gnu.tar.xz
downloading https://static.rust-lang.org/dist/2020-12-30/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
#=#=#                                                                         
##O#- #                                                                       
curl: (35) OpenSSL SSL_connect: SSL_ERROR_SYSCALL in connection to static.rust-lang.org:443 
spurious failure, trying again
downloading https://static.rust-lang.org/dist/2020-12-30/cargo-beta-x86_64-unknown-linux-gnu.tar.xz

######################################################################## 100.0%
---
Checking which error codes lack tests...
Found 435 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_mir/src/borrow_check/diagnostics/conflict_errors.rs:1334: trailing whitespace
some tidy checks failed

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build"
expected success, got: exit code: 1

