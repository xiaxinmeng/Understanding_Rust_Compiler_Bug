plain

---- compile_test stdout ----
diff of stderr:

 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if y == "world" {
 LL | |             println!("world!")
 LL | |         }
 LL | |     }
    |
    |
    = note: `-D clippy::collapsible-else-if` implied by `-D warnings`
 help: collapse nested if block
    |
 LL |     } else if y == "world" {
 LL |         println!("world!")
    |
 
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if let Some(42) = Some(42) {
 LL | |             println!("world!")
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     } else if let Some(42) = Some(42) {
 LL |         println!("world!")
    |
 
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if y == "world" {
 LL | |             println!("world")
 LL | |         }
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: collapse nested if block
    |
 LL |     } else if y == "world" {
 LL |         println!("world")
 LL |     else {
 LL |         println!("!")
 LL |     }
    |
    |
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if let Some(42) = Some(42) {
 LL | |             println!("world")
 LL | |         }
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: collapse nested if block
    |
 LL |     } else if let Some(42) = Some(42) {
 LL |         println!("world")
 LL |     else {
 LL |         println!("!")
 LL |     }
    |
    |
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if let Some(42) = Some(42) {
 LL | |             println!("world")
 LL | |         }
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: collapse nested if block
    |
 LL |     } else if let Some(42) = Some(42) {
 LL |         println!("world")
 LL |     else {
 LL |         println!("!")
 LL |     }
    |
    |
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if x == "hello" {
 LL | |             println!("world")
 LL | |         }
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: collapse nested if block
    |
 LL |     } else if x == "hello" {
 LL |         println!("world")
 LL |     else {
 LL |         println!("!")
 LL |     }
    |
    |
 
 error: this `else { if .. }` block can be collapsed
    |
 LL |       } else {
    |  ____________^
    |  ____________^
 LL | |         if let Some(42) = Some(42) {
 LL | |             println!("world")
 LL | |         }
 LL | |         }
 LL | |     }
    | |_____^
    |
    |
 help: collapse nested if block
    |
 LL |     } else if let Some(42) = Some(42) {
 LL |         println!("world")
 LL |     else {
 LL |         println!("!")
 LL |     }
    |
    |
 
-error: aborting due to 7 previous errors
+error: this `else { if .. }` block can be collapsed
+   |
+LL |       } else {
+   |  ____________^
+   |  ____________^
+LL | |         #[cfg(not(roflol))]
+LL | |         if y == "world" {
+LL | |             println!("world!")
+LL | |         }
+LL | |     }
+   |
+   |
+help: collapse nested if block
+   |
+LL |     } else if y == "world" {
+LL |         println!("world!")
+LL |     }
+
+error: aborting due to 8 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/collapsible_else_if.stage-id.stderr

 // run-rustfix
 // run-rustfix
 #![allow(clippy::assertions_on_constants)]
 
 #[rustfmt::skip]
 #[warn(clippy::collapsible_if)]
 #[warn(clippy::collapsible_else_if)]
 fn main() {
     let x = "hello";
     let x = "hello";
     let y = "world";
     // Collapse `else { if .. }` to `else if ..`
     if x == "hello" {
         print!("Hello ");
     } else if y == "world" {
         println!("world!")
 
 
     if x == "hello" {
         print!("Hello ");
     } else if let Some(42) = Some(42) {
         println!("world!")
 
 
     if x == "hello" {
         print!("Hello ");
     } else if y == "world" {
         println!("world")
     else {
         println!("!")
     }
 
 
     if x == "hello" {
         print!("Hello ");
     } else if let Some(42) = Some(42) {
         println!("world")
     else {
         println!("!")
     }
 
 
     if let Some(42) = Some(42) {
         print!("Hello ");
     } else if let Some(42) = Some(42) {
         println!("world")
     else {
         println!("!")
     }
 
 
     if let Some(42) = Some(42) {
         print!("Hello ");
     } else if x == "hello" {
         println!("world")
     else {
         println!("!")
     }
 
 
     if let Some(42) = Some(42) {
         print!("Hello ");
     } else if let Some(42) = Some(42) {
         println!("world")
     else {
         println!("!")
     }
 
 
     if x == "hello" {
         print!("Hello ");
-    } else {
-        #[cfg(not(roflol))]
-        if y == "world" {
-            println!("world!")
-        }
+    } else if y == "world" {
+        println!("world!")
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/collapsible_else_if.stage-id.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args collapsible_else_if.rs`
error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/collapsible_else_if.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/collapsible_else_if.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/collapsible_else_if.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":308,"byte_end":382,"line_start":14,"line_end":18,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::collapsible-else-if` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":308,"byte_end":382,"line_start":14,"line_end":18,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if y == \"world\" {\n        println!(\"world!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:14:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if y == \"world\" {\nLL | |             println!(\"world!\")\nLL | |         }\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::collapsible-else-if` implied by `-D warnings`\nhelp: collapse nested if block\n   |\nLL |     } else if y == \"world\" {\nLL |         println!(\"world!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":443,"byte_end":528,"line_start":22,"line_end":26,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":443,"byte_end":528,"line_start":22,"line_end":26,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(42) = Some(42) {\n        println!(\"world!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:22:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if let Some(42) = Some(42) {\nLL | |             println!(\"world!\")\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if let Some(42) = Some(42) {\nLL |         println!(\"world!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":589,"byte_end":713,"line_start":30,"line_end":37,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":589,"byte_end":713,"line_start":30,"line_end":37,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if y == \"world\" {\n        println!(\"world\")\n    }\n    else {\n        println!(\"!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:30:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if y == \"world\" {\nLL | |             println!(\"world\")\nLL | |         }\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if y == \"world\" {\nLL |         println!(\"world\")\nLL |     }\nLL |     else {\nLL |         println!(\"!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":774,"byte_end":909,"line_start":41,"line_end":48,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":774,"byte_end":909,"line_start":41,"line_end":48,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(42) = Some(42) {\n        println!(\"world\")\n    }\n    else {\n        println!(\"!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:41:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if let Some(42) = Some(42) {\nLL | |             println!(\"world\")\nLL | |         }\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if let Some(42) = Some(42) {\nLL |         println!(\"world\")\nLL |     }\nLL |     else {\nLL |         println!(\"!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":981,"byte_end":1116,"line_start":52,"line_end":59,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":981,"byte_end":1116,"line_start":52,"line_end":59,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(42) = Some(42) {\n        println!(\"world\")\n    }\n    else {\n        println!(\"!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:52:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if let Some(42) = Some(42) {\nLL | |             println!(\"world\")\nLL | |         }\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if let Some(42) = Some(42) {\nLL |         println!(\"world\")\nLL |     }\nLL |     else {\nLL |         println!(\"!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1188,"byte_end":1312,"line_start":63,"line_end":70,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if x == \"hello\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1188,"byte_end":1312,"line_start":63,"line_end":70,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if x == \"hello\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if x == \"hello\" {\n        println!(\"world\")\n    }\n    else {\n        println!(\"!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:63:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if x == \"hello\" {\nLL | |             println!(\"world\")\nLL | |         }\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if x == \"hello\" {\nLL |         println!(\"world\")\nLL |     }\nLL |     else {\nLL |         println!(\"!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1384,"byte_end":1519,"line_start":74,"line_end":81,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1384,"byte_end":1519,"line_start":74,"line_end":81,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        if let Some(42) = Some(42) {","highlight_start":1,"highlight_end":37},{"text":"            println!(\"world\")","highlight_start":1,"highlight_end":30},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        else {","highlight_start":1,"highlight_end":15},{"text":"            println!(\"!\")","highlight_start":1,"highlight_end":26},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if let Some(42) = Some(42) {\n        println!(\"world\")\n    }\n    else {\n        println!(\"!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:74:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         if let Some(42) = Some(42) {\nLL | |             println!(\"world\")\nLL | |         }\n...  |\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if let Some(42) = Some(42) {\nLL |         println!(\"world\")\nLL |     }\nLL |     else {\nLL |         println!(\"!\")\nLL |     }\n   |\n\n"}
{"message":"this `else { if .. }` block can be collapsed","code":{"code":"clippy::collapsible_else_if","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1580,"byte_end":1682,"line_start":85,"line_end":90,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        #[cfg(not(roflol))]","highlight_start":1,"highlight_end":28},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"collapse nested if block","code":null,"level":"help","spans":[{"file_name":"tests/ui/collapsible_else_if.rs","byte_start":1580,"byte_end":1682,"line_start":85,"line_end":90,"column_start":12,"column_end":6,"is_primary":true,"text":[{"text":"    } else {","highlight_start":12,"highlight_end":13},{"text":"        #[cfg(not(roflol))]","highlight_start":1,"highlight_end":28},{"text":"        if y == \"world\" {","highlight_start":1,"highlight_end":26},{"text":"            println!(\"world!\")","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"if y == \"world\" {\n        println!(\"world!\")\n    }","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `else { if .. }` block can be collapsed\n  --> tests/ui/collapsible_else_if.rs:85:12\n   |\nLL |       } else {\n   |  ____________^\nLL | |         #[cfg(not(roflol))]\nLL | |         if y == \"world\" {\nLL | |             println!(\"world!\")\nLL | |         }\nLL | |     }\n   | |_____^\n   |\nhelp: collapse nested if block\n   |\nLL |     } else if y == \"world\" {\nLL |         println!(\"world!\")\nLL |     }\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

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
 LL |     if x == "hello" && y == "world" {
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:15:5
    |
 LL | /     if x == "hello" || x == "world" {
 LL | |         if y == "world" || y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if (x == "hello" || x == "world") && (y == "world" || y == "hello") {
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:21:5
    |
 LL | /     if x == "hello" && x == "world" {
 LL | |         if y == "world" || y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if x == "hello" && x == "world" && (y == "world" || y == "hello") {
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:27:5
    |
 LL | /     if x == "hello" || x == "world" {
 LL | |         if y == "world" && y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if (x == "hello" || x == "world") && y == "world" && y == "hello" {
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:33:5
    |
 LL | /     if x == "hello" && x == "world" {
 LL | |         if y == "world" && y == "hello" {
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if x == "hello" && x == "world" && y == "world" && y == "hello" {
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:39:5
    |
 LL | /     if 42 == 1337 {
 LL | |         if 'a' != 'A' {
 LL | |             println!("world!")
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if 42 == 1337 && 'a' != 'A' {
 LL |         println!("world!")
    |
 
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:95:5
   --> $DIR/collapsible_if.rs:95:5
    |
 LL | /     if x == "hello" {
 LL | |         if y == "world" { // Collapsible
 LL | |             println!("Hello world!");
 LL | |         }
 LL | |     }
    |
    |
 help: collapse nested if block
    |
 LL |     if x == "hello" && y == "world" { // Collapsible
 LL |     }
    |
 
 error: this `if` statement can be collapsed
 error: this `if` statement can be collapsed
   --> $DIR/collapsible_if.rs:154:5
    |
 LL | /     if matches!(true, true) {
 LL | |         if matches!(true, true) {}
 LL | |     }
    | |_____^ help: collapse nested if block: `if matches!(true, true) && matches!(true, true) {}`
-error: aborting due to 8 previous errors
+error: this `if` statement can be collapsed
+  --> $DIR/collapsible_if.rs:158:5
+   |
+   |
+LL | /     if true {
+LL | |         #[cfg(not(teehee))]
+LL | |         if true {
+LL | |             println!("Hello world!");
+LL | |         }
+LL | |     }
+   |
+   |
+help: collapse nested if block
+LL |     if true && true {
+LL |         println!("Hello world!");
+LL |     }
+   |
+   |
+
+error: aborting due to 9 previous errors
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-4cfe8e74ca7ddba1/out/test_build_base/collapsible_if.stage-id.stderr

 // run-rustfix
 // run-rustfix
 #![allow(clippy::assertions_on_constants)]
 
 #[rustfmt::skip]
 #[warn(clippy::collapsible_if)]
error: test failed, to rerun pass '--test compile-test'
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
     } else {
         println!("Not Hello world");
 
 
     if x == "hello" {
         if y == "world" {
         } else {
         } else {
             println!("Hello something else");
     }
 
 
     if x == "hello" {
         print!("Hello ");
