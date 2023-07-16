plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between e94827e5b09b5b098ea10d0c57a84892fc73b5a7 and 28ee5b3abfd56e4e198892485dc47b41c5944d83
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stderr:

 error: use of `#[inline]` on trait method `default_inline` which has no body
    |
 LL |       #[inline]
    |  _____-^^^^^^^^
 LL | |     fn default_inline();
 LL | |     fn default_inline();
-   | |____- help: remove
+   | |___- help: remove
    |
    = note: `-D clippy::inline-fn-without-body` implied by `-D warnings`
 
 
 error: use of `#[inline]` on trait method `always_inline` which has no body
    |
    |
 LL |       #[inline(always)]
    |  _____-^^^^^^^^^^^^^^^^
 LL | |     fn always_inline();
-   | |____- help: remove
+   | |___- help: remove
 
 error: use of `#[inline]` on trait method `never_inline` which has no body
    |
    |
 LL |       #[inline(never)]
    |  _____-^^^^^^^^^^^^^^^
 LL | |     fn never_inline();
-   | |____- help: remove
+   | |___- help: remove
 error: aborting due to 3 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/inline_fn_without_body.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::inline_fn_without_body)]
 #![allow(clippy::inline_always)]
 trait Foo {
-    fn default_inline();
+     fn default_inline();
 
---
 
 fn main() {}
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/inline_fn_without_body.stage-id.fixed
To only update this specific test, also pass `--test-args inline_fn_without_body.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/inline_fn_without_body.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/inline_fn_without_body.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-2d976feacb0a3a74.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-7c8630ac629d3fc3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-1d810f37163c3a29.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-4554cde6a1339e03.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a77f10b259c274fa.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/inline_fn_without_body.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `#[inline]` on trait method `default_inline` which has no body","code":{"code":"clippy::inline_fn_without_body","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":107,"byte_end":116,"line_start":7,"line_end":7,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"    #[inline]","highlight_start":5,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::inline-fn-without-body` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove","code":null,"level":"help","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":107,"byte_end":120,"line_start":7,"line_end":8,"column_start":5,"column_end":4,"is_primary":true,"text":[{"text":"    #[inline]","highlight_start":5,"highlight_end":14},{"text":"    fn default_inline();","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `#[inline]` on trait method `default_inline` which has no body\n  --> tests/ui/inline_fn_without_body.rs:7:5\n   |\nLL |       #[inline]\n   |  _____-^^^^^^^^\nLL | |     fn default_inline();\n   | |___- help: remove\n   |\n   = note: `-D clippy::inline-fn-without-body` implied by `-D warnings`\n\n"}
{"message":"use of `#[inline]` on trait method `always_inline` which has no body","code":{"code":"clippy::inline_fn_without_body","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":147,"byte_end":164,"line_start":10,"line_end":10,"column_start":5,"column_end":22,"is_primary":true,"text":[{"text":"    #[inline(always)]","highlight_start":5,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove","code":null,"level":"help","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":147,"byte_end":168,"line_start":10,"line_end":11,"column_start":5,"column_end":4,"is_primary":true,"text":[{"text":"    #[inline(always)]","highlight_start":5,"highlight_end":22},{"text":"    fn always_inline();","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `#[inline]` on trait method `always_inline` which has no body\n  --> tests/ui/inline_fn_without_body.rs:10:5\n   |\nLL |       #[inline(always)]\n   |  _____-^^^^^^^^^^^^^^^^\nLL | |     fn always_inline();\n   | |___- help: remove\n\n"}
{"message":"use of `#[inline]` on trait method `never_inline` which has no body","code":{"code":"clippy::inline_fn_without_body","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":194,"byte_end":210,"line_start":13,"line_end":13,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    #[inline(never)]","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove","code":null,"level":"help","spans":[{"file_name":"tests/ui/inline_fn_without_body.rs","byte_start":194,"byte_end":214,"line_start":13,"line_end":14,"column_start":5,"column_end":4,"is_primary":true,"text":[{"text":"    #[inline(never)]","highlight_start":5,"highlight_end":21},{"text":"    fn never_inline();","highlight_start":1,"highlight_end":4}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `#[inline]` on trait method `never_inline` which has no body\n  --> tests/ui/inline_fn_without_body.rs:13:5\n   |\nLL |       #[inline(never)]\n   |  _____-^^^^^^^^^^^^^^^\nLL | |     fn never_inline();\n   | |___- help: remove\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.9.0/src/lib.rs:111:22
