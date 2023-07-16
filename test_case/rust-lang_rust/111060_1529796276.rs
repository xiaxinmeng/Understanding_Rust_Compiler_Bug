plain

-error: empty string literal in `println!`
-  --> $DIR/ice-10148.rs:8:5
-   |
-LL |     println!(with_span!(""something ""));
-   |                         |
-   |                         help: remove the empty string
-   |
-   |
-   = note: `-D clippy::println-empty-string` implied by `-D warnings`
-error: aborting due to previous error
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crashes/ice-10148.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-10148.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

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
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test")`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test")`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
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
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test/ntest")`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "test/ntest").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test/ntest")`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "with {}", value).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {}", value)`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {} {}", 2, value)`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "with {value}").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {value}")`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("macro arg {}", one!())`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 
 error: use of `writeln!(stderr(), ...).unwrap()`
    |
    |
 LL |         writeln!(std::io::stderr(), "{:w$}", value, w = width).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{:w$}", value, w = width)`
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("{}/n", $crate::format_args!($($arg)*))`
 error: aborting due to 13 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![warn(clippy::explicit_write)]
 #![allow(unused_imports)]
 #![allow(clippy::uninlined_format_args)]
 fn stdout() -> String {
     String::new()
 }
 
---
 fn main() {
     // these should warn
     {
         use std::io::Write;
         print!("test");
         eprint!("test");
-        println!("test");
-        eprintln!("test");
+        println!("{}\n", $crate::format_args!($($arg)*));
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
         print!("test");
         eprint!("test");
         // including newlines
         // including newlines
-        println!("test\ntest");
-        eprintln!("test\ntest");
+        println!("{}\n", $crate::format_args!($($arg)*));
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
         let value = 1;
         let value = 1;
-        eprintln!("with {}", value);
-        eprintln!("with {} {}", 2, value);
-        eprintln!("with {value}");
-        eprintln!("macro arg {}", one!());
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
         let width = 2;
-        eprintln!("{:w$}", value, w = width);
+        eprintln!("{}\n", $crate::format_args!($($arg)*));
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `write!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":336,"byte_end":378,"line_start":24,"line_end":24,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-write` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":336,"byte_end":378,"line_start":24,"line_end":24,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":"print!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `write!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:24:9\n   |\nLL |         write!(std::io::stdout(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!(\"test\")`\n   |\n   = note: `-D clippy::explicit-write` implied by `-D warnings`\n\n"}
{"message":"use of `write!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":388,"byte_end":430,"line_start":25,"line_end":25,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":388,"byte_end":430,"line_start":25,"line_end":25,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        write!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":"eprint!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `write!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:25:9\n   |\nLL |         write!(std::io::stderr(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!(\"test\")`\n\n"}
{"message":"use of `writeln!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":440,"byte_end":484,"line_start":26,"line_end":26,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":440,"byte_end":484,"line_start":26,"line_end":26,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"println!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:26:9\n   |\nLL |         writeln!(std::io::stdout(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":494,"byte_end":538,"line_start":27,"line_end":27,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":494,"byte_end":538,"line_start":27,"line_end":27,"column_start":9,"column_end":53,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\").unwrap();","highlight_start":9,"highlight_end":53}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:27:9\n   |\nLL |         writeln!(std::io::stderr(), \"test\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `stdout().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":548,"byte_end":606,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":548,"byte_end":606,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"print!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stdout().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:28:9\n   |\nLL |         std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!(\"test\")`\n\n"}
{"message":"use of `stderr().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":616,"byte_end":674,"line_start":29,"line_end":29,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":616,"byte_end":674,"line_start":29,"line_end":29,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"eprint!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stderr().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:29:9\n   |\nLL |         std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!(\"test\")`\n\n"}
{"message":"use of `writeln!(stdout(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":715,"byte_end":765,"line_start":32,"line_end":32,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":715,"byte_end":765,"line_start":32,"line_end":32,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stdout(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":"println!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stdout(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:32:9\n   |\nLL |         writeln!(std::io::stdout(), \"test\\ntest\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":775,"byte_end":825,"line_start":33,"line_end":33,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":775,"byte_end":825,"line_start":33,"line_end":33,"column_start":9,"column_end":59,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"test\\ntest\").unwrap();","highlight_start":9,"highlight_end":59}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:33:9\n   |\nLL |         writeln!(std::io::stderr(), \"test\\ntest\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":859,"byte_end":913,"line_start":36,"line_end":36,"column_start":9,"column_end":63,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {}\", value).unwrap();","highlight_start":9,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":859,"byte_end":913,"line_start":36,"line_end":36,"column_start":9,"column_end":63,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {}\", value).unwrap();","highlight_start":9,"highlight_end":63}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:36:9\n   |\nLL |         writeln!(std::io::stderr(), \"with {}\", value).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":923,"byte_end":983,"line_start":37,"line_end":37,"column_start":9,"column_end":69,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {} {}\", 2, value).unwrap();","highlight_start":9,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":923,"byte_end":983,"line_start":37,"line_end":37,"column_start":9,"column_end":69,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {} {}\", 2, value).unwrap();","highlight_start":9,"highlight_end":69}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:37:9\n   |\nLL |         writeln!(std::io::stderr(), \"with {} {}\", 2, value).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":993,"byte_end":1045,"line_start":38,"line_end":38,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {value}\").unwrap();","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":993,"byte_end":1045,"line_start":38,"line_end":38,"column_start":9,"column_end":61,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"with {value}\").unwrap();","highlight_start":9,"highlight_end":61}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:38:9\n   |\nLL |         writeln!(std::io::stderr(), \"with {value}\").unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1055,"byte_end":1115,"line_start":39,"line_end":39,"column_start":9,"column_end":69,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"macro arg {}\", one!()).unwrap();","highlight_start":9,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1055,"byte_end":1115,"line_start":39,"line_end":39,"column_start":9,"column_end":69,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"macro arg {}\", one!()).unwrap();","highlight_start":9,"highlight_end":69}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:39:9\n   |\nLL |         writeln!(std::io::stderr(), \"macro arg {}\", one!()).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}
{"message":"use of `writeln!(stderr(), ...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1148,"byte_end":1211,"line_start":41,"line_end":41,"column_start":9,"column_end":72,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();","highlight_start":9,"highlight_end":72}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":1148,"byte_end":1211,"line_start":41,"line_end":41,"column_start":9,"column_end":72,"is_primary":true,"text":[{"text":"        writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();","highlight_start":9,"highlight_end":72}],"label":null,"suggested_replacement":"eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `writeln!(stderr(), ...).unwrap()`\n  --> tests/ui/explicit_write.rs:41:9\n   |\nLL |         writeln!(std::io::stderr(), \"{:w$}\", value, w = width).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!(\"{}\\n\", $crate::format_args!($($arg)*))`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: `to_string` applied to a type that implements `Display` in `format!` args
    |
 LL |     let _ = format!("error: something failed at {}", Location::caller().to_string());
    |                                                                        ^^^^^^^^^^^^ help: remove this
    |
    |
    = note: `-D clippy::to-string-in-format-args` implied by `-D warnings`
 
 error: `to_string` applied to a type that implements `Display` in `write!` args
    |
 LL |         Location::caller().to_string()
    |                           ^^^^^^^^^^^^ help: remove this
 
 
-error: `to_string` applied to a type that implements `Display` in `writeln!` args
-   |
-LL |         Location::caller().to_string()
-   |                           ^^^^^^^^^^^^ help: remove this
-
-
 error: `to_string` applied to a type that implements `Display` in `print!` args
    |
 LL |     print!("error: something failed at {}", Location::caller().to_string());
    |                                                               ^^^^^^^^^^^^ help: remove this
 
 
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-LL |     println!("error: something failed at {}", Location::caller().to_string());
-   |                                                                 ^^^^^^^^^^^^ help: remove this
-
-
 error: `to_string` applied to a type that implements `Display` in `eprint!` args
    |
    |
 LL |     eprint!("error: something failed at {}", Location::caller().to_string());
 
 
-error: `to_string` applied to a type that implements `Display` in `eprintln!` args
-   |
-   |
-LL |     eprintln!("error: something failed at {}", Location::caller().to_string());
-
-
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
 
 
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-LL |     println!("{}", X(1).to_string());
-LL |     println!("{}", X(1).to_string());
-   |                    ^^^^^^^^^^^^^^^^ help: use this: `*X(1)`
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{}", Y(&X(1)).to_string());
-   |                    ^^^^^^^^^^^^^^^^^^^^ help: use this: `***Y(&X(1))`
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{}", Z(1).to_string());
-
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-LL |     println!("{}", x.to_string());
-LL |     println!("{}", x.to_string());
-   |                    ^^^^^^^^^^^^^ help: use this: `**x`
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-LL |     println!("{}", x_ref.to_string());
-LL |     println!("{}", x_ref.to_string());
-   |                    ^^^^^^^^^^^^^^^^^ help: use this: `***x_ref`
-
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
 
 
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |         println!("{}", original[..10].to_string());
-   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use this: `&original[..10]`
-error: aborting due to 25 previous errors
+error: aborting due to 12 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args.stage-id.stderr
diff of fixed:

 //@run-rustfix
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
 
 
     fn deref(&self) -> &u32 {
         &self.0
 }
 
 
 struct Y<'a>(&'a X);
 
 impl<'a> Deref for Y<'a> {
     type Target = &'a X;
 
     fn deref(&self) -> &Self::Target {
         &self.0
 }
 
 
 struct Z(u32);
 
 impl Deref for Z {
 
 
     fn deref(&self) -> &u32 {
         &self.0
 }
 
 impl std::fmt::Display for Z {
 impl std::fmt::Display for Z {
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "Z")
 }
 
 
 macro_rules! my_macro {
     () => {
         // here be dragons, do not enter (or lint)
         println!("error: something failed at {}", Location::caller().to_string());
 }
 
 
 macro_rules! my_other_macro {
         Location::caller().to_string()
     };
 }
 
 
 fn main() {
     let x = &X(1);
     let x_ref = &x;
     let _ = format!("error: something failed at {}", Location::caller());
     let _ = write!(
         stdout(),
         "error: something failed at {}",
---
         "error: something failed at {}",
-        Location::caller()
+        Location::caller().to_string()
     );
     print!("error: something failed at {}", Location::caller());
-    println!("error: something failed at {}", Location::caller());
+    println!("error: something failed at {}", Location::caller().to_string());
     eprint!("error: something failed at {}", Location::caller());
-    eprintln!("error: something failed at {}", Location::caller());
+    eprintln!("error: something failed at {}", Location::caller().to_string());
     let _ = format_args!("error: something failed at {}", Location::caller());
     assert!(true, "error: something failed at {}", Location::caller());
     assert_eq!(0, 0, "error: something failed at {}", Location::caller());
     assert_ne!(0, 0, "error: something failed at {}", Location::caller());
     panic!("error: something failed at {}", Location::caller());
-    println!("{}", *X(1));
-    println!("{}", ***Y(&X(1)));
-    println!("{}", Z(1));
-    println!("{}", **x);
-    println!("{}", ***x_ref);
+    println!("{}", X(1).to_string());
+    println!("{}", Y(&X(1)).to_string());
+    println!("{}", Z(1).to_string());
+    println!("{}", x.to_string());
+    println!("{}", x_ref.to_string());
     // https://github.com/rust-lang/rust-clippy/issues/7903
-    println!("{foo}{bar}", foo = "foo", bar = "bar");
-    println!("{foo}{bar}", foo = "foo", bar = "bar");
-    println!("{foo}{bar}", bar = "bar", foo = "foo");
-    println!("{foo}{bar}", bar = "bar", foo = "foo");
+    println!("{foo}{bar}", foo = "foo".to_string(), bar = "bar");
+    println!("{foo}{bar}", foo = "foo", bar = "bar".to_string());
+    println!("{foo}{bar}", bar = "bar".to_string(), foo = "foo");
+    println!("{foo}{bar}", bar = "bar", foo = "foo".to_string());
     // negative tests
     // negative tests
     println!("error: something failed at {}", Somewhere.to_string());
     // The next two tests are negative because caching the string might be faster than calling `<X as
     // Display>::fmt` twice.
     println!("{} and again {0}", x.to_string());
     println!("{foo}{foo}", foo = "foo".to_string());
     my_macro!();
     println!("error: something failed at {}", my_other_macro!());
     // https://github.com/rust-lang/rust-clippy/issues/7903
     println!("{foo}{foo:?}", foo = "foo".to_string());
     print!("{}", (Location::caller()));
     print!("{}", ((Location::caller())));
 
 
 fn issue8643(vendor_id: usize, product_id: usize, name: &str) {
     println!(
         "{:<9}  {:<10}  {}",
         format!("0x{:x}", vendor_id),
         format!("0x{:x}", product_id),
     );
 }
 
 // https://github.com/rust-lang/rust-clippy/issues/8855
 // https://github.com/rust-lang/rust-clippy/issues/8855
 mod issue_8855 {
     #![allow(dead_code)]
 
     struct A {}
 
     impl std::fmt::Display for A {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
             write!(f, "test")
     }
 
     fn main() {
         let a = A {};
         let a = A {};
         let b = A {};
 
         let x = format!("{} {}", a, b);
         dbg!(x);
 
         let x = format!("{:>6} {:>6}", a, b.to_string());
         dbg!(x);
 }
 
 // https://github.com/rust-lang/rust-clippy/issues/9256
 mod issue_9256 {
 mod issue_9256 {
     #![allow(dead_code)]
 
     fn print_substring(original: &str) {
         assert!(original.len() > 10);
-        println!("{}", &original[..10]);
+        println!("{}", original[..10].to_string());
 
     fn main() {
         print_substring("Hello, world!");
     }
     }
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args.stage-id.fixed
To only update this specific test, also pass `--test-args format_args.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`to_string` applied to a type that implements `Display` in `format!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1337,"byte_end":1349,"line_start":77,"line_end":77,"column_start":72,"column_end":84,"is_primary":true,"text":[{"text":"    let _ = format!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":72,"highlight_end":84}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::to-string-in-format-args` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1337,"byte_end":1349,"line_start":77,"line_end":77,"column_start":72,"column_end":84,"is_primary":true,"text":[{"text":"    let _ = format!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":72,"highlight_end":84}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `format!` args\n  --> tests/ui/format_args.rs:77:72\n   |\nLL |     let _ = format!(\"error: something failed at {}\", Location::caller().to_string());\n   |                                                                        ^^^^^^^^^^^^ help: remove this\n   |\n   = note: `-D clippy::to-string-in-format-args` implied by `-D warnings`\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `write!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1457,"byte_end":1469,"line_start":81,"line_end":81,"column_start":27,"column_end":39,"is_primary":true,"text":[{"text":"        Location::caller().to_string()","highlight_start":27,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1457,"byte_end":1469,"line_start":81,"line_end":81,"column_start":27,"column_end":39,"is_primary":true,"text":[{"text":"        Location::caller().to_string()","highlight_start":27,"highlight_end":39}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `write!` args\n  --> tests/ui/format_args.rs:81:27\n   |\nLL |         Location::caller().to_string()\n   |                           ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `print!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1666,"byte_end":1678,"line_start":88,"line_end":88,"column_start":63,"column_end":75,"is_primary":true,"text":[{"text":"    print!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":63,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1666,"byte_end":1678,"line_start":88,"line_end":88,"column_start":63,"column_end":75,"is_primary":true,"text":[{"text":"    print!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":63,"highlight_end":75}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `print!` args\n  --> tests/ui/format_args.rs:88:63\n   |\nLL |     print!(\"error: something failed at {}\", Location::caller().to_string());\n   |                                                               ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `eprint!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1823,"byte_end":1835,"line_start":90,"line_end":90,"column_start":64,"column_end":76,"is_primary":true,"text":[{"text":"    eprint!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":64,"highlight_end":76}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1823,"byte_end":1835,"line_start":90,"line_end":90,"column_start":64,"column_end":76,"is_primary":true,"text":[{"text":"    eprint!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":64,"highlight_end":76}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `eprint!` args\n  --> tests/ui/format_args.rs:90:64\n   |\nLL |     eprint!(\"error: something failed at {}\", Location::caller().to_string());\n   |                                                                ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `format_args!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1994,"byte_end":2006,"line_start":92,"line_end":92,"column_start":77,"column_end":89,"is_primary":true,"text":[{"text":"    let _ = format_args!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":77,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":1994,"byte_end":2006,"line_start":92,"line_end":92,"column_start":77,"column_end":89,"is_primary":true,"text":[{"text":"    let _ = format_args!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":77,"highlight_end":89}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `format_args!` args\n  --> tests/ui/format_args.rs:92:77\n   |\nLL |     let _ = format_args!(\"error: something failed at {}\", Location::caller().to_string());\n   |                                                                             ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `assert!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2078,"byte_end":2090,"line_start":93,"line_end":93,"column_start":70,"column_end":82,"is_primary":true,"text":[{"text":"    assert!(true, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":70,"highlight_end":82}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2078,"byte_end":2090,"line_start":93,"line_end":93,"column_start":70,"column_end":82,"is_primary":true,"text":[{"text":"    assert!(true, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":70,"highlight_end":82}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `assert!` args\n  --> tests/ui/format_args.rs:93:70\n   |\nLL |     assert!(true, \"error: something failed at {}\", Location::caller().to_string());\n   |                                                                      ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `assert_eq!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2165,"byte_end":2177,"line_start":94,"line_end":94,"column_start":73,"column_end":85,"is_primary":true,"text":[{"text":"    assert_eq!(0, 0, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":73,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2165,"byte_end":2177,"line_start":94,"line_end":94,"column_start":73,"column_end":85,"is_primary":true,"text":[{"text":"    assert_eq!(0, 0, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":73,"highlight_end":85}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `assert_eq!` args\n  --> tests/ui/format_args.rs:94:73\n   |\nLL |     assert_eq!(0, 0, \"error: something failed at {}\", Location::caller().to_string());\n   |                                                                         ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `assert_ne!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2252,"byte_end":2264,"line_start":95,"line_end":95,"column_start":73,"column_end":85,"is_primary":true,"text":[{"text":"    assert_ne!(0, 0, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":73,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2252,"byte_end":2264,"line_start":95,"line_end":95,"column_start":73,"column_end":85,"is_primary":true,"text":[{"text":"    assert_ne!(0, 0, \"error: something failed at {}\", Location::caller().to_string());","highlight_start":73,"highlight_end":85}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `assert_ne!` args\n  --> tests/ui/format_args.rs:95:73\n   |\nLL |     assert_ne!(0, 0, \"error: something failed at {}\", Location::caller().to_string());\n   |                                                                         ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `panic!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2329,"byte_end":2341,"line_start":96,"line_end":96,"column_start":63,"column_end":75,"is_primary":true,"text":[{"text":"    panic!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":63,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":2329,"byte_end":2341,"line_start":96,"line_end":96,"column_start":63,"column_end":75,"is_primary":true,"text":[{"text":"    panic!(\"error: something failed at {}\", Location::caller().to_string());","highlight_start":63,"highlight_end":75}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `panic!` args\n  --> tests/ui/format_args.rs:96:63\n   |\nLL |     panic!(\"error: something failed at {}\", Location::caller().to_string());\n   |                                                               ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `print!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":3420,"byte_end":3432,"line_start":118,"line_end":118,"column_start":37,"column_end":49,"is_primary":true,"text":[{"text":"    print!(\"{}\", (Location::caller().to_string()));","highlight_start":37,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":3420,"byte_end":3432,"line_start":118,"line_end":118,"column_start":37,"column_end":49,"is_primary":true,"text":[{"text":"    print!(\"{}\", (Location::caller().to_string()));","highlight_start":37,"highlight_end":49}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `print!` args\n  --> tests/ui/format_args.rs:118:37\n   |\nLL |     print!(\"{}\", (Location::caller().to_string()));\n   |                                     ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `print!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":3474,"byte_end":3486,"line_start":119,"line_end":119,"column_start":39,"column_end":51,"is_primary":true,"text":[{"text":"    print!(\"{}\", ((Location::caller()).to_string()));","highlight_start":39,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":3474,"byte_end":3486,"line_start":119,"line_end":119,"column_start":39,"column_end":51,"is_primary":true,"text":[{"text":"    print!(\"{}\", ((Location::caller()).to_string()));","highlight_start":39,"highlight_end":51}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `print!` args\n  --> tests/ui/format_args.rs:119:39\n   |\nLL |     print!(\"{}\", ((Location::caller()).to_string()));\n   |                                       ^^^^^^^^^^^^ help: remove this\n\n"}
{"message":"`to_string` applied to a type that implements `Display` in `format!` args","code":{"code":"clippy::to_string_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":4069,"byte_end":4081,"line_start":147,"line_end":147,"column_start":38,"column_end":50,"is_primary":true,"text":[{"text":"        let x = format!(\"{} {}\", a, b.to_string());","highlight_start":38,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove this","code":null,"level":"help","spans":[{"file_name":"tests/ui/format_args.rs","byte_start":4069,"byte_end":4081,"line_start":147,"line_end":147,"column_start":38,"column_end":50,"is_primary":true,"text":[{"text":"        let x = format!(\"{} {}\", a, b.to_string());","highlight_start":38,"highlight_end":50}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `to_string` applied to a type that implements `Display` in `format!` args\n  --> tests/ui/format_args.rs:147:38\n   |\nLL |         let x = format!(\"{} {}\", a, b.to_string());\n   |                                      ^^^^^^^^^^^^ help: remove this\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-   = note: `-D clippy::format-in-format-args` implied by `-D warnings`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{}: {}", error, format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{:?}: {}", error, format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{{}}: {}", format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!(r#"error: "{}""#, format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!(r#"something failed at "{}""#, Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!("something failed at {} {0}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
 error: `format!` in `format!` args
    |
    |
 LL |     let _ = format!("error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `format!(..)` call
    = help: or consider changing `format!` to `format_args!`
+   = note: `-D clippy::format-in-format-args` implied by `-D warnings`
 
 error: `format!` in `write!` args
    |
 LL |       let _ = write!(
    |  _____________^
 LL | |         stdout(),
 LL | |         stdout(),
 LL | |         "error: {}",
 LL | |         format!("something failed at {}", Location::caller())
 LL | |     );
    |
    |
    = help: combine the `format!(..)` arguments with the outer `write!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
-error: `format!` in `writeln!` args
-   |
-LL |       let _ = writeln!(
-   |  _____________^
-LL | |         stdout(),
-LL | |         stdout(),
-LL | |         "error: {}",
-LL | |         format!("something failed at {}", Location::caller())
-LL | |     );
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `writeln!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
 error: `format!` in `print!` args
    |
    |
 LL |     print!("error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `print!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
 error: `format!` in `eprint!` args
    |
    |
 LL |     eprint!("error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `eprint!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
-error: `format!` in `eprintln!` args
-   |
-   |
-LL |     eprintln!("error: {}", format!("something failed at {}", Location::caller()));
-   |
-   |
-   = help: combine the `format!(..)` arguments with the outer `eprintln!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
 error: `format!` in `format_args!` args
    |
    |
 LL |     let _ = format_args!("error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `format_args!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
 error: `format!` in `assert!` args
    |
    |
 LL |     assert!(true, "error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `assert!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
 error: `format!` in `assert_eq!` args
    |
    |
 LL |     assert_eq!(0, 0, "error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `assert_eq!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
 error: `format!` in `assert_ne!` args
    |
    |
 LL |     assert_ne!(0, 0, "error: {}", format!("something failed at {}", Location::caller()));
    |
    |
    = help: combine the `format!(..)` arguments with the outer `assert_ne!(..)` call
    = help: or consider changing `format!` to `format_args!`
 
 error: `format!` in `panic!` args
    |
 LL |     panic!("error: {}", format!("something failed at {}", Location::caller()));
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: combine the `format!(..)` arguments with the outer `panic!(..)` call
    = help: or consider changing `format!` to `format_args!`
-error: aborting due to 18 previous errors
+error: aborting due to 9 previous errors
 
 
---
To only update this specific test, also pass `--test-args format_args_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format_args_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/format_args_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`format!` in `format!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1273,"byte_end":1348,"line_start":33,"line_end":33,"column_start":13,"column_end":88,"is_primary":true,"text":[{"text":"    let _ = format!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":13,"highlight_end":88}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `format!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::format-in-format-args` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `format!` args\n  --> tests/ui/format_args_unfixable.rs:33:13\n   |\nLL |     let _ = format!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `format!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n   = note: `-D clippy::format-in-format-args` implied by `-D warnings`\n\n"}
{"message":"`format!` in `write!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1362,"byte_end":1476,"line_start":34,"line_end":38,"column_start":13,"column_end":6,"is_primary":true,"text":[{"text":"    let _ = write!(","highlight_start":13,"highlight_end":20},{"text":"        stdout(),","highlight_start":1,"highlight_end":18},{"text":"        \"error: {}\",","highlight_start":1,"highlight_end":21},{"text":"        format!(\"something failed at {}\", Location::caller())","highlight_start":1,"highlight_end":62},{"text":"    );","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `write!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `write!` args\n  --> tests/ui/format_args_unfixable.rs:34:13\n   |\nLL |       let _ = write!(\n   |  _____________^\nLL | |         stdout(),\nLL | |         \"error: {}\",\nLL | |         format!(\"something failed at {}\", Location::caller())\nLL | |     );\n   | |_____^\n   |\n   = help: combine the `format!(..)` arguments with the outer `write!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `print!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1612,"byte_end":1686,"line_start":44,"line_end":44,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    print!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `print!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `print!` args\n  --> tests/ui/format_args_unfixable.rs:44:5\n   |\nLL |     print!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `print!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `eprint!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1692,"byte_end":1767,"line_start":45,"line_end":45,"column_start":5,"column_end":80,"is_primary":true,"text":[{"text":"    eprint!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `eprint!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `eprint!` args\n  --> tests/ui/format_args_unfixable.rs:45:5\n   |\nLL |     eprint!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `eprint!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `format_args!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1864,"byte_end":1944,"line_start":47,"line_end":47,"column_start":13,"column_end":93,"is_primary":true,"text":[{"text":"    let _ = format_args!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":13,"highlight_end":93}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `format_args!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `format_args!` args\n  --> tests/ui/format_args_unfixable.rs:47:13\n   |\nLL |     let _ = format_args!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `format_args!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `assert!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":1950,"byte_end":2031,"line_start":48,"line_end":48,"column_start":5,"column_end":86,"is_primary":true,"text":[{"text":"    assert!(true, \"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":86}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `assert!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `assert!` args\n  --> tests/ui/format_args_unfixable.rs:48:5\n   |\nLL |     assert!(true, \"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `assert!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `assert_eq!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":2037,"byte_end":2121,"line_start":49,"line_end":49,"column_start":5,"column_end":89,"is_primary":true,"text":[{"text":"    assert_eq!(0, 0, \"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `assert_eq!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `assert_eq!` args\n  --> tests/ui/format_args_unfixable.rs:49:5\n   |\nLL |     assert_eq!(0, 0, \"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `assert_eq!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `assert_ne!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":2127,"byte_end":2211,"line_start":50,"line_end":50,"column_start":5,"column_end":89,"is_primary":true,"text":[{"text":"    assert_ne!(0, 0, \"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":89}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `assert_ne!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `assert_ne!` args\n  --> tests/ui/format_args_unfixable.rs:50:5\n   |\nLL |     assert_ne!(0, 0, \"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `assert_ne!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}
{"message":"`format!` in `panic!` args","code":{"code":"clippy::format_in_format_args","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format_args_unfixable.rs","byte_start":2217,"byte_end":2291,"line_start":51,"line_end":51,"column_start":5,"column_end":79,"is_primary":true,"text":[{"text":"    panic!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));","highlight_start":5,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"combine the `format!(..)` arguments with the outer `panic!(..)` call","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or consider changing `format!` to `format_args!`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `format!` in `panic!` args\n  --> tests/ui/format_args_unfixable.rs:51:5\n   |\nLL |     panic!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: combine the `format!(..)` arguments with the outer `panic!(..)` call\n   = help: or consider changing `format!` to `format_args!`\n\n"}

------------------------------------------

diff of stderr:
---
-   |     ^^^^^^^^^--^
-   |              |
-   |              help: remove the empty string
-   |
-   = note: `-D clippy::println-empty-string` implied by `-D warnings`
-error: empty string literal in `println!`
-  --> $DIR/println_empty_string.rs:9:14
-   |
-   |
-LL |         _ => println!(""),
-   |              ^^^^^^^^^--^
-   |                       help: remove the empty string
-
-
-error: empty string literal in `eprintln!`
-   |
-LL |     eprintln!("");
-   |     ^^^^^^^^^^--^
-   |               |
-   |               |
-   |               help: remove the empty string
-
-error: empty string literal in `eprintln!`
-   |
-   |
-LL |         _ => eprintln!(""),
-   |              ^^^^^^^^^^--^
-   |                        help: remove the empty string
-
-error: aborting due to 4 previous errors
-
-
-

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![allow(clippy::match_single_binding)]
 fn main() {
     println!();
-    println!();
+    println!("");
+    println!("");
 
     match "a" {
-        _ => println!(),
+        _ => println!(""),
 
     eprintln!();
-    eprintln!();
+    eprintln!("");
+    eprintln!("");
 
     match "a" {
-        _ => eprintln!(),
+        _ => eprintln!(""),
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id.fixed
To only update this specific test, also pass `--test-args println_empty_string.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/println_empty_string.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

 error: literal with an empty format string
   --> $DIR/print_literal.rs:27:24
    |
 LL |     print!("Hello {}", "world");
    |
    |
    = note: `-D clippy::print-literal` implied by `-D warnings`
    |
    |
 LL -     print!("Hello {}", "world");
 LL +     print!("Hello world");
 
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:28:36
-   |
-   |
-LL |     println!("Hello {} {}", world, "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("Hello {} {}", world, "world");
-LL +     println!("Hello {} world", world);
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:29:26
-   |
-   |
-LL |     println!("Hello {}", "world");
-   |                          ^^^^^^^
-   |
-help: try this
-   |
-LL -     println!("Hello {}", "world");
-LL +     println!("Hello world");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:30:26
-   |
-   |
-LL |     println!("{} {:.4}", "a literal", 5);
-   |
-help: try this
-   |
-   |
-LL -     println!("{} {:.4}", "a literal", 5);
-LL +     println!("a literal {:.4}", 5);
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:35:25
-   |
-   |
-LL |     println!("{0} {1}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{0} {1}", "hello", "world");
-LL +     println!("hello {1}", "world");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:35:34
-   |
-   |
-LL |     println!("{0} {1}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{0} {1}", "hello", "world");
-LL +     println!("{0} world", "hello");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:36:34
-   |
-   |
-LL |     println!("{1} {0}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{1} {0}", "hello", "world");
-LL +     println!("world {0}", "hello");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:36:25
-   |
-   |
-LL |     println!("{1} {0}", "hello", "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{1} {0}", "hello", "world");
-LL +     println!("{1} hello", "world");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:39:35
-   |
-   |
-LL |     println!("{foo} {bar}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{foo} {bar}", foo = "hello", bar = "world");
-LL +     println!("hello {bar}", bar = "world");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:39:50
-   |
-   |
-LL |     println!("{foo} {bar}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{foo} {bar}", foo = "hello", bar = "world");
-LL +     println!("{foo} world", foo = "hello");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:40:50
-   |
-   |
-LL |     println!("{bar} {foo}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{bar} {foo}", foo = "hello", bar = "world");
-LL +     println!("world {foo}", foo = "hello");
-
-error: literal with an empty format string
-  --> $DIR/print_literal.rs:40:35
-   |
-   |
-LL |     println!("{bar} {foo}", foo = "hello", bar = "world");
-   |
-help: try this
-   |
-   |
-LL -     println!("{bar} {foo}", foo = "hello", bar = "world");
-LL +     println!("{bar} hello", bar = "world");
-
-error: aborting due to 12 previous errors
+error: aborting due to previous error
 
---
To only update this specific test, also pass `--test-args print_literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/print_literal.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_literal.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/print_literal.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"literal with an empty format string","code":{"code":"clippy::print_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/print_literal.rs","byte_start":1027,"byte_end":1034,"line_start":27,"line_end":27,"column_start":24,"column_end":31,"is_primary":true,"text":[{"text":"    print!(\"Hello {}\", \"world\");","highlight_start":24,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::print-literal` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/print_literal.rs","byte_start":1022,"byte_end":1024,"line_start":27,"line_end":27,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"    print!(\"Hello {}\", \"world\");","highlight_start":19,"highlight_end":21}],"label":null,"suggested_replacement":"world","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/print_literal.rs","byte_start":1025,"byte_end":1034,"line_start":27,"line_end":27,"column_start":22,"column_end":31,"is_primary":true,"text":[{"text":"    print!(\"Hello {}\", \"world\");","highlight_start":22,"highlight_end":31}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/print_literal.rs:27:24\n   |\nLL |     print!(\"Hello {}\", \"world\");\n   |                        ^^^^^^^\n   |\n   = note: `-D clippy::print-literal` implied by `-D warnings`\nhelp: try this\n   |\nLL -     print!(\"Hello {}\", \"world\");\nLL +     print!(\"Hello world\");\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{}'", var);
-   |
-   |
-   = note: `-D clippy::uninlined-format-args` implied by `-D warnings`
-   |
-   |
-LL -     println!("val='{}'", var);
-LL +     println!("val='{var}'");
-
-error: aborting due to previous error
-
-
-

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id.edition2018.stderr
diff of fixed:

 //@revisions: edition2018 edition2021
 //@[edition2018] edition:2018
 //@[edition2021] edition:2021
 //@run-rustfix
 
 #![warn(clippy::uninlined_format_args)]
 fn main() {
     let var = 1;
 
 
-    println!("val='{var}'");
+    println!("val='{}'", var);
 
     if var > 0 {
         panic!("p1 {}", var);
     }
     if var > 0 {
         panic!("p2 {0}", var);
     }
     if var > 0 {
         panic!("p3 {var}", var = var);
 
 
     #[allow(non_fmt_panics)]
     {
         if var > 0 {
             panic!("p4 {var}");
     }
 
 
     assert!(var == 1, "p5 {}", var);
     debug_assert!(var == 1, "p6 {}", var);
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id.edition2018.fixed
To only update this specific test, also pass `--test-args uninlined_format_args_panic.rs`

error in revision `edition2018`: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/uninlined_format_args_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition2018" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

-error: format specifiers have no effect on `format_args!()`
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
-   = help: for the width to apply consider using `format!()`
-help: if the current behavior is intentional, remove the format specifiers
-   |
-LL -     println!("{:5}.", format_args_from_macro!());
-LL +     println!("{}.", format_args_from_macro!());
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:19:15
-   |
-LL |     println!("{args:5}");
-   |
-   |
-   = help: for the width to apply consider using `format!()`
-help: if the current behavior is intentional, remove the format specifiers
-   |
-LL -     println!("{args:5}");
-LL +     println!("{args}");
-
-error: aborting due to 4 previous errors
-
-
---
To only update this specific test, also pass `--test-args unused_format_specs_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_format_specs_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{}'", local_i32);
-   |
-   |
-   = note: `-D clippy::uninlined-format-args` implied by `-D warnings`
-   |
-   |
-LL -     println!("val='{}'", local_i32);
-LL +     println!("val='{local_i32}'");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{   }'", local_i32); // 3 spaces
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{   }'", local_i32); // 3 spaces
-LL +     println!("val='{local_i32}'"); // 3 spaces
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{    }'", local_i32); // tab
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{    }'", local_i32); // tab
-LL +     println!("val='{local_i32}'"); // tab
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{     }'", local_i32); // space+tab
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{     }'", local_i32); // space+tab
-LL +     println!("val='{local_i32}'"); // space+tab
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{     }'", local_i32); // tab+space
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{     }'", local_i32); // tab+space
-LL +     println!("val='{local_i32}'"); // tab+space
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL | /     println!(
-LL | /     println!(
-LL | |         "val='{
-LL | |     }'",
-LL | |         local_i32
-LL | |     );
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{}", local_i32);
-LL +     println!("{local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{}", fn_arg);
-   |     ^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{}", fn_arg);
-LL +     println!("{fn_arg}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{:?}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{:?}", local_i32);
-LL +     println!("{local_i32:?}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{:#?}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{:#?}", local_i32);
-LL +     println!("{local_i32:#?}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:4}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:4}", local_i32);
-LL +     println!("{local_i32:4}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:04}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:04}", local_i32);
-LL +     println!("{local_i32:04}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:<3}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:<3}", local_i32);
-LL +     println!("{local_i32:<3}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{:#010x}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{:#010x}", local_i32);
-LL +     println!("{local_i32:#010x}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:.1}", local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:.1}", local_f64);
-LL +     println!("{local_f64:.1}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{} {}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{} {}", local_i32, local_f64);
-LL +     println!("{local_i32} {local_f64}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{}", val);
-   |     ^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{}", val);
-LL +     println!("{val}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{}", v = val);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{}", v = val);
-LL +     println!("{val}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{/t }'", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{/t }'", local_i32);
-LL +     println!("val='{local_i32}'");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{/n }'", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{/n }'", local_i32);
-LL +     println!("val='{local_i32}'");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{local_i32}'", local_i32 = local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{local_i32}'", local_i32 = local_i32);
-LL +     println!("val='{local_i32}'");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{local_i32}'", local_i32 = fn_arg);
-   |
-help: change this to
-   |
-   |
-LL -     println!("val='{local_i32}'", local_i32 = fn_arg);
-LL +     println!("val='{fn_arg}'");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{0}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{0}", local_i32);
-LL +     println!("{local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{0:?}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{0:?}", local_i32);
-LL +     println!("{local_i32:?}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{0:#?}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{0:#?}", local_i32);
-LL +     println!("{local_i32:#?}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0:04}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0:04}", local_i32);
-LL +     println!("{local_i32:04}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0:<3}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0:<3}", local_i32);
-LL +     println!("{local_i32:<3}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-LL |     println!("{0:#010x}", local_i32);
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   |
-help: change this to
-   |
-LL -     println!("{0:#010x}", local_i32);
-LL +     println!("{local_i32:#010x}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0:.1}", local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0:.1}", local_f64);
-LL +     println!("{local_f64:.1}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0} {0}", local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0} {0}", local_i32);
-LL +     println!("{local_i32} {local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {} {0} {}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {} {0} {}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32} {local_i32} {local_f64}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0} {1}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{0} {1}", local_i32, local_f64);
-LL +     println!("{local_i32} {local_f64}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {0}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {0}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{1} {0} {1} {0}", local_i32, local_f64);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{1} {0} {1} {0}", local_i32, local_f64);
-LL +     println!("{local_f64} {local_i32} {local_f64} {local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{v}", v = local_i32);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{v}", v = local_i32);
-LL +     println!("{local_i32}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:0$}", width);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:0$}", width);
-LL +     println!("{local_i32:width$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:w$}", w = width);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:w$}", w = width);
-LL +     println!("{local_i32:width$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:.0$}", prec);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:.0$}", prec);
-LL +     println!("{local_i32:.prec$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{local_i32:.p$}", p = prec);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{local_i32:.p$}", p = prec);
-LL +     println!("{local_i32:.prec$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{:0$}", v = val);
-   |
-help: change this to
-   |
-   |
-LL -     println!("{:0$}", v = val);
-LL +     println!("{val:val$}");
-
-
-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("{0:0$}", v = val);
-   |
---
To only update this specific test, also pass `--test-args write_literal_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_literal_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

-error: empty string literal in `writeln!`
-  --> $DIR/writeln_empty_string.rs:11:5
-   |
-LL |     writeln!(v, "");
-   |               |
-   |               help: remove the empty string
-   |
-   |
-   = note: `-D clippy::writeln-empty-string` implied by `-D warnings`
-error: empty string literal in `writeln!`
-  --> $DIR/writeln_empty_string.rs:14:5
-   |
-   |
-LL |     writeln!(suggestion, "");
-   |                        |
-   |                        help: remove the empty string
-
-error: aborting due to 2 previous errors
-error: aborting due to 2 previous errors
-
-

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id.stderr
diff of fixed:

 //@run-rustfix
 #![allow(unused_must_use)]
 #![allow(unused_must_use)]
 #![warn(clippy::writeln_empty_string)]
 
 fn main() {
 fn main() {
     let mut v = Vec::new();
     // These should fail
-    writeln!(v);
-    writeln!(v);
+    writeln!(v, "");
     let mut suggestion = Vec::new();
-    writeln!(suggestion);
-    writeln!(suggestion);
+    writeln!(suggestion, "");
     // These should be fine
     writeln!(v);
     writeln!(v);
     writeln!(v, " ");
     write!(v, "");
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id.fixed
To only update this specific test, also pass `--test-args writeln_empty_string.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/writeln_empty_string.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

 error: literal with an empty format string
   --> $DIR/write_literal.rs:31:27
    |
 LL |     write!(v, "Hello {}", "world");
    |
    |
    = note: `-D clippy::write-literal` implied by `-D warnings`
    |
    |
 LL -     write!(v, "Hello {}", "world");
 LL +     write!(v, "Hello world");
 
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
+error: aborting due to previous error
 
---
To only update this specific test, also pass `--test-args write_literal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_literal.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"literal with an empty format string","code":{"code":"clippy::write_literal","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/write_literal.rs","byte_start":1151,"byte_end":1158,"line_start":31,"line_end":31,"column_start":27,"column_end":34,"is_primary":true,"text":[{"text":"    write!(v, \"Hello {}\", \"world\");","highlight_start":27,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::write-literal` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/write_literal.rs","byte_start":1146,"byte_end":1148,"line_start":31,"line_end":31,"column_start":22,"column_end":24,"is_primary":true,"text":[{"text":"    write!(v, \"Hello {}\", \"world\");","highlight_start":22,"highlight_end":24}],"label":null,"suggested_replacement":"world","suggestion_applicability":"MachineApplicable","expansion":null},{"file_name":"tests/ui/write_literal.rs","byte_start":1149,"byte_end":1158,"line_start":31,"line_end":31,"column_start":25,"column_end":34,"is_primary":true,"text":[{"text":"    write!(v, \"Hello {}\", \"world\");","highlight_start":25,"highlight_end":34}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: literal with an empty format string\n  --> tests/ui/write_literal.rs:31:27\n   |\nLL |     write!(v, \"Hello {}\", \"world\");\n   |                           ^^^^^^^\n   |\n   = note: `-D clippy::write-literal` implied by `-D warnings`\nhelp: try this\n   |\nLL -     write!(v, \"Hello {}\", \"world\");\nLL +     write!(v, \"Hello world\");\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
