plain
#########################                                                 35.7%
###################################################################       94.3%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-03-25/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating git repository `https://github.com/alexcrichton/object`
---
    Checking atty v0.2.14
    Checking termize v0.1.1
    Checking rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
    Checking petgraph v0.5.1
    Checking object v0.24.0 (https://github.com/alexcrichton/object?branch=more-arch#e9663fca)
    Checking rand_core v0.6.2
    Checking crossbeam-deque v0.7.3
    Checking parking_lot v0.11.1
    Checking rand_core v0.5.1
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
    Checking crossbeam-queue v0.2.3
    Checking unicode-security v0.0.5
    Checking rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
    Checking petgraph v0.5.1
    Checking object v0.24.0 (https://github.com/alexcrichton/object?branch=more-arch#e9663fca)
    Checking rand v0.8.3
    Checking crossbeam-deque v0.7.3
   Compiling cstr v0.2.8
    Checking regex v1.4.3
---
   Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
    Finished release [optimized] target(s) in 11.06s
tidy check
tidy: Skipping binary file check, read-only filesystem
tidy error: invalid source: "git+https://github.com/alexcrichton/object?branch=more-arch#e9663fca4404f7b8a3d41f8c3b9db76bedf8b997"
* 625 error codes
* highest error code: E0783
Found 517 error codes
Found 0 error codes with no tests
Found 0 error codes with no tests
Done!
* 337 features
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:13
