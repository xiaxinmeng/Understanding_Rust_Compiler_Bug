plain

 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:9:5
    |
 LL | /     if x == "hello" {
 LL | |         if y == "world" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
    = note: `-D clippy::collapsible-if` implied by `-D warnings`
 help: collapse nested if block
    |
 LL ~     if x == "hello" && y == "world" {
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:15:5
    |
    |
 LL | /     if x == "hello" || x == "world" {
 LL | |         if y == "world" || y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if (x == "hello" || x == "world") && (y == "world" || y == "hello") {
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:21:5
    |
    |
 LL | /     if x == "hello" && x == "world" {
 LL | |         if y == "world" || y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if x == "hello" && x == "world" && (y == "world" || y == "hello") {
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:27:5
    |
    |
 LL | /     if x == "hello" || x == "world" {
 LL | |         if y == "world" && y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if (x == "hello" || x == "world") && y == "world" && y == "hello" {
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:33:5
    |
    |
 LL | /     if x == "hello" && x == "world" {
 LL | |         if y == "world" && y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if x == "hello" && x == "world" && y == "world" && y == "hello" {
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:39:5
    |
    |
 LL | /     if 42 == 1337 {
 LL | |         if 'a' != 'A' {
 LL | |             println!("world!")
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if 42 == 1337 && 'a' != 'A' {
 LL +         println!("world!")
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:95:5
    |
    |
 LL | /     if x == "hello" {
 LL | |         if y == "world" { // Collapsible
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
 help: collapse nested if block
    |
    |
 LL ~     if x == "hello" && y == "world" { // Collapsible
 LL +         println!("Hello world!");
 LL +     }
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:154:5
    |
    |
 LL | /     if matches!(true, true) {
 LL | |         if matches!(true, true) {}
 LL | |     }
-   | |_____^ help: collapse nested if block: `if matches!(true, true) && matches!(true, true) {}`
+   | |_____^ help: collapse nested if block: `if matches!(true, true && matches!(true, true {}`
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:159:5
    |
    |
 LL | /     if matches!(true, true) && truth() {
 LL | |         if matches!(true, true) {}
 LL | |     }
-   | |_____^ help: collapse nested if block: `if matches!(true, true) && truth() && matches!(true, true) {}`
+   | |_____^ help: collapse nested if block: `if matches!(true, true && truth() && matches!(true, true {}`
 error: aborting due to 9 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/collapsible_if.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![allow(clippy::assertions_on_constants, clippy::equatable_if_let)]
 #[rustfmt::skip]
 #[rustfmt::skip]
 #[warn(clippy::collapsible_if)]
     let x = "hello";
     let x = "hello";
     let y = "world";
     if x == "hello" && y == "world" {
     }
 
 
     if (x == "hello" || x == "world") && (y == "world" || y == "hello") {
     }
 
 
     if x == "hello" && x == "world" && (y == "world" || y == "hello") {
     }
 
 
     if (x == "hello" || x == "world") && y == "world" && y == "hello" {
     }
 
 
     if x == "hello" && x == "world" && y == "world" && y == "hello" {
     }
 
 
     if 42 == 1337 && 'a' != 'A' {
         println!("world!")
 
 
     // Works because any if with an else statement cannot be collapsed.
     if x == "hello" {
         if y == "world" {
         }
     } else {
         println!("Not Hello world");
     }
     }
 
     if x == "hello" {
         if y == "world" {
         } else {
             println!("Hello something else");
         }
     }
     }
 
     if x == "hello" {
         print!("Hello ");
         if y == "world" {
             println!("world!")
     }
 
     if true {
     } else {
     } else {
         assert!(true); // assert! is just an `if`
 
 
     // The following tests check for the fix of https://github.com/rust-lang/rust-clippy/issues/798
     // The following tests check for the fix of https://github.com/rust-lang/rust-clippy/issues/798
     if x == "hello" {// Not collapsible
         if y == "world" {
         }
     }
 
 
     if x == "hello" { // Not collapsible
         if y == "world" {
         }
     }
 
 
     if x == "hello" {
         // Not collapsible
         if y == "world" {
         }
     }
 
 
     if x == "hello" && y == "world" { // Collapsible
     }
 
 
     if x == "hello" {
         print!("Hello ");
         // Not collapsible
         // Not collapsible
         if y == "world" {
             println!("world!")
     }
 
 
     if x == "hello" {
         print!("Hello ");
         // Not collapsible
         if let Some(42) = Some(42) {
             println!("world!")
         }
         }
     }
 
error: test failed, to rerun pass `--test compile-test`
Build completed unsuccessfully in 0:02:06
     if x == "hello" {
         /* Not collapsible */
         if y == "world" {
         }
     }
 
 
     if x == "hello" { /* Not collapsible */
         if y == "world" {
         }
     }
 
 
     // Test behavior wrt. `let_chains`.
     fn truth() -> bool { true }
 
     // Prefix:
     if let 0 = 1 {
     if let 0 = 1 {
         if truth() {}
     }
 
     // Suffix:
     if truth() {
         if let 0 = 1 {}
 
     // Midfix:
     if truth() {
         if let 0 = 1 {
         if let 0 = 1 {
             if truth() {}
         }
     }
 
     // Fix #5962
-    if matches!(true, true) && matches!(true, true) {}
+    if matches!(true, true && matches!(true, true {}
     // Issue #9375
     // Issue #9375
-    if matches!(true, true) && truth() && matches!(true, true) {}
+    if matches!(true, true && truth() && matches!(true, true {}
     if true {
     if true {
         #[cfg(not(teehee))]
             println!("Hello world!");
         }
     }
 }
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/collapsible_if.stage-id.fixed
To only update this specific test, also pass `--test-args collapsible_if.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/collapsible_if.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/collapsible_if.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/collapsible_if.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":192,"byte_end":289,"line_start":9,"line_end":13,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" {","highlight_start":5,"highlight_end":22},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::collapsible-if` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":192,"byte_end":289,"line_start":9,"line_end":13,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" {","highlight_start":5,"highlight_end":22},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"hello\" && y == \"world\" {\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:9:5\n   |\nLL | /     if x == \"hello\" {\nLL | |         if y == \"world\" {\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::collapsible-if` implied by `-D warnings`\nhelp: collapse nested if block\n   |\nLL ~     if x == \"hello\" && y == \"world\" {\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":295,"byte_end":424,"line_start":15,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" || x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" || y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":295,"byte_end":424,"line_start":15,"line_end":19,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" || x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" || y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if (x == \"hello\" || x == \"world\") && (y == \"world\" || y == \"hello\") {\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:15:5\n   |\nLL | /     if x == \"hello\" || x == \"world\" {\nLL | |         if y == \"world\" || y == \"hello\" {\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if (x == \"hello\" || x == \"world\") && (y == \"world\" || y == \"hello\") {\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":430,"byte_end":559,"line_start":21,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" && x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" || y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":430,"byte_end":559,"line_start":21,"line_end":25,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" && x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" || y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"hello\" && x == \"world\" && (y == \"world\" || y == \"hello\") {\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:21:5\n   |\nLL | /     if x == \"hello\" && x == \"world\" {\nLL | |         if y == \"world\" || y == \"hello\" {\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if x == \"hello\" && x == \"world\" && (y == \"world\" || y == \"hello\") {\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":565,"byte_end":694,"line_start":27,"line_end":31,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" || x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" && y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":565,"byte_end":694,"line_start":27,"line_end":31,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" || x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" && y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if (x == \"hello\" || x == \"world\") && y == \"world\" && y == \"hello\" {\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:27:5\n   |\nLL | /     if x == \"hello\" || x == \"world\" {\nLL | |         if y == \"world\" && y == \"hello\" {\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if (x == \"hello\" || x == \"world\") && y == \"world\" && y == \"hello\" {\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":700,"byte_end":829,"line_start":33,"line_end":37,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" && x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" && y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":700,"byte_end":829,"line_start":33,"line_end":37,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" && x == \"world\" {","highlight_start":5,"highlight_end":38},{"text":"        if y == \"world\" && y == \"hello\" {","highlight_start":1,"highlight_end":42},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"hello\" && x == \"world\" && y == \"world\" && y == \"hello\" {\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:33:5\n   |\nLL | /     if x == \"hello\" && x == \"world\" {\nLL | |         if y == \"world\" && y == \"hello\" {\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if x == \"hello\" && x == \"world\" && y == \"world\" && y == \"hello\" {\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":835,"byte_end":921,"line_start":39,"line_end":43,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if 42 == 1337 {","highlight_start":5,"highlight_end":20},{"text":"        if 'a' != 'A' {","highlight_start":1,"highlight_end":24},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":835,"byte_end":921,"line_start":39,"line_end":43,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if 42 == 1337 {","highlight_start":5,"highlight_end":20},{"text":"        if 'a' != 'A' {","highlight_start":1,"highlight_end":24},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if 42 == 1337 && 'a' != 'A' {\n        println!(\"world!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:39:5\n   |\nLL | /     if 42 == 1337 {\nLL | |         if 'a' != 'A' {\nLL | |             println!(\"world!\")\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if 42 == 1337 && 'a' != 'A' {\nLL +         println!(\"world!\")\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":1998,"byte_end":2110,"line_start":95,"line_end":99,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" {","highlight_start":5,"highlight_end":22},{"text":"        if y == \"world\" { // Collapsible","highlight_start":1,"highlight_end":41},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":1998,"byte_end":2110,"line_start":95,"line_end":99,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if x == \"hello\" {","highlight_start":5,"highlight_end":22},{"text":"        if y == \"world\" { // Collapsible","highlight_start":1,"highlight_end":41},{"text":"            println!(\"Hello world!\");","highlight_start":1,"highlight_end":38},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"hello\" && y == \"world\" { // Collapsible\n        println!(\"Hello world!\");\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:95:5\n   |\nLL | /     if x == \"hello\" {\nLL | |         if y == \"world\" { // Collapsible\nLL | |             println!(\"Hello world!\");\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL ~     if x == \"hello\" && y == \"world\" { // Collapsible\nLL +         println!(\"Hello world!\");\nLL +     }\n   |\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":3075,"byte_end":3141,"line_start":154,"line_end":156,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if matches!(true, true) {","highlight_start":5,"highlight_end":30},{"text":"        if matches!(true, true) {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":3075,"byte_end":3141,"line_start":154,"line_end":156,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if matches!(true, true) {","highlight_start":5,"highlight_end":30},{"text":"        if matches!(true, true) {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if matches!(true, true && matches!(true, true {}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:154:5\n   |\nLL | /     if matches!(true, true) {\nLL | |         if matches!(true, true) {}\nLL | |     }\n   | |_____^ help: collapse nested if block: `if matches!(true, true && matches!(true, true {}`\n\n"}
{"message":"this `if` statement can be collapsed","code":{"code":"clippy::collapsible_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":3166,"byte_end":3243,"line_start":159,"line_end":161,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if matches!(true, true) && truth() {","highlight_start":5,"highlight_end":41},{"text":"        if matches!(true, true) {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_if.rs","byte_start":3166,"byte_end":3243,"line_start":159,"line_end":161,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if matches!(true, true) && truth() {","highlight_start":5,"highlight_end":41},{"text":"        if matches!(true, true) {}","highlight_start":1,"highlight_end":35},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if matches!(true, true && truth() && matches!(true, true {}","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `if` statement can be collapsed\n  --> tests/ui/collapsible_if.rs:159:5\n   |\nLL | /     if matches!(true, true) && truth() {\nLL | |         if matches!(true, true) {}\nLL | |     }\n   | |_____^ help: collapse nested if block: `if matches!(true, true && truth() && matches!(true, true {}`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: this could be simplified with `bool::then`
   --> $DIR/if_then_some_else_none.rs:5:13
    |
 LL |       let _ = if foo() {
    |  _____________^
 LL | |         println!("true!");
 LL | |         Some("foo")
 LL | |     } else {
 LL | |         None
 LL | |     };
    |
    |
    = help: consider using `bool::then` like: `foo().then(|| { /* snippet */ "foo" })`
    = note: `-D clippy::if-then-some-else-none` implied by `-D warnings`
 error: this could be simplified with `bool::then`
   --> $DIR/if_then_some_else_none.rs:13:13
    |
    |
 LL |       let _ = if matches!(true, true) {
    |  _____________^
 LL | |         println!("true!");
 LL | |         Some(matches!(true, false))
 LL | |     } else {
 LL | |         None
 LL | |     };
    |
    |
-   = help: consider using `bool::then` like: `matches!(true, true).then(|| { /* snippet */ matches!(true, false) })`
+   = help: consider using `bool::then` like: `(matches!(true, true).then(|| { /* snippet */ matches!(true, false })`
 error: this could be simplified with `bool::then_some`
   --> $DIR/if_then_some_else_none.rs:22:28
    |
    |
 LL |     let _ = x.and_then(|o| if o < 32 { Some(o) } else { None });
    |
    |
    = help: consider using `bool::then_some` like: `(o < 32).then_some(o)`
 error: this could be simplified with `bool::then_some`
   --> $DIR/if_then_some_else_none.rs:26:13
    |
    |
 LL |     let _ = if !x { Some(0) } else { None };
    |
    |
    = help: consider using `bool::then_some` like: `(!x).then_some(0)`
 error: this could be simplified with `bool::then`
   --> $DIR/if_then_some_else_none.rs:81:13
    |
 LL |       let _ = if foo() {
 LL |       let _ = if foo() {
    |  _____________^
 LL | |         println!("true!");
 LL | |         Some(150)
 LL | |     } else {
 LL | |         None
 LL | |     };
    |
    |
    = help: consider using `bool::then` like: `foo().then(|| { /* snippet */ 150 })`
 error: aborting due to 5 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/if_then_some_else_none.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args if_then_some_else_none.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/if_then_some_else_none.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/if_then_some_else_none.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/if_then_some_else_none.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this could be simplified with `bool::then`","code":{"code":"clippy::if_then_some_else_none","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_some_else_none.rs","byte_start":96,"byte_end":185,"line_start":5,"line_end":10,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = if foo() {","highlight_start":13,"highlight_end":23},{"text":"        println!(\"true!\");","highlight_start":1,"highlight_end":27},{"text":"        Some(\"foo\")","highlight_start":1,"highlight_end":20},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        None","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `bool::then` like: `foo().then(|| { /* snippet */ \"foo\" })`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::if-then-some-else-none` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be simplified with `bool::then`\n  --> tests/ui/if_then_some_else_none.rs:5:13\n   |\nLL |       let _ = if foo() {\n   |  _____________^\nLL | |         println!(\"true!\");\nLL | |         Some(\"foo\")\nLL | |     } else {\nLL | |         None\nLL | |     };\n   | |_____^\n   |\n   = help: consider using `bool::then` like: `foo().then(|| { /* snippet */ \"foo\" })`\n   = note: `-D clippy::if-then-some-else-none` implied by `-D warnings`\n\n"}
{"message":"this could be simplified with `bool::then`","code":{"code":"clippy::if_then_some_else_none","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_some_else_none.rs","byte_start":251,"byte_end":371,"line_start":13,"line_end":18,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = if matches!(true, true) {","highlight_start":13,"highlight_end":38},{"text":"        println!(\"true!\");","highlight_start":1,"highlight_end":27},{"text":"        Some(matches!(true, false))","highlight_start":1,"highlight_end":36},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        None","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `bool::then` like: `(matches!(true, true).then(|| { /* snippet */ matches!(true, false })`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be simplified with `bool::then`\n  --> tests/ui/if_then_some_else_none.rs:13:13\n   |\nLL |       let _ = if matches!(true, true) {\n   |  _____________^\nLL | |         println!(\"true!\");\nLL | |         Some(matches!(true, false))\nLL | |     } else {\nLL | |         None\nLL | |     };\n   | |_____^\n   |\n   = help: consider using `bool::then` like: `(matches!(true, true).then(|| { /* snippet */ matches!(true, false })`\n\n"}
{"message":"this could be simplified with `bool::then_some`","code":{"code":"clippy::if_then_some_else_none","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_some_else_none.rs","byte_start":504,"byte_end":539,"line_start":22,"line_end":22,"column_start":28,"column_end":63,"is_primary":true,"text":[{"text":"    let _ = x.and_then(|o| if o < 32 { Some(o) } else { None });","highlight_start":28,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `bool::then_some` like: `(o < 32).then_some(o)`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be simplified with `bool::then_some`\n  --> tests/ui/if_then_some_else_none.rs:22:28\n   |\nLL |     let _ = x.and_then(|o| if o < 32 { Some(o) } else { None });\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using `bool::then_some` like: `(o < 32).then_some(o)`\n\n"}
{"message":"this could be simplified with `bool::then_some`","code":{"code":"clippy::if_then_some_else_none","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_some_else_none.rs","byte_start":650,"byte_end":681,"line_start":26,"line_end":26,"column_start":13,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = if !x { Some(0) } else { None };","highlight_start":13,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `bool::then_some` like: `(!x).then_some(0)`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be simplified with `bool::then_some`\n  --> tests/ui/if_then_some_else_none.rs:26:13\n   |\nLL |     let _ = if !x { Some(0) } else { None };\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider using `bool::then_some` like: `(!x).then_some(0)`\n\n"}
{"message":"this could be simplified with `bool::then`","code":{"code":"clippy::if_then_some_else_none","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_some_else_none.rs","byte_start":2064,"byte_end":2151,"line_start":81,"line_end":86,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = if foo() {","highlight_start":13,"highlight_end":23},{"text":"        println!(\"true!\");","highlight_start":1,"highlight_end":27},{"text":"        Some(150)","highlight_start":1,"highlight_end":18},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        None","highlight_start":1,"highlight_end":13},{"text":"    };","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `bool::then` like: `foo().then(|| { /* snippet */ 150 })`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: this could be simplified with `bool::then`\n  --> tests/ui/if_then_some_else_none.rs:81:13\n   |\nLL |       let _ = if foo() {\n   |  _____________^\nLL | |         println!(\"true!\");\nLL | |         Some(150)\nLL | |     } else {\nLL | |         None\nLL | |     };\n   | |_____^\n   |\n   = help: consider using `bool::then` like: `foo().then(|| { /* snippet */ 150 })`\n\n"}

------------------------------------------



error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/equatable_if_let.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/equatable_if_let.stage-id" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-426d0cca316c35ed.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c7b10cf951dd44cc.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-25f7a610b7fafce5.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-64c69a0c8d27494b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-bd373ba7cdb9c07f.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-2d8078885e283441.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-a36a3333b2a93763.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-2c5d99d6327d1048.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-3035b3cfcafc3b31.rlib" "--edition=2021" "--crate-name=fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/equatable_if_let.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"match expression looks like `matches!` macro","code":{"code":"clippy::match_like_matches_macro","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.fixed","byte_start":1648,"byte_end":1696,"line_start":80,"line_end":80,"column_start":8,"column_end":56,"is_primary":true,"text":[{"text":"    if matches!(h, NoPartialEqStruct { a: 2, b: false }) {}","highlight_start":8,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-like-matches-macro` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.fixed","byte_start":1648,"byte_end":1696,"line_start":80,"line_end":80,"column_start":8,"column_end":56,"is_primary":true,"text":[{"text":"    if matches!(h, NoPartialEqStruct { a: 2, b: false }) {}","highlight_start":8,"highlight_end":56}],"label":null,"suggested_replacement":"matches!(h, NoPartialEqStruct { a: 2, b: false })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: match expression looks like `matches!` macro\n  --> tests/ui/equatable_if_let.fixed:80:8\n   |\nLL |     if matches!(h, NoPartialEqStruct { a: 2, b: false }) {}\n   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `matches!(h, NoPartialEqStruct { a: 2, b: false })`\n   |\n   = note: `-D clippy::match-like-matches-macro` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:7:13
-   |
-LL |     assert!(matches!('x', 'a'..='z'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `'x'.is_ascii_lowercase()`
-   |
-   = note: `-D clippy::manual-is-ascii-check` implied by `-D warnings`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:8:13
-   |
-   |
-LL |     assert!(matches!('X', 'A'..='Z'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `'X'.is_ascii_uppercase()`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:9:13
-   |
-   |
-LL |     assert!(matches!(b'x', b'a'..=b'z'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'x'.is_ascii_lowercase()`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:10:13
-   |
-   |
-LL |     assert!(matches!(b'X', b'A'..=b'Z'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'X'.is_ascii_uppercase()`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:13:13
-   |
-   |
-LL |     assert!(matches!(num, '0'..='9'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `num.is_ascii_digit()`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:14:13
-   |
-   |
-LL |     assert!(matches!(b'1', b'0'..=b'9'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'1'.is_ascii_digit()`
-error: manual check for common ascii range
-  --> $DIR/manual_is_ascii_check.rs:15:13
-   |
-   |
-LL |     assert!(matches!('x', 'A'..='Z' | 'a'..='z'));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `'x'.is_ascii_alphabetic()`
-error: manual check for common ascii range
   --> $DIR/manual_is_ascii_check.rs:19:5
    |
    |
 LL |     (b'0'..=b'9').contains(&b'0');
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'0'.is_ascii_digit()`
+   |
+   = note: `-D clippy::manual-is-ascii-check` implied by `-D warnings`
 error: manual check for common ascii range
   --> $DIR/manual_is_ascii_check.rs:20:5
    |
    |
 LL |     (b'a'..=b'z').contains(&b'a');
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'a'.is_ascii_lowercase()`
 error: manual check for common ascii range
   --> $DIR/manual_is_ascii_check.rs:21:5
    |
    |
 LL |     (b'A'..=b'Z').contains(&b'A');
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `b'A'.is_ascii_uppercase()`
