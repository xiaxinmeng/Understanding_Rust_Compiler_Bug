plain
[00:04:18]    Vendoring humantime v1.1.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/humantime-1.1.1) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/humantime
[00:04:18]    Vendoring idna v0.1.5 (/cargo/registry/src/github.com-1ecc6299db9ec823/idna-0.1.5) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/idna
[00:04:18]    Vendoring if_chain v0.1.3 (/cargo/registry/src/github.com-1ecc6299db9ec823/if_chain-0.1.3) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/if_chain
[00:04:18]    Vendoring ignore v0.4.3 (/cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.3) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/ignore
[00:04:18]    Vendoring im-rc v12.2.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/im-rc-12.2.0) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/im-rc
[00:04:18]    Vendoring itertools v0.7.8 (/cargo/registry/src/github.com-1ecc6299db9ec823/itertools-0.7.8) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/itertools
[00:04:18]    Vendoring itoa v0.4.3 (/cargo/registry/src/github.com-1ecc6299db9ec823/itoa-0.4.3) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/itoa
[00:04:18]    Vendoring jemalloc-sys v0.1.8 (/cargo/registry/src/github.com-1ecc6299db9ec823/jemalloc-sys-0.1.8) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/jemalloc-sys
[00:04:18]    Vendoring jobserver v0.1.11 (/cargo/registry/src/github.com-1ecc6299db9ec823/jobserver-0.1.11) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/jobserver
---
[00:04:18]    Vendoring thread_local v0.3.6 (/cargo/registry/src/github.com-1ecc6299db9ec823/thread_local-0.3.6) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/thread_local
[00:04:18]    Vendoring time v0.1.40 (/cargo/registry/src/github.com-1ecc6299db9ec823/time-0.1.40) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/time
[00:04:18]    Vendoring toml v0.4.6 (/cargo/registry/src/github.com-1ecc6299db9ec823/toml-0.4.6) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/toml
[00:04:18]    Vendoring toml-query v0.6.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/toml-query-0.6.0) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/toml-query
[00:04:18]    Vendoring typenum v1.10.0 (/cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.10.0) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/typenum
[00:04:18]    Vendoring unicode-bidi v0.3.4 (/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-bidi-0.3.4) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/unicode-bidi
[00:04:18]    Vendoring unicode-normalization v0.1.7 (/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-normalization-0.1.7) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/unicode-normalization
[00:04:18]    Vendoring unicode-segmentation v1.2.1 (/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-segmentation-1.2.1) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/unicode-segmentation
[00:04:18]    Vendoring unicode-width v0.1.5 (/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.5) to /checkout/obj/build/tmp/dist/rustc-1.32.0-dev-src/vendor/unicode-width
---
[00:11:10] configure: 
[00:11:10] configure: run `python /checkout/obj/build/tmp/distcheck/x.py --help`
[00:11:10] configure: 
[00:11:10] Traceback (most recent call last):
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 870, in <module>
[00:11:10]     main()
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 853, in main
[00:11:10]     bootstrap(help_triggered)
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 810, in bootstrap
[00:11:10]     data = stage0_data(build.rust_root)
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 158, in stage0_data
[00:11:10]     with open(nightlies, 'r') as nightlies:
[00:11:10] IOError: [Errno 2] No such file or directory: '/checkout/obj/build/tmp/distcheck/stage0.txt'
[00:11:10] Makefile:58: recipe for target 'check' failed
[00:11:10] make: *** [check] Error 1
[00:11:10] 
[00:11:10] 
[00:11:10] command did not execute successfully: "make" "check"
[00:11:10] 
[00:11:10] 
[00:11:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:11:10] Build completed unsuccessfully in 0:09:19
---
travis_time:end:03c2e37d:start=1543719103845795881,finish=1543719103853989082,duration=8193201
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b532fb8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0029afb8
travis_time:start:0029afb8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d4d7790
$ dmesg | grep -i kill
