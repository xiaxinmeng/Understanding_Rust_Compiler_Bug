plain

---- compile_test stdout ----
diff of stderr:

 error: all if blocks contain the same code at the start
-   |
-LL | /     if true {
-LL | |         println!("Hello World!");
-   | |_________________________________^
-   | |_________________________________^
-   |
-note: the lint level is defined here
-  --> $DIR/shared_at_top.rs:1:9
-   |
-LL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]
-help: consider moving these statements before the if
-   |
-LL ~     println!("Hello World!");
-LL +     if true {
-LL +     if true {
-   |
-
-error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 0 {
 LL | |         let y = 9;
 LL | |         println!("The value y was set to: `{}`", y);
 LL | |         let _z = y;
    |
    |
    = warning: some moved values might need to be renamed to avoid wrong references
+  --> $DIR/shared_at_top.rs:1:9
+   |
+   |
+LL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]
 help: consider moving these statements before the if
    |
    |
 LL ~     let y = 9;
 LL +     println!("The value y was set to: `{}`", y);
 LL +     let _z = y;
 LL +     if x == 0 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     let _ = if x == 7 {
 LL | |         let y = 16;
    |
 help: consider moving these statements before the if
    |
    |
 LL ~     let y = 16;
 LL +     let _ = if x == 7 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 10 {
 LL | |         let used_value_name = "Different type";
 LL | |         println!("Str: {}", used_value_name);
    |
    |
    = warning: some moved values might need to be renamed to avoid wrong references
    |
    |
 LL ~     let used_value_name = "Different type";
 LL +     println!("Str: {}", used_value_name);
 LL +     if x == 10 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 11 {
 LL | |         let can_be_overridden = "Move me";
-LL | |         println!("I'm also moveable");
+   | |__________________________________________^
    |
    |
    = warning: some moved values might need to be renamed to avoid wrong references
    |
    |
 LL ~     let can_be_overridden = "Move me";
-LL +     println!("I'm also moveable");
 LL +     if x == 11 {
 
 
-error: all if blocks contain the same code at the start
-   |
-   |
-LL | /     if x == 2020 {
-LL | |         println!("This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.");
-LL | |         println!("Because `IF_SAME_THEN_ELSE` is allowed here");
-   |
-help: consider moving these statements before the if
-   |
-   |
-LL ~     println!("This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.");
-LL +     println!("Because `IF_SAME_THEN_ELSE` is allowed here");
-LL +     if x == 2020 {
-
-error: this `if` has identical blocks
-  --> $DIR/shared_at_top.rs:97:18
-   |
-   |
-LL |       if x == 2019 {
-   |  __________________^
-LL | |         println!("This should trigger `IS_SAME_THAN_ELSE` as usual");
-LL | |     } else {
-   |
-note: same as this
-  --> $DIR/shared_at_top.rs:99:12
-   |
-   |
-LL |       } else {
-   |  ____________^
-LL | |         println!("This should trigger `IS_SAME_THAN_ELSE` as usual");
-LL | |     }
-note: the lint level is defined here
-  --> $DIR/shared_at_top.rs:1:40
-   |
-   |
-LL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]
-
-error: aborting due to 7 previous errors
+error: aborting due to 4 previous errors
 
---
To only update this specific test, also pass `--test-args branches_sharing_code/shared_at_top.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/branches_sharing_code/shared_at_top.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/shared_at_top.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-62999cf098a4f169.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b83448bd4610d9d2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-8bc4c00ed7c71629.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7f0c26842d997632.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/shared_at_top.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":489,"byte_end":592,"line_start":20,"line_end":23,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let y = 9;","highlight_start":1,"highlight_end":19},{"text":"        println!(\"The value y was set to: `{}`\", y);","highlight_start":1,"highlight_end":53},{"text":"        let _z = y;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":8,"byte_end":37,"line_start":1,"line_end":1,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider moving these statements before the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":489,"byte_end":592,"line_start":20,"line_end":23,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let y = 9;","highlight_start":1,"highlight_end":19},{"text":"        println!(\"The value y was set to: `{}`\", y);","highlight_start":1,"highlight_end":53},{"text":"        let _z = y;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"let y = 9;\n    println!(\"The value y was set to: `{}`\", y);\n    let _z = y;\n    if x == 0 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:20:5\n   |\nLL | /     if x == 0 {\nLL | |         let y = 9;\nLL | |         println!(\"The value y was set to: `{}`\", y);\nLL | |         let _z = y;\n   | |___________________^\n   |\n   = warning: some moved values might need to be renamed to avoid wrong references\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:1:9\n   |\nLL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nhelp: consider moving these statements before the if\n   |\nLL ~     let y = 9;\nLL +     println!(\"The value y was set to: `{}`\", y);\nLL +     let _z = y;\nLL +     if x == 0 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1054,"byte_end":1093,"line_start":41,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = if x == 7 {","highlight_start":5,"highlight_end":24},{"text":"        let y = 16;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving these statements before the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1054,"byte_end":1093,"line_start":41,"line_end":42,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = if x == 7 {","highlight_start":5,"highlight_end":24},{"text":"        let y = 16;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"let y = 16;\n    let _ = if x == 7 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:41:5\n   |\nLL | /     let _ = if x == 7 {\nLL | |         let y = 16;\n   | |___________________^\n   |\nhelp: consider moving these statements before the if\n   |\nLL ~     let y = 16;\nLL +     let _ = if x == 7 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1523,"byte_end":1629,"line_start":59,"line_end":61,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    if x == 10 {","highlight_start":5,"highlight_end":17},{"text":"        let used_value_name = \"Different type\";","highlight_start":1,"highlight_end":48},{"text":"        println!(\"Str: {}\", used_value_name);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements before the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1523,"byte_end":1629,"line_start":59,"line_end":61,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    if x == 10 {","highlight_start":5,"highlight_end":17},{"text":"        let used_value_name = \"Different type\";","highlight_start":1,"highlight_end":48},{"text":"        println!(\"Str: {}\", used_value_name);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":"let used_value_name = \"Different type\";\n    println!(\"Str: {}\", used_value_name);\n    if x == 10 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:59:5\n   |\nLL | /     if x == 10 {\nLL | |         let used_value_name = \"Different type\";\nLL | |         println!(\"Str: {}\", used_value_name);\n   | |_____________________________________________^\n   |\n   = warning: some moved values might need to be renamed to avoid wrong references\nhelp: consider moving these statements before the if\n   |\nLL ~     let used_value_name = \"Different type\";\nLL +     println!(\"Str: {}\", used_value_name);\nLL +     if x == 10 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1957,"byte_end":2012,"line_start":73,"line_end":74,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    if x == 11 {","highlight_start":5,"highlight_end":17},{"text":"        let can_be_overridden = \"Move me\";","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements before the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1957,"byte_end":2012,"line_start":73,"line_end":74,"column_start":5,"column_end":43,"is_primary":true,"text":[{"text":"    if x == 11 {","highlight_start":5,"highlight_end":17},{"text":"        let can_be_overridden = \"Move me\";","highlight_start":1,"highlight_end":43}],"label":null,"suggested_replacement":"let can_be_overridden = \"Move me\";\n    if x == 11 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:73:5\n   |\nLL | /     if x == 11 {\nLL | |         let can_be_overridden = \"Move me\";\n   | |__________________________________________^\n   |\n   = warning: some moved values might need to be renamed to avoid wrong references\nhelp: consider moving these statements before the if\n   |\nLL ~     let can_be_overridden = \"Move me\";\nLL +     if x == 11 {\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: all if blocks contain the same code at the end
-   |
-LL | /         let result = false;
-LL | /         let result = false;
-LL | |         println!("Block end!");
-LL | |         result
-LL | |     };
-   |
-   = note: the end suggestion probably needs some adjustments to use the expression result correctly
-note: the lint level is defined here
-  --> $DIR/shared_at_bottom.rs:1:36
-  --> $DIR/shared_at_bottom.rs:1:36
-   |
-LL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]
-help: consider moving these statements after the if
-   |
-LL ~     }
-LL +     let result = false;
-LL +     let result = false;
-LL +     println!("Block end!");
-LL ~     result;
-
-
-error: all if blocks contain the same code at the end
-   |
-   |
-LL | /         println!("Same end of block");
-LL | |     }
-   |
-help: consider moving these statements after the if
-   |
-LL ~     }
-LL ~     }
-LL +     println!("Same end of block");
-
-
-error: all if blocks contain the same code at the end
    |
 LL | /         println!(
 LL | /         println!(
 LL | |             "I'm moveable because I know: `outer_scope_value`: '{}'",
 LL | |             outer_scope_value
 LL | |         );
 LL | |     }
    |
+note: the lint level is defined here
+  --> $DIR/shared_at_bottom.rs:1:36
+   |
+   |
+LL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]
 help: consider moving these statements after the if
    |
 LL ~     }
 LL +     println!(
 LL +     println!(
 LL +         "I'm moveable because I know: `outer_scope_value`: '{}'",
 LL +         outer_scope_value
 LL +     );
 
 
 error: all if blocks contain the same code at the end
-   |
-LL | /             println!("Hello World");
-LL | |         }
-   | |_________^
-   | |_________^
-   |
-help: consider moving these statements after the if
-   |
-LL ~         }
-LL +         println!("Hello World");
-   |
-
-error: all if blocks contain the same code at the end
    |
    |
 LL | /         let later_used_value = "A string value";
 LL | |         println!("{}", later_used_value);
 LL | |         // I'm expecting a note about this
 LL | |     }
    |
    |
    = warning: some moved values might need to be renamed to avoid wrong references
    |
 LL ~     }
 LL ~     }
 LL +     let later_used_value = "A string value";
 LL +     println!("{}", later_used_value);
 
 
 error: all if blocks contain the same code at the end
    |
    |
 LL | /         let simple_examples = "I now identify as a &str :)";
 LL | |         println!("This is the new simple_example: {}", simple_examples);
 LL | |     }
    |
    |
    = warning: some moved values might need to be renamed to avoid wrong references
    |
 LL ~     }
 LL ~     }
 LL +     let simple_examples = "I now identify as a &str :)";
 LL +     println!("This is the new simple_example: {}", simple_examples);
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /         x << 2
 LL | |     };
    | |_____^
    | |_____^
    |
    = note: the end suggestion probably needs some adjustments to use the expression result correctly
 help: consider moving these statements after the if
    |
 LL ~     }
 LL ~     x << 2;
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /         x * 4
 LL | |     }
    | |_____^
---
 LL ~     }
 LL +     x * 4
    |
 
 error: all if blocks contain the same code at the end
    |
    |
 LL |     if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }
    |
 help: consider moving these statements after the if
    |
    |
 LL ~     if x == 17 { b = 1; a = 0x99; } else { }
 LL +     a = 0x99;
 
-error: aborting due to 9 previous errors
+error: aborting due to 6 previous errors
 
---
To only update this specific test, also pass `--test-args branches_sharing_code/shared_at_bottom.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/branches_sharing_code/shared_at_bottom.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/shared_at_bottom.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-62999cf098a4f169.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b83448bd4610d9d2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-8bc4c00ed7c71629.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7f0c26842d997632.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/shared_at_bottom.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1847,"byte_end":1977,"line_start":66,"line_end":70,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(","highlight_start":5,"highlight_end":18},{"text":"            \"I'm moveable because I know: `outer_scope_value`: '{}'\",","highlight_start":1,"highlight_end":70},{"text":"            outer_scope_value","highlight_start":1,"highlight_end":30},{"text":"        );","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":35,"byte_end":64,"line_start":1,"line_end":1,"column_start":36,"column_end":65,"is_primary":true,"text":[{"text":"#![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]","highlight_start":36,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1847,"byte_end":1977,"line_start":66,"line_end":70,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(","highlight_start":5,"highlight_end":18},{"text":"            \"I'm moveable because I know: `outer_scope_value`: '{}'\",","highlight_start":1,"highlight_end":70},{"text":"            outer_scope_value","highlight_start":1,"highlight_end":30},{"text":"        );","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    println!(\n        \"I'm moveable because I know: `outer_scope_value`: '{}'\",\n        outer_scope_value\n    );","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:66:5\n   |\nLL | /         println!(\nLL | |             \"I'm moveable because I know: `outer_scope_value`: '{}'\",\nLL | |             outer_scope_value\nLL | |         );\nLL | |     }\n   | |_____^\n   |\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:1:36\n   |\nLL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]\n   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL +     println!(\nLL +         \"I'm moveable because I know: `outer_scope_value`: '{}'\",\nLL +         outer_scope_value\nLL +     );\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2510,"byte_end":2645,"line_start":94,"line_end":97,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let later_used_value = \"A string value\";","highlight_start":5,"highlight_end":49},{"text":"        println!(\"{}\", later_used_value);","highlight_start":1,"highlight_end":42},{"text":"        // I'm expecting a note about this","highlight_start":1,"highlight_end":43},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2510,"byte_end":2645,"line_start":94,"line_end":97,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let later_used_value = \"A string value\";","highlight_start":5,"highlight_end":49},{"text":"        println!(\"{}\", later_used_value);","highlight_start":1,"highlight_end":42},{"text":"        // I'm expecting a note about this","highlight_start":1,"highlight_end":43},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let later_used_value = \"A string value\";\n    println!(\"{}\", later_used_value);","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:94:5\n   |\nLL | /         let later_used_value = \"A string value\";\nLL | |         println!(\"{}\", later_used_value);\nLL | |         // I'm expecting a note about this\nLL | |     }\n   | |_____^\n   |\n   = warning: some moved values might need to be renamed to avoid wrong references\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL +     let later_used_value = \"A string value\";\nLL +     println!(\"{}\", later_used_value);\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2923,"byte_end":3058,"line_start":107,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let simple_examples = \"I now identify as a &str :)\";","highlight_start":5,"highlight_end":61},{"text":"        println!(\"This is the new simple_example: {}\", simple_examples);","highlight_start":1,"highlight_end":73},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2923,"byte_end":3058,"line_start":107,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let simple_examples = \"I now identify as a &str :)\";","highlight_start":5,"highlight_end":61},{"text":"        println!(\"This is the new simple_example: {}\", simple_examples);","highlight_start":1,"highlight_end":73},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let simple_examples = \"I now identify as a &str :)\";\n    println!(\"This is the new simple_example: {}\", simple_examples);","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:107:5\n   |\nLL | /         let simple_examples = \"I now identify as a &str :)\";\nLL | |         println!(\"This is the new simple_example: {}\", simple_examples);\nLL | |     }\n   | |_____^\n   |\n   = warning: some moved values might need to be renamed to avoid wrong references\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL +     let simple_examples = \"I now identify as a &str :)\";\nLL +     println!(\"This is the new simple_example: {}\", simple_examples);\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4370,"byte_end":4386,"line_start":172,"line_end":173,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x << 2","highlight_start":5,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the end suggestion probably needs some adjustments to use the expression result correctly","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4370,"byte_end":4386,"line_start":172,"line_end":173,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x << 2","highlight_start":5,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    x << 2","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:172:5\n   |\nLL | /         x << 2\nLL | |     };\n   | |_____^\n   |\n   = note: the end suggestion probably needs some adjustments to use the expression result correctly\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL ~     x << 2;\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4456,"byte_end":4471,"line_start":179,"line_end":180,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x * 4","highlight_start":5,"highlight_end":14},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the end suggestion probably needs some adjustments to use the expression result correctly","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4456,"byte_end":4471,"line_start":179,"line_end":180,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x * 4","highlight_start":5,"highlight_end":14},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    x * 4","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:179:5\n   |\nLL | /         x * 4\nLL | |     }\n   | |_____^\n   |\n   = note: the end suggestion probably needs some adjustments to use the expression result correctly\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL +     x * 4\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4739,"byte_end":4750,"line_start":191,"line_end":191,"column_start":44,"column_end":55,"is_primary":true,"text":[{"text":"    if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }","highlight_start":44,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4739,"byte_end":4750,"line_start":191,"line_end":191,"column_start":44,"column_end":55,"is_primary":true,"text":[{"text":"    if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }","highlight_start":44,"highlight_end":55}],"label":null,"suggested_replacement":"}\n    a = 0x99;","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:191:44\n   |\nLL |     if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }\n   |                                            ^^^^^^^^^^^\n   |\nhelp: consider moving these statements after the if\n   |\nLL ~     if x == 17 { b = 1; a = 0x99; } else { }\nLL +     a = 0x99;\n   |\n\n"}

------------------------------------------

diff of stderr:
---
    | |_____^
 note: the lint level is defined here
   --> $DIR/valid_if_blocks.rs:1:40
    |
 LL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]
 
-error: this `if` has identical blocks
-  --> $DIR/valid_if_blocks.rs:116:15
-  --> $DIR/valid_if_blocks.rs:116:15
+error: all if blocks contain the same code at both the start and the end
    |
    |
-LL |       if x == 0 {
-   |  _______________^
+LL | /     if x == 0 {
 LL | |         let u = 19;
-LL | |         println!("How are u today?");
-LL | |         let _ = "This is a string";
-LL | |     } else {
+   | |___________________^
    |
-note: same as this
-  --> $DIR/valid_if_blocks.rs:120:12
-  --> $DIR/valid_if_blocks.rs:120:12
+note: this code is shared at the end
+  --> $DIR/valid_if_blocks.rs:123:5
    |
-LL |       } else {
-   |  ____________^
-LL | |         let u = 19;
-LL | |         println!("How are u today?");
-LL | |         let _ = "This is a string";
+LL | /         let _ = "This is a string";
 LL | |     }
+note: the lint level is defined here
+  --> $DIR/valid_if_blocks.rs:1:9
+   |
+   |
+LL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]
+help: consider moving these statements before the if
+   |
+   |
+LL ~     let u = 19;
+LL +     if x == 0 {
+help: consider moving these statements after the if
+   |
+LL ~     }
+LL ~     }
+LL +     let _ = "This is a string";
error: test failed, to rerun pass `--test compile-test`
 
 error: this `if` has identical blocks
   --> $DIR/valid_if_blocks.rs:127:23
   --> $DIR/valid_if_blocks.rs:127:23
    |
 LL |     let _ = if x == 6 { 7 } else { 7 };
    |
 note: same as this
   --> $DIR/valid_if_blocks.rs:127:34
    |
    |
 LL |     let _ = if x == 6 { 7 } else { 7 };
 
