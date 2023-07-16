plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 6162529a01473bbb2427fa27354cbafc3c514eee and 1b854e16d52c1695fa4b734d0225cd85c931b528
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:20:10
    |
 LL |     b || diverge();
    |
    |
    = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:21:10
    |
    |
 LL |     b || A.foo();
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:30:26
    |
---
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:34:26
    |
 LL |             3 => true || diverge(),
 
 error: sub-expression diverges
+  --> $DIR/diverging_sub_expression.rs:37:30
+   |
+   |
+LL |                 _ => true || panic!("boo"),
+   |
+   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+
+error: sub-expression diverges
---
To only update this specific test, also pass `--test-args diverging_sub_expression.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/diverging_sub_expression.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/diverging_sub_expression.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c86912d687f6474b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0add1c52618c6f7c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bc8a65c262f269f6.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-e338b85031c86fb1.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a607561cd2d7a9ff.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/diverging_sub_expression.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":364,"byte_end":373,"line_start":20,"line_end":20,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    b || diverge();","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::diverging-sub-expression` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:20:10\n   |\nLL |     b || diverge();\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":384,"byte_end":391,"line_start":21,"line_end":21,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    b || A.foo();","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:21:10\n   |\nLL |     b || A.foo();\n   |          ^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":562,"byte_end":568,"line_start":30,"line_end":30,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"            6 => true || return,","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:30:26\n   |\nLL |             6 => true || return,\n   |                          ^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":595,"byte_end":603,"line_start":31,"line_end":31,"column_start":26,"column_end":34,"is_primary":true,"text":[{"text":"            7 => true || continue,","highlight_start":26,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:31:26\n   |\nLL |             7 => true || continue,\n   |                          ^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":682,"byte_end":691,"line_start":34,"line_end":34,"column_start":26,"column_end":35,"is_primary":true,"text":[{"text":"            3 => true || diverge(),","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:34:26\n   |\nLL |             3 => true || diverge(),\n   |                          ^^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/std/src/panic.rs","byte_start":654,"byte_end":683,"line_start":21,"line_end":21,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"        $crate::rt::begin_panic($msg)","highlight_start":9,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":781,"byte_end":794,"line_start":37,"line_end":37,"column_start":30,"column_end":43,"is_primary":false,"text":[{"text":"                _ => true || panic!(\"boo\"),","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":781,"byte_end":794,"line_start":37,"line_end":37,"column_start":30,"column_end":43,"is_primary":false,"text":[{"text":"                _ => true || panic!(\"boo\"),","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":678,"line_start":13,"line_end":19,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":1},{"text":"    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`","highlight_start":1,"highlight_end":1},{"text":"    // depending on the edition of the caller.","highlight_start":1,"highlight_end":1},{"text":"    ($($arg:tt)*) => {","highlight_start":1,"highlight_end":1},{"text":"        /* compiler built-in */","highlight_start":1,"highlight_end":1},{"text":"    };","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2015!","def_site_span":{"file_name":"/checkout/library/std/src/panic.rs","byte_start":527,"byte_end":950,"line_start":16,"line_end":30,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"pub macro panic_2015 {","highlight_start":1,"highlight_end":1},{"text":"    () => (","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":1,"highlight_end":1},{"text":"    ),","highlight_start":1,"highlight_end":1},{"text":"    ($msg:expr $(,)?) => (","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::begin_panic($msg)","highlight_start":1,"highlight_end":1},{"text":"    ),","highlight_start":1,"highlight_end":1},{"text":"    // Special-case the single-argument case for const_panic.","highlight_start":1,"highlight_end":1},{"text":"    (\"{}\", $arg:expr $(,)?) => (","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::panic_display(&$arg)","highlight_start":1,"highlight_end":1},{"text":"    ),","highlight_start":1,"highlight_end":1},{"text":"    ($fmt:expr, $($arg:tt)+) => (","highlight_start":1,"highlight_end":1},{"text":"        $crate::rt::panic_fmt($crate::const_format_args!($fmt, $($arg)+))","highlight_start":1,"highlight_end":1},{"text":"    ),","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:37:30\n   |\nLL |                 _ => true || panic!(\"boo\"),\n   |                              ^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":836,"byte_end":841,"line_start":39,"line_end":39,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"            _ => true || break,","highlight_start":26,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:39:26\n   |\nLL |             _ => true || break,\n   |                          ^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: only a `panic!` in `if`-then statement
    |
    |
 LL | /     if !a.is_empty() {
 LL | |         panic!("qaqaq{:?}", a);
 LL | |     }
    | |_____^ help: try: `assert!(a.is_empty(), "qaqaq{:?}", a);`
    |
    = note: `-D clippy::if-then-panic` implied by `-D warnings`
 
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
+error: aborting due to previous error
 
 
 

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
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
         panic!("qaqaq{:?}", a);
     }
     assert!(a.is_empty(), "qaqaq{:?}", a);
-    assert!(a.is_empty(), "qwqwq");
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
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/if_then_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c86912d687f6474b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0add1c52618c6f7c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bc8a65c262f269f6.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-e338b85031c86fb1.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a607561cd2d7a9ff.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/if_then_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"only a `panic!` in `if`-then statement","code":{"code":"clippy::if_then_panic","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":369,"byte_end":425,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qaqaq{:?}\", a);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::if-then-panic` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/if_then_panic.rs","byte_start":369,"byte_end":425,"line_start":19,"line_end":21,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    if !a.is_empty() {","highlight_start":5,"highlight_end":23},{"text":"        panic!(\"qaqaq{:?}\", a);","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":"assert!(a.is_empty(), \"qaqaq{:?}\", a);","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: only a `panic!` in `if`-then statement\n  --> tests/ui/if_then_panic.rs:19:5\n   |\nLL | /     if !a.is_empty() {\nLL | |         panic!(\"qaqaq{:?}\", a);\nLL | |     }\n   | |_____^ help: try: `assert!(a.is_empty(), \"qaqaq{:?}\", a);`\n   |\n   = note: `-D clippy::if-then-panic` implied by `-D warnings`\n\n"}

------------------------------------------

normalized stderr:
normalized stderr:
error: sub-expression diverges
  --> $DIR/issue-7447.rs:23:15
   |
LL |     byte_view(panic!());
   |
   |
   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
error: sub-expression diverges
  --> $DIR/issue-7447.rs:24:19
   |
   |
LL |     group_entries(panic!());
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args issue-7447.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/issue-7447.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/issue-7447.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c86912d687f6474b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0add1c52618c6f7c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bc8a65c262f269f6.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-e338b85031c86fb1.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a607561cd2d7a9ff.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/issue-7447.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/std/src/panic.rs","byte_start":570,"byte_end":611,"line_start":18,"line_end":18,"column_start":9,"column_end":50,"is_primary":true,"text":[{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":9,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":436,"byte_end":444,"line_start":23,"line_end":23,"column_start":15,"column_end":23,"is_primary":false,"text":[{"text":"    byte_view(panic!());","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":436,"byte_end":444,"line_start":23,"line_end":23,"column_start":15,"column_end":23,"is_primary":false,"text":[{"text":"    byte_view(panic!());","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":678,"line_start":13,"line_end":19,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":1},{"text":"    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`","highlight_start":1,"highlight_end":1},{"text":"    // depending on the edition of the caller.","highlight_start":1,"highlight_end":1},{"text":"    ($($arg:tt)*) => {","highlight_start":1,"highlight_end":1},{"text":"        /* compiler built-in */","highlight_start":1,"highlight_end":1},{"text":"    };","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2015!","def_site_span":{"file_name":"/checkout/library/std/src/panic.rs","byte_start":527,"byte_end":950,"line_start":16,"line_end":30,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"pub macro panic_2015 {","highlight_start":1,"highlight_end":23},{"text":"    () => (","highlight_start":1,"highlight_end":12},{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":1,"highlight_end":50},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    ($msg:expr $(,)?) => (","highlight_start":1,"highlight_end":27},{"text":"        $crate::rt::begin_panic($msg)","highlight_start":1,"highlight_end":38},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    // Special-case the single-argument case for const_panic.","highlight_start":1,"highlight_end":62},{"text":"    (\"{}\", $arg:expr $(,)?) => (","highlight_start":1,"highlight_end":33},{"text":"        $crate::rt::panic_display(&$arg)","highlight_start":1,"highlight_end":41},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    ($fmt:expr, $($arg:tt)+) => (","highlight_start":1,"highlight_end":34},{"text":"        $crate::rt::panic_fmt($crate::const_format_args!($fmt, $($arg)+))","highlight_start":1,"highlight_end":74},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::diverging-sub-expression` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: sub-expression diverges\n  --> tests/ui/issue-7447.rs:23:15\n   |\nLL |     byte_view(panic!());\n   |               ^^^^^^^^\n   |\n   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`\n   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/std/src/panic.rs","byte_start":570,"byte_end":611,"line_start":18,"line_end":18,"column_start":9,"column_end":50,"is_primary":true,"text":[{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":9,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":465,"byte_end":473,"line_start":24,"line_end":24,"column_start":19,"column_end":27,"is_primary":false,"text":[{"text":"    group_entries(panic!());","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":465,"byte_end":473,"line_start":24,"line_end":24,"column_start":19,"column_end":27,"is_primary":false,"text":[{"text":"    group_entries(panic!());","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":678,"line_start":13,"line_end":19,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":21},{"text":"    // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`","highlight_start":1,"highlight_end":84},{"text":"    // depending on the edition of the caller.","highlight_start":1,"highlight_end":47},{"text":"    ($($arg:tt)*) => {","highlight_start":1,"highlight_end":23},{"text":"        /* compiler built-in */","highlight_start":1,"highlight_end":32},{"text":"    };","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2015!","def_site_span":{"file_name":"/checkout/library/std/src/panic.rs","byte_start":527,"byte_end":950,"line_start":16,"line_end":30,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"pub macro panic_2015 {","highlight_start":1,"highlight_end":23},{"text":"    () => (","highlight_start":1,"highlight_end":12},{"text":"        $crate::rt::begin_panic(\"explicit panic\")","highlight_start":1,"highlight_end":50},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    ($msg:expr $(,)?) => (","highlight_start":1,"highlight_end":27},{"text":"        $crate::rt::begin_panic($msg)","highlight_start":1,"highlight_end":38},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    // Special-case the single-argument case for const_panic.","highlight_start":1,"highlight_end":62},{"text":"    (\"{}\", $arg:expr $(,)?) => (","highlight_start":1,"highlight_end":33},{"text":"        $crate::rt::panic_display(&$arg)","highlight_start":1,"highlight_end":41},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"    ($fmt:expr, $($arg:tt)+) => (","highlight_start":1,"highlight_end":34},{"text":"        $crate::rt::panic_fmt($crate::const_format_args!($fmt, $($arg)+))","highlight_start":1,"highlight_end":74},{"text":"    ),","highlight_start":1,"highlight_end":7},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/issue-7447.rs:24:19\n   |\nLL |     group_entries(panic!());\n   |                   ^^^^^^^^\n   |\n   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: `Err(_)` matches all errors
-   |
-   |
-LL |         Err(_) => panic!("err"),
-   |         ^^^^^^
-   |
-   = note: `-D clippy::match-wild-err-arm` implied by `-D warnings`
-   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable
-
-error: `Err(_)` matches all errors
-   |
-   |
-LL |         Err(_) => panic!(),
-   |         ^^^^^^
-   |
-   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable
-
-error: `Err(_)` matches all errors
    |
    |
 LL |         Err(_) => {
    |
    |
+   = note: `-D clippy::match-wild-err-arm` implied by `-D warnings`
    = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable
 
-error: `Err(_e)` matches all errors
-   |
-   |
-LL |         Err(_e) => panic!(),
-   |         ^^^^^^^
-   |
-   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable
-error: aborting due to 4 previous errors
+error: aborting due to previous error
 
 
---
To only update this specific test, also pass `--test-args match_wild_err_arm.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/match_wild_err_arm.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/match_wild_err_arm.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c86912d687f6474b.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0add1c52618c6f7c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-bc8a65c262f269f6.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-e338b85031c86fb1.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a607561cd2d7a9ff.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/match_wild_err_arm.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`Err(_)` matches all errors","code":{"code":"clippy::match_wild_err_arm","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/match_wild_err_arm.rs","byte_start":498,"byte_end":504,"line_start":23,"line_end":23,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        Err(_) => {","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::match-wild-err-arm` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Err(_)` matches all errors\n  --> tests/ui/match_wild_err_arm.rs:23:9\n   |\nLL |         Err(_) => {\n   |         ^^^^^^\n   |\n   = note: `-D clippy::match-wild-err-arm` implied by `-D warnings`\n   = note: match each error separately or use the error output, or use `.except(msg)` if the error case is unreachable\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
