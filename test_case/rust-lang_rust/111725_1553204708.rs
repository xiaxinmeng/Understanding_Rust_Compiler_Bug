plain

---- compile_test stdout ----
diff of stderr:

 error: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`
    |
 LL | /     if {
 LL | |         let x = 3;
 LL | |         x == 3
 LL | |         x == 3
 LL | |     } {
    | |_____^
    |
    = note: `-D clippy::blocks-in-if-conditions` implied by `-D warnings`
    |
 LL ~     let res = {
 LL +         let x = 3;
 LL +         x == 3
 LL +         x == 3
 LL ~     }; if res {
 
 error: omit braces around single expression condition
   --> $DIR/blocks_in_if_conditions.rs:35:8
    |
    |
 LL |     if { true } { 6 } else { 10 }
    |        ^^^^^^^^ help: try: `true`
-error: this boolean expression can be simplified
-  --> $DIR/blocks_in_if_conditions.rs:40:8
-   |
-   |
-LL |     if true && x == 3 { 6 } else { 10 }
-   |        ^^^^^^^^^^^^^^ help: try: `x == 3`
-   |
-   = note: `-D clippy::nonminimal-bool` implied by `-D warnings`
-error: aborting due to 3 previous errors
+error: aborting due to 2 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![warn(clippy::blocks_in_if_conditions)]
 #![allow(unused, clippy::let_and_return)]
 #![warn(clippy::nonminimal_bool)]
 
 macro_rules! blocky {
     () => {{ true }};
 
 
 macro_rules! blocky_too {
         let r = true;
         r
     }};
 }
 }
 
 fn macro_if() {
     if blocky!() {}
 
     if blocky_too!() {}
 
 fn condition_has_block() -> i32 {
     let res = {
         let x = 3;
         let x = 3;
         x == 3
     }; if res {
     } else {
         10
     }
 }
 }
 
 fn condition_has_block_with_single_expression() -> i32 {
     if true { 6 } else { 10 }
 
 fn condition_is_normal() -> i32 {
     let x = 3;
     let x = 3;
-    if x == 3 { 6 } else { 10 }
+    if true && x == 3 { 6 } else { 10 }
 
 fn condition_is_unsafe_block() {
     let a: i32 = 1;
 
 
     // this should not warn because the condition is an unsafe block
     if unsafe { 1u32 == std::mem::transmute(a) } {
         println!("1u32 == a");
 }
 
 fn block_in_assert() {
     let opt = Some(42);
     let opt = Some(42);
     assert!(
         opt.as_ref()
             .map(|val| {
                 let mut v = val * 2;
                 v -= 1;
                 v * 3
             .is_some()
     );
 }
 
 
 fn main() {}
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions.stage-id.fixed
To only update this specific test, also pass `--test-args blocks_in_if_conditions.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/blocks_in_if_conditions.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`","code":{"code":"clippy::blocks_in_if_conditions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/blocks_in_if_conditions.rs","byte_start":365,"byte_end":409,"line_start":24,"line_end":27,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if {","highlight_start":5,"highlight_end":9},{"text":"        let x = 3;","highlight_start":1,"highlight_end":19},{"text":"        x == 3","highlight_start":1,"highlight_end":15},{"text":"    } {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::blocks-in-if-conditions` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/blocks_in_if_conditions.rs","byte_start":365,"byte_end":409,"line_start":24,"line_end":27,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if {","highlight_start":5,"highlight_end":9},{"text":"        let x = 3;","highlight_start":1,"highlight_end":19},{"text":"        x == 3","highlight_start":1,"highlight_end":15},{"text":"    } {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"let res = {\n        let x = 3;\n        x == 3\n    }; if res","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`\n  --> tests/ui/blocks_in_if_conditions.rs:24:5\n   |\nLL | /     if {\nLL | |         let x = 3;\nLL | |         x == 3\nLL | |     } {\n   | |_____^\n   |\n   = note: `-D clippy::blocks-in-if-conditions` implied by `-D warnings`\nhelp: try\n   |\nLL ~     let res = {\nLL +         let x = 3;\nLL +         x == 3\nLL ~     }; if res {\n   |\n\n"}
{"message":"omit braces around single expression condition","code":{"code":"clippy::blocks_in_if_conditions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/blocks_in_if_conditions.rs","byte_start":519,"byte_end":527,"line_start":35,"line_end":35,"column_start":8,"column_end":16,"is_primary":true,"text":[{"text":"    if { true } { 6 } else { 10 }","highlight_start":8,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/blocks_in_if_conditions.rs","byte_start":519,"byte_end":527,"line_start":35,"line_end":35,"column_start":8,"column_end":16,"is_primary":true,"text":[{"text":"    if { true } { 6 } else { 10 }","highlight_start":8,"highlight_end":16}],"label":null,"suggested_replacement":"true","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: omit braces around single expression condition\n  --> tests/ui/blocks_in_if_conditions.rs:35:8\n   |\nLL |     if { true } { 6 } else { 10 }\n   |        ^^^^^^^^ help: try: `true`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`
-   |
-LL |               |x| {
-   |  _________________^
-LL | |                 let target = 3;
-LL | |                 let target = 3;
-LL | |                 x == target
-LL | |             },
-   | |_____________^
-   |
-   = note: `-D clippy::blocks-in-if-conditions` implied by `-D warnings`
-
-error: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`
    |
 LL |           |x| {
    |  _____________^
 LL | |             let target = 3;
 LL | |             let target = 3;
 LL | |             x == target
 LL | |         },
    | |_________^
+   |
+   = note: `-D clippy::blocks-in-if-conditions` implied by `-D warnings`
-error: aborting due to 2 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args blocks_in_if_conditions_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/blocks_in_if_conditions_closure.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions_closure.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/blocks_in_if_conditions_closure.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`","code":{"code":"clippy::blocks_in_if_conditions","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/blocks_in_if_conditions_closure.rs","byte_start":699,"byte_end":762,"line_start":27,"line_end":30,"column_start":13,"column_end":10,"is_primary":true,"text":[{"text":"        |x| {","highlight_start":13,"highlight_end":14},{"text":"            let target = 3;","highlight_start":1,"highlight_end":28},{"text":"            x == target","highlight_start":1,"highlight_end":24},{"text":"        },","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::blocks-in-if-conditions` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: in an `if` condition, avoid complex blocks or closures with blocks; instead, move the block or closure higher and bind it with a `let`\n  --> tests/ui/blocks_in_if_conditions_closure.rs:27:13\n   |\nLL |           |x| {\n   |  _____________^\nLL | |             let target = 3;\nLL | |             x == target\nLL | |         },\n   | |_________^\n   |\n   = note: `-D clippy::blocks-in-if-conditions` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: called `unwrap` on `x` after checking its variant with `is_ok`
-  --> $DIR/complex_conditionals.rs:8:9
+error: this call to `unwrap()` will always panic
    |
    |
-LL |     if x.is_ok() && y.is_err() {
-   |        --------- the check is happening here
-LL |         x.unwrap(); // unnecessary
+LL |     if x.is_ok() || y.is_ok() {
+   |        --------- because of this check
+...
+LL |         x.unwrap(); // will panic
    |
-   = help: try using `if let` or `match`
 note: the lint level is defined here
-  --> $DIR/complex_conditionals.rs:1:35
-  --> $DIR/complex_conditionals.rs:1:35
-   |
-LL | #![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]
-
-
-error: this call to `unwrap_err()` will always panic
-   |
-   |
-LL |     if x.is_ok() && y.is_err() {
-   |        --------- because of this check
-LL |         x.unwrap(); // unnecessary
-LL |         x.unwrap_err(); // will panic
-   |
-note: the lint level is defined here
   --> $DIR/complex_conditionals.rs:1:9
    |
    |
 LL | #![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]
 
 
-error: this call to `unwrap()` will always panic
-   |
-   |
-LL |     if x.is_ok() && y.is_err() {
-   |                     ---------- because of this check
-...
-LL |         y.unwrap(); // will panic
-
-
-error: called `unwrap_err` on `y` after checking its variant with `is_err`
-   |
-   |
-LL |     if x.is_ok() && y.is_err() {
-   |                     ---------- the check is happening here
-LL |         y.unwrap_err(); // unnecessary
-   |         ^^^^^^^^^^^^^^
-   |
-   = help: try using `if let` or `match`
-   = help: try using `if let` or `match`
-
-error: this call to `unwrap()` will always panic
-   |
-   |
-LL |     if x.is_ok() || y.is_ok() {
-   |        --------- because of this check
-...
-LL |         x.unwrap(); // will panic
-
-
 error: called `unwrap_err` on `x` after checking its variant with `is_ok`
    |
    |
 LL |     if x.is_ok() || y.is_ok() {
    |        --------- the check is happening here
 LL |         x.unwrap_err(); // unnecessary
    |         ^^^^^^^^^^^^^^
    |
    = help: try using `if let` or `match`
    = help: try using `if let` or `match`
+note: the lint level is defined here
+  --> $DIR/complex_conditionals.rs:1:35
+   |
+LL | #![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]
 
 
 error: this call to `unwrap()` will always panic
    |
    |
 LL |     if x.is_ok() || y.is_ok() {
    |                     --------- because of this check
 ...
 LL |         y.unwrap(); // will panic
 
 
 error: called `unwrap_err` on `y` after checking its variant with `is_ok`
    |
    |
 LL |     if x.is_ok() || y.is_ok() {
    |                     --------- the check is happening here
 LL |         y.unwrap_err(); // unnecessary
    |         ^^^^^^^^^^^^^^
    |
    = help: try using `if let` or `match`
    = help: try using `if let` or `match`
 
-error: called `unwrap` on `x` after checking its variant with `is_ok`
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |        --------- the check is happening here
-LL |         x.unwrap(); // unnecessary
-   |
-   = help: try using `if let` or `match`
-
-
-error: this call to `unwrap_err()` will always panic
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |        --------- because of this check
-LL |         x.unwrap(); // unnecessary
-LL |         x.unwrap_err(); // will panic
-
-
 error: this call to `unwrap()` will always panic
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |                       --------- because of this check
-...
-LL |         y.unwrap(); // will panic
-
-
-error: called `unwrap_err` on `y` after checking its variant with `is_ok`
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |                       --------- the check is happening here
-LL |         y.unwrap_err(); // unnecessary
-   |         ^^^^^^^^^^^^^^
-   |
-   = help: try using `if let` or `match`
-   = help: try using `if let` or `match`
-
-error: called `unwrap` on `z` after checking its variant with `is_err`
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |                                    ---------- the check is happening here
-...
-LL |         z.unwrap(); // unnecessary
-   |
-   = help: try using `if let` or `match`
-
-
-error: this call to `unwrap_err()` will always panic
-   |
-   |
-LL |     if x.is_ok() && !(y.is_ok() || z.is_err()) {
-   |                                    ---------- because of this check
-...
-LL |         z.unwrap_err(); // will panic
-
-
-error: this call to `unwrap()` will always panic
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |        --------- because of this check
 ...
 LL |         x.unwrap(); // will panic
 
 
 error: called `unwrap_err` on `x` after checking its variant with `is_ok`
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |        --------- the check is happening here
 LL |         x.unwrap_err(); // unnecessary
    |         ^^^^^^^^^^^^^^
    |
    = help: try using `if let` or `match`
    = help: try using `if let` or `match`
 
 error: called `unwrap` on `y` after checking its variant with `is_ok`
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |                       --------- the check is happening here
 ...
 LL |         y.unwrap(); // unnecessary
    |
    = help: try using `if let` or `match`
 
 
 error: this call to `unwrap_err()` will always panic
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |                       --------- because of this check
 ...
 LL |         y.unwrap_err(); // will panic
 
 
 error: this call to `unwrap()` will always panic
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |                                    ---------- because of this check
 ...
 LL |         z.unwrap(); // will panic
 
 
 error: called `unwrap_err` on `z` after checking its variant with `is_err`
    |
    |
 LL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {
    |                                    ---------- the check is happening here
 ...
 LL |         z.unwrap_err(); // unnecessary
    |
    = help: try using `if let` or `match`
 
-error: aborting due to 20 previous errors
---
To only update this specific test, also pass `--test-args checked_unwrap/complex_conditionals.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/checked_unwrap/complex_conditionals.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/checked_unwrap/complex_conditionals.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/checked_unwrap/complex_conditionals.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this call to `unwrap()` will always panic","code":{"code":"clippy::panicking_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":633,"byte_end":642,"line_start":20,"line_end":20,"column_start":8,"column_end":17,"is_primary":false,"text":[{"text":"    if x.is_ok() || y.is_ok() {","highlight_start":8,"highlight_end":17}],"label":"because of this check","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":823,"byte_end":833,"line_start":25,"line_end":25,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        x.unwrap(); // will panic","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":8,"byte_end":32,"line_start":1,"line_end":1,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: this call to `unwrap()` will always panic\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:25:9\n   |\nLL |     if x.is_ok() || y.is_ok() {\n   |        --------- because of this check\n...\nLL |         x.unwrap(); // will panic\n   |         ^^^^^^^^^^\n   |\nnote: the lint level is defined here\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:1:9\n   |\nLL | #![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"called `unwrap_err` on `x` after checking its variant with `is_ok`","code":{"code":"clippy::unnecessary_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":633,"byte_end":642,"line_start":20,"line_end":20,"column_start":8,"column_end":17,"is_primary":false,"text":[{"text":"    if x.is_ok() || y.is_ok() {","highlight_start":8,"highlight_end":17}],"label":"the check is happening here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":857,"byte_end":871,"line_start":26,"line_end":26,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        x.unwrap_err(); // unnecessary","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `if let` or `match`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":34,"byte_end":60,"line_start":1,"line_end":1,"column_start":35,"column_end":61,"is_primary":true,"text":[{"text":"#![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]","highlight_start":35,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `unwrap_err` on `x` after checking its variant with `is_ok`\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:26:9\n   |\nLL |     if x.is_ok() || y.is_ok() {\n   |        --------- the check is happening here\n...\nLL |         x.unwrap_err(); // unnecessary\n   |         ^^^^^^^^^^^^^^\n   |\n   = help: try using `if let` or `match`\nnote: the lint level is defined here\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:1:35\n   |\nLL | #![deny(clippy::panicking_unwrap, clippy::unnecessary_unwrap)]\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"this call to `unwrap()` will always panic","code":{"code":"clippy::panicking_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":646,"byte_end":655,"line_start":20,"line_end":20,"column_start":21,"column_end":30,"is_primary":false,"text":[{"text":"    if x.is_ok() || y.is_ok() {","highlight_start":21,"highlight_end":30}],"label":"because of this check","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":896,"byte_end":906,"line_start":27,"line_end":27,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        y.unwrap(); // will panic","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this call to `unwrap()` will always panic\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:27:9\n   |\nLL |     if x.is_ok() || y.is_ok() {\n   |                     --------- because of this check\n...\nLL |         y.unwrap(); // will panic\n   |         ^^^^^^^^^^\n\n"}
{"message":"called `unwrap_err` on `y` after checking its variant with `is_ok`","code":{"code":"clippy::unnecessary_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":646,"byte_end":655,"line_start":20,"line_end":20,"column_start":21,"column_end":30,"is_primary":false,"text":[{"text":"    if x.is_ok() || y.is_ok() {","highlight_start":21,"highlight_end":30}],"label":"the check is happening here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":930,"byte_end":944,"line_start":28,"line_end":28,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        y.unwrap_err(); // unnecessary","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `if let` or `match`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `unwrap_err` on `y` after checking its variant with `is_ok`\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:28:9\n   |\nLL |     if x.is_ok() || y.is_ok() {\n   |                     --------- the check is happening here\n...\nLL |         y.unwrap_err(); // unnecessary\n   |         ^^^^^^^^^^^^^^\n   |\n   = help: try using `if let` or `match`\n\n"}
{"message":"this call to `unwrap()` will always panic","code":{"code":"clippy::panicking_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1284,"byte_end":1293,"line_start":39,"line_end":39,"column_start":8,"column_end":17,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":8,"highlight_end":17}],"label":"because of this check","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1511,"byte_end":1521,"line_start":45,"line_end":45,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        x.unwrap(); // will panic","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this call to `unwrap()` will always panic\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:45:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |        --------- because of this check\n...\nLL |         x.unwrap(); // will panic\n   |         ^^^^^^^^^^\n\n"}
{"message":"called `unwrap_err` on `x` after checking its variant with `is_ok`","code":{"code":"clippy::unnecessary_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1284,"byte_end":1293,"line_start":39,"line_end":39,"column_start":8,"column_end":17,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":8,"highlight_end":17}],"label":"the check is happening here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1545,"byte_end":1559,"line_start":46,"line_end":46,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        x.unwrap_err(); // unnecessary","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `if let` or `match`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `unwrap_err` on `x` after checking its variant with `is_ok`\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:46:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |        --------- the check is happening here\n...\nLL |         x.unwrap_err(); // unnecessary\n   |         ^^^^^^^^^^^^^^\n   |\n   = help: try using `if let` or `match`\n\n"}
{"message":"called `unwrap` on `y` after checking its variant with `is_ok`","code":{"code":"clippy::unnecessary_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1299,"byte_end":1308,"line_start":39,"line_end":39,"column_start":23,"column_end":32,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":23,"highlight_end":32}],"label":"the check is happening here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1584,"byte_end":1594,"line_start":47,"line_end":47,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        y.unwrap(); // unnecessary","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `if let` or `match`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `unwrap` on `y` after checking its variant with `is_ok`\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:47:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |                       --------- the check is happening here\n...\nLL |         y.unwrap(); // unnecessary\n   |         ^^^^^^^^^^\n   |\n   = help: try using `if let` or `match`\n\n"}
{"message":"this call to `unwrap_err()` will always panic","code":{"code":"clippy::panicking_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1299,"byte_end":1308,"line_start":39,"line_end":39,"column_start":23,"column_end":32,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":23,"highlight_end":32}],"label":"because of this check","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1619,"byte_end":1633,"line_start":48,"line_end":48,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        y.unwrap_err(); // will panic","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this call to `unwrap_err()` will always panic\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:48:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |                       --------- because of this check\n...\nLL |         y.unwrap_err(); // will panic\n   |         ^^^^^^^^^^^^^^\n\n"}
{"message":"this call to `unwrap()` will always panic","code":{"code":"clippy::panicking_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1312,"byte_end":1322,"line_start":39,"line_end":39,"column_start":36,"column_end":46,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":36,"highlight_end":46}],"label":"because of this check","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1657,"byte_end":1667,"line_start":49,"line_end":49,"column_start":9,"column_end":19,"is_primary":true,"text":[{"text":"        z.unwrap(); // will panic","highlight_start":9,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this call to `unwrap()` will always panic\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:49:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |                                    ---------- because of this check\n...\nLL |         z.unwrap(); // will panic\n   |         ^^^^^^^^^^\n\n"}
{"message":"called `unwrap_err` on `z` after checking its variant with `is_err`","code":{"code":"clippy::unnecessary_unwrap","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1312,"byte_end":1322,"line_start":39,"line_end":39,"column_start":36,"column_end":46,"is_primary":false,"text":[{"text":"    if x.is_ok() || !(y.is_ok() && z.is_err()) {","highlight_start":36,"highlight_end":46}],"label":"the check is happening here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/checked_unwrap/complex_conditionals.rs","byte_start":1691,"byte_end":1705,"line_start":50,"line_end":50,"column_start":9,"column_end":23,"is_primary":true,"text":[{"text":"        z.unwrap_err(); // unnecessary","highlight_start":9,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using `if let` or `match`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `unwrap_err` on `z` after checking its variant with `is_err`\n  --> tests/ui/checked_unwrap/complex_conditionals.rs:50:9\n   |\nLL |     if x.is_ok() || !(y.is_ok() && z.is_err()) {\n   |                                    ---------- the check is happening here\n...\nLL |         z.unwrap_err(); // unnecessary\n   |         ^^^^^^^^^^^^^^\n   |\n   = help: try using `if let` or `match`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:6:8
    |
 LL |     if x == y || x < y {
    |        ^^^^^^^^^^^^^^^ help: try: `x <= y`
    |
    = note: `-D clippy::double-comparisons` implied by `-D warnings`
 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:9:8
    |
    |
 LL |     if x < y || x == y {
    |        ^^^^^^^^^^^^^^^ help: try: `x <= y`
 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:12:8
    |
    |
 LL |     if x == y || x > y {
    |        ^^^^^^^^^^^^^^^ help: try: `x >= y`
 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:15:8
    |
    |
 LL |     if x > y || x == y {
    |        ^^^^^^^^^^^^^^^ help: try: `x >= y`
 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:18:8
    |
    |
 LL |     if x < y || x > y {
    |        ^^^^^^^^^^^^^^ help: try: `x != y`
 error: this binary expression can be simplified
   --> $DIR/double_comparison.rs:21:8
    |
    |
 LL |     if x > y || x < y {
    |        ^^^^^^^^^^^^^^ help: try: `x != y`
-error: this binary expression can be simplified
-  --> $DIR/double_comparison.rs:24:8
-   |
-   |
-LL |     if x <= y && x >= y {
-   |        ^^^^^^^^^^^^^^^^ help: try: `x == y`
-error: this binary expression can be simplified
-  --> $DIR/double_comparison.rs:27:8
-   |
-   |
-LL |     if x >= y && x <= y {
-   |        ^^^^^^^^^^^^^^^^ help: try: `x == y`
-error: aborting due to 8 previous errors
+error: aborting due to 6 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/double_comparison.stage-id.stderr
diff of fixed:

 //@run-rustfix
 fn main() {
     let x = 1;
     let y = 2;
     let y = 2;
     if x <= y {
     }
     }
     if x <= y {
     }
     }
     if x >= y {
     }
     }
     if x >= y {
     }
     }
     if x != y {
     }
     }
     if x != y {
     }
     }
-    if x == y {
+    if x <= y && x >= y {
     }
     }
-    if x == y {
+    if x >= y && x <= y {
     }
 }
 


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/double_comparison.stage-id.fixed
To only update this specific test, also pass `--test-args double_comparison.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/double_comparison.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/double_comparison.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/double_comparison.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":65,"byte_end":80,"line_start":6,"line_end":6,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x == y || x < y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::double-comparisons` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":65,"byte_end":80,"line_start":6,"line_end":6,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x == y || x < y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"x <= y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:6:8\n   |\nLL |     if x == y || x < y {\n   |        ^^^^^^^^^^^^^^^ help: try: `x <= y`\n   |\n   = note: `-D clippy::double-comparisons` implied by `-D warnings`\n\n"}
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":120,"byte_end":135,"line_start":9,"line_end":9,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x < y || x == y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":120,"byte_end":135,"line_start":9,"line_end":9,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x < y || x == y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"x <= y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:9:8\n   |\nLL |     if x < y || x == y {\n   |        ^^^^^^^^^^^^^^^ help: try: `x <= y`\n\n"}
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":175,"byte_end":190,"line_start":12,"line_end":12,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x == y || x > y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":175,"byte_end":190,"line_start":12,"line_end":12,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x == y || x > y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"x >= y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:12:8\n   |\nLL |     if x == y || x > y {\n   |        ^^^^^^^^^^^^^^^ help: try: `x >= y`\n\n"}
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":230,"byte_end":245,"line_start":15,"line_end":15,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x > y || x == y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":230,"byte_end":245,"line_start":15,"line_end":15,"column_start":8,"column_end":23,"is_primary":true,"text":[{"text":"    if x > y || x == y {","highlight_start":8,"highlight_end":23}],"label":null,"suggested_replacement":"x >= y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:15:8\n   |\nLL |     if x > y || x == y {\n   |        ^^^^^^^^^^^^^^^ help: try: `x >= y`\n\n"}
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":285,"byte_end":299,"line_start":18,"line_end":18,"column_start":8,"column_end":22,"is_primary":true,"text":[{"text":"    if x < y || x > y {","highlight_start":8,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":285,"byte_end":299,"line_start":18,"line_end":18,"column_start":8,"column_end":22,"is_primary":true,"text":[{"text":"    if x < y || x > y {","highlight_start":8,"highlight_end":22}],"label":null,"suggested_replacement":"x != y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:18:8\n   |\nLL |     if x < y || x > y {\n   |        ^^^^^^^^^^^^^^ help: try: `x != y`\n\n"}
{"message":"this binary expression can be simplified","code":{"code":"clippy::double_comparisons","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":339,"byte_end":353,"line_start":21,"line_end":21,"column_start":8,"column_end":22,"is_primary":true,"text":[{"text":"    if x > y || x < y {","highlight_start":8,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/double_comparison.rs","byte_start":339,"byte_end":353,"line_start":21,"line_end":21,"column_start":8,"column_end":22,"is_primary":true,"text":[{"text":"    if x > y || x < y {","highlight_start":8,"highlight_end":22}],"label":null,"suggested_replacement":"x != y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this binary expression can be simplified\n  --> tests/ui/double_comparison.rs:21:8\n   |\nLL |     if x > y || x < y {\n   |        ^^^^^^^^^^^^^^ help: try: `x != y`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: variables in the condition are not mutated in the loop body
    |
    |
 LL |     while y < 10 {
    |
    = note: this may lead to an infinite or to a never running loop
    = note: this may lead to an infinite or to a never running loop
    = note: `#[deny(clippy::while_immutable_condition)]` on by default
 
 error: variables in the condition are not mutated in the loop body
-   |
-   |
-LL |     while y < 10 && x < 3 {
-   |
-   = note: this may lead to an infinite or to a never running loop
-
-
-error: variables in the condition are not mutated in the loop body
    |
 LL |     while !cond {
    |           ^^^^^
    |
    |
    = note: this may lead to an infinite or to a never running loop
 
 error: variables in the condition are not mutated in the loop body
    |
 LL |     while i < 3 {
    |           ^^^^^
    |
    |
    = note: this may lead to an infinite or to a never running loop
 
 error: variables in the condition are not mutated in the loop body
-   |
-   |
-LL |     while i < 3 && j > 0 {
-   |
-   = note: this may lead to an infinite or to a never running loop
-
-
-error: variables in the condition are not mutated in the loop body
    |
 LL |     while i < 3 {
    |           ^^^^^
    |
    |
    = note: this may lead to an infinite or to a never running loop
 
 error: variables in the condition are not mutated in the loop body
    |
 LL |     while i < 3 {
    |           ^^^^^
    |
    |
    = note: this may lead to an infinite or to a never running loop
 
 error: variables in the condition are not mutated in the loop body
    |
 LL |     while i < 3 {
    |           ^^^^^
    |
    |
    = note: this may lead to an infinite or to a never running loop
 
 error: variables in the condition are not mutated in the loop body
    |
    |
 LL |         while self.count < n {
    |
    = note: this may lead to an infinite or to a never running loop
 
 
 error: variables in the condition are not mutated in the loop body
    |
    |
 LL |     while y < 10 {
    |
    = note: this may lead to an infinite or to a never running loop
    = note: this may lead to an infinite or to a never running loop
    = note: this loop contains `return`s or `break`s
    = help: rewrite it as `if cond { loop { } }`
 
 error: variables in the condition are not mutated in the loop body
    |
    |
 LL |     while y < 10 {
Build completed unsuccessfully in 0:02:02
    |           ^^^^^^
    |
    = note: this may lead to an infinite or to a never running loop
    = note: this may lead to an infinite or to a never running loop
    = note: this loop contains `return`s or `break`s
    = help: rewrite it as `if cond { loop { } }`
-error: aborting due to 11 previous errors
+error: aborting due to 9 previous errors
 
 
---
To only update this specific test, also pass `--test-args infinite_loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/infinite_loop.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/infinite_loop.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/infinite_loop.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":352,"byte_end":358,"line_start":20,"line_end":20,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    while y < 10 {","highlight_start":11,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"`#[deny(clippy::while_immutable_condition)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:20:11\n   |\nLL |     while y < 10 {\n   |           ^^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n   = note: `#[deny(clippy::while_immutable_condition)]` on by default\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":574,"byte_end":579,"line_start":32,"line_end":32,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    while !cond {","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:32:11\n   |\nLL |     while !cond {\n   |           ^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":1441,"byte_end":1446,"line_start":76,"line_end":76,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    while i < 3 {","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:76:11\n   |\nLL |     while i < 3 {\n   |           ^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":1605,"byte_end":1610,"line_start":85,"line_end":85,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    while i < 3 {","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:85:11\n   |\nLL |     while i < 3 {\n   |           ^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":1858,"byte_end":1863,"line_start":100,"line_end":100,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    while i < 3 {","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:100:11\n   |\nLL |     while i < 3 {\n   |           ^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":1950,"byte_end":1955,"line_start":105,"line_end":105,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"    while i < 3 {","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:105:11\n   |\nLL |     while i < 3 {\n   |           ^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":3119,"byte_end":3133,"line_start":171,"line_end":171,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"        while self.count < n {","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:171:15\n   |\nLL |         while self.count < n {\n   |               ^^^^^^^^^^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":3280,"byte_end":3286,"line_start":179,"line_end":179,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    while y < 10 {","highlight_start":11,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this loop contains `return`s or `break`s","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"rewrite it as `if cond { loop { } }`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:179:11\n   |\nLL |     while y < 10 {\n   |           ^^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n   = note: this loop contains `return`s or `break`s\n   = help: rewrite it as `if cond { loop { } }`\n\n"}
{"message":"variables in the condition are not mutated in the loop body","code":{"code":"clippy::while_immutable_condition","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/infinite_loop.rs","byte_start":3401,"byte_end":3407,"line_start":186,"line_end":186,"column_start":11,"column_end":17,"is_primary":true,"text":[{"text":"    while y < 10 {","highlight_start":11,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this may lead to an infinite or to a never running loop","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this loop contains `return`s or `break`s","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"rewrite it as `if cond { loop { } }`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: variables in the condition are not mutated in the loop body\n  --> tests/ui/infinite_loop.rs:186:11\n   |\nLL |     while y < 10 {\n   |           ^^^^^^\n   |\n   = note: this may lead to an infinite or to a never running loop\n   = note: this loop contains `return`s or `break`s\n   = help: rewrite it as `if cond { loop { } }`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:41:5
    |
 LL | /     if x {
 LL | |         true
 LL | |     } else {
 LL | |         false
 LL | |     };
    | |_____^ help: you can reduce it to: `x`
    |
    = note: `-D clippy::needless-bool` implied by `-D warnings`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:46:5
    |
 LL | /     if x {
 LL | /     if x {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `!x`
 error: this if-then-else expression returns a bool literal
-  --> $DIR/fixable.rs:51:5
-   |
-   |
-LL | /     if x && y {
-LL | |         false
-LL | |     } else {
-LL | |         true
-LL | |     };
-   | |_____^ help: you can reduce it to: `!(x && y)`
-error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:59:5
    |
    |
 LL | /     if a == b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a != b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:64:5
    |
    |
 LL | /     if a != b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a == b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:69:5
    |
    |
 LL | /     if a < b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a >= b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:74:5
    |
    |
 LL | /     if a <= b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a > b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:79:5
    |
    |
 LL | /     if a > b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a <= b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:84:5
    |
    |
 LL | /     if a >= b {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
    | |_____^ help: you can reduce it to: `a < b`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:105:5
    |
 LL | /     if x {
 LL | /     if x {
 LL | |         return true;
 LL | |     } else {
 LL | |         return false;
 LL | |     };
    | |_____^ help: you can reduce it to: `return x`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:113:5
    |
 LL | /     if x {
 LL | /     if x {
 LL | |         return false;
 LL | |     } else {
 LL | |         return true;
 LL | |     };
    | |_____^ help: you can reduce it to: `return !x`
-error: this if-then-else expression returns a bool literal
-  --> $DIR/fixable.rs:121:5
-   |
-   |
-LL | /     if x && y {
-LL | |         return true;
-LL | |     } else {
-LL | |         return false;
-LL | |     };
-   | |_____^ help: you can reduce it to: `return x && y`
-error: this if-then-else expression returns a bool literal
-  --> $DIR/fixable.rs:129:5
-   |
-   |
-LL | /     if x && y {
-LL | |         return false;
-LL | |     } else {
-LL | |         return true;
-LL | |     };
-   | |_____^ help: you can reduce it to: `return !(x && y)`
-
 error: equality checks against true are unnecessary
    |
    |
 LL |     if x == true {};
    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
    |
    = note: `-D clippy::bool-comparison` implied by `-D warnings`
 
 error: equality checks against false can be replaced by a negation
    |
    |
 LL |     if x == false {};
    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
 
 error: equality checks against true are unnecessary
    |
    |
 LL |     if x == true {};
    |        ^^^^^^^^^ help: try simplifying it as shown: `x`
 
 error: equality checks against false can be replaced by a negation
    |
    |
 LL |     if x == false {};
    |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:161:12
    |
    |
 LL |       } else if returns_bool() {
 LL | |         false
 LL | |     } else {
 LL | |         true
 LL | |     };
 LL | |     };
    | |_____^ help: you can reduce it to: `{ !returns_bool() }`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:174:5
    |
    |
 LL | /     if unsafe { no(4) } & 1 != 0 {
 LL | |         true
 LL | |     } else {
 LL | |         false
 LL | |     };
    | |_____^ help: you can reduce it to: `(unsafe { no(4) } & 1 != 0)`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:179:30
    |
    |
 LL |     let _brackets_unneeded = if unsafe { no(4) } & 1 != 0 { true } else { false };
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: you can reduce it to: `unsafe { no(4) } & 1 != 0`
 error: this if-then-else expression returns a bool literal
   --> $DIR/fixable.rs:182:9
    |
    |
 LL |         if unsafe { no(4) } & 1 != 0 { true } else { false }
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: you can reduce it to: `(unsafe { no(4) } & 1 != 0)`
-error: aborting due to 21 previous errors
+error: aborting due to 18 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool/fixable.stage-id.stderr
diff of fixed:

 //@run-rustfix
 
 #![warn(clippy::needless_bool)]
     unused,
     dead_code,
     clippy::no_effect,
     clippy::no_effect,
     clippy::if_same_then_else,
     clippy::equatable_if_let,
     clippy::needless_return,
     clippy::self_named_constructors
 
 use std::cell::Cell;
 
 macro_rules! bool_comparison_trigger {
 macro_rules! bool_comparison_trigger {
     ($($i:ident: $def:expr, $stb:expr );+  $(;)*) => (
         #[derive(Clone)]
         pub struct Trigger {
         pub struct Trigger {
             $($i: (Cell<bool>, bool, bool)),+
 
         #[allow(dead_code)]
         impl Trigger {
         impl Trigger {
             pub fn trigger(&self, key: &str) -> bool {
                 $(
                     if let stringify!($i) = key {
                         return self.$i.1 && self.$i.2 == $def;
                  )+
                 false
             }
         }
         }
     )
 }
 
 fn main() {
     let x = true;
     let y = false;
     x;
     !x;
-    !(x && y);
+    if x && y {
+    } else {
+        true
+    };
     let a = 0;
     let a = 0;
     let b = 1;
 
     a != b;
     a == b;
     a >= b;
     a > b;
     a <= b;
     a < b;
     if x {
     } else {
         false
         false
     }; // would also be questionable, but we don't catch this yet
     bool_ret3(x);
     bool_ret4(x);
     bool_ret5(x, x);
     bool_ret6(x, x);
     needless_bool(x);
     needless_bool2(x);
     needless_bool3(x);
     needless_bool_condition();
 
 
 fn bool_ret3(x: bool) -> bool {
     return x;
 
 
 fn bool_ret4(x: bool) -> bool {
     return !x;
 
 
 fn bool_ret5(x: bool, y: bool) -> bool {
-    return x && y;
+    if x && y {
+    } else {
+        return false;
+    };
 }
 }
 
 fn bool_ret6(x: bool, y: bool) -> bool {
-    return !(x && y);
+    if x && y {
+    } else {
+        return true;
+    };
 }
 }
 
 fn needless_bool(x: bool) {
     if x {};
 
 
 fn needless_bool2(x: bool) {
     if !x {};
 
 
 fn needless_bool3(x: bool) {
     bool_comparison_trigger! {
         test_one:   false, false;
         test_three: false, false;
         test_two:   true, true;
 
 
     if x {};
     if !x {};
 
 
 fn needless_bool_in_the_suggestion_wraps_the_predicate_of_if_else_statement_in_brackets() {
     let b = false;
     let returns_bool = || false;
     let x = if b {
         true
         true
     } else { !returns_bool() };
 
 
 unsafe fn no(v: u8) -> u8 {
 }
 
 
 #[allow(clippy::unnecessary_operation)]
 fn needless_bool_condition() -> bool {
     (unsafe { no(4) } & 1 != 0);
     let _brackets_unneeded = unsafe { no(4) } & 1 != 0;
     fn foo() -> bool {
         // parentheses are needed here
         (unsafe { no(4) } & 1 != 0)
 
     foo()
 }
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool/fixable.stage-id.fixed
To only update this specific test, also pass `--test-args needless_bool/fixable.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_bool/fixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool/fixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool/fixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":854,"byte_end":906,"line_start":41,"line_end":45,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":854,"byte_end":906,"line_start":41,"line_end":45,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:41:5\n   |\nLL | /     if x {\nLL | |         true\nLL | |     } else {\nLL | |         false\nLL | |     };\n   | |_____^ help: you can reduce it to: `x`\n   |\n   = note: `-D clippy::needless-bool` implied by `-D warnings`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":912,"byte_end":964,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":912,"byte_end":964,"line_start":46,"line_end":50,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:46:5\n   |\nLL | /     if x {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `!x`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1064,"byte_end":1121,"line_start":59,"line_end":63,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a == b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1064,"byte_end":1121,"line_start":59,"line_end":63,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a == b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a != b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:59:5\n   |\nLL | /     if a == b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a != b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1127,"byte_end":1184,"line_start":64,"line_end":68,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a != b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1127,"byte_end":1184,"line_start":64,"line_end":68,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a != b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a == b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:64:5\n   |\nLL | /     if a != b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a == b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1190,"byte_end":1246,"line_start":69,"line_end":73,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a < b {","highlight_start":5,"highlight_end":15},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1190,"byte_end":1246,"line_start":69,"line_end":73,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a < b {","highlight_start":5,"highlight_end":15},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a >= b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:69:5\n   |\nLL | /     if a < b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a >= b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1252,"byte_end":1309,"line_start":74,"line_end":78,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a <= b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1252,"byte_end":1309,"line_start":74,"line_end":78,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a <= b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a > b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:74:5\n   |\nLL | /     if a <= b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a > b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1315,"byte_end":1371,"line_start":79,"line_end":83,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a > b {","highlight_start":5,"highlight_end":15},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1315,"byte_end":1371,"line_start":79,"line_end":83,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a > b {","highlight_start":5,"highlight_end":15},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a <= b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:79:5\n   |\nLL | /     if a > b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a <= b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1377,"byte_end":1434,"line_start":84,"line_end":88,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a >= b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1377,"byte_end":1434,"line_start":84,"line_end":88,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if a >= b {","highlight_start":5,"highlight_end":16},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"a < b","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:84:5\n   |\nLL | /     if a >= b {\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `a < b`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1766,"byte_end":1834,"line_start":105,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1766,"byte_end":1834,"line_start":105,"line_end":109,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:105:5\n   |\nLL | /     if x {\nLL | |         return true;\nLL | |     } else {\nLL | |         return false;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return x`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1875,"byte_end":1943,"line_start":113,"line_end":117,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":1875,"byte_end":1943,"line_start":113,"line_end":117,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x {","highlight_start":5,"highlight_end":11},{"text":"        return false;","highlight_start":1,"highlight_end":22},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        return true;","highlight_start":1,"highlight_end":21},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"return !x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:113:5\n   |\nLL | /     if x {\nLL | |         return false;\nLL | |     } else {\nLL | |         return true;\nLL | |     };\n   | |_____^ help: you can reduce it to: `return !x`\n\n"}
{"message":"equality checks against true are unnecessary","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2229,"byte_end":2238,"line_start":137,"line_end":137,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::bool-comparison` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2229,"byte_end":2238,"line_start":137,"line_end":137,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against true are unnecessary\n  --> tests/ui/needless_bool/fixable.rs:137:8\n   |\nLL |     if x == true {};\n   |        ^^^^^^^^^ help: try simplifying it as shown: `x`\n   |\n   = note: `-D clippy::bool-comparison` implied by `-D warnings`\n\n"}
{"message":"equality checks against false can be replaced by a negation","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2282,"byte_end":2292,"line_start":141,"line_end":141,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2282,"byte_end":2292,"line_start":141,"line_end":141,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against false can be replaced by a negation\n  --> tests/ui/needless_bool/fixable.rs:141:8\n   |\nLL |     if x == false {};\n   |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`\n\n"}
{"message":"equality checks against true are unnecessary","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2474,"byte_end":2483,"line_start":151,"line_end":151,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2474,"byte_end":2483,"line_start":151,"line_end":151,"column_start":8,"column_end":17,"is_primary":true,"text":[{"text":"    if x == true {};","highlight_start":8,"highlight_end":17}],"label":null,"suggested_replacement":"x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against true are unnecessary\n  --> tests/ui/needless_bool/fixable.rs:151:8\n   |\nLL |     if x == true {};\n   |        ^^^^^^^^^ help: try simplifying it as shown: `x`\n\n"}
{"message":"equality checks against false can be replaced by a negation","code":{"code":"clippy::bool_comparison","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2495,"byte_end":2505,"line_start":152,"line_end":152,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try simplifying it as shown","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2495,"byte_end":2505,"line_start":152,"line_end":152,"column_start":8,"column_end":18,"is_primary":true,"text":[{"text":"    if x == false {};","highlight_start":8,"highlight_end":18}],"label":null,"suggested_replacement":"!x","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: equality checks against false can be replaced by a negation\n  --> tests/ui/needless_bool/fixable.rs:152:8\n   |\nLL |     if x == false {};\n   |        ^^^^^^^^^^ help: try simplifying it as shown: `!x`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2701,"byte_end":2766,"line_start":161,"line_end":165,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else if returns_bool() {","highlight_start":12,"highlight_end":31},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2701,"byte_end":2766,"line_start":161,"line_end":165,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else if returns_bool() {","highlight_start":12,"highlight_end":31},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"{ !returns_bool() }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:161:12\n   |\nLL |       } else if returns_bool() {\n   |  ____________^\nLL | |         false\nLL | |     } else {\nLL | |         true\nLL | |     };\n   | |_____^ help: you can reduce it to: `{ !returns_bool() }`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2891,"byte_end":2967,"line_start":174,"line_end":178,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if unsafe { no(4) } & 1 != 0 {","highlight_start":5,"highlight_end":35},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2891,"byte_end":2967,"line_start":174,"line_end":178,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if unsafe { no(4) } & 1 != 0 {","highlight_start":5,"highlight_end":35},{"text":"        true","highlight_start":1,"highlight_end":13},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        false","highlight_start":1,"highlight_end":14},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"(unsafe { no(4) } & 1 != 0)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:174:5\n   |\nLL | /     if unsafe { no(4) } & 1 != 0 {\nLL | |         true\nLL | |     } else {\nLL | |         false\nLL | |     };\n   | |_____^ help: you can reduce it to: `(unsafe { no(4) } & 1 != 0)`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2998,"byte_end":3050,"line_start":179,"line_end":179,"column_start":30,"column_end":82,"is_primary":true,"text":[{"text":"    let _brackets_unneeded = if unsafe { no(4) } & 1 != 0 { true } else { false };","highlight_start":30,"highlight_end":82}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":2998,"byte_end":3050,"line_start":179,"line_end":179,"column_start":30,"column_end":82,"is_primary":true,"text":[{"text":"    let _brackets_unneeded = if unsafe { no(4) } & 1 != 0 { true } else { false };","highlight_start":30,"highlight_end":82}],"label":null,"suggested_replacement":"unsafe { no(4) } & 1 != 0","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:179:30\n   |\nLL |     let _brackets_unneeded = if unsafe { no(4) } & 1 != 0 { true } else { false };\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: you can reduce it to: `unsafe { no(4) } & 1 != 0`\n\n"}
{"message":"this if-then-else expression returns a bool literal","code":{"code":"clippy::needless_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":3122,"byte_end":3174,"line_start":182,"line_end":182,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        if unsafe { no(4) } & 1 != 0 { true } else { false }","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool/fixable.rs","byte_start":3122,"byte_end":3174,"line_start":182,"line_end":182,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        if unsafe { no(4) } & 1 != 0 { true } else { false }","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":"(unsafe { no(4) } & 1 != 0)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression returns a bool literal\n  --> tests/ui/needless_bool/fixable.rs:182:9\n   |\nLL |         if unsafe { no(4) } & 1 != 0 { true } else { false }\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: you can reduce it to: `(unsafe { no(4) } & 1 != 0)`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this if-then-else expression assigns a bool literal
-  --> $DIR/needless_bool_assign.rs:15:5
-   |
-LL | /     if random() && random() {
-LL | |         a.field = true;
-LL | |     } else {
-LL | |         a.field = false
-LL | |     }
-   | |_____^ help: you can reduce it to: `a.field = random() && random();`
-   |
-   = note: `-D clippy::needless-bool-assign` implied by `-D warnings`
-
-error: this if-then-else expression assigns a bool literal
-  --> $DIR/needless_bool_assign.rs:20:5
-   |
-LL | /     if random() && random() {
-LL | |         a.field = false;
-LL | |     } else {
-LL | |         a.field = true
-LL | |     }
-   | |_____^ help: you can reduce it to: `a.field = !(random() && random());`
-
-error: this if-then-else expression assigns a bool literal
   --> $DIR/needless_bool_assign.rs:34:5
    |
 LL | /     if random() {
 LL | |         a.field = true;
 LL | |     } else {
 LL | |         a.field = true;
 LL | |     }
    | |_____^ help: you can reduce it to: `random(); a.field = true;`
+   |
+   = note: `-D clippy::needless-bool-assign` implied by `-D warnings`
 error: this `if` has identical blocks
   --> $DIR/needless_bool_assign.rs:34:17
    |
    |
 LL |       if random() {
 LL | |         a.field = true;
 LL | |     } else {
    | |_____^
    |
---
    |  ____________^
 LL | |         a.field = true;
 LL | |     }
    | |_____^
    = note: `#[deny(clippy::if_same_then_else)]` on by default
-error: aborting due to 4 previous errors
+error: aborting due to 2 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool_assign.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![allow(unused)]
 #![allow(unused)]
 #![warn(clippy::needless_bool_assign)]
 fn random() -> bool {
     true
 }
 
 
 fn main() {
     struct Data {
         field: bool,
     };
     let mut a = Data { field: false };
-    a.field = random() && random();
-    a.field = !(random() && random());
+    if random() && random() {
+        a.field = true;
+        a.field = false
+    }
+    }
+    if random() && random() {
+        a.field = false;
+        a.field = true
+    }
     // Do not lint…
     // Do not lint…
     if random() {
         a.field = false;
         // …to avoid losing this comment
         a.field = true
     }
     }
     // This one also triggers lint `clippy::if_same_then_else`
     // which does not suggest a rewrite.
     random(); a.field = true;
     let mut b = false;
     if random() {
         a.field = false;
         b = true;
     }
 }
 
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool_assign.stage-id.fixed
To only update this specific test, also pass `--test-args needless_bool_assign.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/needless_bool_assign.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool_assign.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_bool_assign.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this if-then-else expression assigns a bool literal","code":{"code":"clippy::needless_bool_assign","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool_assign.rs","byte_start":657,"byte_end":737,"line_start":34,"line_end":38,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if random() {","highlight_start":5,"highlight_end":18},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::needless-bool-assign` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"you can reduce it to","code":null,"level":"help","spans":[{"file_name":"tests/ui/needless_bool_assign.rs","byte_start":657,"byte_end":737,"line_start":34,"line_end":38,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if random() {","highlight_start":5,"highlight_end":18},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"random(); a.field = true;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this if-then-else expression assigns a bool literal\n  --> tests/ui/needless_bool_assign.rs:34:5\n   |\nLL | /     if random() {\nLL | |         a.field = true;\nLL | |     } else {\nLL | |         a.field = true;\nLL | |     }\n   | |_____^ help: you can reduce it to: `random(); a.field = true;`\n   |\n   = note: `-D clippy::needless-bool-assign` implied by `-D warnings`\n\n"}
{"message":"this `if` has identical blocks","code":{"code":"clippy::if_same_then_else","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/needless_bool_assign.rs","byte_start":669,"byte_end":700,"line_start":34,"line_end":36,"column_start":17,"column_end":6,"is_primary":true,"text":[{"text":"    if random() {","highlight_start":17,"highlight_end":18},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    } else {","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"same as this","code":null,"level":"note","spans":[{"file_name":"tests/ui/needless_bool_assign.rs","byte_start":706,"byte_end":737,"line_start":36,"line_end":38,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        a.field = true;","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`#[deny(clippy::if_same_then_else)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this `if` has identical blocks\n  --> tests/ui/needless_bool_assign.rs:34:17\n   |\nLL |       if random() {\n   |  _________________^\nLL | |         a.field = true;\nLL | |     } else {\n   | |_____^\n   |\nnote: same as this\n  --> tests/ui/needless_bool_assign.rs:36:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         a.field = true;\nLL | |     }\n   | |_____^\n   = note: `#[deny(clippy::if_same_then_else)]` on by default\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:11:13
    |
 LL |     let _ = !true;
    |             ^^^^^ help: try: `false`
    |
    = note: `-D clippy::nonminimal-bool` implied by `-D warnings`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:12:13
    |
    |
 LL |     let _ = !false;
    |             ^^^^^^ help: try: `true`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:13:13
    |
 LL |     let _ = !!a;
 LL |     let _ = !!a;
    |             ^^^ help: try: `a`
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:14:13
    |
 LL |     let _ = false || a;
    |             ^^^^^^^^^^ help: try: `a`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:18:13
    |
    |
 LL |     let _ = !(!a && b);
    |             ^^^^^^^^^^ help: try: `a || !b`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:19:13
    |
    |
 LL |     let _ = !(!a || b);
    |             ^^^^^^^^^^ help: try: `a && !b`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:20:13
    |
    |
 LL |     let _ = !a && !(b && c);
    |             ^^^^^^^^^^^^^^^ help: try: `!(a || b && c)`
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:28:13
    |
    |
 LL |     let _ = a == b && c == 5 && a == b;
    |
 help: try
    |
    |
 LL |     let _ = !(a != b || c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
 LL |     let _ = a == b && c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:29:13
    |
    |
 LL |     let _ = a == b || c == 5 || a == b;
    |
 help: try
    |
    |
 LL |     let _ = !(a != b && c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
 LL |     let _ = a == b || c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:30:13
    |
    |
 LL |     let _ = a == b && c == 5 && b == a;
    |
 help: try
    |
    |
 LL |     let _ = !(a != b || c != 5);
    |             ~~~~~~~~~~~~~~~~~~~
 LL |     let _ = a == b && c == 5;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:31:13
    |
    |
 LL |     let _ = a != b || !(a != b || c == d);
    |
 help: try
    |
    |
 LL |     let _ = !(a == b && c == d);
    |             ~~~~~~~~~~~~~~~~~~~
 LL |     let _ = a != b || c != d;
 
 error: this boolean expression can be simplified
   --> $DIR/nonminimal_bool.rs:32:13
    |
    |
 LL |     let _ = a != b && !(a != b && c == d);
    |
 help: try
    |
    |
 LL |     let _ = !(a == b || c == d);
    |             ~~~~~~~~~~~~~~~~~~~
 LL |     let _ = a != b && c != d;
 
-error: this boolean expression can be simplified
-  --> $DIR/nonminimal_bool.rs:62:8
-   |
-   |
-LL |     if matches!(true, true) && true {
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `matches!(true, true)`
-error: aborting due to 13 previous errors
+error: aborting due to 12 previous errors
 
 
---
To only update this specific test, also pass `--test-args nonminimal_bool.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/nonminimal_bool.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/nonminimal_bool.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/nonminimal_bool.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":317,"byte_end":322,"line_start":11,"line_end":11,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = !true;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::nonminimal-bool` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":317,"byte_end":322,"line_start":11,"line_end":11,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"    let _ = !true;","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":"false","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:11:13\n   |\nLL |     let _ = !true;\n   |             ^^^^^ help: try: `false`\n   |\n   = note: `-D clippy::nonminimal-bool` implied by `-D warnings`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":336,"byte_end":342,"line_start":12,"line_end":12,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = !false;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":336,"byte_end":342,"line_start":12,"line_end":12,"column_start":13,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = !false;","highlight_start":13,"highlight_end":19}],"label":null,"suggested_replacement":"true","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:12:13\n   |\nLL |     let _ = !false;\n   |             ^^^^^^ help: try: `true`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":356,"byte_end":359,"line_start":13,"line_end":13,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let _ = !!a;","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":356,"byte_end":359,"line_start":13,"line_end":13,"column_start":13,"column_end":16,"is_primary":true,"text":[{"text":"    let _ = !!a;","highlight_start":13,"highlight_end":16}],"label":null,"suggested_replacement":"a","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:13:13\n   |\nLL |     let _ = !!a;\n   |             ^^^ help: try: `a`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":373,"byte_end":383,"line_start":14,"line_end":14,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = false || a;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":373,"byte_end":383,"line_start":14,"line_end":14,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = false || a;","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:14:13\n   |\nLL |     let _ = false || a;\n   |             ^^^^^^^^^^ help: try: `a`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":509,"byte_end":519,"line_start":18,"line_end":18,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a && b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":509,"byte_end":519,"line_start":18,"line_end":18,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a && b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a || !b","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:18:13\n   |\nLL |     let _ = !(!a && b);\n   |             ^^^^^^^^^^ help: try: `a || !b`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":533,"byte_end":543,"line_start":19,"line_end":19,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a || b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":533,"byte_end":543,"line_start":19,"line_end":19,"column_start":13,"column_end":23,"is_primary":true,"text":[{"text":"    let _ = !(!a || b);","highlight_start":13,"highlight_end":23}],"label":null,"suggested_replacement":"a && !b","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:19:13\n   |\nLL |     let _ = !(!a || b);\n   |             ^^^^^^^^^^ help: try: `a && !b`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":557,"byte_end":572,"line_start":20,"line_end":20,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = !a && !(b && c);","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":557,"byte_end":572,"line_start":20,"line_end":20,"column_start":13,"column_end":28,"is_primary":true,"text":[{"text":"    let _ = !a && !(b && c);","highlight_start":13,"highlight_end":28}],"label":null,"suggested_replacement":"!(a || b && c)","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:20:13\n   |\nLL |     let _ = !a && !(b && c);\n   |             ^^^^^^^^^^^^^^^ help: try: `!(a || b && c)`\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":751,"byte_end":777,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":751,"byte_end":777,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b || c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":751,"byte_end":777,"line_start":28,"line_end":28,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b && c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:28:13\n   |\nLL |     let _ = a == b && c == 5 && a == b;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b || c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b && c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":791,"byte_end":817,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":791,"byte_end":817,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b && c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":791,"byte_end":817,"line_start":29,"line_end":29,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b || c == 5 || a == b;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b || c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:29:13\n   |\nLL |     let _ = a == b || c == 5 || a == b;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b && c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b || c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":831,"byte_end":857,"line_start":30,"line_end":30,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":831,"byte_end":857,"line_start":30,"line_end":30,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"!(a != b || c != 5)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":831,"byte_end":857,"line_start":30,"line_end":30,"column_start":13,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = a == b && c == 5 && b == a;","highlight_start":13,"highlight_end":39}],"label":null,"suggested_replacement":"a == b && c == 5","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:30:13\n   |\nLL |     let _ = a == b && c == 5 && b == a;\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a != b || c != 5);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a == b && c == 5;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":871,"byte_end":900,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":871,"byte_end":900,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"!(a == b && c == d)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":871,"byte_end":900,"line_start":31,"line_end":31,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b || !(a != b || c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"a != b || c != d","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:31:13\n   |\nLL |     let _ = a != b || !(a != b || c == d);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a == b && c == d);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a != b || c != d;\n   |             ~~~~~~~~~~~~~~~~\n\n"}
{"message":"this boolean expression can be simplified","code":{"code":"clippy::nonminimal_bool","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":914,"byte_end":943,"line_start":32,"line_end":32,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":914,"byte_end":943,"line_start":32,"line_end":32,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"!(a == b || c == d)","suggestion_applicability":"Unspecified","expansion":null},{"file_name":"tests/ui/nonminimal_bool.rs","byte_start":914,"byte_end":943,"line_start":32,"line_end":32,"column_start":13,"column_end":42,"is_primary":true,"text":[{"text":"    let _ = a != b && !(a != b && c == d);","highlight_start":13,"highlight_end":42}],"label":null,"suggested_replacement":"a != b && c != d","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this boolean expression can be simplified\n  --> tests/ui/nonminimal_bool.rs:32:13\n   |\nLL |     let _ = a != b && !(a != b && c == d);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try\n   |\nLL |     let _ = !(a == b || c == d);\n   |             ~~~~~~~~~~~~~~~~~~~\nLL |     let _ = a != b && c != d;\n   |             ~~~~~~~~~~~~~~~~\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