-error: this `if` has identical blocks
-  --> $DIR/valid_if_blocks.rs:133:23
-   |
-   |
-LL |       } else if x == 68 {
-   |  _______________________^
-LL | |         println!("I'm a doppelgänger");
-LL | |         // Don't listen to my clone below
-LL | |
-LL | |         if y == 90 { "=^.^=" } else { ":D" }
-LL | |     } else {
-   |
-note: same as this
-  --> $DIR/valid_if_blocks.rs:138:12
-   |
-   |
-LL |       } else {
-   |  ____________^
-LL | |         // Don't listen to my clone above
-LL | |         println!("I'm a doppelgänger");
-LL | |
-LL | |         if y == 90 { "=^.^=" } else { ":D" }
-LL | |     };
-
-error: this `if` has identical blocks
-  --> $DIR/valid_if_blocks.rs:147:23
-   |
-   |
-LL |       } else if x == 68 {
-   |  _______________________^
-LL | |         println!("I'm a doppelgänger");
-LL | |         // Don't listen to my clone below
-LL | |     } else {
-   |
-note: same as this
-  --> $DIR/valid_if_blocks.rs:150:12
-   |
-   |
-LL |       } else {
-   |  ____________^
-LL | |         // Don't listen to my clone above
-LL | |         println!("I'm a doppelgänger");
-LL | |     }
-
-error: aborting due to 5 previous errors
+error: aborting due to 3 previous errors
 
