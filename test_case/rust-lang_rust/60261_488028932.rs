plain
travis_time:end:05a691b8:start=1556638388505783147,finish=1556638473400023216,duration=84894240069
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:14:58] ..............................................ii...i..ii............................................ 3500/5475
[01:15:02] .................................................................................................... 3600/5475
[01:15:06] .................................................................................................... 3700/5475
[01:15:10] .....................................................ii............................................. 3800/5475
[01:15:12] ...........................................................................iF....................... 3900/5475
[01:15:14] .............................................F...................................................... 4000/5475
[01:15:16] ..............F....................i................................................................ 4100/5475
[01:15:30] .................................................................................................... 4300/5475
[01:15:34] .................................................................................................... 4400/5475
[01:15:37] .................................................................................................... 4500/5475
[01:15:41] .................................................................................................... 4600/5475
---
[01:16:15] failures:
[01:16:15] 
[01:16:15] ---- [ui] ui/parser/byte-string-literals.rs stdout ----
[01:16:15] 
[01:16:15] error: /checkout/src/test/ui/parser/byte-string-literals.rs:3: expected error not found: unknown byte escape
[01:16:15] 
[01:16:15] error: /checkout/src/test/ui/parser/byte-string-literals.rs:6: expected error not found: unknown byte escape
[01:16:15] 
[01:16:15] error: /checkout/src/test/ui/parser/byte-string-literals.rs:7: expected error not found: invalid character in numeric character escape: Z
[01:16:15] error: /checkout/src/test/ui/parser/byte-string-literals.rs:8: expected error not found: byte constant must be ASCII
[01:16:15] 
[01:16:15] error: 0 unexpected errors found, 4 expected errors not found
[01:16:15] status: exit code: 1
[01:16:15] status: exit code: 1
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/byte-string-literals/auxiliary" "-A" "unused"
[01:16:15]     Error {
[01:16:15]         line_num: 3,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "unknown byte escape",
[01:16:15]     Error {
[01:16:15]         line_num: 6,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "unknown byte escape",
[01:16:15]     Error {
[01:16:15]         line_num: 7,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "invalid character in numeric character escape: Z",
[01:16:15]     Error {
[01:16:15]         line_num: 8,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "byte constant must be ASCII",
[01:16:15] ]
[01:16:15] 
[01:16:15] thread '[ui] ui/parser/byte-string-literals.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1402:13
[01:16:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:15] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:16:15] 
[01:16:15] ---- [ui] ui/parser/issue-23620-invalid-escapes.rs stdout ----
[01:16:15] diff of stderr:
[01:16:15] 
[01:16:15] 14   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[01:16:15] 15    |
[01:16:15] 16 LL |     let _ = b'\u';
[01:16:15] -    |               ^^ incorrect unicode escape sequence
[01:16:15] -    |
[01:16:15] -    = help: format of unicode escape sequences is `\u{...}`
[01:16:15] - error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15] -   --> $DIR/issue-23620-invalid-escapes.rs:10:15
[01:16:15] -    |
[01:16:15] -    |
[01:16:15] - LL |     let _ = b'\u';
[01:16:15] 26 
[01:16:15] 26 
[01:16:15] 27 error: numeric character escape is too short
[01:16:15] -   --> $DIR/issue-23620-invalid-escapes.rs:14:17
[01:16:15] +   --> $DIR/issue-23620-invalid-escapes.rs:14:15
[01:16:15] 29    |
[01:16:15] 29    |
[01:16:15] 30 LL |     let _ = b'\x5';
[01:16:15] +    |               ^^^
[01:16:15] 32 
[01:16:15] 32 
[01:16:15] 33 error: invalid character in numeric character escape: x
[01:16:15] 
[01:16:15] 
[01:16:15] 36 LL |     let _ = b'\xxy';
[01:16:15] 38 
[01:16:15] 38 
[01:16:15] - error: invalid character in numeric character escape: y
[01:16:15] -    |
[01:16:15] -    |
[01:16:15] - LL |     let _ = b'\xxy';
[01:16:15] - 
[01:16:15] - 
[01:16:15] 45 error: numeric character escape is too short
[01:16:15] +   --> $DIR/issue-23620-invalid-escapes.rs:21:14
[01:16:15] 47    |
[01:16:15] 48 LL |     let _ = '\x5';
[01:16:15] -    |                ^
[01:16:15] -    |                ^
[01:16:15] +    |              ^^^
[01:16:15] 50 
[01:16:15] 51 error: invalid character in numeric character escape: x
[01:16:15] 
[01:16:15] 
[01:16:15] 54 LL |     let _ = '\xxy';
[01:16:15] 56 
[01:16:15] 56 
[01:16:15] - error: invalid character in numeric character escape: y
[01:16:15] -    |
[01:16:15] -    |
[01:16:15] - LL |     let _ = '\xxy';
[01:16:15] - 
[01:16:15] 63 error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15] 64   --> $DIR/issue-23620-invalid-escapes.rs:28:15
[01:16:15] 65    |
[01:16:15] 65    |
[01:16:15] 
[01:16:15] 66 LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] +    |               ^^^^^^^^^^^^^^^
[01:16:15] 68 
[01:16:15] 68 
[01:16:15] 69 error: invalid character in numeric character escape:  
[01:16:15] 
[01:16:15] 73    |                           ^
[01:16:15] 74 
[01:16:15] 75 error: incorrect unicode escape sequence
[01:16:15] 75 error: incorrect unicode escape sequence
[01:16:15] -   --> $DIR/issue-23620-invalid-escapes.rs:28:28
[01:16:15] +   --> $DIR/issue-23620-invalid-escapes.rs:28:15
[01:16:15] 77    |
[01:16:15] 78 LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] -    |                            ^^ incorrect unicode escape sequence
[01:16:15] -    |
[01:16:15] -    = help: format of unicode escape sequences is `\u{...}`
[01:16:15] 82 
[01:16:15] - error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15] -   --> $DIR/issue-23620-invalid-escapes.rs:28:28
[01:16:15] -    |
[01:16:15] -    |
[01:16:15] - LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] - 
[01:16:15] - 
[01:16:15] 89 error: invalid character in numeric character escape:  
[01:16:15] 91    |
[01:16:15] 
[01:16:15] 
[01:16:15] 92 LL |     let _ = "\xf \u";
[01:16:15] 94 
[01:16:15] 94 
[01:16:15] - error: this form of character escape may only be used with characters in the range [\x00-\x7f]
[01:16:15] -    |
[01:16:15] -    |
[01:16:15] - LL |     let _ = "\xf \u";
[01:16:15] - 
[01:16:15] 101 error: incorrect unicode escape sequence
[01:16:15] -   --> $DIR/issue-23620-invalid-escapes.rs:34:18
[01:16:15] +   --> $DIR/issue-23620-invalid-escapes.rs:34:14
[01:16:15] +   --> $DIR/issue-23620-invalid-escapes.rs:34:14
[01:16:15] 103    |
[01:16:15] 104 LL |     let _ = "\xf \u";
[01:16:15] -    |                  ^^ incorrect unicode escape sequence
[01:16:15] -    |
[01:16:15] -    = help: format of unicode escape sequences is `\u{...}`
[01:16:15] 108 
[01:16:15] 109 error: incorrect unicode escape sequence
[01:16:15] 110   --> $DIR/issue-23620-invalid-escapes.rs:39:14
[01:16:15] 
[01:16:15] 
[01:16:15] 111    |
[01:16:15] 112 LL |     let _ = "\u8f";
[01:16:15] -    |              ^^--
[01:16:15] -    |              |
[01:16:15] -    |              help: format of unicode escape sequences uses braces: `\u{8f}`
[01:16:15] 116 
[01:16:15] - error: aborting due to 18 previous errors
[01:16:15] + error: aborting due to 13 previous errors
[01:16:15] 118 
[01:16:15] 118 
[01:16:15] 119 
[01:16:15] 
[01:16:15] 
[01:16:15] The actual stderr differed from the expected stderr.
[01:16:15] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/issue-23620-invalid-escapes.stderr
[01:16:15] To update references, rerun the tests and pass the `--bless` flag
[01:16:15] To only update this specific test, also pass `--test-args parser/issue-23620-invalid-escapes.rs`
[01:16:15] error: 1 errors occurred comparing output.
[01:16:15] status: exit code: 1
[01:16:15] status: exit code: 1
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-23620-invalid-escapes/auxiliary" "-A" "unused"
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] stderr:
[01:16:15] stderr:
[01:16:15] ------------------------------------------
[01:16:15] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:4:15
[01:16:15]    |
[01:16:15] LL |     let _ = b"\u{a66e}";
[01:16:15] 
[01:16:15] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:7:15
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b'\u{a66e}';
[01:16:15] 
[01:16:15] error: incorrect unicode escape sequence
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:10:15
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b'\u';
[01:16:15] 
[01:16:15] 
[01:16:15] error: numeric character escape is too short
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b'\x5';
[01:16:15] 
[01:16:15] 
[01:16:15] error: invalid character in numeric character escape: x
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b'\xxy';
[01:16:15] 
[01:16:15] 
[01:16:15] error: numeric character escape is too short
[01:16:15]    |
[01:16:15] LL |     let _ = '\x5';
[01:16:15]    |              ^^^
[01:16:15] 
[01:16:15] 
[01:16:15] error: invalid character in numeric character escape: x
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = '\xxy';
[01:16:15] 
[01:16:15] error: unicode escape sequences cannot be used as a byte or in a byte string
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:28:15
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] 
[01:16:15] 
[01:16:15] error: invalid character in numeric character escape:  
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] 
[01:16:15] error: incorrect unicode escape sequence
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:28:15
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = b"\u{a4a4} \xf \u";
[01:16:15] 
[01:16:15] 
[01:16:15] error: invalid character in numeric character escape:  
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = "\xf \u";
[01:16:15] 
[01:16:15] error: incorrect unicode escape sequence
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:34:14
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = "\xf \u";
[01:16:15] 
[01:16:15] error: incorrect unicode escape sequence
[01:16:15]   --> /checkout/src/test/ui/parser/issue-23620-invalid-escapes.rs:39:14
[01:16:15]    |
[01:16:15]    |
[01:16:15] LL |     let _ = "\u8f";
[01:16:15] 
[01:16:15] error: aborting due to 13 previous errors
[01:16:15] 
[01:16:15] 
[01:16:15] 
[01:16:15] ------------------------------------------
[01:16:15] 
[01:16:15] 
[01:16:15] ---- [ui] ui/parser/lex-bare-cr-string-literal-doc-comment.rs stdout ----
[01:16:15] 
[01:16:15] error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21: unexpected error: '21:15: 21:22: character constant must be escaped: \r'
[01:16:15] 
[01:16:15] error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21: expected error not found: bare CR not allowed in string
[01:16:15] error: 1 unexpected errors found, 1 expected errors not found
[01:16:15] status: exit code: 1
[01:16:15] status: exit code: 1
[01:16:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/auxiliary" "-A" "unused"
[01:16:15]     Error {
[01:16:15]         line_num: 21,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "21:15: 21:22: character constant must be escaped: \\r",
[01:16:15] ]
[01:16:15] 
[01:16:15] not found errors (from test file): [
[01:16:15]     Error {
[01:16:15]     Error {
[01:16:15]         line_num: 21,
[01:16:15]         kind: Some(
[01:16:15]             Error,
[01:16:15]         ),
[01:16:15]         msg: "bare CR not allowed in string",
[01:16:15] ]
[01:16:15] 
[01:16:15] thread '[ui] ui/parser/lex-bare-cr-string-literal-doc-comment.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1402:13
[01:16:15] 
---
[01:16:15] 
[01:16:15] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:16:15] 
[01:16:15] 
[01:16:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:16:15] 
[01:16:15] 
[01:16:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:15] Build completed unsuccessfully in 0:04:27
[01:16:15] Build completed unsuccessfully in 0:04:27
[01:16:15] Makefile:48: recipe for target 'check' failed
[01:16:15] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06fe3778
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 30 16:50:58 UTC 2019
---
travis_time:end:2524b0d0:start=1556643059682159105,finish=1556643059687886180,duration=5727075
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:010e3312
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19bc4f4a
travis_time:start:19bc4f4a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:021feb02
$ dmesg | grep -i kill
