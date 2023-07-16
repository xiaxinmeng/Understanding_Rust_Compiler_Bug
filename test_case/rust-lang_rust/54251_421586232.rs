plain
[00:46:34] ....................................................................................................
[00:46:37] ....................................................................................................
[00:46:41] ....................................................................................................
[00:46:43] ....................................................................................................
[00:46:47] .....................................FFFF...........................................................
[00:46:52] ....................................................................................................
[00:46:56] ...................................i.i..ii..........................................................
[00:46:59] ....................................................................................................
[00:47:02] ...............................................i....................................................
---
[00:51:47] ..................................................................i.................................
[00:51:49] ....................................................................................................
[00:51:53] ....................................................................................................
[00:51:55] ....................................................................................................
:"    let _InappropriateCamelCasing = true;","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_snake_case)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `_InappropriateCamelCasing` should have a snake case name such as `_inappropriate_camel_casing`\n  --> /checkout/src/test/ui/lint/command-line-lint-group-allow.rs:15:9\n   |\nLL |     let _InappropriateCamelCasing = true;\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: #[warn(non_snake_case)] on by default\n\n"}
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] thread '[ui] ui/lint/command-line-lint-group-allow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:58] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:51:58] 
[00:51:58] ---- [ui] ui/lint/command-line-lint-group-deny.rs stdout ----
[00:51:58] 
[00:51:58] error: ui test compiled successfully!
[00:51:58] status: exit code: 0
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-deny.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-deny/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu[00:51:58] ---- [ui] ui/lint/command-line-lint-group-forbid.rs stdout ----
[00:51:58] error: ui test compiled successfully!
[00:51:58] status: exit code: 0
[00:51:58] status: exit code: 0
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-forbid.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-forbid/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-forbid/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"variable `_InappropriateCamelCasing` should have a snake case name such as `_inappropriate_camel_casing`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/command-line-lint-group-forbid.rs","byte_start":519,"byte_end":544,"line_start":14,"line_end":14,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"    let _InappropriateCamelCasing = true; //~ ERROR should have a snake","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_snake_case
[00:51:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/command-line-lint-group-warn.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-warn/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-W" "bad-style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/command-line-lint-group-warn/auxiliary" "-A" "unused"
[00:51:58] ------------------------------------------
[00:51:58] 
[00:51:58] ------------------------------------------
[00:51:58] stderr:
[00:51:58] stderr:
[00:51:58] ------------------------------------------
[00:51:58] {"message":"variable `_InappropriateCamelCasing` should have a snake case name such as `_inappropriate_camel_casing`","code":{"code":"non_snake_case","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/command-line-lint-group-warn.rs","byte_start":535,"byte_end":560,"line_start":15,"line_end":15,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"    let _InappropriateCamelCasing = true;","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(non_snake_case)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: variable `_InappropriateCamelCasing` should have a snake case name such as `_inappropriate_camel_cas

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1de94c84
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:017036be:start=1537024933176532934,finish=1537024933199962094,duration=23429160
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05a1f50e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19edf788
$ dmesg | grep -i kill
