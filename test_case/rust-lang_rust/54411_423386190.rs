plain
[00:46:17] ....................................................................................................
[00:46:19] ....................................................................................................
[00:46:22] ....................................................................................................
[00:46:24] ....................................................................................................
[00:46:26] ................................................................................F...................
[00:46:32] .......i............................................................................................
[00:46:32] .......i............................................................................................
[00:46:36] ..................F.................................................................................
[00:46:41] ....................................................................................................
[00:46:44] ....................................................................................................
[00:46:47] ....................................................................................................
[00:46:49] ....................................................................................................
---
[00:52:02] ....................................................................................................
[00:52:06] ............................................................................................i.......
[00:52:09] ....................................................................................................
[00:52:13] ....................................................................................................
[00:52:18] .......F......................................i.....................................................
[00:52:18] failures:
[00:52:18] 
[00:52:18] ---- [ui] ui/feature-gates/feature-gate-non_ascii_idents.rs stdout ----
[00:52:18] 
[00:52:18] 
[00:52:18] error: ui test compiled successfully!
[00:52:18] status: exit code: 0
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-non_ascii_idents.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_ascii_idents/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_ascii_idents/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
---
[00:52:18] ---- [ui] ui/imports/local-modularized-tricky-fail-2.rs stdout ----
[00:52:18] 
[00:52:18] error: ui test compiled successfully!
[00:52:18] status: exit code: 0
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-2/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
---
[00:52:18] ---- [ui] ui/utf8_idents.rs stdout ----
[00:52:18] 
[00:52:18] error: ui test compiled successfully!
[00:52:18] status: exit code: 0
[00:52:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/utf8_idents.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/utf8_idents/auxiliary" "-A" "unused"
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] ------------------------------------------
[00:52:18] stderr:
[00:52:18] stderr:
[00:52:18] ------------------------------------------
[00:52:18] {"message":"type parameter `γ` should have a camel case name such as `Γ`","code":{"code":"non_camel_case_types","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/utf8_idents.rs","byte_start":542,"byte_end":544,"line_start":14,"line_end":14,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    γ  //~ ERROR non-ascii idents are not fully supported","highlight_start":5,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_camel_case_types)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: type parameter `γ` should have a camel case name such as `Γ`\n  --> /checkout/src/test/ui/utf8_idents.rs:14:5\n   |\nLL |     γ  //~ ERROR non-ascii idents are not fully supported\n   |     ^\n   |\n   = note: #[warn(non_camel_case_types)] on by default\n\n"}
[00:52:18] ------------------------------------------
[00:52:18] 
[00:52:18] thread '[ui] ui/utf8_idents.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:52:18] 
[00:52:18] 
[00:52:18] 
[00:52:18] failures:
[00:52:18]     [ui] ui/feature-gates/feature-gate-non_ascii_idents.rs
[00:52:18]     [ui] ui/imports/local-modularized-tricky-fail-2.rs
[00:52:18]     [ui] ui/utf8_idents.rs
[00:52:18] 
[00:52:18] test result:heckout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07186ca7
travis_time:start:07186ca7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e5f4ba8
$ dmesg | grep -i kill
