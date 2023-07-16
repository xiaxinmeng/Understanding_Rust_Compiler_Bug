plain

[00:04:57] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:57] tidy error: /checkout/src/librustc_target/abi/mod.rs:797: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/mod.rs: missing trailing newline
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mips.rs:47: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mips64.rs:30: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mips64.rs:44: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mips64.rs:86: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mips64.rs:154: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/powerpc64.rs:25: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/powerpc64.rs:29: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc.rs:14: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/mod.rs:248: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/arm.rs:16: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/arm.rs:80: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/arm.rs:104: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/x86_64.rs:35: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/x86_64.rs:40: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/x86_64.rs:182: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/asmjs.rs:19: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/asmjs.rs:45: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/aarch64.rs:15: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/aarch64.rs:44: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/aarch64.rs:78: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/aarch64.rs:112: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:19: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:44: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:46: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:81: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:83: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:107: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/sparc64.rs:109: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/s390x.rs:17: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/s390x.rs:49: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/s390x.rs:75: trailing whitespace
[00:04:57] tidy error: /checkout/src/librustc_target/abi/call/x86.rs:43: trailing whitespace
[00:04:58] tidy error: /checkout/src/libsyntax/test.rs:566: line longer than 100 chars
[00:04:58] tidy error: /checkout/src/librustc/ty/context.rs:1207: trailing whitespace
[00:04:58] tidy error: /checkout/src/librustc/ty/layout.rs:1443: trailing whitespace
[00:04:59] some tidy checks failed
[00:04:59] 
[00:04:59] 
[00:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:59] 
[00:04:59] 
[00:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:59] Build completed unsuccessfully in 0:01:55
[00:04:59] Build completed unsuccessfully in 0:01:55
[00:04:59] Makefile:79: recipe for target 'tidy' failed
[00:04:59] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:26f04f4b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
