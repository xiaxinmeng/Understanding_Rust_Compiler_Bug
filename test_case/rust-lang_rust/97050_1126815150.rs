plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between b36be12d979de84a063f617295674a40d6ddf16d and aaf0d3cf4c0d3849b95a8c9747dad00548e77747
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

-error: use of `write!(stdout(), ...).unwrap()`
-   |
-LL |         write!(std::io::stdout(), "test").unwrap();
-LL |         write!(std::io::stdout(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!("test")`
-   |
-   = note: `-D clippy::explicit-write` implied by `-D warnings`
-
-error: use of `write!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         write!(std::io::stderr(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!("test")`
-
-error: use of `writeln!(stdout(), ...).unwrap()`
-   |
-LL |         writeln!(std::io::stdout(), "test").unwrap();
-LL |         writeln!(std::io::stdout(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test")`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test")`
-
 error: use of `stdout().write_fmt(...).unwrap()`
    |
    |
 LL |         std::io::stdout().write_fmt(format_args!("test")).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!("test")`
+   |
+   = note: `-D clippy::explicit-write` implied by `-D warnings`
 
 error: use of `stderr().write_fmt(...).unwrap()`
    |
    |
 LL |         std::io::stderr().write_fmt(format_args!("test")).unwrap();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!("test")`
 
-error: use of `writeln!(stdout(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stdout(), "test/ntest").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test/ntest")`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "test/ntest").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test/ntest")`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "with {}", value).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {}", value)`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {} {}", 2, value)`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "with {value}").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("with {value}")`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("macro arg {}", one!())`
-error: aborting due to 12 previous errors
+error: aborting due to 2 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![allow(unused_imports)]
 #![warn(clippy::explicit_write)]
 fn stdout() -> String {
     String::new()
 }
 
---
 fn main() {
     // these should warn
     {
         use std::io::Write;
+        write!(std::io::stdout(), "test").unwrap();
+        write!(std::io::stderr(), "test").unwrap();
+        writeln!(std::io::stdout(), "test").unwrap();
+        writeln!(std::io::stderr(), "test").unwrap();
         print!("test");
         eprint!("test");
-        println!("test");
-        eprintln!("test");
-        print!("test");
-        eprint!("test");
         // including newlines
         // including newlines
-        println!("test\ntest");
-        eprintln!("test\ntest");
+        writeln!(std::io::stdout(), "test\ntest").unwrap();
+        writeln!(std::io::stderr(), "test\ntest").unwrap();
         let value = 1;
         let value = 1;
-        eprintln!("with {}", value);
-        eprintln!("with {} {}", 2, value);
-        eprintln!("with {value}");
-        eprintln!("macro arg {}", one!());
+        writeln!(std::io::stderr(), "with {}", value).unwrap();
+        writeln!(std::io::stderr(), "with {} {}", 2, value).unwrap();
+        writeln!(std::io::stderr(), "with {value}").unwrap();
+        writeln!(std::io::stderr(), "macro arg {}", one!()).unwrap();
     }
     // these should not warn, different destination
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-c9e9186f6eb34428.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a83be702bffbb861.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `stdout().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":507,"byte_end":565,"line_start":27,"line_end":27,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::explicit-write` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":507,"byte_end":565,"line_start":27,"line_end":27,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"print!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stdout().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:27:9\n   |\nLL |         std::io::stdout().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!(\"test\")`\n   |\n   = note: `-D clippy::explicit-write` implied by `-D warnings`\n\n"}
{"message":"use of `stderr().write_fmt(...).unwrap()`","code":{"code":"clippy::explicit_write","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":575,"byte_end":633,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/explicit_write.rs","byte_start":575,"byte_end":633,"line_start":28,"line_end":28,"column_start":9,"column_end":67,"is_primary":true,"text":[{"text":"        std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();","highlight_start":9,"highlight_end":67}],"label":null,"suggested_replacement":"eprint!(\"test\")","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `stderr().write_fmt(...).unwrap()`\n  --> tests/ui/explicit_write.rs:28:9\n   |\nLL |         std::io::stderr().write_fmt(format_args!(\"test\")).unwrap();\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!(\"test\")`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