---
To only update this specific test, also pass `--test-args branches_sharing_code/valid_if_blocks.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/branches_sharing_code/valid_if_blocks.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/valid_if_blocks.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-62999cf098a4f169.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b83448bd4610d9d2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-8bc4c00ed7c71629.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7f0c26842d997632.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/branches_sharing_code/valid_if_blocks.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2617,"byte_end":2624,"line_start":105,"line_end":106,"column_start":14,"column_end":6,"is_primary":true,"text":[{"text":"    if false {","highlight_start":14,"highlight_end":15},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2630,"byte_end":2637,"line_start":106,"line_end":107,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":39,"byte_end":64,"line_start":1,"line_end":1,"column_start":40,"column_end":65,"is_primary":true,"text":[{"text":"#![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]","highlight_start":40,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:105:14\n   |\nLL |       if false {\n   |  ______________^\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:106:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |     }\n   | |_____^\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:1:40\n   |\nLL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]\n   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"all if blocks contain the same code at both the start and the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2811,"byte_end":2842,"line_start":116,"line_end":117,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let u = 19;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this code is shared at the end","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2992,"byte_end":3029,"line_start":123,"line_end":124,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let _ = \"This is a string\";","highlight_start":5,"highlight_end":36},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":8,"byte_end":37,"line_start":1,"line_end":1,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"#![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider moving these statements before the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2811,"byte_end":2842,"line_start":116,"line_end":117,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let u = 19;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"let u = 19;\n    if x == 0 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null},{"message":"consider moving these statements after the if","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":2992,"byte_end":3029,"line_start":123,"line_end":124,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let _ = \"This is a string\";","highlight_start":5,"highlight_end":36},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let _ = \"This is a string\";","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at both the start and the end\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:116:5\n   |\nLL | /     if x == 0 {\nLL | |         let u = 19;\n   | |___________________^\n   |\nnote: this code is shared at the end\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:123:5\n   |\nLL | /         let _ = \"This is a string\";\nLL | |     }\n   | |_____^\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:1:9\n   |\nLL | #![deny(clippy::branches_sharing_code, clippy::if_same_then_else)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nhelp: consider moving these statements before the if\n   |\nLL ~     let u = 19;\nLL +     if x == 0 {\n   |\nhelp: consider moving these statements after the if\n   |\nLL ~     }\nLL +     let _ = \"This is a string\";\n   |\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":3081,"byte_end":3086,"line_start":127,"line_end":127,"column_start":23,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = if x == 6 { 7 } else { 7 };","highlight_start":23,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/valid_if_blocks.rs","byte_start":3092,"byte_end":3097,"line_start":127,"line_end":127,"column_start":34,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = if x == 6 { 7 } else { 7 };","highlight_start":34,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:127:23\n   |\nLL |     let _ = if x == 6 { 7 } else { 7 };\n   |                       ^^^^^\n   |\nnote: same as this\n  --> tests/ui/branches_sharing_code/valid_if_blocks.rs:127:34\n   |\nLL |     let _ = if x == 6 { 7 } else { 7 };\n   |                                  ^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this match arm has an identical body to the `_` wildcard arm
    |
 LL | /         42 => {
 LL | |             foo();
 LL | |             foo();
 LL | |             let mut a = 42 + [23].len() as i32;
 LL | |             if true {
 LL | |             a
 LL | |         },
    | |_________^ help: try removing the arm
    |
    |
    = help: or try changing either arm body
 note: `_` wildcard arm here
    |
 LL | /         _ => {
 LL | /         _ => {
 LL | |             //~ ERROR match arms have same body
 LL | |             foo();
 LL | |             let mut a = 42 + [23].len() as i32;
 LL | |             a
 LL | |         },
    | |_________^
    | |_________^
    = note: `-D clippy::match-same-arms` implied by `-D warnings`
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         51 => foo(), //~ ERROR match arms have same body
    |         --^^^^^^^^^
    |         |
    |         help: try merging the arm patterns: `51 | 42`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:37:9
    |
    |
 LL |         42 => foo(),
    |         ^^^^^^^^^^^
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         None => 24, //~ ERROR match arms have same body
    |         |
    |         |
    |         help: try merging the arm patterns: `None | Some(_)`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:43:9
    |
    |
 LL |         Some(_) => 24,
    |         ^^^^^^^^^^^^^
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         (None, Some(a)) => bar(a), //~ ERROR match arms have same body
    |         |
    |         |
    |         help: try merging the arm patterns: `(None, Some(a)) | (Some(a), None)`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:65:9
    |
    |
 LL |         (Some(a), None) => bar(a),
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         (Some(a), ..) => bar(a),
    |         |
    |         |
    |         help: try merging the arm patterns: `(Some(a), ..) | (.., Some(a))`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:72:9
    |
    |
 LL |         (.., Some(a)) => bar(a), //~ ERROR match arms have same body
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         (Ok(x), Some(_)) => println!("ok {}", x),
    |         |
    |         |
    |         help: try merging the arm patterns: `(Ok(x), Some(_)) | (Ok(_), Some(x))`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:106:9
    |
    |
 LL |         (Ok(_), Some(x)) => println!("ok {}", x),
 
 
 error: this match arm has an identical body to another arm
-   |
-   |
-LL |         Ok(_) => println!("ok"),
-   |         |
-   |         |
-   |         help: try merging the arm patterns: `Ok(_) | Ok(3)`
-   = help: or try changing either arm body
-note: other arm here
-  --> $DIR/match_same_arms2.rs:120:9
-   |
-   |
-LL |         Ok(3) => println!("ok"),
-
-
-error: this match arm has an identical body to another arm
    |
 LL |           1 => {
 LL |           1 => {
    |           ^ help: try merging the arm patterns: `1 | 0`
    | |
    | |
 LL | |             empty!(0);
 LL | |         },
    |
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:145:9
   --> $DIR/match_same_arms2.rs:145:9
    |
 LL | /         0 => {
 LL | |             empty!(0);
 LL | |         },
 
 
 error: match expression looks like `matches!` macro
    |
    |
 LL |       let _ans = match x {
    |  ________________^
 LL | |         E::A => false,
 LL | |         E::B => false,
 LL | |         _ => true,
 LL | |     };
    | |_____^ help: try this: `!matches!(x, E::A | E::B)`
    |
    = note: `-D clippy::match-like-matches-macro` implied by `-D warnings`
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         Foo::X(0) => 1,
    |         |
    |         |
    |         help: try merging the arm patterns: `Foo::X(0) | Foo::Z(_)`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:200:9
    |
    |
 LL |         Foo::Z(_) => 1,
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         Foo::Z(_) => 1,
    |         |
    |         |
    |         help: try merging the arm patterns: `Foo::Z(_) | Foo::X(0)`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:206:9
    |
    |
 LL |         Foo::X(0) => 1,
 
 
 error: this match arm has an identical body to another arm
    |
    |
 LL |         Some(Bar { y: 0, x: 5, .. }) => 1,
    |         |
    |         |
    |         help: try merging the arm patterns: `Some(Bar { y: 0, x: 5, .. }) | Some(Bar { x: 0, y: 5, .. })`
    = help: or try changing either arm body
 note: other arm here
   --> $DIR/match_same_arms2.rs:228:9
    |
    |
 LL |         Some(Bar { x: 0, y: 5, .. }) => 1,
 
-error: aborting due to 12 previous errors
+error: aborting due to 11 previous errors
 
---
To only update this specific test, also pass `--test-args match_same_arms2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/match_same_arms2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_same_arms2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-62999cf098a4f169.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-b83448bd4610d9d2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-8bc4c00ed7c71629.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-7f0c26842d997632.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/match_same_arms2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this match arm has an identical body to the `_` wildcard arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":266,"byte_end":449,"line_start":15,"line_end":23,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        42 => {","highlight_start":9,"highlight_end":16},{"text":"            foo();","highlight_start":1,"highlight_end":19},{"text":"            let mut a = 42 + [23].len() as i32;","highlight_start":1,"highlight_end":48},{"text":"            if true {","highlight_start":1,"highlight_end":22},{"text":"                a += 7;","highlight_start":1,"highlight_end":24},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"            a = -31 - a;","highlight_start":1,"highlight_end":25},{"text":"            a","highlight_start":1,"highlight_end":14},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`_` wildcard arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":459,"byte_end":689,"line_start":24,"line_end":33,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        _ => {","highlight_start":9,"highlight_end":15},{"text":"            //~ ERROR match arms have same body","highlight_start":1,"highlight_end":48},{"text":"            foo();","highlight_start":1,"highlight_end":19},{"text":"            let mut a = 42 + [23].len() as i32;","highlight_start":1,"highlight_end":48},{"text":"            if true {","highlight_start":1,"highlight_end":22},{"text":"                a += 7;","highlight_start":1,"highlight_end":24},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"            a = -31 - a;","highlight_start":1,"highlight_end":25},{"text":"            a","highlight_start":1,"highlight_end":14},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`-D clippy::match-same-arms` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try removing the arm","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":266,"byte_end":449,"line_start":15,"line_end":23,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        42 => {","highlight_start":9,"highlight_end":16},{"text":"            foo();","highlight_start":1,"highlight_end":19},{"text":"            let mut a = 42 + [23].len() as i32;","highlight_start":1,"highlight_end":48},{"text":"            if true {","highlight_start":1,"highlight_end":22},{"text":"                a += 7;","highlight_start":1,"highlight_end":24},{"text":"            }","highlight_start":1,"highlight_end":14},{"text":"            a = -31 - a;","highlight_start":1,"highlight_end":25},{"text":"            a","highlight_start":1,"highlight_end":14},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":"","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to the `_` wildcard arm\n  --> tests/ui/match_same_arms2.rs:15:9\n   |\nLL | /         42 => {\nLL | |             foo();\nLL | |             let mut a = 42 + [23].len() as i32;\nLL | |             if true {\n...  |\nLL | |             a\nLL | |         },\n   | |_________^ help: try removing the arm\n   |\n   = help: or try changing either arm body\nnote: `_` wildcard arm here\n  --> tests/ui/match_same_arms2.rs:24:9\n   |\nLL | /         _ => {\nLL | |             //~ ERROR match arms have same body\nLL | |             foo();\nLL | |             let mut a = 42 + [23].len() as i32;\n...  |\nLL | |             a\nLL | |         },\n   | |_________^\n   = note: `-D clippy::match-same-arms` implied by `-D warnings`\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":751,"byte_end":762,"line_start":38,"line_end":38,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        51 => foo(), //~ ERROR match arms have same body","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":730,"byte_end":741,"line_start":37,"line_end":37,"column_start":9,"column_end":20,"is_primary":true,"text":[{"text":"        42 => foo(),","highlight_start":9,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":751,"byte_end":753,"line_start":38,"line_end":38,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        51 => foo(), //~ ERROR match arms have same body","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"51 | 42","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:38:9\n   |\nLL |         51 => foo(), //~ ERROR match arms have same body\n   |         --^^^^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `51 | 42`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:37:9\n   |\nLL |         42 => foo(),\n   |         ^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":887,"byte_end":897,"line_start":44,"line_end":44,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        None => 24, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":864,"byte_end":877,"line_start":43,"line_end":43,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        Some(_) => 24,","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":887,"byte_end":891,"line_start":44,"line_end":44,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"        None => 24, //~ ERROR match arms have same body","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"None | Some(_)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:44:9\n   |\nLL |         None => 24, //~ ERROR match arms have same body\n   |         ----^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `None | Some(_)`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:43:9\n   |\nLL |         Some(_) => 24,\n   |         ^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1364,"byte_end":1389,"line_start":66,"line_end":66,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        (None, Some(a)) => bar(a), //~ ERROR match arms have same body","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1329,"byte_end":1354,"line_start":65,"line_end":65,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        (Some(a), None) => bar(a),","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1364,"byte_end":1379,"line_start":66,"line_end":66,"column_start":9,"column_end":24,"is_primary":true,"text":[{"text":"        (None, Some(a)) => bar(a), //~ ERROR match arms have same body","highlight_start":9,"highlight_end":24}],"label":null,"suggested_replacement":"(None, Some(a)) | (Some(a), None)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:66:9\n   |\nLL |         (None, Some(a)) => bar(a), //~ ERROR match arms have same body\n   |         ---------------^^^^^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `(None, Some(a)) | (Some(a), None)`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:65:9\n   |\nLL |         (Some(a), None) => bar(a),\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1492,"byte_end":1515,"line_start":71,"line_end":71,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        (Some(a), ..) => bar(a),","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1525,"byte_end":1548,"line_start":72,"line_end":72,"column_start":9,"column_end":32,"is_primary":true,"text":[{"text":"        (.., Some(a)) => bar(a), //~ ERROR match arms have same body","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":1492,"byte_end":1505,"line_start":71,"line_end":71,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"        (Some(a), ..) => bar(a),","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":"(Some(a), ..) | (.., Some(a))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:71:9\n   |\nLL |         (Some(a), ..) => bar(a),\n   |         -------------^^^^^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `(Some(a), ..) | (.., Some(a))`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:72:9\n   |\nLL |         (.., Some(a)) => bar(a), //~ ERROR match arms have same body\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":2337,"byte_end":2377,"line_start":105,"line_end":105,"column_start":9,"column_end":49,"is_primary":true,"text":[{"text":"        (Ok(x), Some(_)) => println!(\"ok {}\", x),","highlight_start":9,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":2387,"byte_end":2427,"line_start":106,"line_end":106,"column_start":9,"column_end":49,"is_primary":true,"text":[{"text":"        (Ok(_), Some(x)) => println!(\"ok {}\", x),","highlight_start":9,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":2337,"byte_end":2353,"line_start":105,"line_end":105,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"        (Ok(x), Some(_)) => println!(\"ok {}\", x),","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":"(Ok(x), Some(_)) | (Ok(_), Some(x))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:105:9\n   |\nLL |         (Ok(x), Some(_)) => println!(\"ok {}\", x),\n   |         ----------------^^^^^^^^^^^^^^^^^^^^^^^^\n   |         |\n   |         help: try merging the arm patterns: `(Ok(x), Some(_)) | (Ok(_), Some(x))`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:106:9\n   |\nLL |         (Ok(_), Some(x)) => println!(\"ok {}\", x),\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":3257,"byte_end":3296,"line_start":148,"line_end":150,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        1 => {","highlight_start":9,"highlight_end":15},{"text":"            empty!(0);","highlight_start":1,"highlight_end":23},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":3208,"byte_end":3247,"line_start":145,"line_end":147,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        0 => {","highlight_start":9,"highlight_end":15},{"text":"            empty!(0);","highlight_start":1,"highlight_end":23},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":3257,"byte_end":3258,"line_start":148,"line_end":148,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        1 => {","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"1 | 0","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:148:9\n   |\nLL |           1 => {\n   |           ^ help: try merging the arm patterns: `1 | 0`\n   |  _________|\n   | |\nLL | |             empty!(0);\nLL | |         },\n   | |_________^\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:145:9\n   |\nLL | /         0 => {\nLL | |             empty!(0);\nLL | |         },\n   | |_________^\n\n"}
{"message":"match expression looks like `matches!` macro","code":{"code":"clippy::match_like_matches_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":3534,"byte_end":3614,"line_start":166,"line_end":170,"column_start":16,"column_end":6,"is_primary":true,"text":[{"text":"    let _ans = match x {","highlight_start":16,"highlight_end":25},{"text":"        E::A => false,","highlight_start":1,"highlight_end":23},{"text":"        E::B => false,","highlight_start":1,"highlight_end":23},{"text":"        _ => true,","highlight_start":1,"highlight_end":19},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-like-matches-macro` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":3534,"byte_end":3614,"line_start":166,"line_end":170,"column_start":16,"column_end":6,"is_primary":true,"text":[{"text":"    let _ans = match x {","highlight_start":16,"highlight_end":25},{"text":"        E::A => false,","highlight_start":1,"highlight_end":23},{"text":"        E::B => false,","highlight_start":1,"highlight_end":23},{"text":"        _ => true,","highlight_start":1,"highlight_end":19},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"!matches!(x, E::A | E::B)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: match expression looks like `matches!` macro\n  --> tests/ui/match_same_arms2.rs:166:16\n   |\nLL |       let _ans = match x {\n   |  ________________^\nLL | |         E::A => false,\nLL | |         E::B => false,\nLL | |         _ => true,\nLL | |     };\n   | |_____^ help: try this: `!matches!(x, E::A | E::B)`\n   |\n   = note: `-D clippy::match-like-matches-macro` implied by `-D warnings`\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4157,"byte_end":4171,"line_start":198,"line_end":198,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        Foo::X(0) => 1,","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4217,"byte_end":4231,"line_start":200,"line_end":200,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        Foo::Z(_) => 1,","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4157,"byte_end":4166,"line_start":198,"line_end":198,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"        Foo::X(0) => 1,","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":"Foo::X(0) | Foo::Z(_)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:198:9\n   |\nLL |         Foo::X(0) => 1,\n   |         ---------^^^^^\n   |         |\n   |         help: try merging the arm patterns: `Foo::X(0) | Foo::Z(_)`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:200:9\n   |\nLL |         Foo::Z(_) => 1,\n   |         ^^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4395,"byte_end":4409,"line_start":208,"line_end":208,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        Foo::Z(_) => 1,","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4335,"byte_end":4349,"line_start":206,"line_end":206,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        Foo::X(0) => 1,","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4395,"byte_end":4404,"line_start":208,"line_end":208,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"        Foo::Z(_) => 1,","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":"Foo::Z(_) | Foo::X(0)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:208:9\n   |\nLL |         Foo::Z(_) => 1,\n   |         ---------^^^^^\n   |         |\n   |         help: try merging the arm patterns: `Foo::Z(_) | Foo::X(0)`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:206:9\n   |\nLL |         Foo::X(0) => 1,\n   |         ^^^^^^^^^^^^^^\n\n"}
{"message":"this match arm has an identical body to another arm","code":{"code":"clippy::match_same_arms","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4788,"byte_end":4821,"line_start":231,"line_end":231,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"        Some(Bar { y: 0, x: 5, .. }) => 1,","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"or try changing either arm body","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"other arm here","code":null,"level":"note","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4681,"byte_end":4714,"line_start":228,"line_end":228,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"        Some(Bar { x: 0, y: 5, .. }) => 1,","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"try merging the arm patterns","code":null,"level":"help","spans":[{"file_name":"tests/ui/match_same_arms2.rs","byte_start":4788,"byte_end":4816,"line_start":231,"line_end":231,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        Some(Bar { y: 0, x: 5, .. }) => 1,","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"Some(Bar { y: 0, x: 5, .. }) | Some(Bar { x: 0, y: 5, .. })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this match arm has an identical body to another arm\n  --> tests/ui/match_same_arms2.rs:231:9\n   |\nLL |         Some(Bar { y: 0, x: 5, .. }) => 1,\n   |         ----------------------------^^^^^\n   |         |\n   |         help: try merging the arm patterns: `Some(Bar { y: 0, x: 5, .. }) | Some(Bar { x: 0, y: 5, .. })`\n   |\n   = help: or try changing either arm body\nnote: other arm here\n  --> tests/ui/match_same_arms2.rs:228:9\n   |\nLL |         Some(Bar { x: 0, y: 5, .. }) => 1,\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
