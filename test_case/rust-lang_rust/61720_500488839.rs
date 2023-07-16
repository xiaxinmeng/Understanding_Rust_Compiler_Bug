plain
travis_time:end:1792cf4d:start=1560180228390210893,finish=1560180229177715781,duration=787504888
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:59] 
[01:02:59] running 144 tests
[01:03:02] i..iii.....iii...iiii....i......................i..i.................i.....i..........ii.i..i..i.ii. 100/144
[01:03:04] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:03:04] 
[01:03:04]  finished in 4.505
[01:03:04] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:06] 
[01:03:06] running 9 tests
[01:03:06] iiiiiiiii
[01:03:06] 
[01:03:06]  finished in 0.147
[01:03:06] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:21] 
[01:03:21] running 122 tests
[01:03:46] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:03:50] .i.i......iii.i.....ii
[01:03:50] 
[01:03:50]  finished in 28.999
[01:03:50] travis_fold:end:test_debuginfo

---
[01:19:38] 
[01:19:38] To learn more, run the command again with --verbose.
[01:19:38] 
[01:19:38] 
[01:19:38] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:19:38] 
[01:19:38] 
[01:19:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:19:38] Build completed unsuccessfully in 1:15:43
---
166968 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
156504 ./src/llvm-project/clang
150572 ./obj/build/bootstrap/debug/incremental
135300 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135296 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fd1cypvdk7-ynen56-19ca4iajxr4qm
125304 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
123656 ./src/llvm-project/llvm/test/CodeGen
117304 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib
117248 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu
