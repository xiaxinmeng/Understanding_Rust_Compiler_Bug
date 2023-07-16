plain
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/2d/99/b2c4e9d5a30f6471e410a146232b4118e697fa3ffc06d6a65efde84debd0/futures-3.2.0-py2-none-any.whl
Requirement already satisfied: six>=1.5 in /usr/lib/python2.7/dist-packages (from python-dateutil<3.0.0,>=2.1; python_version >= "2.7"->botocore==1.10.55->awscli)
Building wheels for collected packages: awscli
  Running setup.py bdist_wheel for awscli ... - \ | / - \ done
Successfully built awscli
Installing collected packages: docutils, jmespath, python-dateutil, botocore, colorama, pyasn1, rsa, futures, s3transfer, awscli
Successfully installed awscli-1.15.56 botocore-1.10.55 colorama-0.3.9 docutils-0.14 futures-3.2.0 jmespath-0.9.3 pyasn1-0.4.3 python-dateutil-2.7.3 rsa-3.4.2 s3transfer-0.1.13
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
---
[00:06:58]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:06:59] error[E0308]: mismatched types
[00:06:59]   --> libsyntax_ext/concat.rs:55:74
[00:06:59]    |
[00:06:59] 55 |                 let snippet = cx.codemap().span_to_snippet(sp).unwrap_or("");
[00:06:59]    |                                                                          |
[00:06:59]    |                                                                          expected struct `std::string::String`, found reference
[00:06:59]    |                                                                          expected struct `std::string::String`, found reference
[00:06:59]    |                                                                          help: try using a conversion method: `"".to_string()`
[00:06:59]    = note: expected type `std::string::String`
[00:06:59]    = note: expected type `std::string::String`
[00:06:59]               found type `&'static str`
[00:06:59] error: aborting due to previous error
[00:06:59] 
[00:06:59] For more information about this error, try `rustc --explain E0308`.
[00:06:59] error: Could not compile `syntax_ext`.
[00:06:59] error: Could not compile `syntax_ext`.
[00:06:59] 
[00:06:59] Caused by:
[00:06:59]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name syntax_ext libsyntax_ext/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=e8734f8d1e9b10c1 -C extra-filename=-e8734f8d1e9b10c1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-6ad9c4f0e3eb0853.so --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-cf4549c1ea81a6c6.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-dfab6c84d2674220.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-fb4c86e007981750.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e528c05031478194.so --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-5f603e9854c9c328.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-786292eb849f05d6.so` (exit code: 101)
[00:12:07] error: build failed
[00:12:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:12:07] expected success, got: exit code: 101
[00:12:07] expected success, got: exit code: 101
[00:12:07] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1117:9
[00:12:07] travis_fold:end:stage0-rustc

[00:12:07] travis_time:end:stage0-rustc:start=1531272833485240424,finish=1531273273709935087,duration=440224694663


[00:12:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:12:07] Build completed unsuccessfully in 0:08:12
[00:12:07] Makefile:28: recipe for target 'all' failed
[00:12:07] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0151bb7c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:06f325ec:start=1531273274331332092,finish=1531273274340126511,duration=8794419
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:205bb283
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b270ec
$ dmesg | grep -i kill
