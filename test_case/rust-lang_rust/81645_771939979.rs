plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
warning: panic message is not a string literal
   --> /checkout/library/alloc/src/macros.rs:111:23
    |
110 |  / macro_rules! format {
111 |  |     ($($arg:tt)*) => {{
    |  |_______________________^
112 | ||         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113 | ||         res
114 | ||     }}
    | ||_____^
115 |  | }
    |  |_- in this expansion of `format!`
   ::: src/tools/miri/src/bin/miri.rs:233:32
    |
233 |                            panic!(format!(
    |  _________________________________-
    |  _________________________________-
234 | |                              "-Zmiri-seed must be at most 8 bytes, was {}",
235 | |                              seed_raw.len()
236 | |                          ));
    | |__________________________- in this macro invocation
    |
    = note: `#[warn(non_fmt_panic)]` on by default
    = note: this is no longer accepted in Rust 2021
warning: 1 warning emitted

    Finished release [optimized] target(s) in 1m 36s
Building stage2 tool cargo-miri (x86_64-unknown-linux-gnu)
---
   Compiling tester v0.7.0
warning: panic message is not a string literal
   --> /checkout/library/alloc/src/macros.rs:111:23
    |
110 |  / macro_rules! format {
111 |  |     ($($arg:tt)*) => {{
    |  |_______________________^
112 | ||         let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
113 | ||         res
114 | ||     }}
    | ||_____^
115 |  | }
    |  |_- in this expansion of `format!`
   ::: src/tools/miri/src/bin/miri.rs:233:32
    |
233 |                            panic!(format!(
    |  _________________________________-
    |  _________________________________-
234 | |                              "-Zmiri-seed must be at most 8 bytes, was {}",
235 | |                              seed_raw.len()
236 | |                          ));
    | |__________________________- in this macro invocation
    |
    = note: `#[warn(non_fmt_panic)]` on by default
    = note: this is no longer accepted in Rust 2021
warning: 1 warning emitted

    Finished release [optimized] target(s) in 1m 12s
     Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/miri-d5f36d6a3f04a7a2
---
normalized stderr:
warning: panic message is not a string literal
  --> $DIR/catch_panic.rs:LL:38
   |
54 |     test(None, |old_val| std::panic!(format!("Hello from panic: {:?}", old_val)));
   |
   |
   = note: `#[warn(non_fmt_panic)]` on by default
   = note: this is no longer accepted in Rust 2021

warning: panic message is not a string literal
  --> $DIR/catch_panic.rs:LL:39
   |
   |
56 |     test(None, |_old_val| std::panic!(1337));
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
56 |     test(None, |_old_val| std::panic!("{}", 1337));
   |                                       ^^^^^
help: or use std::panic::panic_any instead
   |
56 |     test(None, |_old_val| std::panic::panic_any(1337));

warning: panic message is not a string literal
  --> $DIR/catch_panic.rs:LL:39
   |
   |
60 |     test(None, |old_val| core::panic!(&format!("Hello from panic: {:?}", old_val)));
   |
   = note: this is no longer accepted in Rust 2021
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
60 |     test(None, |old_val| core::panic!("{}", &format!("Hello from panic: {:?}", old_val)));

thread 'main' panicked at 'Hello from panic: std', $DIR/catch_panic.rs:LL:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Caught panic message (&str): Hello from panic: std
---

+warning: panic message is not a string literal
+  --> $DIR/catch_panic.rs:LL:38
+   |
+54 |     test(None, |old_val| std::panic!(format!("Hello from panic: {:?}", old_val)));
+   |
+   |
+   = note: `#[warn(non_fmt_panic)]` on by default
+   = note: this is no longer accepted in Rust 2021
+
+warning: panic message is not a string literal
+  --> $DIR/catch_panic.rs:LL:39
+   |
+   |
+56 |     test(None, |_old_val| std::panic!(1337));
+   |
+   = note: this is no longer accepted in Rust 2021
+   = note: this is no longer accepted in Rust 2021
+help: add a "{}" format string to Display the message
+   |
+56 |     test(None, |_old_val| std::panic!("{}", 1337));
+   |                                       ^^^^^
+help: or use std::panic::panic_any instead
+   |
+56 |     test(None, |_old_val| std::panic::panic_any(1337));
+
+warning: panic message is not a string literal
+  --> $DIR/catch_panic.rs:LL:39
+   |
+   |
+60 |     test(None, |old_val| core::panic!(&format!("Hello from panic: {:?}", old_val)));
+   |
+   = note: this is no longer accepted in Rust 2021
+   = note: this is no longer accepted in Rust 2021
+help: add a "{}" format string to Display the message
+   |
+60 |     test(None, |old_val| core::panic!("{}", &format!("Hello from panic: {:?}", old_val)));
+
 thread 'main' panicked at 'Hello from panic: std', $DIR/catch_panic.rs:LL:27
 note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 Caught panic message (&str): Hello from panic: std
