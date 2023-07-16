plain
travis_time:end:00194d06:start=1548472635733847386,finish=1548472741724597518,duration=105990750132
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 68.6MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 68.4MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 27.1MB/s 
Collecting botocore==1.12.86 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d7/af/fd9c0f1f0fdc03d3367a56f35093f8b1020ba1a97ead9fa580156895944b/botocore-1.12.86-py2.py3-none-any.whl (5.2MB)
    0% |▏                               | 20kB 19.6MB/s eta 0:00:01
    0% |▏                               | 30kB 22.4MB/s eta 0:00:01
    0% |▎                               | 40kB 23.7MB/s eta 0:00:01
    0% |▎                               | 51kB 25.4MB/s eta 0:00:01
---
[01:09:01] .........i.......................................................................................... 3300/5348
[01:09:05] ..........................................................................ii...i..ii................ 3400/5348
[01:09:09] .................................................................................................... 3500/5348
[01:09:13] .................................................................................................... 3600/5348
[01:09:17] ...................F...................................................ii........................... 3700/5348
[01:09:22] .................................................................................................... 3900/5348
[01:09:24] ..............................................i..................................................... 4000/5348
[01:09:28] .................................................................................................... 4100/5348
[01:09:39] .................................................................................................... 4200/5348
---
[01:10:26] 
[01:10:26] ---- [ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs stdout ----
[01:10:26] diff of stderr:
[01:10:26] 
[01:10:26] - error: unsatisfied lifetime constraints
[01:10:26] + error: lifetime may not live long enough
[01:10:26] 2   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:35:5
[01:10:26] 3    |
[01:10:26] 4 LL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:10:26] 
[01:10:26] 7 LL |     y //~ ERROR unsatisfied lifetime constraints
[01:10:26] 8    |     ^ returning this value requires that `'a` must outlive `'static`
[01:10:26] - error: unsatisfied lifetime constraints
[01:10:26] - error: unsatisfied lifetime constraints
[01:10:26] + error: lifetime may not live long enough
[01:10:26] 11   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:49:5
[01:10:26] 12    |
[01:10:26] 13 LL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:10:26] 
[01:10:26] 16 LL |     y //~ ERROR unsatisfied lifetime constraints
[01:10:26] 17    |     ^ returning this value requires that `'a` must outlive `'static`
[01:10:26] - error: unsatisfied lifetime constraints
[01:10:26] - error: unsatisfied lifetime constraints
[01:10:26] + error: lifetime may not live long enough
[01:10:26] 20   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:62:5
[01:10:26] 21    |
[01:10:26] 22 LL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[01:10:26] 
[01:10:26] The actual stderr differed from the expected stderr.
[01:10:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/issue-55748-pat-types-constrain-bindings.stderr
[01:10:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/issue-55748-pat-types-constrain-bindings.stderr
[01:10:26] To update references, rerun the tests and pass the `--bless` flag
[01:10:26] To only update this specific test, also pass `--test-args nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs`
[01:10:26] error: 1 errors occurred comparing output.
[01:10:26] status: exit code: 1
[01:10:26] status: exit code: 1
[01:10:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/auxiliary" "-A" "unused"
[01:10:26] ------------------------------------------
[01:10:26] 
[01:10:26] ------------------------------------------
[01:10:26] stderr:
[01:10:26] stderr:
[01:10:26] ------------------------------------------
[01:10:26] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":903,"byte_end":905,"line_start":28,"line_end":28,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":24,"highlight_end":26}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1175,"byte_end":1176,"line_start":35,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:35:5\n   |\nLL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                        -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:10:26] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1343,"byte_end":1345,"line_start":42,"line_end":42,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1612,"byte_end":1613,"line_start":49,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:49:5\n   |\nLL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:10:26] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1780,"byte_end":1782,"line_start":56,"line_end":56,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":2041,"byte_end":2042,"line_start":62,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:62:5\n   |\nLL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[01:10:26] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[01:10:26] ------------------------------------------
[01:10:26] 
[01:10:26] thread '[ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:10:26] 
---
[01:10:26] 
[01:10:26] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:10:26] 
[01:10:26] 
[01:10:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:26] 
[01:10:26] 
[01:10:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:26] Build completed unsuccessfully in 0:04:40
[01:10:26] Build completed unsuccessfully in 0:04:40
[01:10:26] Makefile:48: recipe for target 'check' failed
[01:10:26] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00378c98
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 04:29:38 UTC 2019
---
travis_time:end:06202a94:start=1548476980310689291,finish=1548476980318219409,duration=7530118
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:019cd5ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13b91232
$ dmesg | grep -i kill
