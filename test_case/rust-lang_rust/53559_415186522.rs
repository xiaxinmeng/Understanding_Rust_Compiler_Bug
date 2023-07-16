plain
[00:45:38] ....................................................................................................
[00:45:41] ....................................................................................................
[00:45:44] ....................................................................................................
[00:45:46] ....................................................................................................
[00:45:50] ...........................................................................F........................
[00:45:56] ...........................................................................................i.i..ii..
[00:45:59] ....................................................................................................
[00:46:02] .......................................................................................i............
[00:46:05] ....................................................................................................
---
[00:46:26] ...................................................i................................................
[00:46:29] ....................................................................................................
[00:46:31] ....................................................................................................
[00:46:34] ..............................................................................................i.....
 command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lints-in-foreign-macros.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lints-in-foreign-macros/auxiliary" "-A" "unused"
[00:46:35] ------------------------------------------
[00:46:35] 
[00:46:35] ------------------------------------------
[00:46:35] stderr:
[00:46:35] stderr:
[00:46:35] ------------------------------------------
[00:46:35] {"message":"unused import: `std::string::ToString`","code":{"code":"unused_imports","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":659,"byte_end":680,"line_start":21,"line_end":21,"column_start":16,"column_end":37,"is_primary":true,"text":[{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":16,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":718,"byte_end":725,"line_start":24,"line_end":24,"column_start":9,"column_end":16,"is_primary":false,"text":[{"text":"mod a { foo!(); }","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"foo!","def_site_span":{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":625,"byte_end":708,"line_start":20,"line_end":22,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":19},{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":1,"highlight_end":63},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":532,"byte_end":546,"line_start":14,"line_end":14,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"#![warn(unused_imports)]","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused import: `std::string::ToString`\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:21:16\n   |\nLL |     () => {use std::string::ToString;} //~ WARN: unused import\n   |                ^^^^^^^^^^^^^^^^^^^^^\n...\nLL | mod a { foo!(); }\n   |         ------- in this macro invocation\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/lint/lints-in-foreign-macros.rs:14:9\n   |\nLL | #![warn(unused_imports)]\n   |         ^^^^^^^^^^^^^^\n\n"}
[00:46:35] {"message":"unused import: `std::string::ToString`","code":{"code":"unused_imports","explanation":nul"spans":[{"file_name":"/checkout/src/test/ui/lint/lints-in-foreign-macros.rs","byte_start":524,"byte_end":984,"line_start":14,"line_end":30,"column_start":1,"column_end":13,"is_primary":true,"text":[{"text":"#![warn(unused_imports)]","highlight_start":1,"highlight_end":25},{"text":"#![warn(missing_docs)]","highlight_start":1,"highlight_end":23},{"text":"","highlight_start":1,"highlight_end":1},{"text":"#[macro_use]","highlight_start":1,"highlight_end":13},{"text":"extern crate lints_in_foreign_macros;","highlight_start":1,"highlight_end":38},{"text":"","highlight_start":1,"highlight_end":1},{"text":"macro_rules! foo {","highlight_start":1,"highlight_end":19},{"text":"    () => {use std::string::ToString;} //~ WARN: unused import","highlight_start":1,"highlight_end":63},{"text":"}","highlight_start":1,"highlight_end":2},{"text":"","highlight_start":1,"highlight_end":1},{"text":"mod a { foo!(); }","highlight_start":1,"highlight_end":18},{"text":"mod b { bar!(); }","highlight_start":1,"highlight_end":18},{"text":"mod c { baz!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":1,"highlight_end":68},{"text":"mod d { baz2!(use std::string::ToString;); } //~ WARN: unused import","highlight_start":1,"highlight_end":69},{"text":"mod e { baz!(pub fn undocumented() {}); }//~ WARN: missing documentation for a function","highlight_start":1,"highlight_end":88},{"text":"","highlight_start":1,"highlight_end":1},{"text":"fn main() {}","highlight_start":1,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","codekout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:46:35] 
[00:46:35] 
[00:46:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:46:35] Build completed unsuccessfully in 0:03:00
[00:46:35] Build completed unsuccessfully in 0:03:00
[00:46:35] make: *** [check] Error 1
[00:46:35] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:004511c7
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0057742e:start=1534973247611677186,finish=1534973247622287879,duration=10610693
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25fe3c00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2df3483f
$ dmesg | grep -i kill
