plain

---- compile_test stdout ----
diff of stderr:

-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |     writeln!(std::io::stderr(), "foo {}", bar).unwrap();
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::explicit-write` implied by `-D warnings`
-   = help: consider using `eprintln!` instead
-error: aborting due to previous error
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write_non_rustfix.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args explicit_write_non_rustfix.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write_non_rustfix.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write_non_rustfix.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write_non_rustfix.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

-error: use of `write!(stdout(), ...).unwrap()`
-   |
-   |
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
-   |
-LL |         writeln!(std::io::stdout(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `println!("test")`
-
-error: use of `writeln!(stderr(), ...).unwrap()`
-   |
-   |
-LL |         writeln!(std::io::stderr(), "test").unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprintln!("test")`
-
-error: use of `stdout().write_fmt(...).unwrap()`
-   |
-   |
-LL |         std::io::stdout().write_fmt(format_args!("test")).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `print!("test")`
-
-error: use of `stderr().write_fmt(...).unwrap()`
-   |
-   |
-LL |         std::io::stderr().write_fmt(format_args!("test")).unwrap();
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `eprint!("test")`
-
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
-error: aborting due to 8 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write.stage-id.stderr
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
-        print!("test");
-        eprint!("test");
-        println!("test");
-        eprintln!("test");
-        print!("test");
-        eprint!("test");
+        write!(std::io::stdout(), "test").unwrap();
+        write!(std::io::stderr(), "test").unwrap();
+        writeln!(std::io::stdout(), "test").unwrap();
+        writeln!(std::io::stderr(), "test").unwrap();
+        std::io::stdout().write_fmt(format_args!("test")).unwrap();
+        std::io::stderr().write_fmt(format_args!("test")).unwrap();
         // including newlines
         // including newlines
