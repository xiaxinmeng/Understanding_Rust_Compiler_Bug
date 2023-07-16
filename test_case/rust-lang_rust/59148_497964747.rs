plain
travis_time:end:049af385:start=1559409772024619892,finish=1559409857687681859,duration=85663061967
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:25:59]    Compiling rustc-demangle v0.1.15
[00:26:03]    Compiling num_cpus v1.8.0
[00:26:05]    Compiling memmap v0.6.2
[00:26:08]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 375 |         self.count_insn("unchecked_sadd");
[00:26:13] 
[00:26:13] 
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 382 |         self.count_insn("unchecked_uadd");
[00:26:13] 
[00:26:13] 
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 389 |         self.count_insn("unchecked_ssub");
[00:26:13] 
[00:26:13] 
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 396 |         self.count_insn("unchecked_usub");
[00:26:13] 
[00:26:13] 
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 403 |         self.count_insn("unchecked_smul");
[00:26:13] 
[00:26:13] 
[00:26:13] error[E0599]: no method named `count_insn` found for type `&mut builder::Builder<'a, 'll, 'tcx>` in the current scope
[00:26:13]     |
[00:26:13]     |
[00:26:13] 410 |         self.count_insn("unchecked_umul");
[00:26:13] 
[00:26:15] error: aborting due to 6 previous errors
[00:26:15] 
[00:26:15] For more information about this error, try `rustc --explain E0599`.
---
165196 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
156504 ./src/llvm-project/clang
150604 ./obj/build/bootstrap/debug/incremental
135320 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135316 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fcrj2nfacs-1jc07x6-1pqwr1cpuhqyq
108536 ./src/llvm-project/lldb
98012 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
97592 ./src/llvm-project/clang/test
89976 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:0f5903b2:start=1559411442812784488,finish=1559411442817784712,duration=5000224
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15e4ca68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a00112e
travis_time:start:1a00112e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:040396a0
$ dmesg | grep -i kill
