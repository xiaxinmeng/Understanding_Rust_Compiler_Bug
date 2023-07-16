plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0b90256ada21c6a81b4c18f2c7a23151ab5fc232 and 4ecbab4ced45464adc52a5f4ad3d34cb9cc6e8c4
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: using `eprint!()` with a format string that ends in a single newline
    |
    |
 LL |     eprint!("Hello/n");
    |
    |
    = note: `-D clippy::print-with-newline` implied by `-D warnings`
 help: use `eprintln!` instead
    |
 LL -     eprint!("Hello/n");
 LL +     eprintln!("Hello");
 
 
 error: using `eprint!()` with a format string that ends in a single newline
-   |
-   |
-LL |     eprint!("Hello {}/n", "world");
-   |
-   |
-help: use `eprintln!` instead
-   |
-LL -     eprint!("Hello {}/n", "world");
-LL +     eprintln!("Hello {}", "world");
-
-
-error: using `eprint!()` with a format string that ends in a single newline
-   |
-   |
-LL |     eprint!("Hello {} {}/n", "world", "#2");
-   |
-   |
-help: use `eprintln!` instead
-   |
-LL -     eprint!("Hello {} {}/n", "world", "#2");
-LL +     eprintln!("Hello {} {}", "world", "#2");
-
-
-error: using `eprint!()` with a format string that ends in a single newline
-   |
-   |
-LL |     eprint!("{}/n", 1265);
-   |
-   |
-help: use `eprintln!` instead
-   |
-LL -     eprint!("{}/n", 1265);
-LL +     eprintln!("{}", 1265);
-
-
-error: using `eprint!()` with a format string that ends in a single newline
    |
    |
 LL |     eprint!("/n");
    |
    |
 help: use `eprintln!` instead
    |
 LL -     eprint!("/n");
 LL +     eprintln!();
 
 
 error: using `eprint!()` with a format string that ends in a single newline
    |
    |
 LL |     eprint!("//n"); // should fail
    |
    |
 help: use `eprintln!` instead
    |
 LL -     eprint!("//n"); // should fail
 LL +     eprintln!("/"); // should fail
 
 
 error: using `eprint!()` with a format string that ends in a single newline
    |
 LL | /     eprint!(
 LL | |         "
 LL | | "
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `eprintln!` instead
 LL ~     eprintln!(
 LL ~         
    |
 
 
 error: using `eprint!()` with a format string that ends in a single newline
    |
 LL | /     eprint!(
 LL | |         r"
 LL | | "
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `eprintln!` instead
 LL ~     eprintln!(
 LL ~         
    |
 
 
 error: using `eprint!()` with a format string that ends in a single newline
    |
    |
 LL |     eprint!("/r/n"); //~ ERROR
    |
    |
 help: use `eprintln!` instead
    |
 LL -     eprint!("/r/n"); //~ ERROR
 LL +     eprintln!("/r"); //~ ERROR
 
-error: aborting due to 9 previous errors
+error: aborting due to 6 previous errors
 
---
To only update this specific test, also pass `--test-args eprint_with_newline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/eprint_with_newline.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/eprint_with_newline.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/eprint_with_newline.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":87,"byte_end":105,"line_start":5,"line_end":5,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    eprint!(\"Hello\\n\");","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::print-with-newline` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":87,"byte_end":93,"line_start":5,"line_end":5,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(\"Hello\\n\");","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":101,"byte_end":103,"line_start":5,"line_end":5,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"    eprint!(\"Hello\\n\");","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:5:5\n   |\nLL |     eprint!(\"Hello\\n\");\n   |     ^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::print-with-newline` implied by `-D warnings`\nhelp: use `eprintln!` instead\n   |\nLL -     eprint!(\"Hello\\n\");\nLL +     eprintln!(\"Hello\");\n   |\n\n"}
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":219,"byte_end":232,"line_start":9,"line_end":9,"column_start":5,"column_end":18,"is_primary":true,"text":[{"text":"    eprint!(\"\\n\");","highlight_start":5,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":219,"byte_end":225,"line_start":9,"line_end":9,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(\"\\n\");","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":227,"byte_end":231,"line_start":9,"line_end":9,"column_start":13,"column_end":17,"is_primary":true,"text":[{"text":"    eprint!(\"\\n\");","highlight_start":13,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:9:5\n   |\nLL |     eprint!(\"\\n\");\n   |     ^^^^^^^^^^^^^\n   |\nhelp: use `eprintln!` instead\n   |\nLL -     eprint!(\"\\n\");\nLL +     eprintln!();\n   |\n\n"}
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":724,"byte_end":739,"line_start":28,"line_end":28,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":724,"byte_end":730,"line_start":28,"line_end":28,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":735,"byte_end":737,"line_start":28,"line_end":28,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\\\n\"); // should fail","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:28:5\n   |\nLL |     eprint!(\"\\\\\\n\"); // should fail\n   |     ^^^^^^^^^^^^^^^\n   |\nhelp: use `eprintln!` instead\n   |\nLL -     eprint!(\"\\\\\\n\"); // should fail\nLL +     eprintln!(\"\\\\\"); // should fail\n   |\n\n"}
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":873,"byte_end":899,"line_start":35,"line_end":38,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    eprint!(","highlight_start":5,"highlight_end":13},{"text":"        \"","highlight_start":1,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":873,"byte_end":879,"line_start":35,"line_end":35,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":890,"byte_end":893,"line_start":36,"line_end":37,"column_start":9,"column_end":2,"is_primary":true,"text":[{"text":"        \"","highlight_start":9,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:35:5\n   |\nLL | /     eprint!(\nLL | |         \"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `eprintln!` instead\n   |\nLL ~     eprintln!(\nLL ~         \n   |\n\n"}
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":905,"byte_end":932,"line_start":39,"line_end":42,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    eprint!(","highlight_start":5,"highlight_end":13},{"text":"        r\"","highlight_start":1,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":905,"byte_end":911,"line_start":39,"line_end":39,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":922,"byte_end":926,"line_start":40,"line_end":41,"column_start":9,"column_end":2,"is_primary":true,"text":[{"text":"        r\"","highlight_start":9,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:39:5\n   |\nLL | /     eprint!(\nLL | |         r\"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `eprintln!` instead\n   |\nLL ~     eprintln!(\nLL ~         \n   |\n\n"}
{"message":"using `eprint!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":1018,"byte_end":1034,"line_start":47,"line_end":47,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `eprintln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":1018,"byte_end":1024,"line_start":47,"line_end":47,"column_start":5,"column_end":11,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":11}],"label":null,"suggested_replacement":"eprintln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/eprint_with_newline.rs","byte_start":1030,"byte_end":1032,"line_start":47,"line_end":47,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    eprint!(\"\\\\r\\n\"); //~ ERROR","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `eprint!()` with a format string that ends in a single newline\n  --> tests/ui/eprint_with_newline.rs:47:5\n   |\nLL |     eprint!(\"\\\\r\\n\"); //~ ERROR\n   |     ^^^^^^^^^^^^^^^^\n   |\nhelp: use `eprintln!` instead\n   |\nLL -     eprint!(\"\\\\r\\n\"); //~ ERROR\nLL +     eprintln!(\"\\\\r\"); //~ ERROR\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:34:26
    |
 LL |     with_none_and_format.expect(&format!("Error {}: fake error", error_code));
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
    |
    = note: `-D clippy::expect-fun-call` implied by `-D warnings`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:37:26
    |
    |
 LL |     with_none_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
 error: use of `expect` followed by a function call
-  --> $DIR/expect_fun_call.rs:40:37
-   |
-   |
-LL |     with_none_and_format_with_macro.expect(format!("Error {}: fake error", one!()).as_str());
-   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", one!()))`
-error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:50:25
    |
    |
 LL |     with_err_and_format.expect(&format!("Error {}: fake error", error_code));
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:53:25
    |
    |
 LL |     with_err_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
 error: use of `expect` followed by a function call
-  --> $DIR/expect_fun_call.rs:65:17
-   |
-   |
-LL |     Some("foo").expect(format!("{} {}", 1, 2).as_ref());
-   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{} {}", 1, 2))`
-error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:86:21
    |
    |
 LL |         Some("foo").expect(&get_string());
    |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:87:21
    |
    |
 LL |         Some("foo").expect(get_string().as_ref());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:88:21
    |
    |
 LL |         Some("foo").expect(get_string().as_str());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:90:21
    |
    |
 LL |         Some("foo").expect(get_static_str());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_static_str()) })`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:91:21
    |
    |
 LL |         Some("foo").expect(get_non_static_str(&0));
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_non_static_str(&0).to_string()) })`
 error: use of `expect` followed by a function call
-  --> $DIR/expect_fun_call.rs:95:16
-   |
-   |
-LL |     Some(true).expect(&format!("key {}, {}", 1, 2));
-   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("key {}, {}", 1, 2))`
-error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:101:17
    |
    |
 LL |         opt_ref.expect(&format!("{:?}", opt_ref));
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{:?}", opt_ref))`
 error: use of `expect` followed by a function call
   --> $DIR/expect_fun_call.rs:105:20
    |
    |
 LL |     format_capture.expect(&format!("{error_code}"));
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{error_code}"))`
-error: use of `expect` followed by a function call
-  --> $DIR/expect_fun_call.rs:108:30
-   |
-   |
-LL |     format_capture_and_value.expect(&format!("{error_code}, {}", 1));
-   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{error_code}, {}", 1))`
-error: aborting due to 15 previous errors
+error: aborting due to 11 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/expect_fun_call.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::expect_fun_call)]
 #![allow(clippy::to_string_in_format_args, clippy::uninlined_format_args)]
 
 /// Checks implementation of the `EXPECT_FUN_CALL` lint
 macro_rules! one {
     () => {
         1
     };
---
         fn new() -> Self {
             Foo
         }
 
         fn expect(&self, msg: &str) {
             panic!("{}", msg)
     }
 
     let with_some = Some("value");
     with_some.expect("error");
     with_some.expect("error");
 
     let with_none: Option<i32> = None;
     with_none.expect("error");
     let error_code = 123_i32;
     let error_code = 123_i32;
     let with_none_and_format: Option<i32> = None;
     with_none_and_format.unwrap_or_else(|| panic!("Error {}: fake error", error_code));
 
     let with_none_and_as_str: Option<i32> = None;
     with_none_and_as_str.unwrap_or_else(|| panic!("Error {}: fake error", error_code));
 
     let with_none_and_format_with_macro: Option<i32> = None;
-    with_none_and_format_with_macro.unwrap_or_else(|| panic!("Error {}: fake error", one!()));
+    with_none_and_format_with_macro.expect(format!("Error {}: fake error", one!()).as_str());
 
     let with_ok: Result<(), ()> = Ok(());
     with_ok.expect("error");
 
     let with_err: Result<(), ()> = Err(());
     with_err.expect("error");
     let error_code = 123_i32;
     let error_code = 123_i32;
     let with_err_and_format: Result<(), ()> = Err(());
     with_err_and_format.unwrap_or_else(|_| panic!("Error {}: fake error", error_code));
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
     let with_err_and_as_str: Result<(), ()> = Err(());
     with_err_and_as_str.unwrap_or_else(|_| panic!("Error {}: fake error", error_code));
     let with_dummy_type = Foo::new();
     let with_dummy_type = Foo::new();
     with_dummy_type.expect("another test string");
     let with_dummy_type_and_format = Foo::new();
     let with_dummy_type_and_format = Foo::new();
     with_dummy_type_and_format.expect(&format!("Error {}: fake error", error_code));
     let with_dummy_type_and_as_str = Foo::new();
     let with_dummy_type_and_as_str = Foo::new();
     with_dummy_type_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
     //Issue #2937
     //Issue #2937
-    Some("foo").unwrap_or_else(|| panic!("{} {}", 1, 2));
+    Some("foo").expect(format!("{} {}", 1, 2).as_ref());
     //Issue #2979 - this should not lint
     {
         let msg = "bar";
         let msg = "bar";
         Some("foo").expect(msg);
 
     {
         fn get_string() -> String {
             "foo".to_string()
             "foo".to_string()
         }
 
         fn get_static_str() -> &'static str {
             "foo"
         }
 
         fn get_non_static_str(_: &u32) -> &str {
         }
 
 
         Some("foo").unwrap_or_else(|| { panic!("{}", get_string()) });
         Some("foo").unwrap_or_else(|| { panic!("{}", get_string()) });
         Some("foo").unwrap_or_else(|| { panic!("{}", get_string()) });
 
         Some("foo").unwrap_or_else(|| { panic!("{}", get_static_str()) });
         Some("foo").unwrap_or_else(|| { panic!("{}", get_non_static_str(&0).to_string()) });
 
     //Issue #3839
     //Issue #3839
-    Some(true).unwrap_or_else(|| panic!("key {}, {}", 1, 2));
+    Some(true).expect(&format!("key {}, {}", 1, 2));
 
     //Issue #4912 - the receiver is a &Option
         let opt = Some(1);
         let opt_ref = &opt;
         let opt_ref = &opt;
         opt_ref.unwrap_or_else(|| panic!("{:?}", opt_ref));
 
 
     let format_capture: Option<i32> = None;
     format_capture.unwrap_or_else(|| panic!("{error_code}"));
 
     let format_capture_and_value: Option<i32> = None;
-    format_capture_and_value.unwrap_or_else(|| panic!("{error_code}, {}", 1));
+    format_capture_and_value.expect(&format!("{error_code}, {}", 1));
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/expect_fun_call.stage-id.fixed
To only update this specific test, also pass `--test-args expect_fun_call.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/expect_fun_call.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/expect_fun_call.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/expect_fun_call.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":659,"byte_end":711,"line_start":34,"line_end":34,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::expect-fun-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":659,"byte_end":711,"line_start":34,"line_end":34,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"Error {}: fake error\", error_code))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:34:26\n   |\nLL |     with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"Error {}: fake error\", error_code))`\n   |\n   = note: `-D clippy::expect-fun-call` implied by `-D warnings`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":789,"byte_end":849,"line_start":37,"line_end":37,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":789,"byte_end":849,"line_start":37,"line_end":37,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"Error {}: fake error\", error_code))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:37:26\n   |\nLL |     with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"Error {}: fake error\", error_code))`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1264,"byte_end":1316,"line_start":50,"line_end":50,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1264,"byte_end":1316,"line_start":50,"line_end":50,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":"unwrap_or_else(|_| panic!(\"Error {}: fake error\", error_code))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:50:25\n   |\nLL |     with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!(\"Error {}: fake error\", error_code))`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1398,"byte_end":1458,"line_start":53,"line_end":53,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1398,"byte_end":1458,"line_start":53,"line_end":53,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":"unwrap_or_else(|_| panic!(\"Error {}: fake error\", error_code))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:53:25\n   |\nLL |     with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!(\"Error {}: fake error\", error_code))`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2273,"byte_end":2294,"line_start":86,"line_end":86,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(&get_string());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2273,"byte_end":2294,"line_start":86,"line_end":86,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(&get_string());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:86:21\n   |\nLL |         Some(\"foo\").expect(&get_string());\n   |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2316,"byte_end":2345,"line_start":87,"line_end":87,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_ref());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2316,"byte_end":2345,"line_start":87,"line_end":87,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_ref());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:87:21\n   |\nLL |         Some(\"foo\").expect(get_string().as_ref());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2367,"byte_end":2396,"line_start":88,"line_end":88,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_str());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2367,"byte_end":2396,"line_start":88,"line_end":88,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_str());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:88:21\n   |\nLL |         Some(\"foo\").expect(get_string().as_str());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2419,"byte_end":2443,"line_start":90,"line_end":90,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_static_str());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2419,"byte_end":2443,"line_start":90,"line_end":90,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_static_str());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_static_str()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:90:21\n   |\nLL |         Some(\"foo\").expect(get_static_str());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_static_str()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2465,"byte_end":2495,"line_start":91,"line_end":91,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_non_static_str(&0));","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2465,"byte_end":2495,"line_start":91,"line_end":91,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_non_static_str(&0));","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_non_static_str(&0).to_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:91:21\n   |\nLL |         Some(\"foo\").expect(get_non_static_str(&0));\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_non_static_str(&0).to_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2699,"byte_end":2732,"line_start":101,"line_end":101,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"        opt_ref.expect(&format!(\"{:?}\", opt_ref));","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2699,"byte_end":2732,"line_start":101,"line_end":101,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"        opt_ref.expect(&format!(\"{:?}\", opt_ref));","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"{:?}\", opt_ref))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:101:17\n   |\nLL |         opt_ref.expect(&format!(\"{:?}\", opt_ref));\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"{:?}\", opt_ref))`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2804,"byte_end":2836,"line_start":105,"line_end":105,"column_start":20,"column_end":52,"is_primary":true,"text":[{"text":"    format_capture.expect(&format!(\"{error_code}\"));","highlight_start":20,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2804,"byte_end":2836,"line_start":105,"line_end":105,"column_start":20,"column_end":52,"is_primary":true,"text":[{"text":"    format_capture.expect(&format!(\"{error_code}\"));","highlight_start":20,"highlight_end":52}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"{error_code}\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:105:20\n   |\nLL |     format_capture.expect(&format!(\"{error_code}\"));\n   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"{error_code}\"))`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: use of `write!(stdout(), ...).unwrap()`
    |
 LL |         write!(std::io::stdout(), "test").unwrap();
 LL |         write!(std::io::stdout(), "test").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!("test")`
    |
    = note: `-D clippy::explicit-write` implied by `-D warnings`
 
 error: use of `write!(stderr(), ...).unwrap()`
    |
    |
 LL |         write!(std::io::stderr(), "test").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!("test")`
 
 error: use of `writeln!(stdout(), ...).unwrap()`
    |
 LL |         writeln!(std::io::stdout(), "test").unwrap();
 LL |         writeln!(std::io::stdout(), "test").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test")`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "test").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test")`
 
 error: use of `stdout().write_fmt(...).unwrap()`
    |
    |
 LL |         std::io::stdout().write_fmt(format_args!("test")).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!("test")`
 
 error: use of `stderr().write_fmt(...).unwrap()`
    |
    |
 LL |         std::io::stderr().write_fmt(format_args!("test")).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!("test")`
 
 error: use of `writeln!(stdout(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stdout(), "test/ntest").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test/ntest")`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "test/ntest").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test/ntest")`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "with {}", value).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {}", value)`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {} {}", 2, value)`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "with {value}").unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {value}")`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("macro arg {}", one!())`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "{:w$}", value, w = width).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{:w$}", value, w = width)`
-error: aborting due to 13 previous errors
+error: aborting due to 11 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::explicit_write)]
 #![allow(unused_imports)]
 #![allow(clippy::uninlined_format_args)]
 fn stdout() -> String {
     String::new()
 }
 
---
         eprintln!("test");
         print!("test");
         eprint!("test");
 
         // including newlines
         println!("test\ntest");
         eprintln!("test\ntest");
         let value = 1;
         let value = 1;
         eprintln!("with {}", value);
-        eprintln!("with {} {}", 2, value);
+        writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
         eprintln!("with {value}");
-        eprintln!("macro arg {}", one!());
+        writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
         let width = 2;
         eprintln!("{:w$}", value, w = width);
     // these should not warn, different destination
     {
         use std::fmt::Write;
         let mut s = String::new();
         let mut s = String::new();
         write!(s, "test").unwrap();
         write!(s, "test").unwrap();
         writeln!(s, "test").unwrap();
         writeln!(s, "test").unwrap();
         s.write_fmt(format_args!("test")).unwrap();
         s.write_fmt(format_args!("test")).unwrap();
         write!(stdout(), "test").unwrap();
         write!(stderr(), "test").unwrap();
         writeln!(stdout(), "test").unwrap();
         writeln!(stderr(), "test").unwrap();
         stdout().write_fmt(format_args!("test")).unwrap();
         stderr().write_fmt(format_args!("test")).unwrap();
     }
     // these should not warn, no unwrap
         use std::io::Write;
         use std::io::Write;
         std::io::stdout().write_fmt(format_args!("test")).expect("no stdout");
         std::io::stderr().write_fmt(format_args!("test")).expect("no stderr");
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.fixed
To only update this specific test, also pass `--test-args explicit_write.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `write!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":336,"byte_end":378,"line_start":24,"line_end":24,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-write` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":336,"byte_end":378,"line_start":24,"line_end":24,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":"print!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `write!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:24:9\n   |\nLL |         write!(std::io::stdout(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!(\"test\")`\n   |\n   = note: `-D clippy::explicit-write` implied by `-D warnings`\n\n"}
{"message":"use of `write!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":388,"byte_end":430,"line_start":25,"line_end":25,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":388,"byte_end":430,"line_start":25,"line_end":25,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":"eprint!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `write!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:25:9\n   |\nLL |         write!(std::io::stderr(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!(\"test\")`\n\n"}
{"message":"use of `writeln!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":440,"byte_end":484,"line_start":26,"line_end":26,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":440,"byte_end":484,"line_start":26,"line_end":26,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"println!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:26:9\n   |\nLL |         writeln!(std::io::stdout(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!(\"test\")`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":494,"byte_end":538,"line_start":27,"line_end":27,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":494,"byte_end":538,"line_start":27,"line_end":27,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"eprintln!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:27:9\n   |\nLL |         writeln!(std::io::stderr(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"test\")`\n\n"}
{"message":"use of `stdout().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":548,"byte_end":606,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":548,"byte_end":606,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"print!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stdout().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:28:9\n   |\nLL |         std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!(\"test\")`\n\n"}
{"message":"use of `stderr().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":616,"byte_end":674,"line_start":29,"line_end":29,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":616,"byte_end":674,"line_start":29,"line_end":29,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"eprint!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stderr().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:29:9\n   |\nLL |         std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!(\"test\")`\n\n"}
{"message":"use of `writeln!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":715,"byte_end":765,"line_start":32,"line_end":32,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":715,"byte_end":765,"line_start":32,"line_end":32,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":"println!(\"test\\ntest\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:32:9\n   |\nLL |         writeln!(std::io::stdout(), \"test\\ntest\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!(\"test\\ntest\")`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":775,"byte_end":825,"line_start":33,"line_end":33,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":775,"byte_end":825,"line_start":33,"line_end":33,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":"eprintln!(\"test\\ntest\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:33:9\n   |\nLL |         writeln!(std::io::stderr(), \"test\\ntest\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"test\\ntest\")`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":859,"byte_end":913,"line_start":36,"line_end":36,"column_start":9,"column_end":63,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {}\", value).unwrap();","highlight_start":9,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":859,"byte_end":913,"line_start":36,"line_end":36,"column_start":9,"column_end":63,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {}\", value).unwrap();","highlight_start":9,"highlight_end":63}],"label":null,"suggested_replacement":"eprintln!(\"with {}\", value)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:36:9\n   |\nLL |         writeln!(std::io::stderr(), \"with {}\", value).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"with {}\", value)`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":993,"byte_end":1045,"line_start":38,"line_end":38,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {value}\").unwrap();","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":993,"byte_end":1045,"line_start":38,"line_end":38,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {value}\").unwrap();","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":"eprintln!(\"with {value}\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:38:9\n   |\nLL |         writeln!(std::io::stderr(), \"with {value}\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"with {value}\")`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1148,"byte_end":1211,"line_start":41,"line_end":41,"column_start":9,"column_end":72,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();","highlight_start":9,"highlight_end":72}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1148,"byte_end":1211,"line_start":41,"line_end":41,"column_start":9,"column_end":72,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();","highlight_start":9,"highlight_end":72}],"label":null,"suggested_replacement":"eprintln!(\"{:w$}\", value, w = width)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:41:9\n   |\nLL |         writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{:w$}\", value, w = width)`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: useless use of `format!`
   --> $DIR/format.rs:19:5
    |
 LL |     format!("foo");
    |     ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
    |
    = note: `-D clippy::useless-format` implied by `-D warnings`
 error: useless use of `format!`
   --> $DIR/format.rs:20:5
    |
    |
 LL |     format!("{{}}");
    |     ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{}".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:21:5
    |
    |
 LL |     format!("{{}} abc {{}}");
    |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{} abc {}".to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:22:5
    |
 LL | /     format!(
 LL | /     format!(
 LL | |         r##"foo {{}}
 LL | | " bar"##
 LL | |     );
    |
    |
 help: consider using `.to_string()`
    |
 LL ~     r##"foo {}
 LL ~ " bar"##.to_string();
 
 error: useless use of `format!`
   --> $DIR/format.rs:27:13
    |
    |
 LL |     let _ = format!("");
    |             ^^^^^^^^^^^ help: consider using `String::new()`: `String::new()`
 error: useless use of `format!`
-  --> $DIR/format.rs:29:5
-   |
-LL |     format!("{}", "foo");
-LL |     format!("{}", "foo");
-   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
-error: useless use of `format!`
   --> $DIR/format.rs:37:5
    |
 LL |     format!("{}", arg);
 LL |     format!("{}", arg);
    |     ^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:67:5
    |
 LL |     format!("{}", 42.to_string());
 LL |     format!("{}", 42.to_string());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `42.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:69:5
    |
    |
 LL |     format!("{}", x.display().to_string());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.display().to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:73:18
    |
    |
 LL |     let _ = Some(format!("{}", a + "bar"));
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `a + "bar"`
 error: useless use of `format!`
   --> $DIR/format.rs:77:22
    |
    |
 LL |     let _s: String = format!("{}", &*v.join("/n"));
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `(&*v.join("/n")).to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:83:13
    |
    |
 LL |     let _ = format!("{x}");
    |             ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:85:13
    |
    |
 LL |     let _ = format!("{y}", y = x);
    |             ^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:89:13
    |
    |
 LL |     let _ = format!("{abc}");
    |             ^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `abc.to_string()`
 error: useless use of `format!`
   --> $DIR/format.rs:91:13
    |
    |
 LL |     let _ = format!("{xx}");
    |             ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `xx.to_string()`
-error: aborting due to 15 previous errors
+error: aborting due to 14 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::useless_format)]
     unused_tuple_struct_fields,
     clippy::print_literal,
     clippy::redundant_clone,
     clippy::to_string_in_format_args,
     clippy::to_string_in_format_args,
     clippy::needless_borrow,
     clippy::uninlined_format_args
 )]
 
 struct Foo(pub String);
 
 macro_rules! foo {
     ($($t:tt)*) => (Foo(format!($($t)*)))
 
 fn main() {
     "foo".to_string();
     "{}".to_string();
     "{}".to_string();
     "{} abc {}".to_string();
     r##"foo {}
 " bar"##.to_string();
     let _ = String::new();
 
-    "foo".to_string();
+    format!("{}", "foo");
+    format!("{}", "foo");
     format!("{:?}", "foo"); // Don't warn about `Debug`.
     format!("{:8}", "foo");
     format!("{:width$}", "foo", width = 8);
     format!("foo {}", "bar");
     format!("{} bar", "foo");
     let arg = String::new();
     arg.to_string();
     format!("{:?}", arg); // Don't warn about debug.
     format!("{:8}", arg);
     format!("{:8}", arg);
     format!("{:width$}", arg, width = 8);
     format!("foo {}", arg);
     format!("{} bar", arg);
 
     // We don’t want to warn for non-string args; see issue #697.
     format!("{:?}", 42);
     format!("{:?}", 42);
     format!("{:+}", 42);
     format!("foo {}", 42);
     format!("{} bar", 42);
 
     // We only want to warn about `format!` itself.
     println!("{}", "foo");
     println!("foo {}", "foo");
     println!("{}", 42);
     println!("foo {}", 42);
     println!("foo {}", 42);
 
     // A `format!` inside a macro should not trigger a warning.
     foo!("should not warn");
     // Precision on string means slicing without panicking on size.
     // Precision on string means slicing without panicking on size.
     format!("{:.1}", "foo"); // Could be `"foo"[..1]`
     format!("{:.10}", "foo"); // Could not be `"foo"[..10]`
     format!("{:.prec$}", "foo", prec = 1);
     format!("{:.prec$}", "foo", prec = 10);
     42.to_string();
     42.to_string();
     let x = std::path::PathBuf::from("/bar/foo/qux");
     x.display().to_string();
     // False positive
     // False positive
     let a = "foo".to_string();
     let _ = Some(a + "bar");
     // Wrap it with braces
     // Wrap it with braces
     let v: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
     let _s: String = (&*v.join("\n")).to_string();
 
     format!("prepend {:+}", "s");
     // Issue #8290
     let x = "foo";
     let _ = x.to_string();
     let _ = x.to_string();
     let _ = format!("{x:?}"); // Don't lint on debug
     let _ = x.to_string();
     // Issue #9234
     // Issue #9234
     let abc = "abc";
     let _ = abc.to_string();
     let xx = "xx";
     let _ = xx.to_string();
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id.fixed
To only update this specific test, also pass `--test-args format.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":356,"byte_end":370,"line_start":19,"line_end":19,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-format` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":356,"byte_end":370,"line_start":19,"line_end":19,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:19:5\n   |\nLL |     format!(\"foo\");\n   |     ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"foo\".to_string()`\n   |\n   = note: `-D clippy::useless-format` implied by `-D warnings`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":376,"byte_end":391,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"{{}}\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":376,"byte_end":391,"line_start":20,"line_end":20,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"{{}}\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"\"{}\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:20:5\n   |\nLL |     format!(\"{{}}\");\n   |     ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"{}\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":397,"byte_end":421,"line_start":21,"line_end":21,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    format!(\"{{}} abc {{}}\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":397,"byte_end":421,"line_start":21,"line_end":21,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    format!(\"{{}} abc {{}}\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":"\"{} abc {}\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:21:5\n   |\nLL |     format!(\"{{}} abc {{}}\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `\"{} abc {}\".to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":427,"byte_end":471,"line_start":22,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    format!(","highlight_start":5,"highlight_end":13},{"text":"        r##\"foo {{}}","highlight_start":1,"highlight_end":21},{"text":"\" bar\"##","highlight_start":1,"highlight_end":9},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":427,"byte_end":471,"line_start":22,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    format!(","highlight_start":5,"highlight_end":13},{"text":"        r##\"foo {{}}","highlight_start":1,"highlight_end":21},{"text":"\" bar\"##","highlight_start":1,"highlight_end":9},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"r##\"foo {}\n\" bar\"##.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:22:5\n   |\nLL | /     format!(\nLL | |         r##\"foo {{}}\nLL | | \" bar\"##\nLL | |     );\n   | |_____^\n   |\nhelp: consider using `.to_string()`\n   |\nLL ~     r##\"foo {}\nLL ~ \" bar\"##.to_string();\n   |\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":486,"byte_end":497,"line_start":27,"line_end":27,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = format!(\"\");","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `String::new()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":486,"byte_end":497,"line_start":27,"line_end":27,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = format!(\"\");","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":"String::new()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:27:13\n   |\nLL |     let _ = format!(\"\");\n   |             ^^^^^^^^^^^ help: consider using `String::new()`: `String::new()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":749,"byte_end":767,"line_start":37,"line_end":37,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    format!(\"{}\", arg);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":749,"byte_end":767,"line_start":37,"line_end":37,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    format!(\"{}\", arg);","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:37:5\n   |\nLL |     format!(\"{}\", arg);\n   |     ^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1695,"byte_end":1724,"line_start":67,"line_end":67,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    format!(\"{}\", 42.to_string());","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1695,"byte_end":1724,"line_start":67,"line_end":67,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    format!(\"{}\", 42.to_string());","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":"42.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:67:5\n   |\nLL |     format!(\"{}\", 42.to_string());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `42.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1784,"byte_end":1822,"line_start":69,"line_end":69,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    format!(\"{}\", x.display().to_string());","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1784,"byte_end":1822,"line_start":69,"line_end":69,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    format!(\"{}\", x.display().to_string());","highlight_start":5,"highlight_end":43}],"label":null,"suggested_replacement":"x.display().to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:69:5\n   |\nLL |     format!(\"{}\", x.display().to_string());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.display().to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1895,"byte_end":1919,"line_start":73,"line_end":73,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = Some(format!(\"{}\", a + \"bar\"));","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":1895,"byte_end":1919,"line_start":73,"line_end":73,"column_start":18,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = Some(format!(\"{}\", a + \"bar\"));","highlight_start":18,"highlight_end":42}],"label":null,"suggested_replacement":"a + \"bar\"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:73:18\n   |\nLL |     let _ = Some(format!(\"{}\", a + \"bar\"));\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `a + \"bar\"`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2040,"byte_end":2069,"line_start":77,"line_end":77,"column_start":22,"column_end":51,"is_primary":true,"text":[{"text":"    let _s: String = format!(\"{}\", &*v.join(\"\\n\"));","highlight_start":22,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2040,"byte_end":2069,"line_start":77,"line_end":77,"column_start":22,"column_end":51,"is_primary":true,"text":[{"text":"    let _s: String = format!(\"{}\", &*v.join(\"\\n\"));","highlight_start":22,"highlight_end":51}],"label":null,"suggested_replacement":"(&*v.join(\"\\n\")).to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:77:22\n   |\nLL |     let _s: String = format!(\"{}\", &*v.join(\"\\n\"));\n   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `(&*v.join(\"\\n\")).to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2157,"byte_end":2171,"line_start":83,"line_end":83,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = format!(\"{x}\");","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2157,"byte_end":2171,"line_start":83,"line_end":83,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"    let _ = format!(\"{x}\");","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":"x.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:83:13\n   |\nLL |     let _ = format!(\"{x}\");\n   |             ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2238,"byte_end":2259,"line_start":85,"line_end":85,"column_start":13,"column_end":34,"is_primary":true,"text":[{"text":"    let _ = format!(\"{y}\", y = x);","highlight_start":13,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2238,"byte_end":2259,"line_start":85,"line_end":85,"column_start":13,"column_end":34,"is_primary":true,"text":[{"text":"    let _ = format!(\"{y}\", y = x);","highlight_start":13,"highlight_end":34}],"label":null,"suggested_replacement":"x.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:85:13\n   |\nLL |     let _ = format!(\"{y}\", y = x);\n   |             ^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2314,"byte_end":2330,"line_start":89,"line_end":89,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    let _ = format!(\"{abc}\");","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2314,"byte_end":2330,"line_start":89,"line_end":89,"column_start":13,"column_end":29,"is_primary":true,"text":[{"text":"    let _ = format!(\"{abc}\");","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":"abc.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:89:13\n   |\nLL |     let _ = format!(\"{abc}\");\n   |             ^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `abc.to_string()`\n\n"}
{"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":2363,"byte_end":2378,"line_start":91,"line_end":91,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = format!(\"{xx}\");","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `.to_string()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":2363,"byte_end":2378,"line_start":91,"line_end":91,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = format!(\"{xx}\");","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"xx.to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:91:13\n   |\nLL |     let _ = format!(\"{xx}\");\n   |             ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `xx.to_string()`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: `to_string` applied to a type that implements `Display` in `format!` args
    |
    |
 LL |     let _ = format!("error: something failed at {}", Location::caller().to_string());
    |
    |
    = note: `-D clippy::to-string-in-format-args` implied by `-D warnings`
 
 error: `to_string` applied to a type that implements `Display` in `write!` args
    |
 LL |         Location::caller().to_string()
    |                           ^^^^^^^^^^^^ help: remove this
 
 
 error: `to_string` applied to a type that implements `Display` in `writeln!` args
    |
 LL |         Location::caller().to_string()
    |                           ^^^^^^^^^^^^ help: remove this
 
 
 error: `to_string` applied to a type that implements `Display` in `print!` args
    |
    |
 LL |     print!("error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
    |
 LL |     println!("error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `eprint!` args
    |
    |
 LL |     eprint!("error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `eprintln!` args
    |
    |
 LL |     eprintln!("error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `format_args!` args
    |
    |
 LL |     let _ = format_args!("error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `assert!` args
    |
    |
 LL |     assert!(true, "error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `assert_eq!` args
    |
    |
 LL |     assert_eq!(0, 0, "error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `assert_ne!` args
    |
    |
 LL |     assert_ne!(0, 0, "error: something failed at {}", Location::caller().to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `panic!` args
    |
 LL |     panic!("error: something failed at {}", Location::caller().to_string());
    |                                                               ^^^^^^^^^^^^ help: remove this
 
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
 LL |     println!("{}", X(1).to_string());
 LL |     println!("{}", X(1).to_string());
    |                    ^^^^^^^^^^^^^^^^ help: use this: `*X(1)`
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
    |
 LL |     println!("{}", Y(&X(1)).to_string());
    |                    ^^^^^^^^^^^^^^^^^^^^ help: use this: `***Y(&X(1))`
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
    |
 LL |     println!("{}", Z(1).to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
 LL |     println!("{}", x.to_string());
 LL |     println!("{}", x.to_string());
    |                    ^^^^^^^^^^^^^ help: use this: `**x`
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
 LL |     println!("{}", x_ref.to_string());
 LL |     println!("{}", x_ref.to_string());
    |                    ^^^^^^^^^^^^^^^^^ help: use this: `***x_ref`
 
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", foo = "foo".to_string(), bar = "bar");
-
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", foo = "foo", bar = "bar".to_string());
-
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", bar = "bar".to_string(), foo = "foo");
-
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", bar = "bar", foo = "foo".to_string());
-
-
 error: `to_string` applied to a type that implements `Display` in `print!` args
    |
 LL |     print!("{}", (Location::caller().to_string()));
    |                                     ^^^^^^^^^^^^ help: remove this
 
 
 error: `to_string` applied to a type that implements `Display` in `print!` args
    |
    |
 LL |     print!("{}", ((Location::caller()).to_string()));
 
 
 error: `to_string` applied to a type that implements `Display` in `format!` args
    |
    |
 LL |         let x = format!("{} {}", a, b.to_string());
 
 
 error: `to_string` applied to a type that implements `Display` in `println!` args
    |
    |
 LL |         println!("{}", original[..10].to_string());
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use this: `&original[..10]`
-error: aborting due to 25 previous errors
+error: aborting due to 21 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::to_string_in_format_args)]
 #![allow(
 #![allow(
     clippy::assertions_on_constants,
     clippy::double_parens,
     clippy::eq_op,
     clippy::print_literal,
     clippy::uninlined_format_args
 
 
 use std::io::{stdout, Write};
 use std::panic::Location;
 
 
 struct Somewhere;
 
 impl ToString for Somewhere {
     fn to_string(&self) -> String {
         String::from("somewhere")
 }
 
 
 struct X(u32);
 impl Deref for X {
     type Target = u32;
 
---
To only update this specific test, also pass `--test-args print_literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/print_literal.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_literal.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_literal.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: using `print!()` with a format string that ends in a single newline
    |
    |
 LL |     print!("Hello/n");
    |
    |
    = note: `-D clippy::print-with-newline` implied by `-D warnings`
 help: use `println!` instead
    |
 LL -     print!("Hello/n");
 LL +     println!("Hello");
 
 
 error: using `print!()` with a format string that ends in a single newline
-   |
-   |
-LL |     print!("Hello {}/n", "world");
-   |
-   |
-help: use `println!` instead
-   |
-LL -     print!("Hello {}/n", "world");
-LL +     println!("Hello {}", "world");
-
-
-error: using `print!()` with a format string that ends in a single newline
-   |
-   |
-LL |     print!("Hello {} {}/n", "world", "#2");
-   |
-   |
-help: use `println!` instead
-   |
-LL -     print!("Hello {} {}/n", "world", "#2");
-LL +     println!("Hello {} {}", "world", "#2");
-
-
-error: using `print!()` with a format string that ends in a single newline
-   |
-   |
-LL |     print!("{}/n", 1265);
-   |
-   |
-help: use `println!` instead
-   |
-LL -     print!("{}/n", 1265);
-LL +     println!("{}", 1265);
-
-
-error: using `print!()` with a format string that ends in a single newline
    |
    |
 LL |     print!("/n");
    |
    |
 help: use `println!` instead
    |
 LL -     print!("/n");
 LL +     println!();
 
 
 error: using `print!()` with a format string that ends in a single newline
    |
    |
 LL |     print!("//n"); // should fail
    |
    |
 help: use `println!` instead
    |
 LL -     print!("//n"); // should fail
 LL +     println!("/"); // should fail
 
 
 error: using `print!()` with a format string that ends in a single newline
    |
 LL | /     print!(
 LL | |         "
 LL | | "
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `println!` instead
 LL ~     println!(
 LL ~         
    |
 
 
 error: using `print!()` with a format string that ends in a single newline
    |
 LL | /     print!(
 LL | |         r"
 LL | | "
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `println!` instead
 LL ~     println!(
 LL ~         
    |
 
 
 error: using `print!()` with a format string that ends in a single newline
    |
    |
 LL |     print!("/r/n"); //~ ERROR
    |
    |
 help: use `println!` instead
    |
 LL -     print!("/r/n"); //~ ERROR
 LL +     println!("/r"); //~ ERROR
 
-error: aborting due to 9 previous errors
+error: aborting due to 6 previous errors
 
---
To only update this specific test, also pass `--test-args print_with_newline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/print_with_newline.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_with_newline.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_with_newline.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":202,"byte_end":219,"line_start":8,"line_end":8,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    print!(\"Hello\\n\");","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::print-with-newline` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":202,"byte_end":207,"line_start":8,"line_end":8,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(\"Hello\\n\");","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":215,"byte_end":217,"line_start":8,"line_end":8,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"    print!(\"Hello\\n\");","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:8:5\n   |\nLL |     print!(\"Hello\\n\");\n   |     ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::print-with-newline` implied by `-D warnings`\nhelp: use `println!` instead\n   |\nLL -     print!(\"Hello\\n\");\nLL +     println!(\"Hello\");\n   |\n\n"}
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":330,"byte_end":342,"line_start":12,"line_end":12,"column_start":5,"column_end":17,"is_primary":true,"text":[{"text":"    print!(\"\\n\");","highlight_start":5,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":330,"byte_end":335,"line_start":12,"line_end":12,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(\"\\n\");","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":337,"byte_end":341,"line_start":12,"line_end":12,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    print!(\"\\n\");","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:12:5\n   |\nLL |     print!(\"\\n\");\n   |     ^^^^^^^^^^^^\n   |\nhelp: use `println!` instead\n   |\nLL -     print!(\"\\n\");\nLL +     println!();\n   |\n\n"}
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":820,"byte_end":834,"line_start":31,"line_end":31,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    print!(\"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":820,"byte_end":825,"line_start":31,"line_end":31,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(\"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":830,"byte_end":832,"line_start":31,"line_end":31,"column_start":15,"column_end":17,"is_primary":true,"text":[{"text":"    print!(\"\\\\\\n\"); // should fail","highlight_start":15,"highlight_end":17}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:31:5\n   |\nLL |     print!(\"\\\\\\n\"); // should fail\n   |     ^^^^^^^^^^^^^^\n   |\nhelp: use `println!` instead\n   |\nLL -     print!(\"\\\\\\n\"); // should fail\nLL +     println!(\"\\\\\"); // should fail\n   |\n\n"}
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":966,"byte_end":991,"line_start":38,"line_end":41,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    print!(","highlight_start":5,"highlight_end":12},{"text":"        \"","highlight_start":1,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":966,"byte_end":971,"line_start":38,"line_end":38,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":982,"byte_end":985,"line_start":39,"line_end":40,"column_start":9,"column_end":2,"is_primary":true,"text":[{"text":"        \"","highlight_start":9,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:38:5\n   |\nLL | /     print!(\nLL | |         \"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `println!` instead\n   |\nLL ~     println!(\nLL ~         \n   |\n\n"}
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":997,"byte_end":1023,"line_start":42,"line_end":45,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    print!(","highlight_start":5,"highlight_end":12},{"text":"        r\"","highlight_start":1,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":997,"byte_end":1002,"line_start":42,"line_end":42,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":1013,"byte_end":1017,"line_start":43,"line_end":44,"column_start":9,"column_end":2,"is_primary":true,"text":[{"text":"        r\"","highlight_start":9,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:42:5\n   |\nLL | /     print!(\nLL | |         r\"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `println!` instead\n   |\nLL ~     println!(\nLL ~         \n   |\n\n"}
{"message":"using `print!()` with a format string that ends in a single newline","code":{"code":"clippy::print_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":1107,"byte_end":1122,"line_start":50,"line_end":50,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    print!(\"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `println!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_with_newline.rs","byte_start":1107,"byte_end":1112,"line_start":50,"line_end":50,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    print!(\"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"println","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_with_newline.rs","byte_start":1118,"byte_end":1120,"line_start":50,"line_end":50,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"    print!(\"\\\\r\\n\"); //~ ERROR","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `print!()` with a format string that ends in a single newline\n  --> tests/ui/print_with_newline.rs:50:5\n   |\nLL |     print!(\"\\\\r\\n\"); //~ ERROR\n   |     ^^^^^^^^^^^^^^^\n   |\nhelp: use `println!` instead\n   |\nLL -     print!(\"\\\\r\\n\"); //~ ERROR\nLL +     println!(\"\\\\r\"); //~ ERROR\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:12:15
-   |
-LL |     println!("{:5}.", format_args!(""));
-   |
-   |
-   = note: `-D clippy::unused-format-specs` implied by `-D warnings`
-help: for the width to apply consider using `format!()`
-   |
-LL |     println!("{:5}.", format!(""));
-help: if the current behavior is intentional, remove the format specifiers
-   |
-   |
-LL -     println!("{:5}.", format_args!(""));
-LL +     println!("{}.", format_args!(""));
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:14:15
-   |
-LL |     println!("{:.3}", format_args!("abcde"));
-   |
-   |
-help: for the precision to apply consider using `format!()`
-   |
-LL |     println!("{:.3}", format!("abcde"));
-help: if the current behavior is intentional, remove the format specifiers
-   |
-   |
-LL -     println!("{:.3}", format_args!("abcde"));
-LL +     println!("{}", format_args!("abcde"));
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:16:15
-   |
-LL |     println!("{:5}.", format_args_from_macro!());
-   |
-   |
-help: for the width to apply consider using `format!()`
-  --> $DIR/unused_format_specs_unfixable.rs:16:17
-   |
-LL |     println!("{:5}.", format_args_from_macro!());
-help: if the current behavior is intentional, remove the format specifiers
-   |
-   |
-LL -     println!("{:5}.", format_args_from_macro!());
-LL +     println!("{}.", format_args_from_macro!());
-
-
-error: format specifiers have no effect on `format_args!()`
   --> $DIR/unused_format_specs_unfixable.rs:19:15
    |
 LL |     println!("{args:5}");
    |
    |
 help: for the width to apply consider using `format!()`
   --> $DIR/unused_format_specs_unfixable.rs:19:21
    |
 LL |     println!("{args:5}");
    |                     ^
+   = note: `-D clippy::unused-format-specs` implied by `-D warnings`
 help: if the current behavior is intentional, remove the format specifiers
    |
 LL -     println!("{args:5}");
 LL +     println!("{args}");
 
-error: aborting due to 4 previous errors
+error: aborting due to previous error
 
---
To only update this specific test, also pass `--test-args unused_format_specs_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_format_specs_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"format specifiers have no effect on `format_args!()`","code":{"code":"clippy::unused_format_specs","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_format_specs_unfixable.rs","byte_start":414,"byte_end":422,"line_start":19,"line_end":19,"column_start":15,"column_end":23,"is_primary":true,"text":[{"text":"    println!(\"{args:5}\");","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for the width to apply consider using `format!()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_format_specs_unfixable.rs","byte_start":420,"byte_end":421,"line_start":19,"line_end":19,"column_start":21,"column_end":22,"is_primary":true,"text":[{"text":"    println!(\"{args:5}\");","highlight_start":21,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`-D clippy::unused-format-specs` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"if the current behavior is intentional, remove the format specifiers","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_format_specs_unfixable.rs","byte_start":419,"byte_end":421,"line_start":19,"line_end":19,"column_start":20,"column_end":22,"is_primary":true,"text":[{"text":"    println!(\"{args:5}\");","highlight_start":20,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: format specifiers have no effect on `format_args!()`\n  --> tests/ui/unused_format_specs_unfixable.rs:19:15\n   |\nLL |     println!(\"{args:5}\");\n   |               ^^^^^^^^\n   |\nhelp: for the width to apply consider using `format!()`\n  --> tests/ui/unused_format_specs_unfixable.rs:19:21\n   |\nLL |     println!(\"{args:5}\");\n   |                     ^\n   = note: `-D clippy::unused-format-specs` implied by `-D warnings`\nhelp: if the current behavior is intentional, remove the format specifiers\n   |\nLL -     println!(\"{args:5}\");\nLL +     println!(\"{args}\");\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: empty precision specifier has no effect
   --> $DIR/unused_format_specs.rs:8:17
    |
 LL |     println!("{:.}", 1.0);
    |
    = note: a precision specifier is not required to format floats
    = note: a precision specifier is not required to format floats
    = note: `-D clippy::unused-format-specs` implied by `-D warnings`
 help: remove the `.`
    |
 LL -     println!("{:.}", 1.0);
 LL +     println!("{}", 1.0);
 
 
 error: empty precision specifier has no effect
   --> $DIR/unused_format_specs.rs:9:18
    |
 LL |     println!("{f:.} {f:.?}");
    |
    = note: a precision specifier is not required to format floats
 help: remove the `.`
    |
    |
 LL -     println!("{f:.} {f:.?}");
 LL +     println!("{f} {f:.?}");
 
 
 error: empty precision specifier has no effect
   --> $DIR/unused_format_specs.rs:9:24
    |
 LL |     println!("{f:.} {f:.?}");
    |
    = note: a precision specifier is not required to format floats
 help: remove the `.`
    |
    |
 LL -     println!("{f:.} {f:.?}");
 LL +     println!("{f:.} {f:?}");
 
 
-error: empty precision specifier has no effect
-  --> $DIR/unused_format_specs.rs:11:17
-   |
-LL |     println!("{:.}", 1);
-   |
-help: remove the `.`
-   |
-   |
-LL -     println!("{:.}", 1);
-LL +     println!("{}", 1);
-
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
---
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::unused_format_specs)]
 
 fn main() {
     let f = 1.0f64;
     println!("{}", 1.0);
     println!("{}", 1.0);
     println!("{f} {f:?}");
-    println!("{}", 1);
-    println!("{}", 1);
+    println!("{:.}", 1);
 
 fn should_not_lint() {
     let f = 1.0f64;
     let f = 1.0f64;
     println!("{:.1}", 1.0);
     println!("{f:.w$} {f:.*?}", 3, w = 2);
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs.stage-id.fixed
To only update this specific test, also pass `--test-args unused_format_specs.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_format_specs.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"empty precision specifier has no effect","code":{"code":"clippy::unused_format_specs","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":121,"byte_end":122,"line_start":8,"line_end":8,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"    println!(\"{:.}\", 1.0);","highlight_start":17,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a precision specifier is not required to format floats","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::unused-format-specs` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the `.`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":120,"byte_end":122,"line_start":8,"line_end":8,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"    println!(\"{:.}\", 1.0);","highlight_start":16,"highlight_end":18}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: empty precision specifier has no effect\n  --> tests/ui/unused_format_specs.rs:8:17\n   |\nLL |     println!(\"{:.}\", 1.0);\n   |                 ^\n   |\n   = note: a precision specifier is not required to format floats\n   = note: `-D clippy::unused-format-specs` implied by `-D warnings`\nhelp: remove the `.`\n   |\nLL -     println!(\"{:.}\", 1.0);\nLL +     println!(\"{}\", 1.0);\n   |\n\n"}
{"message":"empty precision specifier has no effect","code":{"code":"clippy::unused_format_specs","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":149,"byte_end":150,"line_start":9,"line_end":9,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    println!(\"{f:.} {f:.?}\");","highlight_start":18,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a precision specifier is not required to format floats","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the `.`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":148,"byte_end":150,"line_start":9,"line_end":9,"column_start":17,"column_end":19,"is_primary":true,"text":[{"text":"    println!(\"{f:.} {f:.?}\");","highlight_start":17,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: empty precision specifier has no effect\n  --> tests/ui/unused_format_specs.rs:9:18\n   |\nLL |     println!(\"{f:.} {f:.?}\");\n   |                  ^\n   |\n   = note: a precision specifier is not required to format floats\nhelp: remove the `.`\n   |\nLL -     println!(\"{f:.} {f:.?}\");\nLL +     println!(\"{f} {f:.?}\");\n   |\n\n"}
{"message":"empty precision specifier has no effect","code":{"code":"clippy::unused_format_specs","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":155,"byte_end":156,"line_start":9,"line_end":9,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"    println!(\"{f:.} {f:.?}\");","highlight_start":24,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"a precision specifier is not required to format floats","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the `.`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_format_specs.rs","byte_start":155,"byte_end":156,"line_start":9,"line_end":9,"column_start":24,"column_end":25,"is_primary":true,"text":[{"text":"    println!(\"{f:.} {f:.?}\");","highlight_start":24,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: empty precision specifier has no effect\n  --> tests/ui/unused_format_specs.rs:9:24\n   |\nLL |     println!(\"{f:.} {f:.?}\");\n   |                        ^\n   |\n   = note: a precision specifier is not required to format floats\nhelp: remove the `.`\n   |\nLL -     println!(\"{f:.} {f:.?}\");\nLL +     println!(\"{f:.} {f:?}\");\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:9:23
-   |
-LL |     writeln!(v, "{}", "{hello}");
-   |
-   |
-   = note: `-D clippy::write-literal` implied by `-D warnings`
-   |
-   |
-LL -     writeln!(v, "{}", "{hello}");
-LL +     writeln!(v, "{{hello}}");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:10:24
-   |
-   |
-LL |     writeln!(v, r"{}", r"{hello}");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r"{}", r"{hello}");
-LL +     writeln!(v, r"{{hello}}");
-
-error: literal with an empty format string
   --> $DIR/write_literal_2.rs:11:23
    |
    |
 LL |     writeln!(v, "{}", '/'');
    |
    |
+   = note: `-D clippy::write-literal` implied by `-D warnings`
    |
    |
 LL -     writeln!(v, "{}", '/'');
 LL +     writeln!(v, "'");
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:12:23
    |
    |
 LL |     writeln!(v, "{}", '"');
    |
 help: try this
    |
    |
 LL -     writeln!(v, "{}", '"');
 LL +     writeln!(v, "/"");
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:13:24
    |
    |
 LL |     writeln!(v, r"{}", '"');
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:14:24
    |
    |
 LL |     writeln!(v, r"{}", '/'');
    |
 help: try this
    |
    |
 LL -     writeln!(v, r"{}", '/'');
 LL +     writeln!(v, r"'");
 
 error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:18:9
-   |
-   |
-LL | /         "hello /
-LL | |         world!"
-   | |_______________^
-   |
-help: try this
-   |
-LL ~         "some hello /
-LL ~         world!"
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:9
-   |
-   |
-LL |         "1", "2", "3",
-   |         ^^^
-   |
-help: try this
-   |
-LL ~         "some 1/
-LL ~         {} / {}", "2", "3",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:14
-   |
-   |
-LL |         "1", "2", "3",
-   |              ^^^
-   |
-help: try this
-   |
-LL ~         2 / {}",
-LL ~         "1", "3",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:19
-   |
-   |
-LL |         "1", "2", "3",
-   |                   ^^^
-   |
-help: try this
-   |
-LL ~         {} / 3",
-LL ~         "1", "2",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:27:23
-   |
-   |
-LL |     writeln!(v, "{}", "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", "/");
-LL +     writeln!(v, "/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:28:24
-   |
-   |
-LL |     writeln!(v, r"{}", "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r"{}", "/");
-LL +     writeln!(v, r"/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:29:26
-   |
-   |
-LL |     writeln!(v, r#"{}"#, "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r#"{}"#, "/");
-LL +     writeln!(v, r#"/"#);
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:30:23
-   |
-   |
-LL |     writeln!(v, "{}", r"/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", r"/");
-LL +     writeln!(v, "/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:31:23
-   |
-   |
-LL |     writeln!(v, "{}", "/r");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", "/r");
-LL +     writeln!(v, "/r");
-
-error: literal with an empty format string
   --> $DIR/write_literal_2.rs:32:28
    |
    |
 LL |     writeln!(v, r#"{}{}"#, '#', '"'); // hard mode
 
 error: literal with an empty format string
   --> $DIR/write_literal_2.rs:32:33
    |
    |
 LL |     writeln!(v, r#"{}{}"#, '#', '"'); // hard mode
 
-error: aborting due to 17 previous errors
+error: aborting due to 6 previous errors
 
---
To only update this specific test, also pass `--test-args write_literal_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_literal_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":214,"byte_end":218,"line_start":11,"line_end":11,"column_start":23,"column_end":27,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\\'');","highlight_start":23,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::write-literal` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":209,"byte_end":211,"line_start":11,"line_end":11,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\\'');","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":"'","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":212,"byte_end":218,"line_start":11,"line_end":11,"column_start":21,"column_end":27,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\\'');","highlight_start":21,"highlight_end":27}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:11:23\n   |\nLL |     writeln!(v, \"{}\", '\\'');\n   |                       ^^^^\n   |\n   = note: `-D clippy::write-literal` implied by `-D warnings`\nhelp: try this\n   |\nLL -     writeln!(v, \"{}\", '\\'');\nLL +     writeln!(v, \"'\");\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":243,"byte_end":246,"line_start":12,"line_end":12,"column_start":23,"column_end":26,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\"');","highlight_start":23,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":238,"byte_end":240,"line_start":12,"line_end":12,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\"');","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":"\\\"","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":241,"byte_end":246,"line_start":12,"line_end":12,"column_start":21,"column_end":26,"is_primary":true,"text":[{"text":"    writeln!(v, \"{}\", '\"');","highlight_start":21,"highlight_end":26}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:12:23\n   |\nLL |     writeln!(v, \"{}\", '\"');\n   |                       ^^^\n   |\nhelp: try this\n   |\nLL -     writeln!(v, \"{}\", '\"');\nLL +     writeln!(v, \"\\\"\");\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":272,"byte_end":275,"line_start":13,"line_end":13,"column_start":24,"column_end":27,"is_primary":true,"text":[{"text":"    writeln!(v, r\"{}\", '\"');","highlight_start":24,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:13:24\n   |\nLL |     writeln!(v, r\"{}\", '\"');\n   |                        ^^^\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":301,"byte_end":305,"line_start":14,"line_end":14,"column_start":24,"column_end":28,"is_primary":true,"text":[{"text":"    writeln!(v, r\"{}\", '\\'');","highlight_start":24,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":296,"byte_end":298,"line_start":14,"line_end":14,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"    writeln!(v, r\"{}\", '\\'');","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":"'","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal_2.rs","byte_start":299,"byte_end":305,"line_start":14,"line_end":14,"column_start":22,"column_end":28,"is_primary":true,"text":[{"text":"    writeln!(v, r\"{}\", '\\'');","highlight_start":22,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:14:24\n   |\nLL |     writeln!(v, r\"{}\", '\\'');\n   |                        ^^^^\n   |\nhelp: try this\n   |\nLL -     writeln!(v, r\"{}\", '\\'');\nLL +     writeln!(v, r\"'\");\n   |\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":660,"byte_end":663,"line_start":32,"line_end":32,"column_start":28,"column_end":31,"is_primary":true,"text":[{"text":"    writeln!(v, r#\"{}{}\"#, '#', '\"'); // hard mode","highlight_start":28,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:32:28\n   |\nLL |     writeln!(v, r#\"{}{}\"#, '#', '\"'); // hard mode\n   |                            ^^^\n\n"}
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal_2.rs","byte_start":665,"byte_end":668,"line_start":32,"line_end":32,"column_start":33,"column_end":36,"is_primary":true,"text":[{"text":"    writeln!(v, r#\"{}{}\"#, '#', '\"'); // hard mode","highlight_start":33,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal_2.rs:32:33\n   |\nLL |     writeln!(v, r#\"{}{}\"#, '#', '\"'); // hard mode\n   |                                 ^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: literal with an empty format string
-  --> $DIR/write_literal.rs:31:27
-   |
-LL |     write!(v, "Hello {}", "world");
-   |
-   |
-   = note: `-D clippy::write-literal` implied by `-D warnings`
-   |
-   |
-LL -     write!(v, "Hello {}", "world");
-LL +     write!(v, "Hello world");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:32:39
-   |
-   |
-LL |     writeln!(v, "Hello {} {}", world, "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "Hello {} {}", world, "world");
-LL +     writeln!(v, "Hello {} world", world);
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:33:29
-   |
-   |
-LL |     writeln!(v, "Hello {}", "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "Hello {}", "world");
-LL +     writeln!(v, "Hello world");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:34:29
-   |
-   |
-LL |     writeln!(v, "{} {:.4}", "a literal", 5);
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{} {:.4}", "a literal", 5);
-LL +     writeln!(v, "a literal {:.4}", 5);
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:39:28
-   |
-   |
-LL |     writeln!(v, "{0} {1}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{0} {1}", "hello", "world");
-LL +     writeln!(v, "hello {1}", "world");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:39:37
-   |
-   |
-LL |     writeln!(v, "{0} {1}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{0} {1}", "hello", "world");
-LL +     writeln!(v, "{0} world", "hello");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:40:37
-   |
-   |
-LL |     writeln!(v, "{1} {0}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{1} {0}", "hello", "world");
-LL +     writeln!(v, "world {0}", "hello");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:40:28
-   |
-   |
-LL |     writeln!(v, "{1} {0}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{1} {0}", "hello", "world");
-LL +     writeln!(v, "{1} hello", "world");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:43:38
-   |
-   |
-LL |     writeln!(v, "{foo} {bar}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{foo} {bar}", foo = "hello", bar = "world");
-LL +     writeln!(v, "hello {bar}", bar = "world");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:43:53
-   |
-   |
-LL |     writeln!(v, "{foo} {bar}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{foo} {bar}", foo = "hello", bar = "world");
-LL +     writeln!(v, "{foo} world", foo = "hello");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:44:53
-   |
-   |
-LL |     writeln!(v, "{bar} {foo}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{bar} {foo}", foo = "hello", bar = "world");
-LL +     writeln!(v, "world {foo}", foo = "hello");
-
-error: literal with an empty format string
-  --> $DIR/write_literal.rs:44:38
-   |
-   |
-LL |     writeln!(v, "{bar} {foo}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{bar} {foo}", foo = "hello", bar = "world");
-LL +     writeln!(v, "{bar} hello", bar = "world");
-
-error: aborting due to 12 previous errors
-
-
---
To only update this specific test, also pass `--test-args write_literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_literal.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: using `write!()` with a format string that ends in a single newline
    |
    |
 LL |     write!(v, "Hello/n");
    |
    |
    = note: `-D clippy::write-with-newline` implied by `-D warnings`
 help: use `writeln!` instead
    |
 LL -     write!(v, "Hello/n");
 LL +     writeln!(v, "Hello");
 
 
 error: using `write!()` with a format string that ends in a single newline
-   |
-   |
-LL |     write!(v, "Hello {}/n", "world");
-   |
-   |
-help: use `writeln!` instead
-   |
-LL -     write!(v, "Hello {}/n", "world");
-LL +     writeln!(v, "Hello {}", "world");
-
-
-error: using `write!()` with a format string that ends in a single newline
-   |
-   |
-LL |     write!(v, "Hello {} {}/n", "world", "#2");
-   |
-   |
-help: use `writeln!` instead
-   |
-LL -     write!(v, "Hello {} {}/n", "world", "#2");
-LL +     writeln!(v, "Hello {} {}", "world", "#2");
-
-
-error: using `write!()` with a format string that ends in a single newline
-   |
-   |
-LL |     write!(v, "{}/n", 1265);
-   |
-   |
-help: use `writeln!` instead
-   |
-LL -     write!(v, "{}/n", 1265);
-LL +     writeln!(v, "{}", 1265);
-
-
-error: using `write!()` with a format string that ends in a single newline
    |
    |
 LL |     write!(v, "/n");
    |
    |
 help: use `writeln!` instead
    |
 LL -     write!(v, "/n");
 LL +     writeln!(v);
 
 
 error: using `write!()` with a format string that ends in a single newline
    |
    |
 LL |     write!(v, "//n"); // should fail
    |
    |
 help: use `writeln!` instead
    |
 LL -     write!(v, "//n"); // should fail
 LL +     writeln!(v, "/"); // should fail
 
 
 error: using `write!()` with a format string that ends in a single newline
    |
 LL | /     write!(
 LL | |         v,
 LL | |         "
 LL | |         "
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `writeln!` instead
 LL ~     writeln!(
 LL ~         v
    |
 
 
 error: using `write!()` with a format string that ends in a single newline
    |
 LL | /     write!(
 LL | |         v,
 LL | |         r"
 LL | |         r"
 LL | | "
 LL | |     );
    | |_____^
    |
 help: use `writeln!` instead
 LL ~     writeln!(
 LL ~         v
    |
 
 
 error: using `write!()` with a format string that ends in a single newline
    |
    |
 LL |     write!(v, "/r/n"); //~ ERROR
    |
    |
 help: use `writeln!` instead
    |
 LL -     write!(v, "/r/n"); //~ ERROR
 LL +     writeln!(v, "/r"); //~ ERROR
 
-error: aborting due to 9 previous errors
+error: aborting due to 6 previous errors
 
---
To only update this specific test, also pass `--test-args write_with_newline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_with_newline.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_with_newline.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-329149454788e573.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-b365f58a30eccc6d.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-3d962e7e23a10c27.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_with_newline.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":277,"byte_end":297,"line_start":13,"line_end":13,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    write!(v, \"Hello\\n\");","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::write-with-newline` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":277,"byte_end":282,"line_start":13,"line_end":13,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(v, \"Hello\\n\");","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":293,"byte_end":295,"line_start":13,"line_end":13,"column_start":21,"column_end":23,"is_primary":true,"text":[{"text":"    write!(v, \"Hello\\n\");","highlight_start":21,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:13:5\n   |\nLL |     write!(v, \"Hello\\n\");\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::write-with-newline` implied by `-D warnings`\nhelp: use `writeln!` instead\n   |\nLL -     write!(v, \"Hello\\n\");\nLL +     writeln!(v, \"Hello\");\n   |\n\n"}
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":417,"byte_end":432,"line_start":17,"line_end":17,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    write!(v, \"\\n\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":417,"byte_end":422,"line_start":17,"line_end":17,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(v, \"\\n\");","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":425,"byte_end":431,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    write!(v, \"\\n\");","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:17:5\n   |\nLL |     write!(v, \"\\n\");\n   |     ^^^^^^^^^^^^^^^\n   |\nhelp: use `writeln!` instead\n   |\nLL -     write!(v, \"\\n\");\nLL +     writeln!(v);\n   |\n\n"}
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":954,"byte_end":971,"line_start":36,"line_end":36,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":954,"byte_end":959,"line_start":36,"line_end":36,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\\\n\"); // should fail","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":967,"byte_end":969,"line_start":36,"line_end":36,"column_start":18,"column_end":20,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\\\n\"); // should fail","highlight_start":18,"highlight_end":20}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:36:5\n   |\nLL |     write!(v, \"\\\\\\n\"); // should fail\n   |     ^^^^^^^^^^^^^^^^^\n   |\nhelp: use `writeln!` instead\n   |\nLL -     write!(v, \"\\\\\\n\"); // should fail\nLL +     writeln!(v, \"\\\\\"); // should fail\n   |\n\n"}
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1109,"byte_end":1145,"line_start":43,"line_end":47,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    write!(","highlight_start":5,"highlight_end":12},{"text":"        v,","highlight_start":1,"highlight_end":11},{"text":"        \"","highlight_start":1,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1109,"byte_end":1114,"line_start":43,"line_end":43,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":1126,"byte_end":1139,"line_start":44,"line_end":46,"column_start":10,"column_end":2,"is_primary":true,"text":[{"text":"        v,","highlight_start":10,"highlight_end":11},{"text":"        \"","highlight_start":1,"highlight_end":10},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:43:5\n   |\nLL | /     write!(\nLL | |         v,\nLL | |         \"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `writeln!` instead\n   |\nLL ~     writeln!(\nLL ~         v\n   |\n\n"}
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1151,"byte_end":1188,"line_start":48,"line_end":52,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    write!(","highlight_start":5,"highlight_end":12},{"text":"        v,","highlight_start":1,"highlight_end":11},{"text":"        r\"","highlight_start":1,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1151,"byte_end":1156,"line_start":48,"line_end":48,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":1168,"byte_end":1182,"line_start":49,"line_end":51,"column_start":10,"column_end":2,"is_primary":true,"text":[{"text":"        v,","highlight_start":10,"highlight_end":11},{"text":"        r\"","highlight_start":1,"highlight_end":11},{"text":"\"","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:48:5\n   |\nLL | /     write!(\nLL | |         v,\nLL | |         r\"\nLL | | \"\nLL | |     );\n   | |_____^\n   |\nhelp: use `writeln!` instead\n   |\nLL ~     writeln!(\nLL ~         v\n   |\n\n"}
{"message":"using `write!()` with a format string that ends in a single newline","code":{"code":"clippy::write_with_newline","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1278,"byte_end":1296,"line_start":57,"line_end":57,"column_start":5,"column_end":23,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `writeln!` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_with_newline.rs","byte_start":1278,"byte_end":1283,"line_start":57,"line_end":57,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\r\\n\"); //~ ERROR","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"writeln","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_with_newline.rs","byte_start":1292,"byte_end":1294,"line_start":57,"line_end":57,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"    write!(v, \"\\\\r\\n\"); //~ ERROR","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: using `write!()` with a format string that ends in a single newline\n  --> tests/ui/write_with_newline.rs:57:5\n   |\nLL |     write!(v, \"\\\\r\\n\"); //~ ERROR\n   |     ^^^^^^^^^^^^^^^^^^\n   |\nhelp: use `writeln!` instead\n   |\nLL -     write!(v, \"\\\\r\\n\"); //~ ERROR\nLL +     writeln!(v, \"\\\\r\"); //~ ERROR\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
