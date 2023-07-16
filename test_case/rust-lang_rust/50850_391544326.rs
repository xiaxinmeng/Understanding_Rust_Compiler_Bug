plain
    100% |████████████████████████████████| 4.2MB 289kB/s 
Collecting pyasn1>=0.1.3 (from rsa<=3.5.0,>=3.1.2->awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/a0/70/2c27740f08e477499ce19eefe05dbcae6f19fdc49e9e82ce4768be0643b9/pyasn1-0.4.3-py2.py3-none-any.whl (72kB)
    14% |████▌                           | 10kB 42.3MB/s eta 0:00:01
    28% |█████████                       | 20kB 40.5MB/s eta 0:00:01
    42% |█████████████▌                  | 30kB 46.6MB/s eta 0:00:01
    56% |██████████████████              | 40kB 50.6MB/s eta 0:00:01
---

[00:04:59] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:59] tidy error: /checkout/src/libcore/future.rs: incorrect license
[00:04:59] tidy error: /checkout/src/librustc_metadata/encoder.rs:1240: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustdoc/html/render.rs:2606: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/libsyntax/visit.rs:230: trailing whitespace
[00:04:59] tidy error: /checkout/src/libsyntax/test.rs:130: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/libsyntax/parse/parser.rs:3305: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/libsyntax/print/pprust.rs:2165: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:300: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:1744: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:1757: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:1762: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:1798: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:1812: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:2874: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/librustc/hir/lowering.rs:3199: line longer than 100 chars
[00:04:59] tidy error: /checkout/src/libcore/future.rs:2: mismatches to corresponding lang feature in: ["tracking issue"]
[00:04:59] tidy error: /checkout/src/libcore/future.rs:5: mismatches to corresponding lang feature in: ["tracking issue"]
[00:05:00] tidy error: /checkout/src/libstd/lib.rs:394: mismatches to corresponding lang feature in: ["tracking issue"]
[00:05:00] Expected a gate test for the feature 'async_await'.
[00:05:00] tidy error: Found 1 features without a gate test.
[00:05:00] Hint: create a failing test file named 'feature-gate-async_await.rs'
[00:05:00]       in the 'ui' test suite, with its failures due to
[00:05:00]       missing usage of #![feature(async_await)].
[00:05:00] Hint: If you already have such a test and don't want to rename it,
[00:05:00]       you can also add a // gate-test-async_await line to the test file.
[00:05:00] some tidy checks failed
[00:05:00] 
[00:05:00] 
[00:05:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:05:00] 
[00:05:00] 
[00:05:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:00] Build completed unsuccessfully in 0:01:56
[00:05:00] Build completed unsuccessfully in 0:01:56
[00:05:00] Makefile:79: recipe for target 'tidy' failed
[00:05:00] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3615d82a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
