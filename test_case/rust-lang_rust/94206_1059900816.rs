plain
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/exact_div4.rs" "-L" "/tmp/compiletestdfnph1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdfnph1/intrinsics/exact_div4.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdfnph1/intrinsics/exact_div4.stage-id.aux"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---

error: 1 unexpected errors found, 1 expected errors not found
   0: std::panicking::begin_panic::<&str>
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/intrinsics/unchecked_div1.rs" "-L" "/tmp/compiletestdfnph1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestdfnph1/intrinsics/unchecked_div1.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestdfnph1/intrinsics/unchecked_div1.stage-id.aux"
    Error {
        line_num: 4,
        kind: Some(
            Error,
---
failures:

---- compile_test stdout ----

error: auxiliary build of "tests/ui/auxiliary/proc_macro_attr.rs" failed to compile: 
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/auxiliary/proc_macro_attr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_line_after_outer_attribute.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--emit=link" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_line_after_outer_attribute.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
------------------------------------------

diff of stderr:


 error: it is more concise to loop over references to containers instead of using explicit iteration methods
    |
    |
 LL |     for _v in vec.iter() {}
    |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`
    |
    = note: `-D clippy::explicit-iter-loop` implied by `-D warnings`
 
 error: it is more concise to loop over references to containers instead of using explicit iteration methods
    |
    |
 LL |     for _v in vec.iter_mut() {}
    |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`
 
 error: it is more concise to loop over containers instead of using explicit iteration methods
    |
 LL |     for _v in out_vec.into_iter() {}
 LL |     for _v in out_vec.into_iter() {}
    |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
    |
    = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
 
 error: it is more concise to loop over references to containers instead of using explicit iteration methods
    |
    |
 LL |     for _v in [1, 2, 3].iter() {}
    |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
 
 error: it is more concise to loop over references to containers instead of using explicit iteration methods
    |
    |
 LL |     for _v in [0; 32].iter() {}
    |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
 
 error: it is more concise to loop over references to containers instead of using explicit iteration methods
    |
    |
 LL |     for _v in ll.iter() {}
    |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
 
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in vd.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
 
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in bh.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
-
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in hm.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
-
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in bt.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
-
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in hs.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
-
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |     for _v in bs.iter() {}
-   |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
-
-error: it is more concise to loop over containers instead of using explicit iteration methods
-   |
-LL |         for i in iterator.into_iter() {
-LL |         for i in iterator.into_iter() {
-   |                  ^^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `iterator`
-
-error: it is more concise to loop over references to containers instead of using explicit iteration methods
-   |
-   |
-LL |         for _ in t.into_iter() {}
-   |                  ^^^^^^^^^^^^^ help: to write this more concisely, try: `&t`
-
-error: it is more concise to loop over containers instead of using explicit iteration methods
-   |
-   |
-LL |         for _ in r.into_iter() {}
-   |                  ^^^^^^^^^^^^^ help: to write this more concisely, try: `r`
-error: aborting due to 15 previous errors
-
+thread 'rustc' has overflowed its stack
+thread 'rustc' has overflowed its stack
+fatal runtime error: stack overflow

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loop_fixable.stage-id.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/for_loop_fixable.stage-id.stderr
thread '[ui] ui/for_loop_fixable.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 8, column: 2)', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/runtest.rs:2371:15
diff of stderr:

 error: this let-binding has unit value
   --> $DIR/let_unit.rs:14:5
   --> $DIR/let_unit.rs:14:5
    |
 LL |     let _x = println!("x");
    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `println!("x");`
    |
    = note: `-D clippy::let-unit-value` implied by `-D warnings`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:18:9
    |
    |
 LL |         let _a = ();
    |         ^^^^^^^^^^^^ help: omit the `let` binding: `();`
-error: this let-binding has unit value
-  --> $DIR/let_unit.rs:53:5
-   |
-LL | /     let _ = v
-LL | /     let _ = v
-LL | |         .into_iter()
-LL | |         .map(|i| i * 2)
-LL | |         .filter(|i| i % 2 == 0)
-LL | |         .map(|_| ())
-LL | |         .next()
-LL | |         .unwrap();
-   | |__________________^
-   |
-help: omit the `let` binding
-   |
-LL ~     v
-LL +         .into_iter()
-LL +         .map(|i| i * 2)
-LL +         .filter(|i| i % 2 == 0)
-LL +         .map(|_| ())
-LL +         .next()
 
-error: aborting due to 3 previous errors
-
+thread 'rustc' has overflowed its stack
+thread 'rustc' has overflowed its stack
+fatal runtime error: stack overflow

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/let_unit.stage-id.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/let_unit.stage-id.stderr
thread '[ui] ui/let_unit.rs' panicked at 'Could not retrieve suggestions from JSON: Error("expected ident", line: 4, column: 2)', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/runtest.rs:2371:15

error: auxiliary build of "tests/ui/auxiliary/proc_macro_attr.rs" failed to compile: 
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/auxiliary/proc_macro_attr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--emit=link" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
------------------------------------------

diff of stderr:


 error: using `self.to_string` in `fmt::Display` implementation will cause infinite recursion
    |
 LL |         write!(f, "{}", self.to_string())
error: test failed, to rerun pass '--test compile-test'
    |                         ^^^^^^^^^^^^^^^^
    |                         ^^^^^^^^^^^^^^^^
    |
    = note: `-D clippy::recursive-format-impl` implied by `-D warnings`
 error: unnecessary use of `to_string`
   --> $DIR/recursive_format_impl.rs:61:50
    |
    |
 LL |             Self::E(string) => write!(f, "E {}", string.to_string()),
    |
    |
    = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`
 
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
 LL |         write!(f, "{}", self)
    |         ^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
 LL |         write!(f, "{}", &self)
    |         ^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Debug` in `impl Debug` will cause infinite recursion
    |
 LL |         write!(f, "{:?}", &self)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
    |
 LL |         write!(f, "{}", &&&self)
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
 LL |         write!(f, "{}", &*self)
    |         ^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Debug` in `impl Debug` will cause infinite recursion
    |
 LL |         write!(f, "{:?}", &*self)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
 LL |         write!(f, "{}", *self)
    |         ^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
    |
 LL |         write!(f, "{}", **&&*self)
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 
 error: using `self` as `Display` in `impl Display` will cause infinite recursion
    |
    |
 LL |         write!(f, "{}", &&**&&*self)
    |
    = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)
 
-error: aborting due to 11 previous errors
-error: aborting due to 11 previous errors
 
+thread 'rustc' has overflowed its stack
+fatal runtime error: stack overflow

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/recursive_format_impl.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursive_format_impl.rs`

error: 1 errors occurred comparing output.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/recursive_format_impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/recursive_format_impl.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/recursive_format_impl.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `self.to_string` in `fmt::Display` implementation will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":466,"byte_end":482,"line_start":29,"line_end":29,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"        write!(f, \"{}\", self.to_string())","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::recursive-format-impl` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: using `self.to_string` in `fmt::Display` implementation will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:29:25\n   |\nLL |         write!(f, \"{}\", self.to_string())\n   |                         ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::recursive-format-impl` implied by `-D warnings`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1078,"byte_end":1096,"line_start":61,"line_end":61,"column_start":50,"column_end":68,"is_primary":true,"text":[{"text":"            Self::E(string) => write!(f, \"E {}\", string.to_string()),","highlight_start":50,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17446,"byte_end":17476,"line_start":485,"line_end":485,"column_start":49,"column_end":79,"is_primary":false,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":49,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1060,"byte_end":1097,"line_start":61,"line_end":61,"column_start":32,"column_end":69,"is_primary":false,"text":[{"text":"            Self::E(string) => write!(f, \"E {}\", string.to_string()),","highlight_start":32,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::format_args!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":29912,"byte_end":29936,"line_start":851,"line_end":851,"column_start":5,"column_end":29,"is_primary":false,"text":[{"text":"    macro_rules! format_args {","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::unnecessary-to-owned` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/recursive_format_impl.rs:61:50\n   |\nLL |             Self::E(string) => write!(f, \"E {}\", string.to_string()),\n   |                                                  ^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`\n   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1369,"byte_end":1390,"line_start":73,"line_end":73,"column_start":9,"column_end":30,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", self)","highlight_start":9,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:73:9\n   |\nLL |         write!(f, \"{}\", self)\n   |         ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1558,"byte_end":1580,"line_start":82,"line_end":82,"column_start":9,"column_end":31,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", &self)","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:82:9\n   |\nLL |         write!(f, \"{}\", &self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Debug` in `impl Debug` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1700,"byte_end":1724,"line_start":88,"line_end":88,"column_start":9,"column_end":33,"is_primary":false,"text":[{"text":"        write!(f, \"{:?}\", &self)","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Debug` in `impl Debug` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:88:9\n   |\nLL |         write!(f, \"{:?}\", &self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":1903,"byte_end":1927,"line_start":97,"line_end":97,"column_start":9,"column_end":33,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", &&&self)","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:97:9\n   |\nLL |         write!(f, \"{}\", &&&self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":3315,"byte_end":3338,"line_start":171,"line_end":171,"column_start":9,"column_end":32,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", &*self)","highlight_start":9,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:171:9\n   |\nLL |         write!(f, \"{}\", &*self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Debug` in `impl Debug` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":3454,"byte_end":3479,"line_start":177,"line_end":177,"column_start":9,"column_end":34,"is_primary":false,"text":[{"text":"        write!(f, \"{:?}\", &*self)","highlight_start":9,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Debug` in `impl Debug` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:177:9\n   |\nLL |         write!(f, \"{:?}\", &*self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":3729,"byte_end":3751,"line_start":193,"line_end":193,"column_start":9,"column_end":31,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", *self)","highlight_start":9,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:193:9\n   |\nLL |         write!(f, \"{}\", *self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":4001,"byte_end":4027,"line_start":209,"line_end":209,"column_start":9,"column_end":35,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", **&&*self)","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:209:9\n   |\nLL |         write!(f, \"{}\", **&&*self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"using `self` as `Display` in `impl Display` will cause infinite recursion","code":{"code":"clippy::recursive_format_impl","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17431,"byte_end":17477,"line_start":485,"line_end":485,"column_start":34,"column_end":80,"is_primary":true,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":34,"highlight_end":80}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/recursive_format_impl.rs","byte_start":4277,"byte_end":4305,"line_start":225,"line_end":225,"column_start":9,"column_end":37,"is_primary":false,"text":[{"text":"        write!(f, \"{}\", &&**&&*self)","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17395,"line_start":484,"line_end":484,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: using `self` as `Display` in `impl Display` will cause infinite recursion\n  --> tests/ui/recursive_format_impl.rs:225:9\n   |\nLL |         write!(f, \"{}\", &&**&&*self)\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `write` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
thread 'rustc' has overflowed its stack
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
------------------------------------------

diff of stderr:


 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:46:11
+  --> $DIR/significant_drop_in_scrutinee.rs:48:11
    |
-LL |     match mutex.lock().unwrap().foo() { // Should trigger lint
+LL |     match mutex.lock().unwrap().foo() {
    |
    |
    = note: `-D clippy::significant-drop-in-scrutinee` implied by `-D warnings`
    = help: Try this:
            let value = mutex.lock().unwrap().foo();
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:107:11
+  --> $DIR/significant_drop_in_scrutinee.rs:117:11
    |
-LL |     match s.lock_m().get_the_value() { // Should trigger lint
+LL |     match s.lock_m().get_the_value() {
    |
    = help: Try this:
    = help: Try this:
            let value = s.lock_m().get_the_value();
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:123:11
+  --> $DIR/significant_drop_in_scrutinee.rs:134:11
    |
-LL |     match s.lock_m_m().get_the_value() { // Should trigger lint
+LL |     match s.lock_m_m().get_the_value() {
    |
    = help: Try this:
    = help: Try this:
            let value = s.lock_m_m().get_the_value();
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:167:11
+  --> $DIR/significant_drop_in_scrutinee.rs:179:11
    |
-LL |     match counter.temp_increment().len() { // Should trigger lint
+LL |     match counter.temp_increment().len() {
    |
    = help: Try this:
            let value = counter.temp_increment().len();
            match value {
            match value {
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:186:11
+  --> $DIR/significant_drop_in_scrutinee.rs:199:11
    |
-LL |     match (mutex1.lock().unwrap().s.len(), true) { // Should trigger lint
+LL |     match (mutex1.lock().unwrap().s.len(), true) {
    |
    = help: Try this:
    = help: Try this:
            let value = (mutex1.lock().unwrap().s.len(), true);
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:199:11
+  --> $DIR/significant_drop_in_scrutinee.rs:213:11
    |
-LL |     match mutex.lock().unwrap().s.len() > 1 { // Should trigger lint
+LL |     match mutex.lock().unwrap().s.len() > 1 {
    |
    = help: Try this:
    = help: Try this:
            let value = mutex.lock().unwrap().s.len() > 1;
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:225:11
+  --> $DIR/significant_drop_in_scrutinee.rs:241:11
    |
-LL |     match get_mutex_guard().s.len() > 1 { // Should trigger lint
+LL |     match get_mutex_guard().s.len() > 1 {
    |
    = help: Try this:
    = help: Try this:
            let value = get_mutex_guard().s.len() > 1;
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:239:11
+  --> $DIR/significant_drop_in_scrutinee.rs:256:11
    |
-LL |     match match i { 100 => mutex1.lock().unwrap(), _ => mutex2.lock().unwrap() }.s.len() > 1 { // Should trigger lint
-   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL |       match match i {
+   |  ___________^
+LL | |         100 => mutex1.lock().unwrap(),
+LL | |         _ => mutex2.lock().unwrap(),
+LL | |     }
+LL | |     .s
+LL | |     .len()
+LL | |         > 1
    |
    = help: Try this:
    = help: Try this:
-           let value = match i { 100 => mutex1.lock().unwrap(), _ => mutex2.lock().unwrap() }.s.len() > 1;
+           let value = match i {
+                   100 => mutex1.lock().unwrap(),
+                   _ => mutex2.lock().unwrap(),
+               .s
+               .len()
+                   > 1;
            match value {
            match value {
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:253:11
+  --> $DIR/significant_drop_in_scrutinee.rs:278:11
    |
-LL |     match if i > 1 { mutex1.lock().unwrap() } else { mutex2.lock().unwrap() }.s.len() > 1 { // Should trigger lint
-   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL |       match if i > 1 {
+LL | |         mutex1.lock().unwrap()
+LL | |     } else {
+LL | |         mutex2.lock().unwrap()
+...  |
+...  |
+LL | |     .len()
+LL | |         > 1
+   | |___________^
    |
    = help: Try this:
-           let value = if i > 1 { mutex1.lock().unwrap() } else { mutex2.lock().unwrap() }.s.len() > 1;
+           let value = if i > 1 {
+                   mutex1.lock().unwrap()
+                   mutex2.lock().unwrap()
+               }
+               .s
+               .len()
+               .len()
+                   > 1;
            match value {
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:289:11
+  --> $DIR/significant_drop_in_scrutinee.rs:328:11
    |
-LL |     match s.lock().deref().deref() { // Should trigger lint
+LL |     match s.lock().deref().deref() {
    |
    = help: Try this:
    = help: Try this:
            let value = s.lock().deref().deref();
 
 
 error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.
-  --> $DIR/significant_drop_in_scrutinee.rs:304:11
+  --> $DIR/significant_drop_in_scrutinee.rs:344:11
    |
-LL |     match mutex.lock().unwrap().i = i { // Should trigger lint
+LL |     match mutex.lock().unwrap().i = i {
    |
    = help: Try this:
    = help: Try this:
            let value = mutex.lock().unwrap().i = i;
 
 error: aborting due to 11 previous errors
 
 
---
To only update this specific test, also pass `--test-args significant_drop_in_scrutinee.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/significant_drop_in_scrutinee.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/significant_drop_in_scrutinee.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/significant_drop_in_scrutinee.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":1054,"byte_end":1081,"line_start":48,"line_end":48,"column_start":11,"column_end":38,"is_primary":true,"text":[{"text":"    match mutex.lock().unwrap().foo() {","highlight_start":11,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::significant-drop-in-scrutinee` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"Try this:\nlet value = mutex.lock().unwrap().foo();\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:48:11\n   |\nLL |     match mutex.lock().unwrap().foo() {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::significant-drop-in-scrutinee` implied by `-D warnings`\n   = help: Try this:\n           let value = mutex.lock().unwrap().foo();\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":2422,"byte_end":2448,"line_start":117,"line_end":117,"column_start":11,"column_end":37,"is_primary":true,"text":[{"text":"    match s.lock_m().get_the_value() {","highlight_start":11,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = s.lock_m().get_the_value();\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:117:11\n   |\nLL |     match s.lock_m().get_the_value() {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = s.lock_m().get_the_value();\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":2902,"byte_end":2930,"line_start":134,"line_end":134,"column_start":11,"column_end":39,"is_primary":true,"text":[{"text":"    match s.lock_m_m().get_the_value() {","highlight_start":11,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = s.lock_m_m().get_the_value();\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:134:11\n   |\nLL |     match s.lock_m_m().get_the_value() {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = s.lock_m_m().get_the_value();\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":3945,"byte_end":3975,"line_start":179,"line_end":179,"column_start":11,"column_end":41,"is_primary":true,"text":[{"text":"    match counter.temp_increment().len() {","highlight_start":11,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = counter.temp_increment().len();\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:179:11\n   |\nLL |     match counter.temp_increment().len() {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = counter.temp_increment().len();\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":4430,"byte_end":4468,"line_start":199,"line_end":199,"column_start":11,"column_end":49,"is_primary":true,"text":[{"text":"    match (mutex1.lock().unwrap().s.len(), true) {","highlight_start":11,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = (mutex1.lock().unwrap().s.len(), true);\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:199:11\n   |\nLL |     match (mutex1.lock().unwrap().s.len(), true) {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = (mutex1.lock().unwrap().s.len(), true);\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":4807,"byte_end":4840,"line_start":213,"line_end":213,"column_start":11,"column_end":44,"is_primary":true,"text":[{"text":"    match mutex.lock().unwrap().s.len() > 1 {","highlight_start":11,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = mutex.lock().unwrap().s.len() > 1;\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:213:11\n   |\nLL |     match mutex.lock().unwrap().s.len() > 1 {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = mutex.lock().unwrap().s.len() > 1;\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":5532,"byte_end":5561,"line_start":241,"line_end":241,"column_start":11,"column_end":40,"is_primary":true,"text":[{"text":"    match get_mutex_guard().s.len() > 1 {","highlight_start":11,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = get_mutex_guard().s.len() > 1;\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:241:11\n   |\nLL |     match get_mutex_guard().s.len() > 1 {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = get_mutex_guard().s.len() > 1;\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":5926,"byte_end":6047,"line_start":256,"line_end":262,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    match match i {","highlight_start":11,"highlight_end":20},{"text":"        100 => mutex1.lock().unwrap(),","highlight_start":1,"highlight_end":39},{"text":"        _ => mutex2.lock().unwrap(),","highlight_start":1,"highlight_end":37},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"    .s","highlight_start":1,"highlight_end":7},{"text":"    .len()","highlight_start":1,"highlight_end":11},{"text":"        > 1","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = match i {\n        100 => mutex1.lock().unwrap(),\n        _ => mutex2.lock().unwrap(),\n    }\n    .s\n    .len()\n        > 1;\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:256:11\n   |\nLL |       match match i {\n   |  ___________^\nLL | |         100 => mutex1.lock().unwrap(),\nLL | |         _ => mutex2.lock().unwrap(),\nLL | |     }\nLL | |     .s\nLL | |     .len()\nLL | |         > 1\n   | |___________^\n   |\n   = help: Try this:\n           let value = match i {\n                   100 => mutex1.lock().unwrap(),\n                   _ => mutex2.lock().unwrap(),\n               }\n               .s\n               .len()\n                   > 1;\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":6413,"byte_end":6534,"line_start":278,"line_end":285,"column_start":11,"column_end":12,"is_primary":true,"text":[{"text":"    match if i > 1 {","highlight_start":11,"highlight_end":21},{"text":"        mutex1.lock().unwrap()","highlight_start":1,"highlight_end":31},{"text":"    } else {","highlight_start":1,"highlight_end":13},{"text":"        mutex2.lock().unwrap()","highlight_start":1,"highlight_end":31},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"    .s","highlight_start":1,"highlight_end":7},{"text":"    .len()","highlight_start":1,"highlight_end":11},{"text":"        > 1","highlight_start":1,"highlight_end":12}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = if i > 1 {\n        mutex1.lock().unwrap()\n    } else {\n        mutex2.lock().unwrap()\n    }\n    .s\n    .len()\n        > 1;\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:278:11\n   |\nLL |       match if i > 1 {\n   |  ___________^\nLL | |         mutex1.lock().unwrap()\nLL | |     } else {\nLL | |         mutex2.lock().unwrap()\n...  |\nLL | |     .len()\nLL | |         > 1\n   | |___________^\n   |\n   = help: Try this:\n           let value = if i > 1 {\n                   mutex1.lock().unwrap()\n               } else {\n                   mutex2.lock().unwrap()\n               }\n               .s\n               .len()\n                   > 1;\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":7439,"byte_end":7463,"line_start":328,"line_end":328,"column_start":11,"column_end":35,"is_primary":true,"text":[{"text":"    match s.lock().deref().deref() {","highlight_start":11,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = s.lock().deref().deref();\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:328:11\n   |\nLL |     match s.lock().deref().deref() {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = s.lock().deref().deref();\n           match value {\n\n"}
{"message":"The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.","code":{"code":"clippy::significant_drop_in_scrutinee","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/significant_drop_in_scrutinee.rs","byte_start":7785,"byte_end":7812,"line_start":344,"line_end":344,"column_start":11,"column_end":38,"is_primary":true,"text":[{"text":"    match mutex.lock().unwrap().i = i {","highlight_start":11,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"Try this:\nlet value = mutex.lock().unwrap().i = i;\nmatch value {","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: The lifetimes of temporaries in a match scrutinee last to the end of the match expressions. One or more of the temporaries in this match scrutinee have a drop with a side-effect (such as unlocking a mutex). Consider calling the scrutinee outside of the match expression and using the result instead.\n  --> tests/ui/significant_drop_in_scrutinee.rs:344:11\n   |\nLL |     match mutex.lock().unwrap().i = i {\n   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: Try this:\n           let value = mutex.lock().unwrap().i = i;\n           match value {\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
