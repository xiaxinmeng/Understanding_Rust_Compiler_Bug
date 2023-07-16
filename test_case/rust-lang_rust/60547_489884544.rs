plain
travis_time:end:2f0d5dc2:start=1557192507740566299,finish=1557192510485496203,duration=2744929904
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:02:27]   Downloaded ryu v0.2.7
[00:02:27]    Compiling proc-macro2 v0.4.24
[00:02:27]    Compiling unicode-xid v0.1.0
[00:02:27]    Compiling ryu v0.2.7
[00:02:27]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:02:28]    Compiling itoa v0.4.3
[00:02:28]    Compiling ordermap v0.3.5
[00:02:28]    Compiling cc v1.0.35
[00:02:28]    Compiling cfg-if v0.1.6
---
travis_time:start:stage0-std
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:03:36]    Compiling cc v1.0.35
[00:03:36]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:03:36]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:03:37]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:03:42]    Compiling compiler_builtins v0.1.10
[00:03:42]    Compiling cmake v0.1.38
[00:03:42]    Compiling backtrace-sys v0.1.27
---
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:04:52]    Compiling semver-parser v0.7.0
[00:04:52]    Compiling nodrop v0.1.12
[00:04:52]    Compiling cfg-if v0.1.6
[00:04:52]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:04:53]    Compiling rand_core v0.3.0
[00:04:53]    Compiling void v1.0.2
[00:04:53]    Compiling proc-macro2 v0.4.24
[00:04:53]    Compiling scopeguard v0.3.3
---
[00:24:45] travis_time:end:stage0-rustc:start=1557192816313526218,finish=1557194009452501155,duration=1193138974937

[00:24:45] Building stage0 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:24:45]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:24:45]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:24:45]    Compiling rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[00:24:46]    Compiling rustc-demangle v0.1.10
[00:24:53]    Compiling num_cpus v1.8.0
[00:24:53]    Compiling memmap v0.6.2
---
travis_time:start:stage1-std
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:26:09]    Compiling cc v1.0.35
[00:26:09]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:26:09]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:26:09]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:26:15]    Compiling compiler_builtins v0.1.10
[00:26:15]    Compiling cmake v0.1.38
[00:26:15]    Compiling backtrace-sys v0.1.27
---
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:27:45]    Compiling semver-parser v0.7.0
[00:27:45]    Compiling cfg-if v0.1.6
[00:27:45]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:27:45]    Compiling void v1.0.2
[00:27:46]    Compiling memoffset v0.2.1
[00:27:46]    Compiling rand_core v0.3.0
[00:27:46]    Compiling scopeguard v0.3.3
---

[00:52:36] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, llvm)
[00:52:37]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:52:37]    Compiling cc v1.0.35
[00:52:37]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:52:38]    Compiling rustc-demangle v0.1.10
[00:52:47]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:52:49]    Compiling memmap v0.6.2
[00:52:49]    Compiling num_cpus v1.8.0
---
[00:54:17] travis_fold:start:stage2-rustdoc
travis_time:start:stage2-rustdoc
Building rustdoc for stage2 (x86_64-unknown-linux-gnu)
[00:54:18]    Compiling semver-parser v0.7.0
[00:54:18]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:54:18]    Compiling void v1.0.2
[00:54:18]    Compiling version_check v0.1.5
[00:54:19]    Compiling memchr v2.2.0
[00:54:19]    Compiling stable_deref_trait v1.1.0
---

[00:58:11] travis_fold:start:stage0-rustbook
travis_time:start:stage0-rustbook
Building stage0 tool rustbook (x86_64-unknown-linux-gnu)
[00:58:11]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[00:58:11]    Compiling string_cache_shared v0.3.0
[00:58:11]    Compiling semver-parser v0.7.0
[00:58:11]    Compiling cc v1.0.35
[00:58:12]    Compiling void v1.0.2
---
[01:02:33] travis_fold:start:stage1-rustdoc
travis_time:start:stage1-rustdoc
Building rustdoc for stage1 (x86_64-unknown-linux-gnu)
[01:02:34]    Compiling semver-parser v0.7.0
[01:02:34]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[01:02:34]    Compiling rand_core v0.3.0
[01:02:34]    Compiling version_check v0.1.5
[01:02:35]    Compiling memchr v2.2.0
[01:02:35]    Compiling stable_deref_trait v1.1.0
---
[01:05:13]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:05:17]     Finished release [optimized] target(s) in 37.24s
[01:05:18]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:05:43]     Finished release [optimized] target(s) in 25.31s
[01:05:44]     Checking libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[01:05:44]     Checking alloc v0.0.0 (/checkout/src/liballoc)
[01:05:44]     Checking rustc-demangle v0.1.10
[01:05:45]     Checking unwind v0.0.0 (/checkout/src/libunwind)
[01:05:45]     Checking backtrace-sys v0.1.27
---
[01:06:25] travis_fold:start:stage2-error_index_generator
travis_time:start:stage2-error_index_generator
Building stage2 tool error_index_generator (x86_64-unknown-linux-gnu)
[01:06:26]    Compiling semver-parser v0.7.0
[01:06:26]    Compiling libc v0.2.54 (https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489)
[01:06:26]    Compiling rand_core v0.3.0
[01:06:27]    Compiling void v1.0.2
[01:06:27]    Compiling memchr v2.2.0
[01:06:28]    Compiling stable_deref_trait v1.1.0
---

[01:09:17] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[01:09:17] tidy error: /checkout/src/libstd/sys/unix/rand.rs:181: trailing whitespace
[01:09:17] tidy error: /checkout/src/libstd/sys/unix/process/process_unix.rs:192: TODO is deprecated; use FIXME
[01:09:22] tidy error: The Unstable Book has a 'library feature' section 'n16' which doesn't correspond to an unstable library feature
[01:09:22] invalid source: "git+https://github.com/redox-os/liblibc.git?branch=redox-unix#eb75c489172e10bfcb57ca679ff337827cf783a9"
[01:09:22] some tidy checks failed
[01:09:22] 
[01:09:22] 
[01:09:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[01:09:22] 
[01:09:22] 
[01:09:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:09:22] Build completed unsuccessfully in 0:00:08
[01:09:22] Build completed unsuccessfully in 0:00:08
[01:09:22] Makefile:48: recipe for target 'check' failed
[01:09:22] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:010b74d9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May  7 02:38:07 UTC 2019
---
travis_time:end:0086ea9e:start=1557196688755112459,finish=1557196688760260562,duration=5148103
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3671c6ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:204ad4ff
travis_time:start:204ad4ff
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:35314ff0
$ dmesg | grep -i kill
