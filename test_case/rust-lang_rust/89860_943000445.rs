plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 7807a694c2f079fd3f395821bcc357eee8650071 and 4d6ae25db949bbe4d16625fc9cb551cb9950d695
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: all if blocks contain the same code at the start
    |
 LL | /     if true {
 LL | |         println!("Hello World!");
-   | |________________________________^
-   | |________________________________^
+   | |_________________________________^
    |
 note: the lint level is defined here
   --> $DIR/shared_at_top.rs:2:36
    |
 LL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]
    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 help: consider moving the start statements out like this
-LL ~     println!("Hello World!")
-LL ~     if true {;
+LL ~     println!("Hello World!");
+LL +     if true {
+LL +     if true {
    |
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 0 {
 LL | |         let y = 9;
 LL | |         println!("The value y was set to: `{}`", y);
 LL | |         let _z = y;
    |
    |
    = warning: Some moved values might need to be renamed to avoid wrong references
 help: consider moving the start statements out like this
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
    |
 help: consider moving the start statements out like this
    |
 LL ~     let y = 16;
 LL +     let _ = if x == 7 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 10 {
 LL | |         let used_value_name = "Different type";
 LL | |         println!("Str: {}", used_value_name);
-   | |____________________________________________^
    |
    |
    = warning: Some moved values might need to be renamed to avoid wrong references
 help: consider moving the start statements out like this
    |
 LL ~     let used_value_name = "Different type";
-LL +     println!("Str: {}", used_value_name)
-LL ~     if x == 10 {;
+LL +     println!("Str: {}", used_value_name);
+LL +     if x == 10 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 11 {
 LL | |         let can_be_overridden = "Move me";
 LL | |         println!("I'm also moveable");
-   | |_____________________________________^
    |
    |
    = warning: Some moved values might need to be renamed to avoid wrong references
 help: consider moving the start statements out like this
    |
 LL ~     let can_be_overridden = "Move me";
-LL +     println!("I'm also moveable")
-LL ~     if x == 11 {;
+LL +     println!("I'm also moveable");
+LL +     if x == 11 {
 
 
 error: all if blocks contain the same code at the start
    |
    |
 LL | /     if x == 2020 {
 LL | |         println!("This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.");
 LL | |         println!("Because `IF_SAME_THEN_ELSE` is allowed here");
-   | |_______________________________________________________________^
    |
    |
 help: consider moving the start statements out like this
    |
 LL ~     println!("This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.");
-LL +     println!("Because `IF_SAME_THEN_ELSE` is allowed here")
-LL ~     if x == 2020 {;
+LL +     println!("Because `IF_SAME_THEN_ELSE` is allowed here");
+LL +     if x == 2020 {
 
 
 error: this `if` has identical blocks
    |
    |
 LL |       if x == 2019 {
    |  __________________^
 LL | |         println!("This should trigger `IS_SAME_THAN_ELSE` as usual");
 LL | |     } else {
    |
 note: the lint level is defined here
   --> $DIR/shared_at_top.rs:2:9
    |
    |
 LL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]
 note: same as this
   --> $DIR/shared_at_top.rs:98:12
    |
 LL |       } else {
 LL |       } else {
    |  ____________^
 LL | |         println!("This should trigger `IS_SAME_THAN_ELSE` as usual");
 LL | |     }
 
 error: aborting due to 7 previous errors
 
 
---
To only update this specific test, also pass `--test-args branches_sharing_code/shared_at_top.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/branches_sharing_code/shared_at_top.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/branches_sharing_code/shared_at_top.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/branches_sharing_code/shared_at_top.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":246,"byte_end":289,"line_start":10,"line_end":11,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    if true {","highlight_start":5,"highlight_end":14},{"text":"        println!(\"Hello World!\");","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":87,"byte_end":116,"line_start":2,"line_end":2,"column_start":36,"column_end":65,"is_primary":true,"text":[{"text":"#![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]","highlight_start":36,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":246,"byte_end":289,"line_start":10,"line_end":11,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    if true {","highlight_start":5,"highlight_end":14},{"text":"        println!(\"Hello World!\");","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":"println!(\"Hello World!\");\n    if true {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:10:5\n   |\nLL | /     if true {\nLL | |         println!(\"Hello World!\");\n   | |_________________________________^\n   |\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:2:36\n   |\nLL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]\n   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nhelp: consider moving the start statements out like this\n   |\nLL ~     println!(\"Hello World!\");\nLL +     if true {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":439,"byte_end":542,"line_start":19,"line_end":22,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let y = 9;","highlight_start":1,"highlight_end":19},{"text":"        println!(\"The value y was set to: `{}`\", y);","highlight_start":1,"highlight_end":53},{"text":"        let _z = y;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":439,"byte_end":542,"line_start":19,"line_end":22,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    if x == 0 {","highlight_start":5,"highlight_end":16},{"text":"        let y = 9;","highlight_start":1,"highlight_end":19},{"text":"        println!(\"The value y was set to: `{}`\", y);","highlight_start":1,"highlight_end":53},{"text":"        let _z = y;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"let y = 9;\n    println!(\"The value y was set to: `{}`\", y);\n    let _z = y;\n    if x == 0 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:19:5\n   |\nLL | /     if x == 0 {\nLL | |         let y = 9;\nLL | |         println!(\"The value y was set to: `{}`\", y);\nLL | |         let _z = y;\n   | |___________________^\n   |\n   = warning: Some moved values might need to be renamed to avoid wrong references\nhelp: consider moving the start statements out like this\n   |\nLL ~     let y = 9;\nLL +     println!(\"The value y was set to: `{}`\", y);\nLL +     let _z = y;\nLL +     if x == 0 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1004,"byte_end":1043,"line_start":40,"line_end":41,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = if x == 7 {","highlight_start":5,"highlight_end":24},{"text":"        let y = 16;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1004,"byte_end":1043,"line_start":40,"line_end":41,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    let _ = if x == 7 {","highlight_start":5,"highlight_end":24},{"text":"        let y = 16;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"let y = 16;\n    let _ = if x == 7 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:40:5\n   |\nLL | /     let _ = if x == 7 {\nLL | |         let y = 16;\n   | |___________________^\n   |\nhelp: consider moving the start statements out like this\n   |\nLL ~     let y = 16;\nLL +     let _ = if x == 7 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1473,"byte_end":1579,"line_start":58,"line_end":60,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    if x == 10 {","highlight_start":5,"highlight_end":17},{"text":"        let used_value_name = \"Different type\";","highlight_start":1,"highlight_end":48},{"text":"        println!(\"Str: {}\", used_value_name);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1473,"byte_end":1579,"line_start":58,"line_end":60,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    if x == 10 {","highlight_start":5,"highlight_end":17},{"text":"        let used_value_name = \"Different type\";","highlight_start":1,"highlight_end":48},{"text":"        println!(\"Str: {}\", used_value_name);","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":"let used_value_name = \"Different type\";\n    println!(\"Str: {}\", used_value_name);\n    if x == 10 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:58:5\n   |\nLL | /     if x == 10 {\nLL | |         let used_value_name = \"Different type\";\nLL | |         println!(\"Str: {}\", used_value_name);\n   | |_____________________________________________^\n   |\n   = warning: Some moved values might need to be renamed to avoid wrong references\nhelp: consider moving the start statements out like this\n   |\nLL ~     let used_value_name = \"Different type\";\nLL +     println!(\"Str: {}\", used_value_name);\nLL +     if x == 10 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1907,"byte_end":2001,"line_start":72,"line_end":74,"column_start":5,"column_end":39,"is_primary":true,"text":[{"text":"    if x == 11 {","highlight_start":5,"highlight_end":17},{"text":"        let can_be_overridden = \"Move me\";","highlight_start":1,"highlight_end":43},{"text":"        println!(\"I'm also moveable\");","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":1907,"byte_end":2001,"line_start":72,"line_end":74,"column_start":5,"column_end":39,"is_primary":true,"text":[{"text":"    if x == 11 {","highlight_start":5,"highlight_end":17},{"text":"        let can_be_overridden = \"Move me\";","highlight_start":1,"highlight_end":43},{"text":"        println!(\"I'm also moveable\");","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":"let can_be_overridden = \"Move me\";\n    println!(\"I'm also moveable\");\n    if x == 11 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:72:5\n   |\nLL | /     if x == 11 {\nLL | |         let can_be_overridden = \"Move me\";\nLL | |         println!(\"I'm also moveable\");\n   | |______________________________________^\n   |\n   = warning: Some moved values might need to be renamed to avoid wrong references\nhelp: consider moving the start statements out like this\n   |\nLL ~     let can_be_overridden = \"Move me\";\nLL +     println!(\"I'm also moveable\");\nLL +     if x == 11 {\n   |\n\n"}
{"message":"all if blocks contain the same code at the start","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":2338,"byte_end":2495,"line_start":88,"line_end":90,"column_start":5,"column_end":65,"is_primary":true,"text":[{"text":"    if x == 2020 {","highlight_start":5,"highlight_end":19},{"text":"        println!(\"This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.\");","highlight_start":1,"highlight_end":78},{"text":"        println!(\"Because `IF_SAME_THEN_ELSE` is allowed here\");","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the start statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":2338,"byte_end":2495,"line_start":88,"line_end":90,"column_start":5,"column_end":65,"is_primary":true,"text":[{"text":"    if x == 2020 {","highlight_start":5,"highlight_end":19},{"text":"        println!(\"This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.\");","highlight_start":1,"highlight_end":78},{"text":"        println!(\"Because `IF_SAME_THEN_ELSE` is allowed here\");","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":"println!(\"This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.\");\n    println!(\"Because `IF_SAME_THEN_ELSE` is allowed here\");\n    if x == 2020 {","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the start\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:88:5\n   |\nLL | /     if x == 2020 {\nLL | |         println!(\"This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.\");\nLL | |         println!(\"Because `IF_SAME_THEN_ELSE` is allowed here\");\n   | |________________________________________________________________^\n   |\nhelp: consider moving the start statements out like this\n   |\nLL ~     println!(\"This should trigger the `SHARED_CODE_IN_IF_BLOCKS` lint.\");\nLL +     println!(\"Because `IF_SAME_THEN_ELSE` is allowed here\");\nLL +     if x == 2020 {\n   |\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":2676,"byte_end":2753,"line_start":96,"line_end":98,"column_start":18,"column_end":6,"is_primary":true,"text":[{"text":"    if x == 2019 {","highlight_start":18,"highlight_end":19},{"text":"        println!(\"This should trigger `IS_SAME_THAN_ELSE` as usual\");","highlight_start":1,"highlight_end":70},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":60,"byte_end":85,"line_start":2,"line_end":2,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"#![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_top.rs","byte_start":2759,"byte_end":2836,"line_start":98,"line_end":100,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        println!(\"This should trigger `IS_SAME_THAN_ELSE` as usual\");","highlight_start":1,"highlight_end":70},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:96:18\n   |\nLL |       if x == 2019 {\n   |  __________________^\nLL | |         println!(\"This should trigger `IS_SAME_THAN_ELSE` as usual\");\nLL | |     } else {\n   | |_____^\n   |\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:2:9\n   |\nLL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\nnote: same as this\n  --> tests/ui/branches_sharing_code/shared_at_top.rs:98:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         println!(\"This should trigger `IS_SAME_THAN_ELSE` as usual\");\nLL | |     }\n   | |_____^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: all if blocks contain the same code at the end
    |
 LL | /         let result = false;
 LL | /         let result = false;
 LL | |         println!("Block end!");
 LL | |         result
 LL | |     };
    |
 note: the lint level is defined here
   --> $DIR/shared_at_bottom.rs:2:36
    |
    |
 LL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]
    = note: The end suggestion probably needs some adjustments to use the expression result correctly
    = note: The end suggestion probably needs some adjustments to use the expression result correctly
 help: consider moving the end statements out like this
 LL ~     }
 LL +     let result = false;
 LL +     let result = false;
 LL +     println!("Block end!");
 LL ~     result;
 
 
 error: all if blocks contain the same code at the end
    |
    |
 LL | /         println!("Same end of block");
 LL | |     }
    | |_____^
    |
    |
 help: consider moving the end statements out like this
 LL ~     }
 LL ~     }
-LL +     println!("Same end of block")
+LL +     println!("Same end of block");
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /         println!(
 LL | /         println!(
 LL | |             "I'm moveable because I know: `outer_scope_value`: '{}'",
 LL | |             outer_scope_value
 LL | |         );
 LL | |     }
    |
    |
 help: consider moving the end statements out like this
 LL ~     }
 LL +     println!(
 LL +     println!(
 LL +         "I'm moveable because I know: `outer_scope_value`: '{}'",
 LL +         outer_scope_value
-LL +     )
+LL +     );
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /             println!("Hello World");
 LL | |         }
    | |_________^
    | |_________^
    |
 help: consider moving the end statements out like this
 LL ~         }
-LL +         println!("Hello World")
+LL +         println!("Hello World");
    |
    |
 
 error: all if blocks contain the same code at the end
    |
    |
 LL | /         let later_used_value = "A string value";
 LL | |         println!("{}", later_used_value);
 LL | |         // I'm expecting a note about this
 LL | |     }
    |
    |
    = warning: Some moved values might need to be renamed to avoid wrong references
 help: consider moving the end statements out like this
 LL ~     }
 LL ~     }
 LL +     let later_used_value = "A string value";
-LL +     println!("{}", later_used_value)
+LL +     println!("{}", later_used_value);
 
 
 error: all if blocks contain the same code at the end
    |
    |
 LL | /         let simple_examples = "I now identify as a &str :)";
 LL | |         println!("This is the new simple_example: {}", simple_examples);
 LL | |     }
    |
    |
    = warning: Some moved values might need to be renamed to avoid wrong references
 help: consider moving the end statements out like this
 LL ~     }
 LL ~     }
 LL +     let simple_examples = "I now identify as a &str :)";
-LL +     println!("This is the new simple_example: {}", simple_examples)
+LL +     println!("This is the new simple_example: {}", simple_examples);
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /         x << 2
 LL | |     };
    | |_____^
    | |_____^
    |
    = note: The end suggestion probably needs some adjustments to use the expression result correctly
 help: consider moving the end statements out like this
 LL ~     }
 LL ~     }
 LL ~     x << 2;
 
 
 error: all if blocks contain the same code at the end
    |
 LL | /         x * 4
 LL | |     }
    | |_____^
    | |_____^
    |
    = note: The end suggestion probably needs some adjustments to use the expression result correctly
 help: consider moving the end statements out like this
 LL ~     }
 LL +     x * 4
    |
 
 
 error: all if blocks contain the same code at the end
    |
    |
 LL |     if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }
    |
    |
 help: consider moving the end statements out like this
    |
 LL ~     if x == 17 { b = 1; a = 0x99; } else { }
 LL +     a = 0x99;
 
 error: aborting due to 9 previous errors
 
 
---
To only update this specific test, also pass `--test-args branches_sharing_code/shared_at_bottom.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/branches_sharing_code/shared_at_bottom.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/branches_sharing_code/shared_at_bottom.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/branches_sharing_code/shared_at_bottom.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":823,"byte_end":899,"line_start":30,"line_end":33,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let result = false;","highlight_start":5,"highlight_end":28},{"text":"        println!(\"Block end!\");","highlight_start":1,"highlight_end":32},{"text":"        result","highlight_start":1,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":82,"byte_end":111,"line_start":2,"line_end":2,"column_start":36,"column_end":65,"is_primary":true,"text":[{"text":"#![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]","highlight_start":36,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"The end suggestion probably needs some adjustments to use the expression result correctly","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":823,"byte_end":899,"line_start":30,"line_end":33,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let result = false;","highlight_start":5,"highlight_end":28},{"text":"        println!(\"Block end!\");","highlight_start":1,"highlight_end":32},{"text":"        result","highlight_start":1,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let result = false;\n    println!(\"Block end!\");\n    result","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:30:5\n   |\nLL | /         let result = false;\nLL | |         println!(\"Block end!\");\nLL | |         result\nLL | |     };\n   | |_____^\n   |\nnote: the lint level is defined here\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:2:36\n   |\nLL | #![deny(clippy::if_same_then_else, clippy::branches_sharing_code)]\n   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = note: The end suggestion probably needs some adjustments to use the expression result correctly\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     let result = false;\nLL +     println!(\"Block end!\");\nLL ~     result;\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1315,"byte_end":1355,"line_start":48,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(\"Same end of block\");","highlight_start":5,"highlight_end":39},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1315,"byte_end":1355,"line_start":48,"line_end":49,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(\"Same end of block\");","highlight_start":5,"highlight_end":39},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    println!(\"Same end of block\");","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:48:5\n   |\nLL | /         println!(\"Same end of block\");\nLL | |     }\n   | |_____^\n   |\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     println!(\"Same end of block\");\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1806,"byte_end":1936,"line_start":65,"line_end":69,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(","highlight_start":5,"highlight_end":18},{"text":"            \"I'm moveable because I know: `outer_scope_value`: '{}'\",","highlight_start":1,"highlight_end":70},{"text":"            outer_scope_value","highlight_start":1,"highlight_end":30},{"text":"        );","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":1806,"byte_end":1936,"line_start":65,"line_end":69,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        println!(","highlight_start":5,"highlight_end":18},{"text":"            \"I'm moveable because I know: `outer_scope_value`: '{}'\",","highlight_start":1,"highlight_end":70},{"text":"            outer_scope_value","highlight_start":1,"highlight_end":30},{"text":"        );","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    println!(\n        \"I'm moveable because I know: `outer_scope_value`: '{}'\",\n        outer_scope_value\n    );","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:65:5\n   |\nLL | /         println!(\nLL | |             \"I'm moveable because I know: `outer_scope_value`: '{}'\",\nLL | |             outer_scope_value\nLL | |         );\nLL | |     }\n   | |_____^\n   |\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     println!(\nLL +         \"I'm moveable because I know: `outer_scope_value`: '{}'\",\nLL +         outer_scope_value\nLL +     );\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2092,"byte_end":2130,"line_start":77,"line_end":78,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"            println!(\"Hello World\");","highlight_start":9,"highlight_end":37},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2092,"byte_end":2130,"line_start":77,"line_end":78,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"            println!(\"Hello World\");","highlight_start":9,"highlight_end":37},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":"}\n        println!(\"Hello World\");","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:77:9\n   |\nLL | /             println!(\"Hello World\");\nLL | |         }\n   | |_________^\n   |\nhelp: consider moving the end statements out like this\n   |\nLL ~         }\nLL +         println!(\"Hello World\");\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2469,"byte_end":2604,"line_start":93,"line_end":96,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let later_used_value = \"A string value\";","highlight_start":5,"highlight_end":49},{"text":"        println!(\"{}\", later_used_value);","highlight_start":1,"highlight_end":42},{"text":"        // I'm expecting a note about this","highlight_start":1,"highlight_end":43},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2469,"byte_end":2604,"line_start":93,"line_end":96,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let later_used_value = \"A string value\";","highlight_start":5,"highlight_end":49},{"text":"        println!(\"{}\", later_used_value);","highlight_start":1,"highlight_end":42},{"text":"        // I'm expecting a note about this","highlight_start":1,"highlight_end":43},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let later_used_value = \"A string value\";\n    println!(\"{}\", later_used_value);","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:93:5\n   |\nLL | /         let later_used_value = \"A string value\";\nLL | |         println!(\"{}\", later_used_value);\nLL | |         // I'm expecting a note about this\nLL | |     }\n   | |_____^\n   |\n   = warning: Some moved values might need to be renamed to avoid wrong references\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     let later_used_value = \"A string value\";\nLL +     println!(\"{}\", later_used_value);\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2882,"byte_end":3017,"line_start":106,"line_end":108,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let simple_examples = \"I now identify as a &str :)\";","highlight_start":5,"highlight_end":61},{"text":"        println!(\"This is the new simple_example: {}\", simple_examples);","highlight_start":1,"highlight_end":73},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Some moved values might need to be renamed to avoid wrong references","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":2882,"byte_end":3017,"line_start":106,"line_end":108,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        let simple_examples = \"I now identify as a &str :)\";","highlight_start":5,"highlight_end":61},{"text":"        println!(\"This is the new simple_example: {}\", simple_examples);","highlight_start":1,"highlight_end":73},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    let simple_examples = \"I now identify as a &str :)\";\n    println!(\"This is the new simple_example: {}\", simple_examples);","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:106:5\n   |\nLL | /         let simple_examples = \"I now identify as a &str :)\";\nLL | |         println!(\"This is the new simple_example: {}\", simple_examples);\nLL | |     }\n   | |_____^\n   |\n   = warning: Some moved values might need to be renamed to avoid wrong references\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     let simple_examples = \"I now identify as a &str :)\";\nLL +     println!(\"This is the new simple_example: {}\", simple_examples);\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4329,"byte_end":4345,"line_start":171,"line_end":172,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x << 2","highlight_start":5,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The end suggestion probably needs some adjustments to use the expression result correctly","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4329,"byte_end":4345,"line_start":171,"line_end":172,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x << 2","highlight_start":5,"highlight_end":15},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    x << 2","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:171:5\n   |\nLL | /         x << 2\nLL | |     };\n   | |_____^\n   |\n   = note: The end suggestion probably needs some adjustments to use the expression result correctly\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL ~     x << 2;\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4415,"byte_end":4430,"line_start":178,"line_end":179,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x * 4","highlight_start":5,"highlight_end":14},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"The end suggestion probably needs some adjustments to use the expression result correctly","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4415,"byte_end":4430,"line_start":178,"line_end":179,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"        x * 4","highlight_start":5,"highlight_end":14},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"}\n    x * 4","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:178:5\n   |\nLL | /         x * 4\nLL | |     }\n   | |_____^\n   |\n   = note: The end suggestion probably needs some adjustments to use the expression result correctly\nhelp: consider moving the end statements out like this\n   |\nLL ~     }\nLL +     x * 4\n   |\n\n"}
{"message":"all if blocks contain the same code at the end","code":{"code":"clippy::branches_sharing_code","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4698,"byte_end":4709,"line_start":190,"line_end":190,"column_start":44,"column_end":55,"is_primary":true,"text":[{"text":"    if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }","highlight_start":44,"highlight_end":55}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider moving the end statements out like this","code":null,"level":"help","spans":[{"file_name":"tests/ui/branches_sharing_code/shared_at_bottom.rs","byte_start":4698,"byte_end":4709,"line_start":190,"line_end":190,"column_start":44,"column_end":55,"is_primary":true,"text":[{"text":"    if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }","highlight_start":44,"highlight_end":55}],"label":null,"suggested_replacement":"}\n    a = 0x99;","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: all if blocks contain the same code at the end\n  --> tests/ui/branches_sharing_code/shared_at_bottom.rs:190:44\n   |\nLL |     if x == 17 { b = 1; a = 0x99; } else { a = 0x99; }\n   |                                            ^^^^^^^^^^^\n   |\nhelp: consider moving the end statements out like this\n   |\nLL ~     if x == 17 { b = 1; a = 0x99; } else { }\nLL +     a = 0x99;\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
   --> $DIR/ice-6255.rs:6:9
 LL |         extern crate std as core;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
 ...
 ...
 LL | define_other_core!();
-   | --------------------- in this macro invocation
    |
    = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args crashes/ice-6255.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6255.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6255.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/crashes/ice-6255.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"macro-expanded `extern crate` items cannot shadow names passed with `--extern`","code":null,"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-6255.rs","byte_start":179,"byte_end":204,"line_start":6,"line_end":6,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        extern crate std as core;","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/crashes/ice-6255.rs","byte_start":348,"byte_end":368,"line_start":15,"line_end":15,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"define_other_core!();","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_other_core!","def_site_span":{"file_name":"tests/ui/crashes/ice-6255.rs","byte_start":125,"byte_end":311,"line_start":4,"line_end":9,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! define_other_core {","highlight_start":1,"highlight_end":33},{"text":"    ( ) => {","highlight_start":1,"highlight_end":13},{"text":"        extern crate std as core;","highlight_start":1,"highlight_end":34},{"text":"        //~^ ERROR macro-expanded `extern crate` items cannot shadow names passed with `--extern`","highlight_start":1,"highlight_end":98},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`\n  --> tests/ui/crashes/ice-6255.rs:6:9\n   |\nLL |         extern crate std as core;\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n...\nLL | define_other_core!();\n   | -------------------- in this macro invocation\n   |\n   = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: a `const` item should never be interior mutable
   --> $DIR/others.rs:9:1
    |
 LL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable
    | |
    | |
    | make this a static item (maybe with lazy_static)
    |
    = note: `-D clippy::declare-interior-mutable-const` implied by `-D warnings`
 
 error: a `const` item should never be interior mutable
   --> $DIR/others.rs:10:1
    |
 LL | const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable
    | |
    | |
    | make this a static item (maybe with lazy_static)
 
 error: a `const` item should never be interior mutable
   --> $DIR/others.rs:11:1
    |
 LL | const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);
    | |
    | |
    | make this a static item (maybe with lazy_static)
 
 error: a `const` item should never be interior mutable
   --> $DIR/others.rs:16:9
    |
 LL |         const $name: $ty = $e;
 ...
 ...
 LL | declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable
-   | ------------------------------------------ in this macro invocation
    |
    = note: this error originates in the macro `declare_const` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: aborting due to 4 previous errors
---
To only update this specific test, also pass `--test-args declare_interior_mutable_const/others.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/declare_interior_mutable_const/others.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/declare_interior_mutable_const/others.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/declare_interior_mutable_const/others.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":174,"byte_end":179,"line_start":9,"line_end":9,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":174,"byte_end":222,"line_start":9,"line_end":9,"column_start":1,"column_end":49,"is_primary":true,"text":[{"text":"const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable","highlight_start":1,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::declare-interior-mutable-const` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/others.rs:9:1\n   |\nLL | const ATOMIC: AtomicUsize = AtomicUsize::new(5); //~ ERROR interior mutable\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n   |\n   = note: `-D clippy::declare-interior-mutable-const` implied by `-D warnings`\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":250,"byte_end":255,"line_start":10,"line_end":10,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":250,"byte_end":289,"line_start":10,"line_end":10,"column_start":1,"column_end":40,"is_primary":true,"text":[{"text":"const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable","highlight_start":1,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/others.rs:10:1\n   |\nLL | const CELL: Cell<usize> = Cell::new(6); //~ ERROR interior mutable\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":317,"byte_end":322,"line_start":11,"line_end":11,"column_start":1,"column_end":6,"is_primary":false,"text":[{"text":"const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);","highlight_start":1,"highlight_end":6}],"label":"make this a static item (maybe with lazy_static)","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":317,"byte_end":406,"line_start":11,"line_end":11,"column_start":1,"column_end":90,"is_primary":true,"text":[{"text":"const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);","highlight_start":1,"highlight_end":90}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/others.rs:11:1\n   |\nLL | const ATOMIC_TUPLE: ([AtomicUsize; 1], Vec<AtomicUsize>, u8) = ([ATOMIC], Vec::new(), 7);\n   | -----^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   | |\n   | make this a static item (maybe with lazy_static)\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":514,"byte_end":536,"line_start":16,"line_end":16,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        const $name: $ty = $e;","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":546,"byte_end":587,"line_start":19,"line_end":19,"column_start":1,"column_end":42,"is_primary":false,"text":[{"text":"declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable","highlight_start":1,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"declare_const!","def_site_span":{"file_name":"tests/ui/declare_interior_mutable_const/others.rs","byte_start":436,"byte_end":545,"line_start":14,"line_end":18,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! declare_const {","highlight_start":1,"highlight_end":29},{"text":"    ($name:ident: $ty:ty = $e:expr) => {","highlight_start":1,"highlight_end":41},{"text":"        const $name: $ty = $e;","highlight_start":1,"highlight_end":31},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/others.rs:16:9\n   |\nLL |         const $name: $ty = $e;\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n...\nLL | declare_const!(_ONCE: Once = Once::new()); //~ ERROR interior mutable\n   | ----------------------------------------- in this macro invocation\n   |\n   = note: this error originates in the macro `declare_const` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: a `const` item should never be interior mutable
    |
    |
 LL |     const ATOMIC: AtomicUsize; //~ ERROR interior mutable
    |
    |
    = note: `-D clippy::declare-interior-mutable-const` implied by `-D warnings`
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |         const $name: $ty = $e;
 ...
 ...
 LL |     declare_const!(ANOTHER_ATOMIC: AtomicUsize = Self::ATOMIC); //~ ERROR interior mutable
-   |     ----------------------------------------------------------- in this macro invocation
    |
    = note: this error originates in the macro `declare_const` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const TO_BE_CONCRETE: AtomicUsize = AtomicUsize::new(11); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const TO_BE_UNFROZEN: Self::ToBeUnfrozen = AtomicUsize::new(13); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const WRAPPED_TO_BE_UNFROZEN: Wrapper<Self::ToBeUnfrozen> = Wrapper(AtomicUsize::new(14)); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const BOUNDED: T::ToBeBounded; //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const SELF: Self = AtomicUsize::new(17); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const WRAPPED_SELF: Option<Self> = Some(AtomicUsize::new(21)); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const INDIRECT: Cell<*const T>; //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const ATOMIC: AtomicUsize = AtomicUsize::new(18); //~ ERROR interior mutable
 
 
 error: a `const` item should never be interior mutable
    |
    |
 LL |     const BOUNDED_ASSOC_TYPE: T::ToBeBounded = AtomicUsize::new(19); //~ ERROR interior mutable
 
 error: aborting due to 11 previous errors
 
 
---
To only update this specific test, also pass `--test-args declare_interior_mutable_const/traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/declare_interior_mutable_const/traits.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/declare_interior_mutable_const/traits.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/declare_interior_mutable_const/traits.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":352,"byte_end":378,"line_start":15,"line_end":15,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    const ATOMIC: AtomicUsize; //~ ERROR interior mutable","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::declare-interior-mutable-const` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:15:5\n   |\nLL |     const ATOMIC: AtomicUsize; //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::declare-interior-mutable-const` implied by `-D warnings`\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":208,"byte_end":230,"line_start":9,"line_end":9,"column_start":9,"column_end":31,"is_primary":true,"text":[{"text":"        const $name: $ty = $e;","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":460,"byte_end":518,"line_start":18,"line_end":18,"column_start":5,"column_end":63,"is_primary":false,"text":[{"text":"    declare_const!(ANOTHER_ATOMIC: AtomicUsize = Self::ATOMIC); //~ ERROR interior mutable","highlight_start":5,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"declare_const!","def_site_span":{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":130,"byte_end":239,"line_start":7,"line_end":11,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! declare_const {","highlight_start":1,"highlight_end":29},{"text":"    ($name:ident: $ty:ty = $e:expr) => {","highlight_start":1,"highlight_end":41},{"text":"        const $name: $ty = $e;","highlight_start":1,"highlight_end":31},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:9:9\n   |\nLL |         const $name: $ty = $e;\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n...\nLL |     declare_const!(ANOTHER_ATOMIC: AtomicUsize = Self::ATOMIC); //~ ERROR interior mutable\n   |     ---------------------------------------------------------- in this macro invocation\n   |\n   = note: this error originates in the macro `declare_const` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":1188,"byte_end":1245,"line_start":43,"line_end":43,"column_start":5,"column_end":62,"is_primary":true,"text":[{"text":"    const TO_BE_CONCRETE: AtomicUsize = AtomicUsize::new(11); //~ ERROR interior mutable","highlight_start":5,"highlight_end":62}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:43:5\n   |\nLL |     const TO_BE_CONCRETE: AtomicUsize = AtomicUsize::new(11); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":2021,"byte_end":2085,"line_start":68,"line_end":68,"column_start":5,"column_end":69,"is_primary":true,"text":[{"text":"    const TO_BE_UNFROZEN: Self::ToBeUnfrozen = AtomicUsize::new(13); //~ ERROR interior mutable","highlight_start":5,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:68:5\n   |\nLL |     const TO_BE_UNFROZEN: Self::ToBeUnfrozen = AtomicUsize::new(13); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":2117,"byte_end":2207,"line_start":69,"line_end":69,"column_start":5,"column_end":95,"is_primary":true,"text":[{"text":"    const WRAPPED_TO_BE_UNFROZEN: Wrapper<Self::ToBeUnfrozen> = Wrapper(AtomicUsize::new(14)); //~ ERROR interior mutable","highlight_start":5,"highlight_end":95}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:69:5\n   |\nLL |     const WRAPPED_TO_BE_UNFROZEN: Wrapper<Self::ToBeUnfrozen> = Wrapper(AtomicUsize::new(14)); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":2765,"byte_end":2795,"line_start":88,"line_end":88,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    const BOUNDED: T::ToBeBounded; //~ ERROR interior mutable","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:88:5\n   |\nLL |     const BOUNDED: T::ToBeBounded; //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":3666,"byte_end":3706,"line_start":116,"line_end":116,"column_start":5,"column_end":45,"is_primary":true,"text":[{"text":"    const SELF: Self = AtomicUsize::new(17); //~ ERROR interior mutable","highlight_start":5,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:116:5\n   |\nLL |     const SELF: Self = AtomicUsize::new(17); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":3738,"byte_end":3800,"line_start":117,"line_end":117,"column_start":5,"column_end":67,"is_primary":true,"text":[{"text":"    const WRAPPED_SELF: Option<Self> = Some(AtomicUsize::new(21)); //~ ERROR interior mutable","highlight_start":5,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:117:5\n   |\nLL |     const WRAPPED_SELF: Option<Self> = Some(AtomicUsize::new(21)); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":4097,"byte_end":4128,"line_start":125,"line_end":125,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    const INDIRECT: Cell<*const T>; //~ ERROR interior mutable","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:125:5\n   |\nLL |     const INDIRECT: Cell<*const T>; //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":4587,"byte_end":4636,"line_start":141,"line_end":141,"column_start":5,"column_end":54,"is_primary":true,"text":[{"text":"    const ATOMIC: AtomicUsize = AtomicUsize::new(18); //~ ERROR interior mutable","highlight_start":5,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:141:5\n   |\nLL |     const ATOMIC: AtomicUsize = AtomicUsize::new(18); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"a `const` item should never be interior mutable","code":{"code":"clippy::declare_interior_mutable_const","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/declare_interior_mutable_const/traits.rs","byte_start":4834,"byte_end":4898,"line_start":147,"line_end":147,"column_start":5,"column_end":69,"is_primary":true,"text":[{"text":"    const BOUNDED_ASSOC_TYPE: T::ToBeBounded = AtomicUsize::new(19); //~ ERROR interior mutable","highlight_start":5,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: a `const` item should never be interior mutable\n  --> tests/ui/declare_interior_mutable_const/traits.rs:147:5\n   |\nLL |     const BOUNDED_ASSOC_TYPE: T::ToBeBounded = AtomicUsize::new(19); //~ ERROR interior mutable\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: unsafe function's docs miss `# Safety` section
    |
    |
 LL | / pub unsafe fn destroy_the_planet() {
 LL | |     unimplemented!();
 LL | | }
    |
    |
    = note: `-D clippy::missing-safety-doc` implied by `-D warnings`
 
 error: unsafe function's docs miss `# Safety` section
    |
    |
 LL | /     pub unsafe fn republished() {
 LL | |         unimplemented!();
 LL | |     }
 
 
 error: unsafe function's docs miss `# Safety` section
    |
    |
 LL |     unsafe fn woefully_underdocumented(self);
 
 
 error: docs for unsafe trait missing `# Safety` section
    |
    |
 LL | / pub unsafe trait UnsafeTrait {
 LL | |     fn method();
 LL | | }
 
 
 error: unsafe function's docs miss `# Safety` section
    |
    |
 LL | /     pub unsafe fn more_undocumented_unsafe() -> Self {
 LL | |         unimplemented!();
 LL | |     }
 
 
 error: unsafe function's docs miss `# Safety` section
    |
    |
 LL | /         pub unsafe fn whee() {
 LL | |             unimplemented!()
 LL | |         }
 ...
 ...
 LL |   very_unsafe!();
-   |   --------------- in this macro invocation
    |
    |
    = note: this error originates in the macro `very_unsafe` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: aborting due to 6 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/doc_unsafe.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args doc_unsafe.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/doc_unsafe.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/doc_unsafe.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/doc_unsafe.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unsafe function's docs miss `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":121,"byte_end":181,"line_start":7,"line_end":9,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub unsafe fn destroy_the_planet() {","highlight_start":1,"highlight_end":37},{"text":"    unimplemented!();","highlight_start":1,"highlight_end":22},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::missing-safety-doc` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unsafe function's docs miss `# Safety` section\n  --> tests/ui/doc_unsafe.rs:7:1\n   |\nLL | / pub unsafe fn destroy_the_planet() {\nLL | |     unimplemented!();\nLL | | }\n   | |_^\n   |\n   = note: `-D clippy::missing-safety-doc` implied by `-D warnings`\n\n"}
{"message":"unsafe function's docs miss `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":575,"byte_end":636,"line_start":30,"line_end":32,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub unsafe fn republished() {","highlight_start":5,"highlight_end":34},{"text":"        unimplemented!();","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsafe function's docs miss `# Safety` section\n  --> tests/ui/doc_unsafe.rs:30:5\n   |\nLL | /     pub unsafe fn republished() {\nLL | |         unimplemented!();\nLL | |     }\n   | |_____^\n\n"}
{"message":"unsafe function's docs miss `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":714,"byte_end":755,"line_start":38,"line_end":38,"column_start":5,"column_end":46,"is_primary":true,"text":[{"text":"    unsafe fn woefully_underdocumented(self);","highlight_start":5,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsafe function's docs miss `# Safety` section\n  --> tests/ui/doc_unsafe.rs:38:5\n   |\nLL |     unsafe fn woefully_underdocumented(self);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"docs for unsafe trait missing `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":827,"byte_end":876,"line_start":44,"line_end":46,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"pub unsafe trait UnsafeTrait {","highlight_start":1,"highlight_end":31},{"text":"    fn method();","highlight_start":1,"highlight_end":17},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: docs for unsafe trait missing `# Safety` section\n  --> tests/ui/doc_unsafe.rs:44:1\n   |\nLL | / pub unsafe trait UnsafeTrait {\nLL | |     fn method();\nLL | | }\n   | |_^\n\n"}
{"message":"unsafe function's docs miss `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":1327,"byte_end":1409,"line_start":74,"line_end":76,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub unsafe fn more_undocumented_unsafe() -> Self {","highlight_start":5,"highlight_end":55},{"text":"        unimplemented!();","highlight_start":1,"highlight_end":26},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: unsafe function's docs miss `# Safety` section\n  --> tests/ui/doc_unsafe.rs:74:5\n   |\nLL | /     pub unsafe fn more_undocumented_unsafe() -> Self {\nLL | |         unimplemented!();\nLL | |     }\n   | |_____^\n\n"}
{"message":"unsafe function's docs miss `# Safety` section","code":{"code":"clippy::missing_safety_doc","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/doc_unsafe.rs","byte_start":1621,"byte_end":1682,"line_start":90,"line_end":92,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        pub unsafe fn whee() {","highlight_start":9,"highlight_end":31},{"text":"            unimplemented!()","highlight_start":1,"highlight_end":29},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/doc_unsafe.rs","byte_start":1835,"byte_end":1849,"line_start":103,"line_end":103,"column_start":1,"column_end":15,"is_primary":false,"text":[{"text":"very_unsafe!();","highlight_start":1,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"very_unsafe!","def_site_span":{"file_name":"tests/ui/doc_unsafe.rs","byte_start":1574,"byte_end":1833,"line_start":88,"line_end":101,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! very_unsafe {","highlight_start":1,"highlight_end":27},{"text":"    () => {","highlight_start":1,"highlight_end":12},{"text":"        pub unsafe fn whee() {","highlight_start":1,"highlight_end":31},{"text":"            unimplemented!()","highlight_start":1,"highlight_end":29},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"","highlight_start":1,"highlight_end":1},{"text":"        /// # Safety","highlight_start":1,"highlight_end":21},{"text":"        ///","highlight_start":1,"highlight_end":12},{"text":"        /// Please keep the seat belt fastened","highlight_start":1,"highlight_end":47},{"text":"        pub unsafe fn drive() {","highlight_start":1,"highlight_end":32},{"text":"            whee()","highlight_start":1,"highlight_end":19},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: unsafe function's docs miss `# Safety` section\n  --> tests/ui/doc_unsafe.rs:90:9\n   |\nLL | /         pub unsafe fn whee() {\nLL | |             unimplemented!()\nLL | |         }\n   | |_________^\n...\nLL |   very_unsafe!();\n   |   -------------- in this macro invocation\n   |\n   = note: this error originates in the macro `very_unsafe` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: identical args used in this `assert_eq!` macro call
-  --> $DIR/eq_op_macros.rs:7:20
-   |
-LL |         assert_eq!(a, a);
-   |                    ^^^^
-...
-LL |     assert_in_macro_def!();
-   |     ---------------------- in this macro invocation
-   |
-   = note: `-D clippy::eq-op` implied by `-D warnings`
-   = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)
-
-error: identical args used in this `assert_ne!` macro call
-  --> $DIR/eq_op_macros.rs:8:20
-   |
-LL |         assert_ne!(a, a);
-   |                    ^^^^
-...
-LL |     assert_in_macro_def!();
-   |     ---------------------- in this macro invocation
-   |
-   = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)
-
-error: identical args used in this `assert_eq!` macro call
-  --> $DIR/eq_op_macros.rs:22:16
-   |
-LL |     assert_eq!(a, a);
-   |                ^^^^
-
-error: identical args used in this `assert_eq!` macro call
-  --> $DIR/eq_op_macros.rs:23:16
-   |
-LL |     assert_eq!(a + 1, a + 1);
-   |                ^^^^^^^^^^^^
-
-error: identical args used in this `assert_ne!` macro call
-  --> $DIR/eq_op_macros.rs:30:16
-   |
-LL |     assert_ne!(a, a);
-   |                ^^^^
-
-error: identical args used in this `assert_ne!` macro call
-  --> $DIR/eq_op_macros.rs:31:16
-   |
-LL |     assert_ne!(a + 1, a + 1);
-   |                ^^^^^^^^^^^^
-
 error: identical args used in this `debug_assert_eq!` macro call
   --> $DIR/eq_op_macros.rs:9:26
 LL |         debug_assert_eq!(a, a);
    |                          ^^^^
 ...
 LL |     assert_in_macro_def!();
 LL |     assert_in_macro_def!();
    |     ---------------------- in this macro invocation
    |
+   = note: `-D clippy::eq-op` implied by `-D warnings`
    = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: identical args used in this `debug_assert_ne!` macro call
   --> $DIR/eq_op_macros.rs:10:26
    |
 LL |         debug_assert_ne!(a, a);
 ...
 LL |     assert_in_macro_def!();
    |     ---------------------- in this macro invocation
    |
    |
    = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: identical args used in this `debug_assert_eq!` macro call
   --> $DIR/eq_op_macros.rs:38:22
 LL |     debug_assert_eq!(a, a);
    |                      ^^^^
 
 
 error: identical args used in this `debug_assert_eq!` macro call
   --> $DIR/eq_op_macros.rs:39:22
    |
 LL |     debug_assert_eq!(a + 1, a + 1);
 
 
 error: identical args used in this `debug_assert_ne!` macro call
   --> $DIR/eq_op_macros.rs:46:22
    |
 LL |     debug_assert_ne!(a, a);
 
 
 error: identical args used in this `debug_assert_ne!` macro call
   --> $DIR/eq_op_macros.rs:47:22
    |
 LL |     debug_assert_ne!(a + 1, a + 1);
 
-error: aborting due to 12 previous errors
+error: aborting due to 6 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/eq_op_macros.stage-id.stderr
To only update this specific test, also pass `--test-args eq_op_macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/eq_op_macros.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/eq_op_macros.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/eq_op_macros.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"identical args used in this `debug_assert_eq!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":202,"byte_end":206,"line_start":9,"line_end":9,"column_start":26,"column_end":30,"is_primary":true,"text":[{"text":"        debug_assert_eq!(a, a);","highlight_start":26,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/eq_op_macros.rs","byte_start":335,"byte_end":357,"line_start":16,"line_end":16,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    assert_in_macro_def!();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_in_macro_def!","def_site_span":{"file_name":"tests/ui/eq_op_macros.rs","byte_start":58,"byte_end":249,"line_start":4,"line_end":12,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! assert_in_macro_def {","highlight_start":1,"highlight_end":35},{"text":"    () => {","highlight_start":1,"highlight_end":12},{"text":"        let a = 42;","highlight_start":1,"highlight_end":20},{"text":"        assert_eq!(a, a);","highlight_start":1,"highlight_end":26},{"text":"        assert_ne!(a, a);","highlight_start":1,"highlight_end":26},{"text":"        debug_assert_eq!(a, a);","highlight_start":1,"highlight_end":32},{"text":"        debug_assert_ne!(a, a);","highlight_start":1,"highlight_end":32},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::eq-op` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: identical args used in this `debug_assert_eq!` macro call\n  --> tests/ui/eq_op_macros.rs:9:26\n   |\nLL |         debug_assert_eq!(a, a);\n   |                          ^^^^\n...\nLL |     assert_in_macro_def!();\n   |     ---------------------- in this macro invocation\n   |\n   = note: `-D clippy::eq-op` implied by `-D warnings`\n   = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"identical args used in this `debug_assert_ne!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":234,"byte_end":238,"line_start":10,"line_end":10,"column_start":26,"column_end":30,"is_primary":true,"text":[{"text":"        debug_assert_ne!(a, a);","highlight_start":26,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/eq_op_macros.rs","byte_start":335,"byte_end":357,"line_start":16,"line_end":16,"column_start":5,"column_end":27,"is_primary":false,"text":[{"text":"    assert_in_macro_def!();","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_in_macro_def!","def_site_span":{"file_name":"tests/ui/eq_op_macros.rs","byte_start":58,"byte_end":249,"line_start":4,"line_end":12,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! assert_in_macro_def {","highlight_start":1,"highlight_end":35},{"text":"    () => {","highlight_start":1,"highlight_end":12},{"text":"        let a = 42;","highlight_start":1,"highlight_end":20},{"text":"        assert_eq!(a, a);","highlight_start":1,"highlight_end":26},{"text":"        assert_ne!(a, a);","highlight_start":1,"highlight_end":26},{"text":"        debug_assert_eq!(a, a);","highlight_start":1,"highlight_end":32},{"text":"        debug_assert_ne!(a, a);","highlight_start":1,"highlight_end":32},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: identical args used in this `debug_assert_ne!` macro call\n  --> tests/ui/eq_op_macros.rs:10:26\n   |\nLL |         debug_assert_ne!(a, a);\n   |                          ^^^^\n...\nLL |     assert_in_macro_def!();\n   |     ---------------------- in this macro invocation\n   |\n   = note: this error originates in the macro `assert_in_macro_def` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"identical args used in this `debug_assert_eq!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":829,"byte_end":833,"line_start":38,"line_end":38,"column_start":22,"column_end":26,"is_primary":true,"text":[{"text":"    debug_assert_eq!(a, a);","highlight_start":22,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: identical args used in this `debug_assert_eq!` macro call\n  --> tests/ui/eq_op_macros.rs:38:22\n   |\nLL |     debug_assert_eq!(a, a);\n   |                      ^^^^\n\n"}
{"message":"identical args used in this `debug_assert_eq!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":857,"byte_end":869,"line_start":39,"line_end":39,"column_start":22,"column_end":34,"is_primary":true,"text":[{"text":"    debug_assert_eq!(a + 1, a + 1);","highlight_start":22,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: identical args used in this `debug_assert_eq!` macro call\n  --> tests/ui/eq_op_macros.rs:39:22\n   |\nLL |     debug_assert_eq!(a + 1, a + 1);\n   |                      ^^^^^^^^^^^^\n\n"}
{"message":"identical args used in this `debug_assert_ne!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":1049,"byte_end":1053,"line_start":46,"line_end":46,"column_start":22,"column_end":26,"is_primary":true,"text":[{"text":"    debug_assert_ne!(a, a);","highlight_start":22,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: identical args used in this `debug_assert_ne!` macro call\n  --> tests/ui/eq_op_macros.rs:46:22\n   |\nLL |     debug_assert_ne!(a, a);\n   |                      ^^^^\n\n"}
{"message":"identical args used in this `debug_assert_ne!` macro call","code":{"code":"clippy::eq_op","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/eq_op_macros.rs","byte_start":1077,"byte_end":1089,"line_start":47,"line_end":47,"column_start":22,"column_end":34,"is_primary":true,"text":[{"text":"    debug_assert_ne!(a + 1, a + 1);","highlight_start":22,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: identical args used in this `debug_assert_ne!` macro call\n  --> tests/ui/eq_op_macros.rs:47:22\n   |\nLL |     debug_assert_ne!(a + 1, a + 1);\n   |                      ^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if !a.is_empty() {
-LL | |         panic!("qaqaq{:?}", a);
-LL | |     }
-   | |_____^ help: try: `assert!(a.is_empty(), "qaqaq{:?}", a);`
-   |
-   = note: `-D clippy::if-then-panic` implied by `-D warnings`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if !a.is_empty() {
-LL | |         panic!("qwqwq");
-LL | |     }
-   | |_____^ help: try: `assert!(a.is_empty(), "qwqwq");`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() {
-LL | |         panic!("panic1");
-LL | |     }
-   | |_____^ help: try: `assert!(!b.is_empty(), "panic1");`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() && a.is_empty() {
-LL | |         panic!("panic2");
-LL | |     }
-   | |_____^ help: try: `assert!(!(b.is_empty() && a.is_empty()), "panic2");`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a.is_empty() && !b.is_empty() {
-LL | |         panic!("panic3");
-LL | |     }
-   | |_____^ help: try: `assert!(!(a.is_empty() && !b.is_empty()), "panic3");`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if b.is_empty() || a.is_empty() {
-LL | |         panic!("panic4");
-LL | |     }
-   | |_____^ help: try: `assert!(!(b.is_empty() || a.is_empty()), "panic4");`
-
-error: only a `panic!` in `if`-then statement
-   |
-   |
-LL | /     if a.is_empty() || !b.is_empty() {
-LL | |         panic!("panic5");
-LL | |     }
-   | |_____^ help: try: `assert!(!(a.is_empty() || !b.is_empty()), "panic5");`
-error: aborting due to 7 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![warn(clippy::if_then_panic)]
 fn main() {
 fn main() {
     let a = vec![1, 2, 3];
     let c = Some(2);
     if !a.is_empty()
         && a.len() == 3
         && c != None
         && !a.is_empty()
         && a.len() == 3
         && !a.is_empty()
         && a.len() == 3
         && !a.is_empty()
         && a.len() == 3
     {
         panic!("qaqaq{:?}", a);
     }
-    assert!(a.is_empty(), "qaqaq{:?}", a);
-    assert!(a.is_empty(), "qwqwq");
+    if !a.is_empty() {
+        panic!("qaqaq{:?}", a);
+    }
+    if !a.is_empty() {
+        panic!("qwqwq");
+    }
     if a.len() == 3 {
         println!("qwq");
         println!("qwq");
         println!("qwq");
     }
     if let Some(b) = c {
         panic!("orz {}", b);
     }
     if a.len() == 3 {
         panic!("qaqaq");
     } else {
         println!("qwq");
     }
     let b = vec![1, 2, 3];
-    assert!(!b.is_empty(), "panic1");
-    assert!(!(b.is_empty() && a.is_empty()), "panic2");
-    assert!(!(a.is_empty() && !b.is_empty()), "panic3");
-    assert!(!(b.is_empty() || a.is_empty()), "panic4");
-    assert!(!(a.is_empty() || !b.is_empty()), "panic5");
+    if b.is_empty() {
+        panic!("panic1");
+    }
+    if b.is_empty() && a.is_empty() {
+        panic!("panic2");
+    }
+    if a.is_empty() && !b.is_empty() {
+        panic!("panic3");
+    }
+    if b.is_empty() || a.is_empty() {
+        panic!("panic4");
+    }
+    if a.is_empty() || !b.is_empty() {
+        panic!("panic5");
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.fixed
To only update this specific test, also pass `--test-args if_then_panic.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/if_then_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: impl for `HashMap` should be generalized over different hashers
    |
    |
 LL | impl<K: Hash + Eq, V> Foo<i8> for HashMap<K, V> {
    |
 note: the lint level is defined here
   --> $DIR/implicit_hasher.rs:3:9
    |
    |
 LL | #![deny(clippy::implicit_hasher)]
 help: consider adding a type parameter
    |
    |
 LL | impl<K: Hash + Eq, V, S: ::std::hash::BuildHasher + Default> Foo<i8> for HashMap<K, V, S> {
    |     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~             ~~~~~~~~~~~~~~~~
 help: ...and use generic constructor
    |
 LL |         (HashMap::default(), HashMap::with_capacity_and_hasher(10, Default::default()))
    |          ~~~~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 
 error: impl for `HashMap` should be generalized over different hashers
    |
    |
 LL | impl<K: Hash + Eq, V> Foo<i8> for (HashMap<K, V>,) {
    |
 help: consider adding a type parameter
    |
    |
 LL | impl<K: Hash + Eq, V, S: ::std::hash::BuildHasher + Default> Foo<i8> for (HashMap<K, V, S>,) {
    |     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~              ~~~~~~~~~~~~~~~~
 help: ...and use generic constructor
    |
 LL |         ((HashMap::default(),), (HashMap::with_capacity_and_hasher(10, Default::default()),))
 
 
 error: impl for `HashMap` should be generalized over different hashers
    |
    |
 LL | impl Foo<i16> for HashMap<String, String> {
    |
 help: consider adding a type parameter
    |
    |
 LL | impl<S: ::std::hash::BuildHasher + Default> Foo<i16> for HashMap<String, String, S> {
    |     +++++++++++++++++++++++++++++++++++++++              ~~~~~~~~~~~~~~~~~~~~~~~~~~
 help: ...and use generic constructor
    |
 LL |         (HashMap::default(), HashMap::with_capacity_and_hasher(10, Default::default()))
    |          ~~~~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 
 error: impl for `HashSet` should be generalized over different hashers
    |
    |
 LL | impl<T: Hash + Eq> Foo<i8> for HashSet<T> {
    |
 help: consider adding a type parameter
    |
    |
 LL | impl<T: Hash + Eq, S: ::std::hash::BuildHasher + Default> Foo<i8> for HashSet<T, S> {
    |     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~             ~~~~~~~~~~~~~
 help: ...and use generic constructor
    |
 LL |         (HashSet::default(), HashSet::with_capacity_and_hasher(10, Default::default()))
    |          ~~~~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 
 error: impl for `HashSet` should be generalized over different hashers
    |
    |
 LL | impl Foo<i16> for HashSet<String> {
    |
 help: consider adding a type parameter
    |
    |
 LL | impl<S: ::std::hash::BuildHasher + Default> Foo<i16> for HashSet<String, S> {
    |     +++++++++++++++++++++++++++++++++++++++              ~~~~~~~~~~~~~~~~~~
 help: ...and use generic constructor
    |
 LL |         (HashSet::default(), HashSet::with_capacity_and_hasher(10, Default::default()))
    |          ~~~~~~~~~~~~~~~~~~  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 
 error: parameter of type `HashMap` should be generalized over different hashers
    |
    |
 LL | pub fn foo(_map: &mut HashMap<i32, i32>, _set: &mut HashSet<i32>) {}
    |
 help: consider adding a type parameter
    |
    |
 LL | pub fn foo<S: ::std::hash::BuildHasher>(_map: &mut HashMap<i32, i32, S>, _set: &mut HashSet<i32>) {}
    |           +++++++++++++++++++++++++++++            ~~~~~~~~~~~~~~~~~~~~
 
 error: parameter of type `HashSet` should be generalized over different hashers
    |
    |
 LL | pub fn foo(_map: &mut HashMap<i32, i32>, _set: &mut HashSet<i32>) {}
    |
 help: consider adding a type parameter
    |
    |
 LL | pub fn foo<S: ::std::hash::BuildHasher>(_map: &mut HashMap<i32, i32>, _set: &mut HashSet<i32, S>) {}
    |           +++++++++++++++++++++++++++++                                          ~~~~~~~~~~~~~~~
 
 error: impl for `HashMap` should be generalized over different hashers
    |
    |
 LL |         impl<K: Hash + Eq, V> Foo<u8> for HashMap<K, V> {
 ...
 ...
 LL | gen!(impl);
-   | ----------- in this macro invocation
    |
    |
    = note: this error originates in the macro `gen` (in Nightly builds, run with -Z macro-backtrace for more info)
    |
    |
---
To only update this specific test, also pass `--test-args match_wild_err_arm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/match_wild_err_arm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/match_wild_err_arm.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/match_wild_err_arm.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`Err(_)` matches all errors","code":{"code":"clippy::match_wild_err_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_wild_err_arm.rs","byte_start":263,"byte_end":269,"line_start":11,"line_end":11,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        Err(_) => panic!(\"err\"),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-wild-err-arm` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Err(_)` matches all errors\n  --> tests/ui/match_wild_err_arm.rs:11:9\n   |\nLL |         Err(_) => panic!(\"err\"),\n   |         ^^^^^^\n   |\n   = note: `-D clippy::match-wild-err-arm` implied by `-D warnings`\n   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable\n\n"}
{"message":"`Err(_)` matches all errors","code":{"code":"clippy::match_wild_err_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_wild_err_arm.rs","byte_start":383,"byte_end":389,"line_start":17,"line_end":17,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        Err(_) => panic!(),","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Err(_)` matches all errors\n  --> tests/ui/match_wild_err_arm.rs:17:9\n   |\nLL |         Err(_) => panic!(),\n   |         ^^^^^^\n   |\n   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable\n\n"}
{"message":"`Err(_e)` matches all errors","code":{"code":"clippy::match_wild_err_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_wild_err_arm.rs","byte_start":638,"byte_end":645,"line_start":31,"line_end":31,"column_start":9,"column_end":16,"is_primary":true,"text":[{"text":"        Err(_e) => panic!(),","highlight_start":9,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Err(_e)` matches all errors\n  --> tests/ui/match_wild_err_arm.rs:31:9\n   |\nLL |         Err(_e) => panic!(),\n   |         ^^^^^^^\n   |\n   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this `else` block is redundant
   --> $DIR/needless_continue.rs:28:16
 LL |           } else {
    |  ________________^
 LL | |             continue;
 LL | |         }
 LL | |         }
    | |_________^
    |
    = note: `-D clippy::needless-continue` implied by `-D warnings`
    = help: consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block
                    if i % 2 == 0 && i % 3 == 0 {
                        println!("{}", i + 1);
                        println!("{}", i + 1);
                        if i % 5 == 0 {
                            println!("{}", i + 2);
                        let i = 0;
                        let i = 0;
                        println!("bar {} ", i);
                        // merged code follows:
-                       println!("bleh")
+                       println!("bleh");
                        {
                            println!("blah");
                        }
                        if !(!(i == 2) || !(i == 5)) {
                            println!("lama");
                        }
                        if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {
                        } else {
                        } else {
                            println!("Blabber");
                            println!("Jabber");
                        }
-                       println!("bleh")
+                       println!("bleh");
 
 
 error: there is no need for an explicit `else` block for this `if` expression
   --> $DIR/needless_continue.rs:43:9
    |
 LL | /         if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {
 LL | |             continue;
 LL | |         } else {
 LL | |             println!("Blabber");
 LL | |             println!("Jabber");
 LL | |         }
    |
    |
    = help: consider dropping the `else` clause
                    if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {
                    }
                    {
                    {
                        println!("Blabber");
                        println!("Jabber");
 
 
 error: this `continue` expression is redundant
   --> $DIR/needless_continue.rs:56:9
 LL |         continue; // should lint here
    |         ^^^^^^^^^
    |
    |
    = help: consider dropping the `continue` expression
 
 error: this `continue` expression is redundant
   --> $DIR/needless_continue.rs:63:9
 LL |         continue; // should lint here
    |         ^^^^^^^^^
    |
    |
    = help: consider dropping the `continue` expression
 
 error: this `continue` expression is redundant
   --> $DIR/needless_continue.rs:70:9
 LL |         continue // should lint here
    |         ^^^^^^^^
    |
    |
    = help: consider dropping the `continue` expression
 
 error: this `continue` expression is redundant
   --> $DIR/needless_continue.rs:78:9
 LL |         continue // should lint here
    |         ^^^^^^^^
    |
    |
    = help: consider dropping the `continue` expression
 
 error: this `else` block is redundant
   --> $DIR/needless_continue.rs:128:24
 LL |                   } else {
    |  ________________________^
    |  ________________________^
 LL | |                     continue 'inner; // should lint here
 LL | |                 }
    |
    |
    = help: consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block
                            if condition() {
                                println!("bar-3");
                                // merged code follows:
-                               println!("bar-4")
+                               println!("bar-4");
                                update_condition();
                                if condition() {
                                    continue; // should lint here
                                } else {
                                    println!("bar-5");
-                               println!("bar-6")
-                               println!("bar-6")
+                               println!("bar-6");
 
 
 error: there is no need for an explicit `else` block for this `if` expression
   --> $DIR/needless_continue.rs:134:17
 LL | /                 if condition() {
 LL | |                     continue; // should lint here
 LL | |                 } else {
 LL | |                 } else {
 LL | |                     println!("bar-5");
 LL | |                 }
    |
    |
    = help: consider dropping the `else` clause
                            if condition() {
                                continue; // should lint here
                            {
                            {
                                println!("bar-5");
 
 error: aborting due to 8 previous errors
 
 
---
To only update this specific test, also pass `--test-args needless_continue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_continue.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_continue.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/needless_continue.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `else` block is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":499,"byte_end":532,"line_start":28,"line_end":30,"column_start":16,"column_end":10,"is_primary":true,"text":[{"text":"        } else {","highlight_start":16,"highlight_end":17},{"text":"            continue;","highlight_start":1,"highlight_end":22},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-continue` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block\n        if i % 2 == 0 && i % 3 == 0 {\n            println!(\"{}\", i);\n            println!(\"{}\", i + 1);\n            if i % 5 == 0 {\n                println!(\"{}\", i + 2);\n            }\n            let i = 0;\n            println!(\"bar {} \", i);\n            // merged code follows:\n            println!(\"bleh\");\n            {\n                println!(\"blah\");\n            }\n            if !(!(i == 2) || !(i == 5)) {\n                println!(\"lama\");\n            }\n            if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {\n                continue;\n            } else {\n                println!(\"Blabber\");\n                println!(\"Jabber\");\n            }\n            println!(\"bleh\");\n        }","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `else` block is redundant\n  --> tests/ui/needless_continue.rs:28:16\n   |\nLL |           } else {\n   |  ________________^\nLL | |             continue;\nLL | |         }\n   | |_________^\n   |\n   = note: `-D clippy::needless-continue` implied by `-D warnings`\n   = help: consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block\n                   if i % 2 == 0 && i % 3 == 0 {\n                       println!(\"{}\", i);\n                       println!(\"{}\", i + 1);\n                       if i % 5 == 0 {\n                           println!(\"{}\", i + 2);\n                       }\n                       let i = 0;\n                       println!(\"bar {} \", i);\n                       // merged code follows:\n                       println!(\"bleh\");\n                       {\n                           println!(\"blah\");\n                       }\n                       if !(!(i == 2) || !(i == 5)) {\n                           println!(\"lama\");\n                       }\n                       if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {\n                           continue;\n                       } else {\n                           println!(\"Blabber\");\n                           println!(\"Jabber\");\n                       }\n                       println!(\"bleh\");\n                   }\n\n"}
{"message":"there is no need for an explicit `else` block for this `if` expression","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":822,"byte_end":988,"line_start":43,"line_end":48,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {","highlight_start":9,"highlight_end":61},{"text":"            continue;","highlight_start":1,"highlight_end":22},{"text":"        } else {","highlight_start":1,"highlight_end":17},{"text":"            println!(\"Blabber\");","highlight_start":1,"highlight_end":33},{"text":"            println!(\"Jabber\");","highlight_start":1,"highlight_end":32},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `else` clause\n        if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {\n            continue;\n        }\n        {\n            println!(\"Blabber\");\n            println!(\"Jabber\");\n        }","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: there is no need for an explicit `else` block for this `if` expression\n  --> tests/ui/needless_continue.rs:43:9\n   |\nLL | /         if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {\nLL | |             continue;\nLL | |         } else {\nLL | |             println!(\"Blabber\");\nLL | |             println!(\"Jabber\");\nLL | |         }\n   | |_________^\n   |\n   = help: consider dropping the `else` clause\n                   if (zero!(i % 2) || nonzero!(i % 5)) && i % 3 != 0 {\n                       continue;\n                   }\n                   {\n                       println!(\"Blabber\");\n                       println!(\"Jabber\");\n                   }\n\n"}
{"message":"this `continue` expression is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":1063,"byte_end":1072,"line_start":56,"line_end":56,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"        continue; // should lint here","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `continue` expression","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `continue` expression is redundant\n  --> tests/ui/needless_continue.rs:56:9\n   |\nLL |         continue; // should lint here\n   |         ^^^^^^^^^\n   |\n   = help: consider dropping the `continue` expression\n\n"}
{"message":"this `continue` expression is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":1167,"byte_end":1176,"line_start":63,"line_end":63,"column_start":9,"column_end":18,"is_primary":true,"text":[{"text":"        continue; // should lint here","highlight_start":9,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `continue` expression","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `continue` expression is redundant\n  --> tests/ui/needless_continue.rs:63:9\n   |\nLL |         continue; // should lint here\n   |         ^^^^^^^^^\n   |\n   = help: consider dropping the `continue` expression\n\n"}
{"message":"this `continue` expression is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":1262,"byte_end":1270,"line_start":70,"line_end":70,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        continue // should lint here","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `continue` expression","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `continue` expression is redundant\n  --> tests/ui/needless_continue.rs:70:9\n   |\nLL |         continue // should lint here\n   |         ^^^^^^^^\n   |\n   = help: consider dropping the `continue` expression\n\n"}
{"message":"this `continue` expression is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":1382,"byte_end":1390,"line_start":78,"line_end":78,"column_start":9,"column_end":17,"is_primary":true,"text":[{"text":"        continue // should lint here","highlight_start":9,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `continue` expression","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `continue` expression is redundant\n  --> tests/ui/needless_continue.rs:78:9\n   |\nLL |         continue // should lint here\n   |         ^^^^^^^^\n   |\n   = help: consider dropping the `continue` expression\n\n"}
{"message":"this `else` block is redundant","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":2711,"byte_end":2787,"line_start":128,"line_end":130,"column_start":24,"column_end":18,"is_primary":true,"text":[{"text":"                } else {","highlight_start":24,"highlight_end":25},{"text":"                    continue 'inner; // should lint here","highlight_start":1,"highlight_end":57},{"text":"                }","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block\n                if condition() {\n                    println!(\"bar-3\");\n                    // merged code follows:\n                    println!(\"bar-4\");\n                    update_condition();\n                    if condition() {\n                        continue; // should lint here\n                    } else {\n                        println!(\"bar-5\");\n                    }\n                    println!(\"bar-6\");\n                }","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this `else` block is redundant\n  --> tests/ui/needless_continue.rs:128:24\n   |\nLL |                   } else {\n   |  ________________________^\nLL | |                     continue 'inner; // should lint here\nLL | |                 }\n   | |_________________^\n   |\n   = help: consider dropping the `else` clause and merging the code that follows (in the loop) with the `if` block\n                           if condition() {\n                               println!(\"bar-3\");\n                               // merged code follows:\n                               println!(\"bar-4\");\n                               update_condition();\n                               if condition() {\n                                   continue; // should lint here\n                               } else {\n                                   println!(\"bar-5\");\n                               }\n                               println!(\"bar-6\");\n                           }\n\n"}
{"message":"there is no need for an explicit `else` block for this `if` expression","code":{"code":"clippy::needless_continue","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_continue.rs","byte_start":2876,"byte_end":3024,"line_start":134,"line_end":138,"column_start":17,"column_end":18,"is_primary":true,"text":[{"text":"                if condition() {","highlight_start":17,"highlight_end":33},{"text":"                    continue; // should lint here","highlight_start":1,"highlight_end":50},{"text":"                } else {","highlight_start":1,"highlight_end":25},{"text":"                    println!(\"bar-5\");","highlight_start":1,"highlight_end":39},{"text":"                }","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider dropping the `else` clause\n                if condition() {\n                    continue; // should lint here\n                }\n                {\n                    println!(\"bar-5\");\n                }","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: there is no need for an explicit `else` block for this `if` expression\n  --> tests/ui/needless_continue.rs:134:17\n   |\nLL | /                 if condition() {\nLL | |                     continue; // should lint here\nLL | |                 } else {\nLL | |                     println!(\"bar-5\");\nLL | |                 }\n   | |_________________^\n   |\n   = help: consider dropping the `else` clause\n                           if condition() {\n                               continue; // should lint here\n                           }\n                           {\n                               println!(\"bar-5\");\n                           }\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: unneeded unit return type
   --> $DIR/unused_unit.rs:19:28
    |
 LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
    |                            ^^^^^^ help: remove the `-> ()`
 note: the lint level is defined here
   --> $DIR/unused_unit.rs:12:9
    |
    |
 LL | #![deny(clippy::unused_unit)]
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:20:18
    |
    |
 LL |     where G: Fn() -> () {
    |                  ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:19:58
    |
    |
 LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
    |                                                          ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:21:26
    |
    |
 LL |         let _y: &dyn Fn() -> () = &f;
    |                          ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:28:18
    |
 LL |     fn into(self) -> () {
---
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:34:29
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:36:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:37:16
    |
    |
 LL |         H: Fn() -> ();
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:41:29
    |
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:43:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:44:16
    |
    |
 LL |         H: Fn() -> () {}
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:47:17
    |
    |
 LL | fn return_unit() -> () { () }
    |                 ^^^^^^ help: remove the `-> ()`
 error: unneeded unit expression
   --> $DIR/unused_unit.rs:47:26
    |
    |
 LL | fn return_unit() -> () { () }
    |                          ^^ help: remove the final `()`
 error: unneeded `()`
   --> $DIR/unused_unit.rs:57:14
    |
 LL |         break();
---
 
+error: unneeded unit expression
+  --> $DIR/unused_unit.rs:71:27
+   |
+LL |         recv(rx) -> _x => ()
+   |                           ^^ help: remove the final `()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:76:10
    |
    |
 LL | fn test()->(){}
    |          ^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:79:11
    |
    |
 LL | fn test2() ->(){}
    |           ^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:82:11
    |
    |
 LL | fn test3()-> (){}
    |           ^^^^^ help: remove the `-> ()`
-error: aborting due to 19 previous errors
+error: aborting due to 20 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 // The output for humans should just highlight the whole span without showing
 // the suggested replacement, but we also want to test that suggested
 // replacement only removes one set of parentheses, rather than naïvely
 // stripping away any starting or ending parenthesis characters—hence this
 // test of the JSON error format.
 #![feature(custom_inner_attributes)]
 #![feature(custom_inner_attributes)]
 #![rustfmt::skip]
 
 #![deny(clippy::unused_unit)]
 #![allow(dead_code)]
 #![allow(clippy::from_over_into)]
 struct Unitter;
 impl Unitter {
 impl Unitter {
     #[allow(clippy::no_effect)]
     pub fn get_unit<F: Fn(), G>(&self, f: F, _g: G)
     where G: Fn() {
         let _y: &dyn Fn() = &f;
         (); // this should not lint, as it's not in return type position
 }
 
 
 impl Into<()> for Unitter {
     #[rustfmt::skip]
     fn into(self) {
     }
 }
 
 trait Trait {
 trait Trait {
     fn redundant<F: FnOnce(), G, H>(&self, _f: F, _g: G, _h: H)
     where
         G: FnMut(),
         H: Fn();
 
 impl Trait for Unitter {
 impl Trait for Unitter {
     fn redundant<F: FnOnce(), G, H>(&self, _f: F, _g: G, _h: H)
     where
         G: FnMut(),
         H: Fn() {}
 
 fn return_unit() {  }
 
 
 #[allow(clippy::needless_return)]
 #[allow(clippy::never_loop)]
 #[allow(clippy::unit_cmp)]
 fn main() {
     let u = Unitter;
     assert_eq!(u.get_unit(|| {}, return_unit), u.into());
     return_unit();
     loop {
     }
     return;
 }
 
 
 // https://github.com/rust-lang/rust-clippy/issues/4076
 fn foo() {
     macro_rules! foo {
         (recv($r:expr) -> $res:pat => $body:expr) => {
             $body
     }
 
     foo! {
     foo! {
-        recv(rx) -> _x => ()
+        recv(rx) -> _x => 
 }
 
 
 #[rustfmt::skip]
 fn test(){}
 
 #[rustfmt::skip]
 fn test2(){}
 
 #[rustfmt::skip]
 fn test3(){}
 fn macro_expr() {
     macro_rules! e {
         () => (());
     }
     }
     e!()
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id.fixed
To only update this specific test, also pass `--test-args unused_unit.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_unit.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/unused_unit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":413,"byte_end":432,"line_start":12,"line_end":12,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![deny(clippy::unused_unit)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:28\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                            ^^^^^^ help: remove the `-> ()`\n   |\nnote: the lint level is defined here\n  --> tests/ui/unused_unit.rs:12:9\n   |\nLL | #![deny(clippy::unused_unit)]\n   |         ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:20:18\n   |\nLL |     where G: Fn() -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:58\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                                                          ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:21:26\n   |\nLL |         let _y: &dyn Fn() -> () = &f;\n   |                          ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:28:18\n   |\nLL |     fn into(self) -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:29:9\n   |\nLL |         ()\n   |         ^^ help: remove the final `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:34:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:36:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:37:16\n   |\nLL |         H: Fn() -> ();\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:41:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:43:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:44:16\n   |\nLL |         H: Fn() -> () {}\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:47:17\n   |\nLL | fn return_unit() -> () { () }\n   |                 ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:47:26\n   |\nLL | fn return_unit() -> () { () }\n   |                          ^^ help: remove the final `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:57:14\n   |\nLL |         break();\n   |              ^^ help: remove the `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:59:11\n   |\nLL |     return();\n   |           ^^ help: remove the `()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1663,"byte_end":1665,"line_start":71,"line_end":71,"column_start":27,"column_end":29,"is_primary":true,"text":[{"text":"        recv(rx) -> _x => ()","highlight_start":27,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1663,"byte_end":1665,"line_start":71,"line_end":71,"column_start":27,"column_end":29,"is_primary":true,"text":[{"text":"        recv(rx) -> _x => ()","highlight_start":27,"highlight_end":29}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:71:27\n   |\nLL |         recv(rx) -> _x => ()\n   |                           ^^ help: remove the final `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:76:10\n   |\nLL | fn test()->(){}\n   |          ^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:79:11\n   |\nLL | fn test2() ->(){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:82:11\n   |\nLL | fn test3()-> (){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
