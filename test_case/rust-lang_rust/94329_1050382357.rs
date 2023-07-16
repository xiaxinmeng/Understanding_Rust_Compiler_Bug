plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 4b043faba34ccc053a4d0110634c323f6c03765e and 2e0e8ab6c9d8faa1c1980d8d28a562fea0c6cbd2
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
error: tests/compile-fail/validity/fn_ptr_offset.rs:9: expected error not found: expected a function pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/fn_ptr_offset.rs" "-L" "/tmp/compiletestOt0Olm" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestOt0Olm/validity/fn_ptr_offset.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestOt0Olm/validity/fn_ptr_offset.stage-id.aux"
    Error {
        line_num: 9,
        kind: Some(
thread '[compile-fail] compile-fail/validity/fn_ptr_offset.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/runtest.rs:1092:13
---
error: tests/compile-fail/validity/invalid_fnptr_uninit.rs:9: expected error not found: encountered uninitialized bytes, but expected a function pointer

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/invalid_fnptr_uninit.rs" "-L" "/tmp/compiletestOt0Olm" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestOt0Olm/validity/invalid_fnptr_uninit.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestOt0Olm/validity/invalid_fnptr_uninit.stage-id.aux"
   3: <compiletest_rs::make_test_closure::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    Error {
        line_num: 9,
---
error: tests/compile-fail/validity/execute_memory.rs:6: expected error not found: expected a function pointer

error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/validity/execute_memory.rs" "-L" "/tmp/compiletestOt0Olm" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestOt0Olm/validity/execute_memory.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestOt0Olm/validity/execute_memory.stage-id.aux"
    Error {
        line_num: 6,
        kind: Some(
            Error,
---

---- compile_test stdout ----
diff of stderr:

 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     if let Some(n) = dbg!(n.checked_sub(4)) { n } else { n }
    |
    |
    = note: `-D clippy::dbg-macro` implied by `-D warnings`
 help: ensure to avoid having uses of it in version control
    |
 LL |     if let Some(n) = n.checked_sub(4) { n } else { n }
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     if dbg!(n <= 1) {
    |
 help: ensure to avoid having uses of it in version control
    |
    |
 LL |     if n <= 1 {
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
 LL |         dbg!(1)
    |         ^^^^^^^
    |
    |
 help: ensure to avoid having uses of it in version control
    |
 LL |         1
    |
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |         dbg!(n * factorial(n - 1))
    |
 help: ensure to avoid having uses of it in version control
    |
    |
 LL |         n * factorial(n - 1)
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     dbg!(42);
    |
 help: ensure to avoid having uses of it in version control
    |
 LL |     42;
 LL |     42;
    |     ~~
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     dbg!(dbg!(dbg!(42)));
    |
 help: ensure to avoid having uses of it in version control
    |
    |
 LL |     dbg!(dbg!(42));
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     foo(3) + dbg!(factorial(4));
    |
 help: ensure to avoid having uses of it in version control
    |
    |
 LL |     foo(3) + factorial(4);
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     dbg!(1, 2, dbg!(3, 4));
    |
 help: ensure to avoid having uses of it in version control
    |
    |
 LL |     (1, 2, dbg!(3, 4));
 
 
 error: `dbg!` macro is intended as a debugging tool
    |
    |
 LL |     dbg!(1, 2, 3, 4, 5);
    |
 help: ensure to avoid having uses of it in version control
    |
 LL |     (1, 2, 3, 4, 5);
 LL |     (1, 2, 3, 4, 5);
    |     ~~~~~~~~~~~~~~~
 
 error: `dbg!` macro is intended as a debugging tool
   --> $DIR/dbg_macro.rs:40:9
    |
    |
 LL |         dbg!(2);
    |
 help: ensure to avoid having uses of it in version control
    |
 LL |         2;
---
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
+
+note: Clippy version: clippy 0.1.61 (2e0e8ab 2022-02-24)
+query stack during panic:
+query stack during panic:
+#0 [lint_mod] linting top-level module
+#1 [analysis] running analysis passes on this crate
+end of query stack
 
 

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/dbg_macro.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dbg_macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/dbg_macro.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/dbg_macro.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/dbg_macro.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":74,"byte_end":96,"line_start":4,"line_end":4,"column_start":22,"column_end":44,"is_primary":true,"text":[{"text":"    if let Some(n) = dbg!(n.checked_sub(4)) { n } else { n }","highlight_start":22,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::dbg-macro` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":74,"byte_end":96,"line_start":4,"line_end":4,"column_start":22,"column_end":44,"is_primary":true,"text":[{"text":"    if let Some(n) = dbg!(n.checked_sub(4)) { n } else { n }","highlight_start":22,"highlight_end":44}],"label":null,"suggested_replacement":"n.checked_sub(4)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:4:22\n   |\nLL |     if let Some(n) = dbg!(n.checked_sub(4)) { n } else { n }\n   |                      ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::dbg-macro` implied by `-D warnings`\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     if let Some(n) = n.checked_sub(4) { n } else { n }\n   |                      ~~~~~~~~~~~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":154,"byte_end":166,"line_start":8,"line_end":8,"column_start":8,"column_end":20,"is_primary":true,"text":[{"text":"    if dbg!(n <= 1) {","highlight_start":8,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":154,"byte_end":166,"line_start":8,"line_end":8,"column_start":8,"column_end":20,"is_primary":true,"text":[{"text":"    if dbg!(n <= 1) {","highlight_start":8,"highlight_end":20}],"label":null,"suggested_replacement":"n <= 1","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:8:8\n   |\nLL |     if dbg!(n <= 1) {\n   |        ^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     if n <= 1 {\n   |        ~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":177,"byte_end":184,"line_start":9,"line_end":9,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        dbg!(1)","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":177,"byte_end":184,"line_start":9,"line_end":9,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        dbg!(1)","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"1","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:9:9\n   |\nLL |         dbg!(1)\n   |         ^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |         1\n   |\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":206,"byte_end":232,"line_start":11,"line_end":11,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        dbg!(n * factorial(n - 1))","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":206,"byte_end":232,"line_start":11,"line_end":11,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"        dbg!(n * factorial(n - 1))","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":"n * factorial(n - 1)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:11:9\n   |\nLL |         dbg!(n * factorial(n - 1))\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |         n * factorial(n - 1)\n   |\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":258,"byte_end":266,"line_start":16,"line_end":16,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    dbg!(42);","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":258,"byte_end":266,"line_start":16,"line_end":16,"column_start":5,"column_end":13,"is_primary":true,"text":[{"text":"    dbg!(42);","highlight_start":5,"highlight_end":13}],"label":null,"suggested_replacement":"42","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:16:5\n   |\nLL |     dbg!(42);\n   |     ^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     42;\n   |     ~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":272,"byte_end":292,"line_start":17,"line_end":17,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    dbg!(dbg!(dbg!(42)));","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":272,"byte_end":292,"line_start":17,"line_end":17,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    dbg!(dbg!(dbg!(42)));","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":"dbg!(dbg!(42))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:17:5\n   |\nLL |     dbg!(dbg!(dbg!(42)));\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     dbg!(dbg!(42));\n   |     ~~~~~~~~~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":307,"byte_end":325,"line_start":18,"line_end":18,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    foo(3) + dbg!(factorial(4));","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":307,"byte_end":325,"line_start":18,"line_end":18,"column_start":14,"column_end":32,"is_primary":true,"text":[{"text":"    foo(3) + dbg!(factorial(4));","highlight_start":14,"highlight_end":32}],"label":null,"suggested_replacement":"factorial(4)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:18:14\n   |\nLL |     foo(3) + dbg!(factorial(4));\n   |              ^^^^^^^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     foo(3) + factorial(4);\n   |              ~~~~~~~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":331,"byte_end":353,"line_start":19,"line_end":19,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    dbg!(1, 2, dbg!(3, 4));","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":331,"byte_end":353,"line_start":19,"line_end":19,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    dbg!(1, 2, dbg!(3, 4));","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"(1, 2, dbg!(3, 4))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:19:5\n   |\nLL |     dbg!(1, 2, dbg!(3, 4));\n   |     ^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     (1, 2, dbg!(3, 4));\n   |     ~~~~~~~~~~~~~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":359,"byte_end":378,"line_start":20,"line_end":20,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    dbg!(1, 2, 3, 4, 5);","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":359,"byte_end":378,"line_start":20,"line_end":20,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    dbg!(1, 2, 3, 4, 5);","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"(1, 2, 3, 4, 5)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:20:5\n   |\nLL |     dbg!(1, 2, 3, 4, 5);\n   |     ^^^^^^^^^^^^^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |     (1, 2, 3, 4, 5);\n   |     ~~~~~~~~~~~~~~~\n\n"}
{"message":"`dbg!` macro is intended as a debugging tool","code":{"code":"clippy::dbg_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":736,"byte_end":743,"line_start":40,"line_end":40,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        dbg!(2);","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"ensure to avoid having uses of it in version control","code":null,"level":"help","spans":[{"file_name":"tests/ui/dbg_macro.rs","byte_start":736,"byte_end":743,"line_start":40,"line_end":40,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        dbg!(2);","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":"2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `dbg!` macro is intended as a debugging tool\n  --> tests/ui/dbg_macro.rs:40:9\n   |\nLL |         dbg!(2);\n   |         ^^^^^^^\n   |\nhelp: ensure to avoid having uses of it in version control\n   |\nLL |         2;\n   |         ~\n\n"}
  left: `3`,
 right: `2`', compiler/rustc_lint/src/unused.rs:244:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.61 (2e0e8ab 2022-02-24)
query stack during panic:
query stack during panic:
#0 [lint_mod] linting top-level module
#1 [analysis] running analysis passes on this crate
{"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}

------------------------------------------

