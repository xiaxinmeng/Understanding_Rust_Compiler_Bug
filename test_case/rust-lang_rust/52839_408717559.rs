plain
travis_time:start:tidy
tidy check
[00:03:48] * 553 error codes
[00:03:48] * highest error code: E0710
[00:03:49] tidy error: /checkout/src/libstd/os/mod.rs:13: mismatches to previous in: ["since"]
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:24: platform-specific cfg: cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
[00:03:49]                                        target_arch = "arm",
[00:03:49]                                        target_arch = "powerpc",
[00:03:49]                                        target_arch = "powerpc64",
[00:03:49]                                        target_arch = "s390x")),
[00:03:49]           all(target_os = "android", any(target_arch = "aarch64",
[00:03:49]                                          target_arch = "arm")),
[00:03:49]           all(target_os = "l4re", target_arch = "x86_64"),
[00:03:49]           all(target_os = "openbsd", target_arch = "aarch64"),
[00:03:49]           all(target_os = "fuchsia", target_arch = "aarch64")))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:36: platform-specific cfg: cfg(not(any(all(target_os = "linux", any(target_arch = "aarch64",
[00:03:49]                                            target_arch = "arm",
[00:03:49]                                            target_arch = "powerpc",
[00:03:49]                                            target_arch = "powerpc64",
[00:03:49]                                            target_arch = "s390x")),
[00:03:49]               all(target_os = "android", any(target_arch = "aarch64",
[00:03:49]                                              target_arch = "arm")),
[00:03:49]               all(target_os = "l4re", target_arch = "x86_64"),
[00:03:49]               all(target_os = "openbsd", target_arch = "aarch64"),
[00:03:49]               all(target_os = "fuchsia", target_arch = "aarch64"))))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:60: platform-specific cfg: cfg(any(target_pointer_width = "32", windows))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:63: platform-specific cfg: cfg(any(target_pointer_width = "32", windows))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:66: platform-specific cfg: cfg(all(target_pointer_width = "64", not(windows)))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs:69: platform-specific cfg: cfg(all(target_pointer_width = "64", not(windows)))
[00:03:49] tidy error: /checkout/src/libcore/os/raw/mod.rs contains #[test]; libcore tests must be placed inside `src/libcore/tests/`
[00:03:49] some tidy checks failed
[00:03:49] 
[00:03:49] 
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:49] 
[00:03:49] 
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:49] Build completed unsuccessfully in 0:00:49
[00:03:49] Build completed unsuccessfully in 0:00:49
[00:03:49] make: *** [tidy] Error 1
[00:03:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02b04461
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05de5fe2:start=1532910262562898840,finish=1532910262569517003,duration=6618163
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20f85e7e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0002263b
travis_time:start:0002263b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07bd1774
$ dmesg | grep -i kill
