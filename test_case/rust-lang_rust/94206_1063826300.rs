plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 01ad0ad653d57a5ccecffb08aff3c5564012f133 and b33d2114b68372d4170a55e81ae705eeb47661c3
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
failures:

---- compile_test stdout ----

error: auxiliary build of "tests/ui/auxiliary/proc_macro_attr.rs" failed to compile: 
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/auxiliary/proc_macro_attr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_line_after_outer_attribute.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--emit=link" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/empty_line_after_outer_attribute.stage-id.aux"
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/auxiliary/proc_macro_attr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--emit=link" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/needless_arbitrary_self_type_unfixable.stage-id.aux"
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
    |                         ^^^^^^^^^^^^^^^^
    |
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/recursive_format_impl.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/recursive_format_impl.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/recursive_format_impl.stage-id.aux"
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

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22

