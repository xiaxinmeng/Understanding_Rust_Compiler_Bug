plain
[00:56:05] 
[00:56:05] ---- [ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs stdout ----
[00:56:05] diff of stderr:
[00:56:05] 
[00:56:05] - error: unsatisfied lifetime constraints
[00:56:05] + error: lifetime may not live long enough
[00:56:05] 2   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:35:5
[00:56:05] 3    |
[00:56:05] 4 LL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[00:56:05] 
[00:56:05] 7 LL |     y //~ ERROR unsatisfied lifetime constraints
[00:56:05] 8    |     ^ returning this value requires that `'a` must outlive `'static`
[00:56:05] - error: unsatisfied lifetime constraints
[00:56:05] - error: unsatisfied lifetime constraints
[00:56:05] + error: lifetime may not live long enough
[00:56:05] 11   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:49:5
[00:56:05] 12    |
[00:56:05] 13 LL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[00:56:05] 
[00:56:05] 16 LL |     y //~ ERROR unsatisfied lifetime constraints
[00:56:05] 17    |     ^ returning this value requires that `'a` must outlive `'static`
[00:56:05] - error: unsatisfied lifetime constraints
[00:56:05] - error: unsatisfied lifetime constraints
[00:56:05] + error: lifetime may not live long enough
[00:56:05] 20   --> $DIR/issue-55748-pat-types-constrain-bindings.rs:62:5
[00:56:05] 21    |
[00:56:05] 22 LL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
[00:56:05] 
[00:56:05] The actual stderr differed from the expected stderr.
[00:56:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/issue-55748-pat-types-constrain-bindings.stderr
[00:56:05] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/issue-55748-pat-types-constrain-bindings.stderr
[00:56:05] To update references, rerun the tests and pass the `--bless` flag
[00:56:05] To only update this specific test, also pass `--test-args nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs`
[00:56:05] error: 1 errors occurred comparing output.
[00:56:05] status: exit code: 1
[00:56:05] status: exit code: 1
[00:56:05] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings/auxiliary" "-A" "unused"
[00:56:05] ------------------------------------------
[00:56:05] 
[00:56:05] ------------------------------------------
[00:56:05] stderr:
[00:56:05] stderr:
[00:56:05] ------------------------------------------
[00:56:05] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":903,"byte_end":905,"line_start":28,"line_end":28,"column_start":24,"column_end":26,"is_primary":false,"text":[{"text":"fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":24,"highlight_end":26}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1175,"byte_end":1176,"line_start":35,"line_end":35,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:35:5\n   |\nLL | fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                        -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:56:05] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1343,"byte_end":1345,"line_start":42,"line_end":42,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1612,"byte_end":1613,"line_start":49,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:49:5\n   |\nLL | fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:56:05] {"message":"lifetime may not live long enough","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":1780,"byte_end":1782,"line_start":56,"line_end":56,"column_start":22,"column_end":24,"is_primary":false,"text":[{"text":"fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {","highlight_start":22,"highlight_end":24}],"label":"lifetime `'a` defined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs","byte_start":2041,"byte_end":2042,"line_start":62,"line_end":62,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    y //~ ERROR unsatisfied lifetime constraints","highlight_start":5,"highlight_end":6}],"label":"returning this value requires that `'a` must outlive `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: lifetime may not live long enough\n  --> /checkout/src/test/ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs:62:5\n   |\nLL | fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {\n   |                      -- lifetime `'a` defined here\n...\nLL |     y //~ ERROR unsatisfied lifetime constraints\n   |     ^ returning this value requires that `'a` must outlive `'static`\n\n"}
[00:56:05] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:56:05] ------------------------------------------
[00:56:05] 
[00:56:05] thread '[ui] ui/nll/user-annotations/issue-55748-pat-types-constrain-bindings.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:56:05] 
---
[00:56:05] 
[00:56:05] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:56:05] 
[00:56:05] 
[00:56:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:05] 
[00:56:05] 
[00:56:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:56:05] Build completed unsuccessfully in 0:52:42
---
travis_time:end:3847779a:start=1548481524131822862,finish=1548481524138407223,duration=6584361
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03ef1773
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01d3787f
travis_time:start:01d3787f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a0986ce
$ dmesg | grep -i kill