-        println!("test\ntest");
-        eprintln!("test\ntest");
+        writeln!(std::io::stdout(), "test\ntest").unwrap();
+        writeln!(std::io::stderr(), "test\ntest").unwrap();
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
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write.stage-id.fixed
To only update this specific test, also pass `--test-args explicit_write.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/explicit_write.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: use of `expect` followed by a function call
    |
    |
 LL |     with_none_and_format.expect(&format!("Error {}: fake error", error_code));
-   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
    |
    |
    = note: `-D clippy::expect-fun-call` implied by `-D warnings`
+   |
+   |
+LL ~     with_none_and_format.unwrap_or_else(|| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |     with_none_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
-   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
+   |
+help: try this
+   |
+   |
+LL ~     with_none_and_as_str.unwrap_or_else(|| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |     with_err_and_format.expect(&format!("Error {}: fake error", error_code));
-   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
+   |
+help: try this
+   |
+   |
+LL ~     with_err_and_format.unwrap_or_else(|_| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |     with_err_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
-   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
+   |
+help: try this
+   |
+   |
+LL ~     with_err_and_as_str.unwrap_or_else(|_| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |     Some("foo").expect(format!("{} {}", 1, 2).as_ref());
-   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{} {}", 1, 2))`
+   |
+help: try this
+   |
+   |
+LL ~     Some("foo").unwrap_or_else(|| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         Some("foo").expect(&get_string());
    |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         Some("foo").expect(get_string().as_ref());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         Some("foo").expect(get_string().as_str());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_string()) })`
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         Some("foo").expect(get_static_str());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_static_str()) })`
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         Some("foo").expect(get_non_static_str(&0));
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!("{}", get_non_static_str(&0).to_string()) })`
 
 error: use of `expect` followed by a function call
    |
    |
 LL |     Some(true).expect(&format!("key {}, {}", 1, 2));
-   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("key {}, {}", 1, 2))`
+   |
+help: try this
+   |
+   |
+LL ~     Some(true).unwrap_or_else(|| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
 
 
 error: use of `expect` followed by a function call
    |
    |
 LL |         opt_ref.expect(&format!("{:?}", opt_ref));
-   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("{:?}", opt_ref))`
+   |
+help: try this
+   |
+   |
+LL ~         opt_ref.unwrap_or_else(|| { panic!("{}", {
+LL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
+LL +         res
+LL ~     }) });
error: test failed, to rerun pass '--test compile-test'
 
 error: aborting due to 12 previous errors
 
---
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::expect_fun_call)]
 #![allow(clippy::to_string_in_format_args)]
 
 /// Checks implementation of the `EXPECT_FUN_CALL` lint
 fn main() {
     struct Foo;
 
     impl Foo {
     impl Foo {
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
-    with_none_and_format.unwrap_or_else(|| panic!("Error {}: fake error", error_code));
+    with_none_and_format.unwrap_or_else(|| { panic!("{}", {
+        res
+    }) });
 
 
     let with_none_and_as_str: Option<i32> = None;
-    with_none_and_as_str.unwrap_or_else(|| panic!("Error {}: fake error", error_code));
+    with_none_and_as_str.unwrap_or_else(|| { panic!("{}", {
+        res
+    }) });
 
 
     let with_ok: Result<(), ()> = Ok(());
     with_ok.expect("error");
 
     let with_err: Result<(), ()> = Err(());
     with_err.expect("error");
     let error_code = 123_i32;
     let error_code = 123_i32;
     let with_err_and_format: Result<(), ()> = Err(());
-    with_err_and_format.unwrap_or_else(|_| panic!("Error {}: fake error", error_code));
+    with_err_and_format.unwrap_or_else(|_| { panic!("{}", {
+        res
+    }) });
 
 
     let with_err_and_as_str: Result<(), ()> = Err(());
-    with_err_and_as_str.unwrap_or_else(|_| panic!("Error {}: fake error", error_code));
+    with_err_and_as_str.unwrap_or_else(|_| { panic!("{}", {
+        res
+    }) });
 
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
+    Some("foo").unwrap_or_else(|| { panic!("{}", {
+        res
+    }) });
 
     //Issue #2979 - this should not lint
     //Issue #2979 - this should not lint
     {
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
+    Some(true).unwrap_or_else(|| { panic!("{}", {
+        res
+    }) });
 
 
     //Issue #4912 - the receiver is a &Option
         let opt = Some(1);
         let opt_ref = &opt;
         let opt_ref = &opt;
-        opt_ref.unwrap_or_else(|| panic!("{:?}", opt_ref));
+        opt_ref.unwrap_or_else(|| { panic!("{}", {
+        res
+    }) });
     }
 }
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/expect_fun_call.stage-id.fixed
To only update this specific test, also pass `--test-args expect_fun_call.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/expect_fun_call.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/expect_fun_call.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/expect_fun_call.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":578,"byte_end":630,"line_start":29,"line_end":29,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::expect-fun-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":578,"byte_end":630,"line_start":29,"line_end":29,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:29:26\n   |\nLL |     with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::expect-fun-call` implied by `-D warnings`\nhelp: try this\n   |\nLL ~     with_none_and_format.unwrap_or_else(|| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":708,"byte_end":768,"line_start":32,"line_end":32,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":708,"byte_end":768,"line_start":32,"line_end":32,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:32:26\n   |\nLL |     with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~     with_none_and_as_str.unwrap_or_else(|| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1027,"byte_end":1079,"line_start":42,"line_end":42,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1027,"byte_end":1079,"line_start":42,"line_end":42,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":"unwrap_or_else(|_| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:42:25\n   |\nLL |     with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~     with_err_and_format.unwrap_or_else(|_| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1161,"byte_end":1221,"line_start":45,"line_end":45,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1161,"byte_end":1221,"line_start":45,"line_end":45,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":"unwrap_or_else(|_| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:45:25\n   |\nLL |     with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~     with_err_and_as_str.unwrap_or_else(|_| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1626,"byte_end":1665,"line_start":57,"line_end":57,"column_start":17,"column_end":56,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect(format!(\"{} {}\", 1, 2).as_ref());","highlight_start":17,"highlight_end":56}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1626,"byte_end":1665,"line_start":57,"line_end":57,"column_start":17,"column_end":56,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect(format!(\"{} {}\", 1, 2).as_ref());","highlight_start":17,"highlight_end":56}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:57:17\n   |\nLL |     Some(\"foo\").expect(format!(\"{} {}\", 1, 2).as_ref());\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~     Some(\"foo\").unwrap_or_else(|| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2036,"byte_end":2057,"line_start":78,"line_end":78,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(&get_string());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2036,"byte_end":2057,"line_start":78,"line_end":78,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(&get_string());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:78:21\n   |\nLL |         Some(\"foo\").expect(&get_string());\n   |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2079,"byte_end":2108,"line_start":79,"line_end":79,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_ref());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2079,"byte_end":2108,"line_start":79,"line_end":79,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_ref());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:79:21\n   |\nLL |         Some(\"foo\").expect(get_string().as_ref());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2130,"byte_end":2159,"line_start":80,"line_end":80,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_str());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2130,"byte_end":2159,"line_start":80,"line_end":80,"column_start":21,"column_end":50,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_string().as_str());","highlight_start":21,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:80:21\n   |\nLL |         Some(\"foo\").expect(get_string().as_str());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2182,"byte_end":2206,"line_start":82,"line_end":82,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_static_str());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2182,"byte_end":2206,"line_start":82,"line_end":82,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_static_str());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_static_str()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:82:21\n   |\nLL |         Some(\"foo\").expect(get_static_str());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_static_str()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2228,"byte_end":2258,"line_start":83,"line_end":83,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_non_static_str(&0));","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2228,"byte_end":2258,"line_start":83,"line_end":83,"column_start":21,"column_end":51,"is_primary":true,"text":[{"text":"        Some(\"foo\").expect(get_non_static_str(&0));","highlight_start":21,"highlight_end":51}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", get_non_static_str(&0).to_string()) })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:83:21\n   |\nLL |         Some(\"foo\").expect(get_non_static_str(&0));\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { panic!(\"{}\", get_non_static_str(&0).to_string()) })`\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2300,"byte_end":2336,"line_start":87,"line_end":87,"column_start":16,"column_end":52,"is_primary":true,"text":[{"text":"    Some(true).expect(&format!(\"key {}, {}\", 1, 2));","highlight_start":16,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2300,"byte_end":2336,"line_start":87,"line_end":87,"column_start":16,"column_end":52,"is_primary":true,"text":[{"text":"    Some(true).expect(&format!(\"key {}, {}\", 1, 2));","highlight_start":16,"highlight_end":52}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:87:16\n   |\nLL |     Some(true).expect(&format!(\"key {}, {}\", 1, 2));\n   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~     Some(true).unwrap_or_else(|| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}
{"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2462,"byte_end":2495,"line_start":93,"line_end":93,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"        opt_ref.expect(&format!(\"{:?}\", opt_ref));","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2462,"byte_end":2495,"line_start":93,"line_end":93,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"        opt_ref.expect(&format!(\"{:?}\", opt_ref));","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| { panic!(\"{}\", {\n        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\n        res\n    }) })","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:93:17\n   |\nLL |         opt_ref.expect(&format!(\"{:?}\", opt_ref));\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nhelp: try this\n   |\nLL ~         opt_ref.unwrap_or_else(|| { panic!(\"{}\", {\nLL +         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));\nLL +         res\nLL ~     }) });\n   |\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: `to_string` applied to a type that implements `Display` in `format!` args
-   |
-   |
-LL |     let _ = format!("error: something failed at {}", Location::caller().to_string());
-   |                                                                        ^^^^^^^^^^^^ help: remove this
-   |
-   = note: `-D clippy::to-string-in-format-args` implied by `-D warnings`
-
-error: `to_string` applied to a type that implements `Display` in `write!` args
-   |
-LL |         Location::caller().to_string()
-   |                           ^^^^^^^^^^^^ help: remove this
-
-
-error: `to_string` applied to a type that implements `Display` in `writeln!` args
-   |
-LL |         Location::caller().to_string()
-   |                           ^^^^^^^^^^^^ help: remove this
-
-
-error: `to_string` applied to a type that implements `Display` in `print!` args
-   |
-   |
-LL |     print!("error: something failed at {}", Location::caller().to_string());
-   |                                                               ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("error: something failed at {}", Location::caller().to_string());
-   |                                                                 ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `eprint!` args
-   |
-   |
-LL |     eprint!("error: something failed at {}", Location::caller().to_string());
-   |                                                                ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `eprintln!` args
-   |
-   |
-LL |     eprintln!("error: something failed at {}", Location::caller().to_string());
-   |                                                                  ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `format_args!` args
-   |
-   |
-LL |     let _ = format_args!("error: something failed at {}", Location::caller().to_string());
-   |                                                                             ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `assert!` args
-   |
-   |
-LL |     assert!(true, "error: something failed at {}", Location::caller().to_string());
-   |                                                                      ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `assert_eq!` args
-   |
-   |
-LL |     assert_eq!(0, 0, "error: something failed at {}", Location::caller().to_string());
-   |                                                                         ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `assert_ne!` args
-   |
-   |
-LL |     assert_ne!(0, 0, "error: something failed at {}", Location::caller().to_string());
-   |                                                                         ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `panic!` args
-   |
-   |
-LL |     panic!("error: something failed at {}", Location::caller().to_string());
-   |                                                               ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
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
-   |                        ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{}", x.to_string());
-   |                    ^^^^^^^^^^^^^ help: use this: `**x`
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{}", x_ref.to_string());
-   |                    ^^^^^^^^^^^^^^^^^ help: use this: `***x_ref`
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", foo = "foo".to_string(), bar = "bar");
-   |                                       ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", foo = "foo", bar = "bar".to_string());
-   |                                                    ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", bar = "bar".to_string(), foo = "foo");
-   |                                       ^^^^^^^^^^^^ help: remove this
-
-error: `to_string` applied to a type that implements `Display` in `println!` args
-   |
-   |
-LL |     println!("{foo}{bar}", bar = "bar", foo = "foo".to_string());
-   |                                                    ^^^^^^^^^^^^ help: remove this
-error: aborting due to 21 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![allow(unreachable_code)]
 #![allow(unused_macros)]
 #![allow(unused_variables)]
 #![allow(clippy::assertions_on_constants)]
 #![allow(clippy::eq_op)]
 #![allow(clippy::print_literal)]
 #![warn(clippy::to_string_in_format_args)]
 
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
     type Target = u32;
 
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
 
-    let _ = format!("error: something failed at {}", Location::caller());
+    let _ = format!("error: something failed at {}", Location::caller().to_string());
     let _ = write!(
         stdout(),
         "error: something failed at {}",
-        Location::caller()
+        Location::caller().to_string()
     let _ = writeln!(
         stdout(),
         stdout(),
         "error: something failed at {}",
-        Location::caller()
+        Location::caller().to_string()
     );
-    print!("error: something failed at {}", Location::caller());
-    println!("error: something failed at {}", Location::caller());
-    eprint!("error: something failed at {}", Location::caller());
-    eprintln!("error: something failed at {}", Location::caller());
-    let _ = format_args!("error: something failed at {}", Location::caller());
-    assert!(true, "error: something failed at {}", Location::caller());
-    assert_eq!(0, 0, "error: something failed at {}", Location::caller());
-    assert_ne!(0, 0, "error: something failed at {}", Location::caller());
-    panic!("error: something failed at {}", Location::caller());
-    println!("{}", *X(1));
-    println!("{}", ***Y(&X(1)));
-    println!("{}", Z(1));
-    println!("{}", **x);
-    println!("{}", ***x_ref);
+    print!("error: something failed at {}", Location::caller().to_string());
+    println!("error: something failed at {}", Location::caller().to_string());
+    eprint!("error: something failed at {}", Location::caller().to_string());
+    eprintln!("error: something failed at {}", Location::caller().to_string());
+    let _ = format_args!("error: something failed at {}", Location::caller().to_string());
+    assert!(true, "error: something failed at {}", Location::caller().to_string());
+    assert_eq!(0, 0, "error: something failed at {}", Location::caller().to_string());
+    assert_ne!(0, 0, "error: something failed at {}", Location::caller().to_string());
+    panic!("error: something failed at {}", Location::caller().to_string());
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
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args.stage-id.fixed
To only update this specific test, also pass `--test-args format_args.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format_args.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---
diff of stderr:

-error: useless use of `format!`
-  --> $DIR/format.rs:13:5
-   |
-LL |     format!("foo");
-   |     ^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
-   |
-   = note: `-D clippy::useless-format` implied by `-D warnings`
-error: useless use of `format!`
-  --> $DIR/format.rs:14:5
-   |
-   |
-LL |     format!("{{}}");
-   |     ^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{}".to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:15:5
-   |
-   |
-LL |     format!("{{}} abc {{}}");
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"{} abc {}".to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:16:5
-   |
-LL | /     format!(
-LL | /     format!(
-LL | |         r##"foo {{}}
-LL | | " bar"##
-LL | |     );
-   | |_____^
-   |
-help: consider using `.to_string()`
-   |
-LL ~     r##"foo {}
-LL ~ " bar"##.to_string();
-   |
-error: useless use of `format!`
-  --> $DIR/format.rs:21:13
-   |
-   |
-LL |     let _ = format!("");
-   |             ^^^^^^^^^^^ help: consider using `String::new()`: `String::new()`
-error: useless use of `format!`
-  --> $DIR/format.rs:23:5
-   |
-LL |     format!("{}", "foo");
-LL |     format!("{}", "foo");
-   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:27:5
-   |
-   |
-LL |     format!("{:+}", "foo"); // Warn when the format makes no difference.
-   |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:28:5
-   |
-   |
-LL |     format!("{:<}", "foo"); // Warn when the format makes no difference.
-   |     ^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `"foo".to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:33:5
-   |
-LL |     format!("{}", arg);
-LL |     format!("{}", arg);
-   |     ^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:37:5
-   |
-   |
-LL |     format!("{:+}", arg); // Warn when the format makes no difference.
-   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:38:5
-   |
-   |
-LL |     format!("{:<}", arg); // Warn when the format makes no difference.
-   |     ^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `arg.to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:65:5
-   |
-   |
-LL |     format!("{}", 42.to_string());
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `42.to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:67:5
-   |
-   |
-LL |     format!("{}", x.display().to_string());
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `x.display().to_string()`
-error: useless use of `format!`
-  --> $DIR/format.rs:71:18
-   |
-   |
-LL |     let _ = Some(format!("{}", a + "bar"));
-   |                  ^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `a + "bar"`
-error: useless use of `format!`
-  --> $DIR/format.rs:75:22
-   |
-   |
-LL |     let _s: String = format!("{}", &*v.join("/n"));
-   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `.to_string()`: `(&*v.join("/n")).to_string()`
-error: aborting due to 15 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![allow(clippy::print_literal, clippy::redundant_clone, clippy::to_string_in_format_args)]
 #![warn(clippy::useless_format)]
 struct Foo(pub String);
 
 macro_rules! foo {
 macro_rules! foo {
     ($($t:tt)*) => (Foo(format!($($t)*)))
 
 fn main() {
-    "foo".to_string();
-    "{}".to_string();
-    "{}".to_string();
-    "{} abc {}".to_string();
-    r##"foo {}
-" bar"##.to_string();
+    format!("foo");
+    format!("{{}}");
+    format!("{{}} abc {{}}");
+    format!(
+        r##"foo {{}}
+" bar"##
 
-    let _ = String::new();
+    let _ = format!("");
 
 
-    "foo".to_string();
+    format!("{}", "foo");
     format!("{:?}", "foo"); // Don't warn about `Debug`.
     format!("{:8}", "foo");
     format!("{:width$}", "foo", width = 8);
-    "foo".to_string(); // Warn when the format makes no difference.
-    "foo".to_string(); // Warn when the format makes no difference.
+    format!("{:+}", "foo"); // Warn when the format makes no difference.
+    format!("{:<}", "foo"); // Warn when the format makes no difference.
     format!("foo {}", "bar");
     format!("{} bar", "foo");
 
     let arg: String = "".to_owned();
-    arg.to_string();
     format!("{:?}", arg); // Don't warn about debug.
     format!("{:8}", arg);
     format!("{:8}", arg);
     format!("{:width$}", arg, width = 8);
-    arg.to_string(); // Warn when the format makes no difference.
-    arg.to_string(); // Warn when the format makes no difference.
+    format!("{:+}", arg); // Warn when the format makes no difference.
+    format!("{:<}", arg); // Warn when the format makes no difference.
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
-    42.to_string();
+    format!("{}", 42.to_string());
+    format!("{}", 42.to_string());
     let x = std::path::PathBuf::from("/bar/foo/qux");
-    x.display().to_string();
+    format!("{}", x.display().to_string());
     // False positive
     // False positive
     let a = "foo".to_string();
-    let _ = Some(a + "bar");
+    let _ = Some(format!("{}", a + "bar"));
 
     // Wrap it with braces
     let v: Vec<String> = vec!["foo".to_string(), "bar".to_string()];
-    let _s: String = (&*v.join("\n")).to_string();
+    let _s: String = format!("{}", &*v.join("\n"));
 
     format!("prepend {:+}", "s");
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format.stage-id.fixed
To only update this specific test, also pass `--test-args format.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = note: `-D clippy::format-in-format-args` implied by `-D warnings`
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{}: {}", error, format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{:?}: {}", error, format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("{{}}: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!(r#"error: "{}""#, format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!(r#"something failed at "{}""#, Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `println!` args
-   |
-   |
-LL |     println!("error: {}", format!("something failed at {} {0}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `println!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `format!` args
-   |
-   |
-LL |     let _ = format!("error: {}", format!("something failed at {}", Location::caller()));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `format!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `write!` args
-   |
-LL |       let _ = write!(
-   |  _____________^
-LL | |         stdout(),
-LL | |         stdout(),
-LL | |         "error: {}",
-LL | |         format!("something failed at {}", Location::caller())
-LL | |     );
-   | |_____^
-   |
-   = help: combine the `format!(..)` arguments with the outer `write!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `writeln!` args
-   |
-LL |       let _ = writeln!(
-   |  _____________^
-LL | |         stdout(),
-LL | |         stdout(),
-LL | |         "error: {}",
-LL | |         format!("something failed at {}", Location::caller())
-LL | |     );
-   | |_____^
-   |
-   = help: combine the `format!(..)` arguments with the outer `writeln!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `print!` args
-   |
-   |
-LL |     print!("error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `print!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `eprint!` args
-   |
-   |
-LL |     eprint!("error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `eprint!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `eprintln!` args
-   |
-   |
-LL |     eprintln!("error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `eprintln!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `format_args!` args
-   |
-   |
-LL |     let _ = format_args!("error: {}", format!("something failed at {}", Location::caller()));
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `format_args!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `assert!` args
-   |
-   |
-LL |     assert!(true, "error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `assert!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `assert_eq!` args
-   |
-   |
-LL |     assert_eq!(0, 0, "error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `assert_eq!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `assert_ne!` args
-   |
-   |
-LL |     assert_ne!(0, 0, "error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `assert_ne!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-
-error: `format!` in `panic!` args
-   |
-   |
-LL |     panic!("error: {}", format!("something failed at {}", Location::caller()));
-   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = help: combine the `format!(..)` arguments with the outer `panic!(..)` call
-   = help: or consider changing `format!` to `format_args!`
-error: aborting due to 18 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args_unfixable.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args format_args_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/format_args_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-0285a3716fdfa0ac.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-738a932898af8104.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0d9270169d1e3a9.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-432c9b3871214358.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bbe41a872bc8e443.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-dead5f2b179ae6e1.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-31d96b843c92ffee.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/format_args_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if !a.is_empty() {
 LL | |         panic!("qaqaq{:?}", a);
 LL | |     }
-   | |_____^ help: try: `assert!(a.is_empty(), "qaqaq{:?}", a);`
+   | |_____^ help: try: `assert!(a.is_empty(), $crate::const_format_args!($($t)+));`
    |
    = note: `-D clippy::manual-assert` implied by `-D warnings`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if !a.is_empty() {
 LL | |         panic!("qwqwq");
 LL | |     }
-   | |_____^ help: try: `assert!(a.is_empty(), "qwqwq");`
+   | |_____^ help: try: `assert!(a.is_empty(), $crate::const_format_args!($($t)+));`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if b.is_empty() {
 LL | |         panic!("panic1");
 LL | |     }
-   | |_____^ help: try: `assert!(!b.is_empty(), "panic1");`
+   | |_____^ help: try: `assert!(!b.is_empty(), $crate::const_format_args!($($t)+));`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if b.is_empty() && a.is_empty() {
 LL | |         panic!("panic2");
 LL | |     }
-   | |_____^ help: try: `assert!(!(b.is_empty() && a.is_empty()), "panic2");`
+   | |_____^ help: try: `assert!(!(b.is_empty() && a.is_empty()), $crate::const_format_args!($($t)+));`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if a.is_empty() && !b.is_empty() {
 LL | |         panic!("panic3");
 LL | |     }
-   | |_____^ help: try: `assert!(!(a.is_empty() && !b.is_empty()), "panic3");`
+   | |_____^ help: try: `assert!(!(a.is_empty() && !b.is_empty()), $crate::const_format_args!($($t)+));`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if b.is_empty() || a.is_empty() {
 LL | |         panic!("panic4");
 LL | |     }
-   | |_____^ help: try: `assert!(!(b.is_empty() || a.is_empty()), "panic4");`
+   | |_____^ help: try: `assert!(!(b.is_empty() || a.is_empty()), $crate::const_format_args!($($t)+));`
 
 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if a.is_empty() || !b.is_empty() {
 LL | |         panic!("panic5");
 LL | |     }
-   | |_____^ help: try: `assert!(!(a.is_empty() || !b.is_empty()), "panic5");`
+   | |_____^ help: try: `assert!(!(a.is_empty() || !b.is_empty()), $crate::const_format_args!($($t)+));`
 error: aborting due to 7 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/manual_assert.stage-id.edition2018.stderr
diff of fixed:

 // revisions: edition2018 edition2021
 // [edition2018] edition:2018
 // [edition2021] edition:2021
 // run-rustfix
 #![warn(clippy::manual_assert)]
