plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/13/89/f273dcdf5778abf55a1a6f99e504e89a1a4dd87f94d8ae016d6a61020f17/awscli-1.16.17-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 12.7MB/s eta 0:00:01
    1% |▌                               | 20kB 1.9MB/s eta 0:00:01
    2% |▊                               | 30kB 2.2MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
[00:24:47]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:24:52] error: hidden lifetime parameters in types are deprecated
[00:24:52]    --> librustc/ty/context.rs:834:66
[00:24:52]     |
[00:24:52] 834 |         static ASSERT_TY_KIND: () = [()][!(::std::mem::size_of::<ty::TyKind>() <= 24) as usize];
[00:24:52] 
[00:24:52] error: hidden lifetime parameters in types are deprecated
[00:24:52]    --> librustc/ty/context.rs:837:62
[00:24:52]     |
[00:24:52]     |
[00:24:52] 837 |         static ASSERT_TYS: () = [()][!(::std::mem::size_of::<ty::TyS>() <= 32) as usize];
[00:24:52] 
[00:25:24] error: aborting due to 2 previous errors
[00:25:24] 
[00:25:24] error: Could not compile `rustc`.
[00:25:24] error: Could not compile `rustc`.
[00:25:24] 
[00:25:24] To learn more, run the command again with --verbose.
[00:25:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:24] expected success, got: exit code: 101
[00:25:24] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1135:9
[00:25:24] travis_fold:end:stage1-rustc

[00:25:24] travis_time:end:stage1-rustc:start=1537417480549279364,finish=1537417665068472602,duration=184519193238


[00:25:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:24] Build completed unsuccessfully in 0:20:40
[00:25:24] make: *** [all] Error 1
[00:25:24] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1330cd3b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148532 ./obj/build/bootstrap/debug/incremental
134100 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg
134096 ./obj/build/bootstrap/debug/incremental/bootstrap-14ucxxomeo8gg/s-f4yvb01mvk-11vybxu-1c0ivx6vup97q
104700 ./src/tools/lldb
99280 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
98940 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
---
426840:start=1537417665809204973,finish=1537417665820635747,duration=11430774
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d9d650c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0509a1ec
$ dmesg | grep -i kill
