plain
travis_time:end:03314bfb:start=1544836014533678297,finish=1544836015526263274,duration=992584977
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 61kB 10.5MB/s 
Collecting botocore==1.12.66 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/51/da/3ed787b6ca3d33f626c1ba4e014449825db0d557981c4bef71f886fb1424/botocore-1.12.66-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 41.8MB/s eta 0:00:01
    0% |▏                               | 20kB 32.4MB/s eta 0:00:01
    0% |▏                               | 30kB 38.9MB/s eta 0:00:01
    0% |▎                               | 40kB 41.7MB/s eta 0:00:01
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:02] 
[00:51:02] running 121 tests
[00:51:05] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:51:05] i..ii.i.....iiii.....
[00:51:05] 
[00:51:05]  finished in 3.270
[00:51:05] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:51:19] 
[00:51:19] running 119 tests
[00:51:41] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:51:44] i......iii.i.....ii
[00:51:44] 
[00:51:44]  finished in 25.739
[00:51:44] travis_fold:end:test_debuginfo

---
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:07]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:08:48] error: linking with `cc` failed: exit code: 1
[01:08:48]   |
[01:08:48]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-397cfd5e4e6d606b.std.arnmi4sa-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-397cfd5e4e6d606b" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-6659039ef800420a/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/backtrace-sys-907ef870abdd5ff6/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-ldl" "-lrt" "-lpthread" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand-0bfd6d631ee7f3ea.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_xorshift-490a566088d81b1c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_pcg-5c7433bbfecf51f7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_hc-adf9d66988f5cd5c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_chacha-37ce22804a8af247.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_isaac-92be7b825c38a765.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand_core-83aab7b3082e2d4c.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-ltest-8c5a918140514252" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lterm-d4f9c8feeb3a5be5" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-7b1f19d9661da0ab" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d2c9cef04cfdc7ca.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../lib"
[01:08:48]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-397cfd5e4e6d606b.std.arnmi4sa-cgu.0.rcgu.o: In function `_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$::position::_$u7b$$u7b$closure$u7d$$u7d$::hd2c5380083132055':
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$8position28_$u7b$$u7b$closure$u7d$$u7d$17hd2c5380083132055E+0x65): undefined reference to `__rdos_backtrace_create_state'
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$8position28_$u7b$$u7b$closure$u7d$$u7d$17hd2c5380083132055E+0x99): undefined reference to `__rdos_backtrace_syminfo'
[01:08:48]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/std-397cfd5e4e6d606b.std.arnmi4sa-cgu.0.rcgu.o: In function `std::sys_common::backtrace::_print::h3b20bac7ba0d43e1':
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN3std10sys_common9backtrace6_print17h3b20bac7ba0d43e1E+0xb05): undefined reference to `__rdos_backtrace_create_state'
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN3std10sys_common9backtrace6_print17h3b20bac7ba0d43e1E+0xb3c): undefined reference to `__rdos_backtrace_syminfo'
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN3std10sys_common9backtrace6_print17h3b20bac7ba0d43e1E+0x1003): undefined reference to `__rdos_backtrace_create_state'
[01:08:48]           std.arnmi4sa-cgu.0:(.text._ZN3std10sys_common9backtrace6_print17h3b20bac7ba0d43e1E+0x103f): undefined reference to `__rdos_backtrace_pcinfo'
[01:08:48]           
[01:08:48] 
[01:08:48] error: aborting due to previous error
[01:08:48] 
[01:08:48] 
[01:08:48] error: Could not compile `std`.
[01:08:48] 
[01:08:48] To learn more, run the command again with --verbose.
[01:08:48] 
[01:08:48] 
[01:08:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:08:48] 
[01:08:48] 
[01:08:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:08:48] Build completed unsuccessfully in 0:27:43
[01:08:48] Build completed unsuccessfully in 0:27:43
[01:08:48] make: *** [check] Error 1
[01:08:48] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1259fb80
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Dec 15 02:15:54 UTC 2018
