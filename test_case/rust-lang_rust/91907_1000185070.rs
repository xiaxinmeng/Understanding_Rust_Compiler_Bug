plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 390bb3406d6c15894139830f6a30e16a1e92053f and 86ad6bcbd8c8cca58e94973569f8face9d6f9e59
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

---- compile_test stdout ----
diff of stdout:

 if_chain! {
     if let ExprKind::Repeat(value, length) = expr.kind;
     if let ExprKind::Lit(ref lit) = value.kind;
     if let LitKind::Int(1, LitIntType::Unsigned(UintTy::U8)) = lit.node;
-    let expr1 = &cx.tcx.hir().body(length.body).value;
+    if let ArrayLen::Body(anon_const) = length;
+    let expr1 = &cx.tcx.hir().body(anon_const.body).value;
     if let ExprKind::Lit(ref lit1) = expr1.kind;
     if let LitKind::Int(5, LitIntType::Unsuffixed) = lit1.node;
     then {
         // report your lint here
 }
 
error: test failed, to rerun pass '--test compile-test'


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/author/repeat.stage-id.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args author/repeat.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/author/repeat.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/author/repeat.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-8a400cd14c1fc33b.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/author/repeat.stage-id.aux"
------------------------------------------
if_chain! {
if_chain! {
    if let ExprKind::Repeat(value, length) = expr.kind;
    if let ExprKind::Lit(ref lit) = value.kind;
    if let LitKind::Int(1, LitIntType::Unsigned(UintTy::U8)) = lit.node;
    if let ArrayLen::Body(anon_const) = length;
    let expr1 = &cx.tcx.hir().body(anon_const.body).value;
    if let ExprKind::Lit(ref lit1) = expr1.kind;
    if let LitKind::Int(5, LitIntType::Unsuffixed) = lit1.node;
    then {
        // report your lint here
}

------------------------------------------
stderr:
