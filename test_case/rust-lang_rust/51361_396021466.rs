plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:41:46] 
[00:41:46] running 1513 tests
[00:41:50] ............................................................................................i.......
[00:41:56] ................................................FF.F.F.F........i...................................
[00:42:03] ....................................................................................................
[00:42:06] ....................................................................................................
[00:42:09] ....................................................................................................
[00:42:14] ....................................................................................................
---
[00:42:56] .............
[00:42:56] failures:
[00:42:56] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:42:56] 
[00:42:56] ---- [ui] ui/const-eval/ub-uninhabit.rs stdout ----
[00:42:56] 
[00:42:56] 
[00:42:56] - error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous.
[00:42:56] + error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler
[00:42:56] +                 repository if you believe it should not be considered undefined behavior
[00:42:56] 2   --> $DIR/ub-uninhabit.rs:19:1
[00:42:56] 3    |
[00:42:56] 4 LL | const BAD_BAD_BAD: Bar = unsafe { Foo { a: 1 }.b};
[00:42:56] 
[00:42:56] The actual stderr differed from the expected stderr.
[00:42:56] The actual stderr differed from the expected stderr.
[00:42:56] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-eval/ub-uninhabit/ub-uninhabit.stderr
[00:42:56] To update references, rerun the tests and pass the `--bless` flag
[00:42:56] To only update this specific test, also pass `--test-args const-eval/ub-uninhabit.rs`
[00:42:56] error: 1 errors occurred comparing output.
[00:42:56] status: exit code: 101
[00:42:56] status: exit code: 101
[00:42:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-eval/ub-uninhabitules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior\n  --> /checkout/src/test/ui/const-eval/ub-uninhabit.rs:19:1\n   |\nLL | const BAD_BAD_BAD: Bar = unsafe { Foo { a: 1 }.b};\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of an uninhabited type\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:42:56] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:56] ------------------------------------------
[00:42:56] 
[00:42:56] 
[00:42:56] thread '[ui] ui/const-eval/ub-uninhabit.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:42:56] 
[00:42:56] 
[00:42:56] ---- [ui] ui/const-eval/ub-enum-ptr.rs stdout ----
[00:42:56] 
[00:42:56] 
[00:42:56] - error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous.
[00:42:56] + error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler
[00:42:56] +                 repository if you believe it should not be considered undefined behavior
[00:42:56] 2   --> $DIR/ub-enum-ptr.rs:23:1
[00:42:56] 3    |
[00:42:56] 4 LL | const BAevel":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/ub-enum-ptr.rs","byte_start":615,"byte_end":664,"line_start":23,"line_end":23,"column_start":1,"column_end":50,"is_primary":true,"text":[{"text":"const BAD_ENUM: Enum = unsafe { Foo { a: &1 }.b};","highlight_start":1,"highlight_end":50}],"label":"type validation failed: encountered pointer at .TAG, but expected something in the range 0..=0","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior\n  --> /checkout/src/test/ui/const-eval/ub-enum-ptr.rs:23:1\n   |\nLL | const BAD_ENUM: Enum = unsafe { Foo { a: &1 }.b};\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered pointer at .TAG, but expected something in the range 0..=0\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:42:56] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:56] ------------------------------------------
[00:42:56] 
[00:42:56] 
[00:42:56] thread '[ui] ui/const-eval/ub-enum-ptr.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:42:56] 
[00:42:56] ---- [uisitory if you believe it should not be considered undefined behavior","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/union-const-eval-field.rs","byte_start":975,"byte_end":1022,"line_start":37,"line_end":37,"column_start":5,"column_end":52,"is_primary":true,"text":[{"text":"    const FIELD3: Field3 = unsafe { UNION.field3 }; //~ ERROR const_err","highlight_start":5,"highlight_end":52}],"label":"type validation failed: encountered undefined bytes","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior\n  --> /checkout/src/test/ui/const-eval/union-const-eval-field.rs:37:5\n   |\nLL |     const FIELD3: Field3 = unsafe { UNION.field3 }; //~ ERROR const_err\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered undefined bytes\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:42:56] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:42:56] ------------------------------------------
[00:42:56] 
[00:42:56] 
[00:42:56] thread '[ui] ui/const-eval/union-const-eval-:61,"is_primary":true,"text":[{"text":"const BAD_BOOL: bool = unsafe { DummyUnion { u8: 42 }.bool};","highlight_start":1,"highlight_end":61}],"label":"type validation failed: encountered 42, but expected something in the range 0..=1","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[deny(const_err)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior\n  --> /checkout/src/test/ui/const-eval/union-ub.rs:36:1\n   |\nLL | const BAD_BOOL: bool = unsafe { DummyUnion { u8: 42 }.bool};\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered 42, but expected something in the range 0..=1\n   |\n   = note: #[deny(const_err)] on by default\n\n"}
[00:42:56] {"message":"this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior","code":{"code":"const_err","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/const-eval/union-ub.rs","byte_start":897,"byte_end":950,"line_start":39,"line_end":39,"column_start":1,"column_end":54,"is_primary":true,"text":[{"text":"const BAD_UNION: Foo = unsafe { Bar { u8: 42 }.foo };","highlight_start":1,"highlight_end":54}],"label":"type validation failed: encountered union whose bit pattern matches none of its 2 fields","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this constant likely exhibits undefined behavior. The rules on what exactly is undefined behavior aren't clear, so this check might be ovezealous. Please open an issue on the rust compiler\n                repository if you believe it should not be considered undefined behavior\n  --> /checkout/src/test/ui/const-eval/union-ub.rs:39:1\n   |\nLL | const BAD_UNION: Foo = unsafe { Bar { u8: 42 }.foo };\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered union whose bit pattern matches none of its 2 fields\n\n"}
[00:42:56] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:42:56] ------------------------------------------
[00:42:56] 
[00:42:56] thread '[ui] ui/const-eval/union-ub.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:42:56] 
[00:42:56] 
[00:42:56] 
[00:42:56] failures:
[00:42:56]     [ui] ui/const-eval/ub-enum-ptr.rs
[00:42:56]     [ui] ui/const-eval/ub-uninhabit.rs
[00:42:56]     [ui] ui/const-eval/union-ice.rs
[00:42:56]     [ui] ui/const-eval/union-ub.rs
[00:42:56] 
[00:42:56] test result: FAILED. 1503 passed; 5 failed; 5 ignored; 0 measured; 0 filtered out
[00:42:56] test result: FAILED. 1503 passed; 5 failed; 5 ignored; 0 measured; 0 filtered out
[/obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103928 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1ugt50kek-11cpoqr-no75lmxs5rt9
102904 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
102900 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
99520 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
91560 ./obj/build/x86_64-unknown-linux-gnu/stage1
---
65412 ./src/llvm-emscripten/test/CodeGen
62956 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools[0Ktravis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09c6719e
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07b98f49
$ dmesg | grep -i kill