---
 Success!
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletestSmvdcU/panic/catch_panic.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestSmvdcU' 'panic/catch_panic.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestSmvdcU" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestSmvdcU/panic/catch_panic.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-symbolic-alignment-check" "-L" "/tmp/compiletestSmvdcU/panic/catch_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"panic message is not a string literal","code":{"code":"non_fmt_panic","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/library/alloc/src/macros.rs","byte_start":3730,"byte_end":3830,"line_start":111,"line_end":114,"column_start":23,"column_end":6,"is_primary":true,"text":[{"text":"    ($($arg:tt)*) => {{","highlight_start":23,"highlight_end":24},{"text":"        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));","highlight_start":1,"highlight_end":81},{"text":"        res","highlight_start":1,"highlight_end":12},{"text":"    }}","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1678,"byte_end":1720,"line_start":54,"line_end":54,"column_start":38,"column_end":80,"is_primary":false,"text":[{"text":"    test(None, |old_val| std::panic!(format!(\"Hello from panic: {:?}\", old_val)));","highlight_start":38,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"/checkout/library/alloc/src/macros.rs","byte_start":3686,"byte_end":3833,"line_start":110,"line_end":115,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! format {","highlight_start":1,"highlight_end":1},{"text":"    ($($arg:tt)*) => {{","highlight_start":1,"highlight_end":1},{"text":"        let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));","highlight_start":1,"highlight_end":1},{"text":"        res","highlight_start":1,"highlight_end":1},{"text":"    }}","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`#[warn(non_fmt_panic)]` on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is no longer accepted in Rust 2021","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: panic message is not a string literal\n  --> tests/run-pass/panic/catch_panic.rs:54:38\n   |\n54 |     test(None, |old_val| std::panic!(format!(\"Hello from panic: {:?}\", old_val)));\n   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `#[warn(non_fmt_panic)]` on by default\n   = note: this is no longer accepted in Rust 2021\n   = note: this warning originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"panic message is not a string literal","code":{"code":"non_fmt_panic","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1836,"byte_end":1840,"line_start":56,"line_end":56,"column_start":39,"column_end":43,"is_primary":true,"text":[{"text":"    test(None, |_old_val| std::panic!(1337));","highlight_start":39,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is no longer accepted in Rust 2021","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add a \"{}\" format string to Display the message","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1836,"byte_end":1836,"line_start":56,"line_end":56,"column_start":39,"column_end":39,"is_primary":true,"text":[{"text":"    test(None, |_old_val| std::panic!(1337));","highlight_start":39,"highlight_end":39}],"label":null,"suggested_replacement":"\"{}\", ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null},{"message":"or use std::panic::panic_any instead","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1824,"byte_end":1836,"line_start":56,"line_end":56,"column_start":27,"column_end":39,"is_primary":true,"text":[{"text":"    test(None, |_old_val| std::panic!(1337));","highlight_start":27,"highlight_end":39}],"label":null,"suggested_replacement":"std::panic::panic_any(","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: panic message is not a string literal\n  --> tests/run-pass/panic/catch_panic.rs:56:39\n   |\n56 |     test(None, |_old_val| std::panic!(1337));\n   |                                       ^^^^\n   |\n   = note: this is no longer accepted in Rust 2021\nhelp: add a \"{}\" format string to Display the message\n   |\n56 |     test(None, |_old_val| std::panic!(\"{}\", 1337));\n   |                                       ^^^^^\nhelp: or use std::panic::panic_any instead\n   |\n56 |     test(None, |_old_val| std::panic::panic_any(1337));\n   |                           ^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"panic message is not a string literal","code":{"code":"non_fmt_panic","explanation":null},"level":"warning","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1969,"byte_end":2012,"line_start":60,"line_end":60,"column_start":39,"column_end":82,"is_primary":true,"text":[{"text":"    test(None, |old_val| core::panic!(&format!(\"Hello from panic: {:?}\", old_val)));","highlight_start":39,"highlight_end":82}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is no longer accepted in Rust 2021","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add a \"{}\" format string to Display the message","code":null,"level":"help","spans":[{"file_name":"tests/run-pass/panic/catch_panic.rs","byte_start":1969,"byte_end":1969,"line_start":60,"line_end":60,"column_start":39,"column_end":39,"is_primary":true,"text":[{"text":"    test(None, |old_val| core::panic!(&format!(\"Hello from panic: {:?}\", old_val)));","highlight_start":39,"highlight_end":39}],"label":null,"suggested_replacement":"\"{}\", ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: panic message is not a string literal\n  --> tests/run-pass/panic/catch_panic.rs:60:39\n   |\n60 |     test(None, |old_val| core::panic!(&format!(\"Hello from panic: {:?}\", old_val)));\n   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this is no longer accepted in Rust 2021\nhelp: add a \"{}\" format string to Display the message\n   |\n60 |     test(None, |old_val| core::panic!(\"{}\", &format!(\"Hello from panic: {:?}\", old_val)));\n   |                                       ^^^^^\n\n"}
thread 'main' panicked at 'Hello from panic: std', tests/run-pass/panic/catch_panic.rs:53:27
Caught panic message (&str): Hello from panic: std
thread 'main' panicked at 'Hello from panic: 1', tests/run-pass/panic/catch_panic.rs:54:26
Caught panic message (String): Hello from panic: 1
thread 'main' panicked at 'Hello from panic: 2', tests/run-pass/panic/catch_panic.rs:55:26
---
failures:

---- compile_test stdout ----
normalized stderr:
error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(true);
   |
   |
   = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false)` should probably be replaced
   |
   |
LL |     assert!(false);
   |
   |
   = help: use `panic!()` or `unreachable!()`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(true, "true message");
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false, "false message")` should probably be replaced
   |
   |
LL |     assert!(false, "false message");
   |
   |
   = help: use `panic!("false message")` or `unreachable!("false message")`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(false, msg.to_uppercase())` should probably be replaced
   |
   |
LL |     assert!(false, msg.to_uppercase());
   |
   |
   = help: use `panic!(msg.to_uppercase())` or `unreachable!(msg.to_uppercase())`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(B);
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false)` should probably be replaced
   |
   |
LL |     assert!(C);
   |
   |
   = help: use `panic!()` or `unreachable!()`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(false, "C message")` should probably be replaced
   |
   |
LL |     assert!(C, "C message");
   |
   |
   = help: use `panic!("C message")` or `unreachable!("C message")`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `debug_assert!(true)` will be optimized out by the compiler
   |
   |
LL |     debug_assert!(true);
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: panic message is not a string literal
  --> $DIR/assertions_on_constants.rs:15:20
   |
LL |     assert!(false, msg.to_uppercase());
   |
   |
   = note: `-D non-fmt-panic` implied by `-D warnings`
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |     assert!(false, "{}", msg.to_uppercase());

error: aborting due to 10 previous errors




expected stderr:
error: test failed, to rerun pass '--test compile-test'
error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(true);
   |
   |
   = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false)` should probably be replaced
   |
   |
LL |     assert!(false);
   |
   |
   = help: use `panic!()` or `unreachable!()`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(true, "true message");
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false, "false message")` should probably be replaced
   |
   |
LL |     assert!(false, "false message");
   |
   |
   = help: use `panic!("false message")` or `unreachable!("false message")`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(false, msg.to_uppercase())` should probably be replaced
   |
   |
LL |     assert!(false, msg.to_uppercase());
   |
   |
   = help: use `panic!(msg.to_uppercase())` or `unreachable!(msg.to_uppercase())`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(true)` will be optimized out by the compiler
   |
   |
LL |     assert!(B);
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: `assert!(false)` should probably be replaced
   |
   |
LL |     assert!(C);
   |
   |
   = help: use `panic!()` or `unreachable!()`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `assert!(false, "C message")` should probably be replaced
   |
   |
LL |     assert!(C, "C message");
   |
   |
   = help: use `panic!("C message")` or `unreachable!("C message")`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: `debug_assert!(true)` will be optimized out by the compiler
   |
   |
LL |     debug_assert!(true);
   |
   = help: remove it
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: aborting due to 9 previous errors



diff of stderr:

 error: `assert!(true)` will be optimized out by the compiler
    |
    |
 LL |     assert!(true);
    |
    |
    = note: `-D clippy::assertions-on-constants` implied by `-D warnings`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: `assert!(false)` should probably be replaced
    |
    |
 LL |     assert!(false);
    |
    |
    = help: use `panic!()` or `unreachable!()`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: `assert!(true)` will be optimized out by the compiler
    |
    |
 LL |     assert!(true, "true message");
    |
    = help: remove it
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: `assert!(false, "false message")` should probably be replaced
    |
    |
 LL |     assert!(false, "false message");
    |
    |
    = help: use `panic!("false message")` or `unreachable!("false message")`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: `assert!(false, msg.to_uppercase())` should probably be replaced
    |
    |
 LL |     assert!(false, msg.to_uppercase());
    |
    |
    = help: use `panic!(msg.to_uppercase())` or `unreachable!(msg.to_uppercase())`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: `assert!(true)` will be optimized out by the compiler
    |
    |
 LL |     assert!(B);
    |
    = help: remove it
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: `assert!(false)` should probably be replaced
    |
    |
 LL |     assert!(C);
    |
    |
    = help: use `panic!()` or `unreachable!()`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: `assert!(false, "C message")` should probably be replaced
    |
    |
 LL |     assert!(C, "C message");
    |
    |
    = help: use `panic!("C message")` or `unreachable!("C message")`
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: `debug_assert!(true)` will be optimized out by the compiler
    |
    |
 LL |     debug_assert!(true);
    |
    = help: remove it
    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
-error: aborting due to 9 previous errors
+error: panic message is not a string literal
+  --> $DIR/assertions_on_constants.rs:15:20
+   |
+LL |     assert!(false, msg.to_uppercase());
+   |
+   |
+   = note: `-D non-fmt-panic` implied by `-D warnings`
+   = note: this is no longer accepted in Rust 2021
+help: add a "{}" format string to Display the message
+   |
+LL |     assert!(false, "{}", msg.to_uppercase());
+
+error: aborting due to 10 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/assertions_on_constants.stderr
To update references, run this command from build directory:
tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base' 'assertions_on_constants.rs'
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/assertions_on_constants.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/assertions_on_constants.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-45a3ec2898545ae5.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3e5755171965ca17.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-efc540c81be69f24.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-c43e08f62c64f186.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-1bcf52e6f16bdb4f.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-58613e8381df8652/out/test_build_base/assertions_on_constants.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":135,"byte_end":149,"line_start":9,"line_end":9,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    assert!(true);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":135,"byte_end":149,"line_start":9,"line_end":9,"column_start":5,"column_end":19,"is_primary":false,"text":[{"text":"    assert!(true);","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":1},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":1},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":1},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::assertions-on-constants` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:9:5\n   |\nLL |     assert!(true);\n   |     ^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::assertions-on-constants` implied by `-D warnings`\n   = help: remove it\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(false)` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":154,"byte_end":169,"line_start":10,"line_end":10,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    assert!(false);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":154,"byte_end":169,"line_start":10,"line_end":10,"column_start":5,"column_end":20,"is_primary":false,"text":[{"text":"    assert!(false);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!()` or `unreachable!()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false)` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:10:5\n   |\nLL |     assert!(false);\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!()` or `unreachable!()`\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":174,"byte_end":204,"line_start":11,"line_end":11,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    assert!(true, \"true message\");","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":174,"byte_end":204,"line_start":11,"line_end":11,"column_start":5,"column_end":35,"is_primary":false,"text":[{"text":"    assert!(true, \"true message\");","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:11:5\n   |\nLL |     assert!(true, \"true message\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: remove it\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(false, \"false message\")` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":209,"byte_end":241,"line_start":12,"line_end":12,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    assert!(false, \"false message\");","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":209,"byte_end":241,"line_start":12,"line_end":12,"column_start":5,"column_end":37,"is_primary":false,"text":[{"text":"    assert!(false, \"false message\");","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!(\"false message\")` or `unreachable!(\"false message\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false, \"false message\")` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:12:5\n   |\nLL |     assert!(false, \"false message\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!(\"false message\")` or `unreachable!(\"false message\")`\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(false, msg.to_uppercase())` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":278,"byte_end":313,"line_start":15,"line_end":15,"column_start":5,"column_end":40,"is_primary":true,"text":[{"text":"    assert!(false, msg.to_uppercase());","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":278,"byte_end":313,"line_start":15,"line_end":15,"column_start":5,"column_end":40,"is_primary":false,"text":[{"text":"    assert!(false, msg.to_uppercase());","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!(msg.to_uppercase())` or `unreachable!(msg.to_uppercase())`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false, msg.to_uppercase())` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:15:5\n   |\nLL |     assert!(false, msg.to_uppercase());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!(msg.to_uppercase())` or `unreachable!(msg.to_uppercase())`\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":345,"byte_end":356,"line_start":18,"line_end":18,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    assert!(B);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":345,"byte_end":356,"line_start":18,"line_end":18,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    assert!(B);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:18:5\n   |\nLL |     assert!(B);\n   |     ^^^^^^^^^^^\n   |\n   = help: remove it\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(false)` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":389,"byte_end":400,"line_start":21,"line_end":21,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    assert!(C);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":389,"byte_end":400,"line_start":21,"line_end":21,"column_start":5,"column_end":16,"is_primary":false,"text":[{"text":"    assert!(C);","highlight_start":5,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!()` or `unreachable!()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false)` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:21:5\n   |\nLL |     assert!(C);\n   |     ^^^^^^^^^^^\n   |\n   = help: use `panic!()` or `unreachable!()`\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`assert!(false, \"C message\")` should probably be replaced","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":405,"byte_end":429,"line_start":22,"line_end":22,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    assert!(C, \"C message\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":405,"byte_end":429,"line_start":22,"line_end":22,"column_start":5,"column_end":29,"is_primary":false,"text":[{"text":"    assert!(C, \"C message\");","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"use `panic!(\"C message\")` or `unreachable!(\"C message\")`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `assert!(false, \"C message\")` should probably be replaced\n  --> tests/ui/assertions_on_constants.rs:22:5\n   |\nLL |     assert!(C, \"C message\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: use `panic!(\"C message\")` or `unreachable!(\"C message\")`\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"`debug_assert!(true)` will be optimized out by the compiler","code":{"code":"clippy::assertions_on_constants","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":6461,"byte_end":6487,"line_start":184,"line_end":184,"column_start":59,"column_end":85,"is_primary":true,"text":[{"text":"    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })","highlight_start":59,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":6461,"byte_end":6487,"line_start":184,"line_end":184,"column_start":59,"column_end":85,"is_primary":false,"text":[{"text":"    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })","highlight_start":59,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":435,"byte_end":455,"line_start":24,"line_end":24,"column_start":5,"column_end":25,"is_primary":false,"text":[{"text":"    debug_assert!(true);","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"debug_assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":6375,"byte_end":6492,"line_start":183,"line_end":185,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! debug_assert {","highlight_start":1,"highlight_end":28},{"text":"    ($($arg:tt)*) => (if $crate::cfg!(debug_assertions) { $crate::assert!($($arg)*); })","highlight_start":1,"highlight_end":88},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::assert!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":41883,"byte_end":42039,"line_start":1238,"line_end":1241,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! assert {","highlight_start":5,"highlight_end":26},{"text":"        ($cond:expr $(,)?) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":61},{"text":"        ($cond:expr, $($arg:tt)+) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"remove it","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: `debug_assert!(true)` will be optimized out by the compiler\n  --> tests/ui/assertions_on_constants.rs:24:5\n   |\nLL |     debug_assert!(true);\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: remove it\n   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"panic message is not a string literal","code":{"code":"non_fmt_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":293,"byte_end":311,"line_start":15,"line_end":15,"column_start":20,"column_end":38,"is_primary":true,"text":[{"text":"    assert!(false, msg.to_uppercase());","highlight_start":20,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D non-fmt-panic` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"this is no longer accepted in Rust 2021","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add a \"{}\" format string to Display the message","code":null,"level":"help","spans":[{"file_name":"tests/ui/assertions_on_constants.rs","byte_start":293,"byte_end":293,"line_start":15,"line_end":15,"column_start":20,"column_end":20,"is_primary":true,"text":[{"text":"    assert!(false, msg.to_uppercase());","highlight_start":20,"highlight_end":20}],"label":null,"suggested_replacement":"\"{}\", ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: panic message is not a string literal\n  --> tests/ui/assertions_on_constants.rs:15:20\n   |\nLL |     assert!(false, msg.to_uppercase());\n   |                    ^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D non-fmt-panic` implied by `-D warnings`\n   = note: this is no longer accepted in Rust 2021\nhelp: add a \"{}\" format string to Display the message\n   |\nLL |     assert!(false, \"{}\", msg.to_uppercase());\n   |                    ^^^^^\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:5:1
   |
LL | / impl From<String> for Foo {
LL | |     fn from(s: String) -> Self {
LL | |         Foo(s.parse().unwrap())
LL | |     }
LL | | }
   |
note: the lint level is defined here
  --> $DIR/fallible_impl_from.rs:1:9
   |
   |
LL | #![deny(clippy::fallible_impl_from)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:7:13
   |
LL |         Foo(s.parse().unwrap())


error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:26:1
   |
LL | / impl From<usize> for Invalid {
LL | |     fn from(i: usize) -> Invalid {
LL | |         if i != 42 {
LL | |             panic!();
LL | |     }
LL | | }
   | |_^
   |
   |
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:29:13
   |
LL |             panic!();
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:35:1
   |
LL | / impl From<Option<String>> for Invalid {
LL | |     fn from(s: Option<String>) -> Invalid {
LL | |         let s = s.unwrap();
LL | |         if !s.is_empty() {
LL | |     }
LL | | }
   | |_^
   |
   |
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:37:17
LL |         let s = s.unwrap();
   |                 ^^^^^^^^^^
   |                 ^^^^^^^^^^
LL |         if !s.is_empty() {
LL |             panic!(42);
   |             ^^^^^^^^^^^
LL |         } else if s.parse::<u32>().unwrap() != 42 {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
LL |             panic!("{:?}", s);
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:53:1
   |
LL | / impl<'a> From<&'a mut <Box<u32> as ProjStrTrait>::ProjString> for Invalid {
LL | |     fn from(s: &'a mut <Box<u32> as ProjStrTrait>::ProjString) -> Invalid {
LL | |         if s.parse::<u32>().ok().unwrap() != 42 {
LL | |             panic!("{:?}", s);
LL | |     }
LL | | }
   | |_^
   |
   |
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:55:12
   |
LL |         if s.parse::<u32>().ok().unwrap() != 42 {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL |             panic!("{:?}", s);
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: panic message is not a string literal
  --> $DIR/fallible_impl_from.rs:39:20
  --> $DIR/fallible_impl_from.rs:39:20
   |
LL |             panic!(42);
   |
   |
   = note: `-D non-fmt-panic` implied by `-D warnings`
   = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
   |
LL |             panic!("{}", 42);
   |                    ^^^^^
help: or use std::panic::panic_any instead
LL |             std::panic::panic_any(42);
   |             ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 5 previous errors
error: aborting due to 5 previous errors



expected stderr:
error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:5:1
   |
LL | / impl From<String> for Foo {
LL | |     fn from(s: String) -> Self {
LL | |         Foo(s.parse().unwrap())
LL | |     }
LL | | }
   |
note: the lint level is defined here
  --> $DIR/fallible_impl_from.rs:1:9
   |
   |
LL | #![deny(clippy::fallible_impl_from)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:7:13
   |
LL |         Foo(s.parse().unwrap())


error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:26:1
   |
LL | / impl From<usize> for Invalid {
LL | |     fn from(i: usize) -> Invalid {
LL | |         if i != 42 {
LL | |             panic!();
LL | |     }
LL | | }
   | |_^
   |
   |
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:29:13
   |
LL |             panic!();
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: consider implementing `TryFrom` instead
  --> $DIR/fallible_impl_from.rs:35:1
   |
LL | / impl From<Option<String>> for Invalid {
LL | |     fn from(s: Option<String>) -> Invalid {
LL | |         let s = s.unwrap();
LL | |         if !s.is_empty() {
LL | |     }
LL | | }
   | |_^
   |
   |
   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail.
note: potential failure(s)
  --> $DIR/fallible_impl_from.rs:37:17
LL |         let s = s.unwrap();
   |                 ^^^^^^^^^^
   |                 ^^^^^^^^^^
LL |         if !s.is_empty() {
LL |             panic!(42);
   |             ^^^^^^^^^^^
LL |         } else if s.parse::<u32>().unwrap() != 42 {
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^
