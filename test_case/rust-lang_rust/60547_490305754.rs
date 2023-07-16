plain
travis_time:end:15c6e2d8:start=1557273005422692049,finish=1557273093284164653,duration=87861472604
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:01:48]    Compiling proc-macro2 v0.4.24
[00:01:48]    Compiling unicode-xid v0.1.0
[00:01:48]    Compiling serde v1.0.82
[00:01:48]    Compiling ryu v0.2.7
[00:01:48]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:01:48]    Compiling cc v1.0.35
[00:01:48]    Compiling cfg-if v0.1.6
[00:01:49]    Compiling itoa v0.4.3
[00:01:49]    Compiling unicode-width v0.1.5
---
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:02:55]    Compiling cc v1.0.35
[00:02:55]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:02:55]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:02:56]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:01]    Compiling compiler_builtins v0.1.10
[00:03:01]    Compiling cmake v0.1.38
[00:03:01]    Compiling backtrace-sys v0.1.27
---
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:14]    Compiling semver-parser v0.7.0
[00:04:14]    Compiling nodrop v0.1.12
[00:04:14]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:04:14]    Compiling proc-macro2 v0.4.24
[00:04:14]    Compiling lazy_static v1.2.0
[00:04:15]    Compiling scopeguard v0.3.3
[00:04:15]    Compiling rand_core v0.3.0
---

[00:24:39] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:24:40]    Compiling cc v1.0.35
[00:24:40]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:24:40]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:24:41]    Compiling rustc-demangle v0.1.10
[00:24:48]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:24:49]    Compiling memmap v0.6.2
[00:24:49]    Compiling num_cpus v1.8.0
---
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:26:06]    Compiling cc v1.0.35
[00:26:06]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:06]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:26:06]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:12]    Compiling compiler_builtins v0.1.10
[00:26:12]    Compiling cmake v0.1.38
[00:26:12]    Compiling backtrace-sys v0.1.27
---
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:27:47]    Compiling semver-parser v0.7.0
[00:27:47]    Compiling nodrop v0.1.12
[00:27:47]    Compiling cfg-if v0.1.6
[00:27:47]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:27:48]    Compiling rand_core v0.3.0
[00:27:48]    Compiling memoffset v0.2.1
[00:27:48]    Compiling void v1.0.2
[00:27:48]    Compiling proc-macro2 v0.4.24
---
[00:53:05] travis_time:end:stage1-rustc:start=1557274769133720657,finish=1557276287332770914,duration=1518199050257

[00:53:05] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:53:05]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:53:05]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:53:05]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:53:07]    Compiling rustc-demangle v0.1.10
[00:53:15]    Compiling num_cpus v1.8.0
[00:53:18]    Compiling memmap v0.6.2
---
[00:54:49] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:54:49]    Compiling semver-parser v0.7.0
[00:54:49]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:54:49]    Compiling rand_core v0.3.0
[00:54:49]    Compiling version_check v0.1.5
[00:54:51]    Compiling memchr v2.2.0
[00:54:51]    Compiling stable_deref_trait v1.1.0
---
[00:57:52]    Compiling syn v0.15.22
[00:57:57]    Compiling serde_json v1.0.33
[00:58:11]    Compiling serde_derive v1.0.81
[00:58:32]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:58:33] error: unused variable: `source`
[00:58:33]   --> src/tools/tidy/src/extdeps.rs:27:13
[00:58:33]    |
[00:58:33] 27 |         let source = line.splitn(2, '=').nth(1).unwrap().trim();
[00:58:33]    |             ^^^^^^ help: consider prefixing with an underscore: `_source`
[00:58:33]    = note: `-D unused-variables` implied by `-D warnings`
[00:58:33] 
[00:58:33] error: unused variable: `bad`
[00:58:33] error: unused variable: `bad`
[00:58:33]   --> src/tools/tidy/src/extdeps.rs:12:27
[00:58:33]    |
[00:58:33] 12 | pub fn check(path: &Path, bad: &mut bool) {
[00:58:33]    |                           ^^^ help: consider prefixing with an underscore: `_bad`
[00:58:33] error: aborting due to 2 previous errors
[00:58:33] 
[00:58:33] 
[00:58:33] error: Could not compile `tidy`.
[00:58:33] To learn more, run the command again with --verbose.
[00:58:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/unstable-book-gen/Cargo.toml" "--message-format" "json"
[00:58:33] expected success, got: exit code: 101
[00:58:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
---
travis_time:end:041f71ba:start=1557276616828939446,finish=1557276616833863840,duration=4924394
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:018268d4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:077218a4
travis_time:start:077218a4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00091b09
$ dmesg | grep -i kill
