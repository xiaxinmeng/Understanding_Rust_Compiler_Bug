plain
travis_time:end:0b8aa516:start=1549936620209112508,finish=1549936622171590509,duration=1962478001
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    73% |███████████████████████▍        | 51kB 41.8MB/s eta 0:00:01
    87% |████████████████████████████    | 61kB 45.0MB/s eta 0:00:01
    100% |████████████████████████████████| 71kB 26.4MB/s 
Collecting botocore==1.12.92 (from awscli)
  Downloading https://files.pythonhosted.org/packages/a6/ec/e68d5d9b5eaa53d3552de0638231a8678c327737f4fc9fa62733483260fc/botocore-1.12.92-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 25.6MB/s eta 0:00:01
    0% |▏                               | 30kB 32.4MB/s eta 0:00:01
    0% |▎                               | 40kB 35.1MB/s eta 0:00:01
    0% |▎                               | 51kB 35.9MB/s eta 0:00:01
---
[01:02:11] ..............................i..................................................................... 4600/5381
[01:02:17] .................................................................................................... 4700/5381
[01:02:20] .................................................................................................... 4800/5381
[01:02:24] .................................................................................................... 4900/5381
[01:02:28] ................................................F................................................... 5000/5381
[01:02:35] .................................................................................................... 5200/5381
[01:02:38] .................................................................................................... 5300/5381
[01:02:41] ....................i............................................................
[01:02:41] failures:
[01:02:41] failures:
[01:02:41] 
[01:02:41] ---- [ui] ui/traits/trait-alias-syntax.rs stdout ----
[01:02:41] diff of stderr:
[01:02:41] 
[01:02:41] 1 error: trait aliases cannot be `auto`
[01:02:41] -   |
[01:02:41] -   |
[01:02:41] - 4 | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
[01:02:41] -   |                   ^ trait aliases cannot be `auto`
[01:02:41] +    |
[01:02:41] +    |
[01:02:41] + LL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
[01:02:41] +    |                   ^ trait aliases cannot be `auto`
[01:02:41] 6 
[01:02:41] 7 error: trait aliases cannot be `unsafe`
[01:02:41] -   |
[01:02:41] -   |
[01:02:41] - 5 | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
[01:02:41] -   |                     ^ trait aliases cannot be `unsafe`
[01:02:41] +    |
[01:02:41] +    |
[01:02:41] + LL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
[01:02:41] +    |                     ^ trait aliases cannot be `unsafe`
[01:02:41] - error: aborting due to previous error
[01:02:41] - 
[01:02:41] 15 error: aborting due to 2 previous errors
[01:02:41] + 
[01:02:41] + 
[01:02:41] 16 
[01:02:41] 
[01:02:41] 
[01:02:41] The actual stderr differed from the expected stderr.
[01:02:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/trait-alias-syntax.stderr
[01:02:41] To update references, rerun the tests and pass the `--bless` flag
[01:02:41] To only update this specific test, also pass `--test-args traits/trait-alias-syntax.rs`
[01:02:41] error: 1 errors occurred comparing output.
[01:02:41] status: exit code: 1
[01:02:41] status: exit code: 1
[01:02:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias-syntax/auxiliary" "-A" "unused"
[01:02:41] ------------------------------------------
[01:02:41] 
[01:02:41] ------------------------------------------
[01:02:41] stderr:
[01:02:41] stderr:
[01:02:41] ------------------------------------------
[01:02:41] {"message":"trait aliases cannot be `auto`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":57,"byte_end":58,"line_start":4,"line_end":4,"column_start":19,"column_end":20,"is_primary":true,"text":[{"text":"auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`","highlight_start":19,"highlight_end":20}],"label":"trait aliases cannot be `auto`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `auto`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:4:19\n   |\nLL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`\n   |                   ^ trait aliases cannot be `auto`\n\n"}
[01:02:41] {"message":"trait aliases cannot be `unsafe`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/traits/trait-alias-syntax.rs","byte_start":120,"byte_end":121,"line_start":5,"line_end":5,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`","highlight_start":21,"highlight_end":22}],"label":"trait aliases cannot be `unsafe`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: trait aliases cannot be `unsafe`\n  --> /checkout/src/test/ui/traits/trait-alias-syntax.rs:5:21\n   |\nLL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`\n   |                     ^ trait aliases cannot be `unsafe`\n\n"}
[01:02:41] 
[01:02:41] ------------------------------------------
[01:02:41] 
[01:02:41] thread '[ui] ui/traits/trait-alias-syntax.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
---
[01:02:41] 
[01:02:41] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:02:41] 
[01:02:41] 
[01:02:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:41] 
[01:02:41] 
[01:02:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:41] Build completed unsuccessfully in 0:04:24
[01:02:41] Build completed unsuccessfully in 0:04:24
[01:02:41] make: *** [check] Error 1
[01:02:41] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17d727c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 02:59:54 UTC 2019
