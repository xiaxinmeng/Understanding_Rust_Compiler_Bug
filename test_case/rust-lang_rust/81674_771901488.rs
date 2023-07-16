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

    Finished release [optimized] target(s) in 1m 13s
     Running build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/miri-5aa992efebf876f1
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
Actual stderr saved to /tmp/compiletestwKMAzX/panic/catch_panic.stderr
To update references, run this command from build directory:
tests/run-pass/update-references.sh '/tmp/compiletestwKMAzX' 'panic/catch_panic.rs'
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestwKMAzX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestwKMAzX/panic/catch_panic.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-symbolic-alignment-check" "-L" "/tmp/compiletestwKMAzX/panic/catch_panic.stage-id.aux"
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
    |
110 | / macro_rules! format {
111 |       ($($arg:tt)*) => {{
    |  _______________________^
112 |           let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
114 | |     }}
    | |_____^
115 | | }
    | |_- in this expansion of `format!`
    | |_- in this expansion of `format!`
    | 
   ::: src/tools/clippy/tests/missing-test-files.rs:12:13
    |
12  | /             format!(
13  | |                 "Didn't see a test file for the following files:\n\n{}\n",
15  | |                     .iter()
...   |
...   |
18  | |                     .join("\n")
    | |_____________- in this macro invocation
    |
    |
    = note: `-D non-fmt-panic` implied by `-D warnings`
    = note: this is no longer accepted in Rust 2021
error: aborting due to previous error

error: could not compile `clippy`

