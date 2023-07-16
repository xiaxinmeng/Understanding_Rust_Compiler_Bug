plain
tidy check
[00:05:48] * 539 error codes
[00:05:48] * highest error code: E0911
[00:05:48] * 197 features
[00:05:48] tidy error: /checkout/src/libcore/va_list.rs:34: platform-specific cfg: cfg(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc64",
[00:05:48]           target_arch = "x86", windows))
[00:05:48] tidy error: /checkout/src/libcore/va_list.rs:55: platform-specific cfg: cfg(all(target_arch = "x86_64", not(windows)))
[00:05:49] some tidy checks failed
[00:05:49] 
[00:05:49] 
[00:05:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:49] 
[00:05:49] 
[00:05:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:49] Build completed unsuccessfully in 0:02:29
[00:05:49] Build completed unsuccessfully in 0:02:29
[00:05:49] make: *** [tidy] Error 1
[00:05:49] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05994a1a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
