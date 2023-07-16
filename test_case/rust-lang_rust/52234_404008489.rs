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
[00:41:07]     Checking rustc_tsan v0.0.0 (file:///checkout/src/librustc_tsan)
[00:41:07]     Checking rustc_msan v0.0.0 (file:///checkout/src/librustc_msan)
[00:41:07]     Checking rustc_asan v0.0.0 (file:///checkout/src/librustc_asan)
[00:41:07]  Documenting std v0.0.0 (file:///checkout/src/libstd)
[00:41:08] error: macro expansion ignores token `{` and any following
[00:41:08]     |
[00:41:08]     |
[00:41:08] 771 |         ($file:expr) => ({ /* compiler built-in */ });
[00:41:08]     |
[00:41:08]     |
[00:41:08] note: caused by the macro expansion here; the usage of `include!` is likely invalid in item context
[00:41:08]    --> libstd/lib.rs:548:1
[00:41:08]     |
[00:41:08] 548 | include!("primitive_docs.rs");
[00:41:08] 
[00:41:08] 
[00:41:08] error: macro expansion ignores token `{` and any following
[00:41:08]     |
[00:41:08]     |
[00:41:08] 771 |         ($file:expr) => ({ /* compiler built-in */ });
[00:41:08]     |
[00:41:08]     |
[00:41:08] note: caused by the macro expansion here; the usage of `include!` is likely invalid in item context
[00:41:08]    --> libstd/lib.rs:553:1
[00:41:08]     |
[00:41:08] 553 | include!("keyword_docs.rs");
[00:41:08] 
[00:41:08] 
[00:41:08] error: Could not document `std`.
[00:41:08] Caused by:
[00:41:08] Caused by:
[00:41:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std libstd/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg feature="alloc_jemalloc" --cfg feature="backtrace" --cfg feature="jemalloc" --cfg feature="panic-unwind" --cfg feature="panic_unwind" -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-f4ae61a0ac9de403.rmeta --extern alloc_jemalloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_jemalloc-6e31f4ac527cac4f.rmeta --extern alloc_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc_system-953937df74cec9d8.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-9cb094b550f36de1.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-b15f1ff96a8f614d.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-b593463e25ad8bc9.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-d6f76d88eb491ae8.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-241c8eb04554268c.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-8fb2bf0fd88472e9.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-ec08d8e5e1d73289.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-1341914eb50a09c0.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-acea38b6e3885725.rmeta --extern std_unicode=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libstd_unicode-f63bf17f6ac88315.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-39eca009eb64ef4f.rmeta` (exit code: 101)
[00:41:08] 
[00:41:08] 
[00:41:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--no-deps" "-p" "alloc" "-p" "core" "-p" "std" "-p" "std_unicode"
[00:41:08] 
[00:41:08] 
[00:41:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:41:08] Build completed unsuccessfully in 0:04:52
[00:41:08] Build completed unsuccessfully in 0:04:52
[00:41:08] make: *** [all] Error 1
[00:41:08] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0755e612
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:02926932:start=1531269441066222874,finish=1531269441073890522,duration=7667648
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:054d9cfa
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11e929ba
$ dmesg | grep -i kill
