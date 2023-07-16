plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5728bd64b49b0e78d0180efed75ef0870ae60266 and 7259516caf2073b02a11bae9b660bf0ef2fb4b08
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:49:8
+  --> $DIR/equatable_if_let.rs:49:16
    |
 LL |     if let 2 = a {}
-   |        ^^^^^^^^^ help: try: `a == 2`
+   |                ^ help: try: `a == 2`
    |
    = note: `-D clippy::equatable-if-let` implied by `-D warnings`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:50:8
+  --> $DIR/equatable_if_let.rs:50:32
    |
    |
 LL |     if let Ordering::Greater = a.cmp(&b) {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `a.cmp(&b) == Ordering::Greater`
+   |                                ^^^^^^^^^ help: try: `a.cmp(&b) == Ordering::Greater`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:51:8
+  --> $DIR/equatable_if_let.rs:51:22
    |
    |
 LL |     if let Some(2) = c {}
-   |        ^^^^^^^^^^^^^^^ help: try: `c == Some(2)`
+   |                      ^ help: try: `c == Some(2)`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:52:8
+  --> $DIR/equatable_if_let.rs:52:40
    |
    |
 LL |     if let Struct { a: 2, b: false } = d {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `d == (Struct { a: 2, b: false })`
+   |                                        ^ help: try: `d == (Struct { a: 2, b: false })`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:53:8
+  --> $DIR/equatable_if_let.rs:53:41
    |
    |
 LL |     if let Enum::TupleVariant(32, 64) = e {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::TupleVariant(32, 64)`
+   |                                         ^ help: try: `e == Enum::TupleVariant(32, 64)`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:54:8
+  --> $DIR/equatable_if_let.rs:54:51
    |
    |
 LL |     if let Enum::RecordVariant { a: 64, b: 32 } = e {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == (Enum::RecordVariant { a: 64, b: 32 })`
+   |                                                   ^ help: try: `e == (Enum::RecordVariant { a: 64, b: 32 })`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:55:8
+  --> $DIR/equatable_if_let.rs:55:32
    |
    |
 LL |     if let Enum::UnitVariant = e {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `e == Enum::UnitVariant`
+   |                                ^ help: try: `e == Enum::UnitVariant`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:56:8
+  --> $DIR/equatable_if_let.rs:56:62
    |
    |
 LL |     if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })`
+   |                                                              ^^^^^^^ help: try: `(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:66:8
+  --> $DIR/equatable_if_let.rs:66:33
    |
    |
 LL |     if let NotStructuralEq::A = g {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `g == NotStructuralEq::A`
+   |                                 ^ help: try: `g == NotStructuralEq::A`
 error: this pattern matching can be expressed using equality
-  --> $DIR/equatable_if_let.rs:68:8
+  --> $DIR/equatable_if_let.rs:68:39
    |
    |
 LL |     if let Some(NotStructuralEq::A) = Some(g) {}
-   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `Some(g) == Some(NotStructuralEq::A)`
+   |                                       ^^^^^^^ help: try: `Some(g) == Some(NotStructuralEq::A)`
 error: aborting due to 10 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/equatable_if_let.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![allow(unused_variables, dead_code)]
 #![warn(clippy::equatable_if_let)]
 use std::cmp::Ordering;
 
 
 #[derive(PartialEq)]
 enum Enum {
     TupleVariant(i32, u64),
     RecordVariant { a: i64, b: u32 },
     UnitVariant,
     Recursive(Struct),
 
 
 #[derive(PartialEq)]
 struct Struct {
     a: i32,
     b: bool,
 
 
 enum NotPartialEq {
     A,
     B,
 
 enum NotStructuralEq {
     A,
     B,
     B,
 }
 
 impl PartialEq for NotStructuralEq {
     fn eq(&self, _: &NotStructuralEq) -> bool {
     }
 }
 
 fn main() {
 fn main() {
     let a = 2;
     let b = 3;
     let c = Some(2);
     let d = Struct { a: 2, b: false };
     let e = Enum::UnitVariant;
     let f = NotPartialEq::A;
     let g = NotStructuralEq::A;
     // true
 
-    if a == 2 {}
-    if a == 2 {}
-    if a.cmp(&b) == Ordering::Greater {}
-    if c == Some(2) {}
-    if d == (Struct { a: 2, b: false }) {}
-    if e == Enum::TupleVariant(32, 64) {}
-    if e == (Enum::RecordVariant { a: 64, b: 32 }) {}
-    if e == Enum::UnitVariant {}
-    if (e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false }) {}
+    if let 2 = a == 2 {}
+    if let Ordering::Greater = a.cmp(&b) == Ordering::Greater {}
+    if let Some(2) = c == Some(2) {}
+    if let Struct { a: 2, b: false } = d == (Struct { a: 2, b: false }) {}
+    if let Enum::TupleVariant(32, 64) = e == Enum::TupleVariant(32, 64) {}
+    if let Enum::RecordVariant { a: 64, b: 32 } = e == (Enum::RecordVariant { a: 64, b: 32 }) {}
+    if let Enum::UnitVariant = e == Enum::UnitVariant {}
+    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false }) {}
     // false
 
 
     if let 2 | 3 = a {}
     if let x @ 2 = a {}
     if let Some(3 | 4) = c {}
     if let Struct { a, b: false } = d {}
     if let Struct { a: 2, b: x } = d {}
     if let NotPartialEq::A = f {}
-    if g == NotStructuralEq::A {}
+    if let NotStructuralEq::A = g == NotStructuralEq::A {}
     if let Some(NotPartialEq::A) = Some(f) {}
-    if Some(g) == Some(NotStructuralEq::A) {}
+    if let Some(NotStructuralEq::A) = Some(g) == Some(NotStructuralEq::A) {}
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/equatable_if_let.stage-id.fixed
To only update this specific test, also pass `--test-args equatable_if_let.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/equatable_if_let.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/equatable_if_let.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-89222f6e0d369b36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-b3e76e8f62643cc6.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-ea1db0c41efed0d6.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-2c42e7af51424ce5.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-392b40a2759b0504.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bc2b0c8a9dcb9fb3.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-c6bd037ba33baa25.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8bd56d0234290b82.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b0f19a5fad83a10b.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/equatable_if_let.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":731,"byte_end":732,"line_start":49,"line_end":49,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    if let 2 = a {}","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::equatable-if-let` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":731,"byte_end":732,"line_start":49,"line_end":49,"column_start":16,"column_end":17,"is_primary":true,"text":[{"text":"    if let 2 = a {}","highlight_start":16,"highlight_end":17}],"label":null,"suggested_replacement":"a == 2","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:49:16\n   |\nLL |     if let 2 = a {}\n   |                ^ help: try: `a == 2`\n   |\n   = note: `-D clippy::equatable-if-let` implied by `-D warnings`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":767,"byte_end":776,"line_start":50,"line_end":50,"column_start":32,"column_end":41,"is_primary":true,"text":[{"text":"    if let Ordering::Greater = a.cmp(&b) {}","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":767,"byte_end":776,"line_start":50,"line_end":50,"column_start":32,"column_end":41,"is_primary":true,"text":[{"text":"    if let Ordering::Greater = a.cmp(&b) {}","highlight_start":32,"highlight_end":41}],"label":null,"suggested_replacement":"a.cmp(&b) == Ordering::Greater","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:50:32\n   |\nLL |     if let Ordering::Greater = a.cmp(&b) {}\n   |                                ^^^^^^^^^ help: try: `a.cmp(&b) == Ordering::Greater`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":801,"byte_end":802,"line_start":51,"line_end":51,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    if let Some(2) = c {}","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":801,"byte_end":802,"line_start":51,"line_end":51,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    if let Some(2) = c {}","highlight_start":22,"highlight_end":23}],"label":null,"suggested_replacement":"c == Some(2)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:51:22\n   |\nLL |     if let Some(2) = c {}\n   |                      ^ help: try: `c == Some(2)`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":845,"byte_end":846,"line_start":52,"line_end":52,"column_start":40,"column_end":41,"is_primary":true,"text":[{"text":"    if let Struct { a: 2, b: false } = d {}","highlight_start":40,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":845,"byte_end":846,"line_start":52,"line_end":52,"column_start":40,"column_end":41,"is_primary":true,"text":[{"text":"    if let Struct { a: 2, b: false } = d {}","highlight_start":40,"highlight_end":41}],"label":null,"suggested_replacement":"d == (Struct { a: 2, b: false })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:52:40\n   |\nLL |     if let Struct { a: 2, b: false } = d {}\n   |                                        ^ help: try: `d == (Struct { a: 2, b: false })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":890,"byte_end":891,"line_start":53,"line_end":53,"column_start":41,"column_end":42,"is_primary":true,"text":[{"text":"    if let Enum::TupleVariant(32, 64) = e {}","highlight_start":41,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":890,"byte_end":891,"line_start":53,"line_end":53,"column_start":41,"column_end":42,"is_primary":true,"text":[{"text":"    if let Enum::TupleVariant(32, 64) = e {}","highlight_start":41,"highlight_end":42}],"label":null,"suggested_replacement":"e == Enum::TupleVariant(32, 64)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:53:41\n   |\nLL |     if let Enum::TupleVariant(32, 64) = e {}\n   |                                         ^ help: try: `e == Enum::TupleVariant(32, 64)`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":945,"byte_end":946,"line_start":54,"line_end":54,"column_start":51,"column_end":52,"is_primary":true,"text":[{"text":"    if let Enum::RecordVariant { a: 64, b: 32 } = e {}","highlight_start":51,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":945,"byte_end":946,"line_start":54,"line_end":54,"column_start":51,"column_end":52,"is_primary":true,"text":[{"text":"    if let Enum::RecordVariant { a: 64, b: 32 } = e {}","highlight_start":51,"highlight_end":52}],"label":null,"suggested_replacement":"e == (Enum::RecordVariant { a: 64, b: 32 })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:54:51\n   |\nLL |     if let Enum::RecordVariant { a: 64, b: 32 } = e {}\n   |                                                   ^ help: try: `e == (Enum::RecordVariant { a: 64, b: 32 })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":981,"byte_end":982,"line_start":55,"line_end":55,"column_start":32,"column_end":33,"is_primary":true,"text":[{"text":"    if let Enum::UnitVariant = e {}","highlight_start":32,"highlight_end":33}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":981,"byte_end":982,"line_start":55,"line_end":55,"column_start":32,"column_end":33,"is_primary":true,"text":[{"text":"    if let Enum::UnitVariant = e {}","highlight_start":32,"highlight_end":33}],"label":null,"suggested_replacement":"e == Enum::UnitVariant","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:55:32\n   |\nLL |     if let Enum::UnitVariant = e {}\n   |                                ^ help: try: `e == Enum::UnitVariant`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1047,"byte_end":1054,"line_start":56,"line_end":56,"column_start":62,"column_end":69,"is_primary":true,"text":[{"text":"    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}","highlight_start":62,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1047,"byte_end":1054,"line_start":56,"line_end":56,"column_start":62,"column_end":69,"is_primary":true,"text":[{"text":"    if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}","highlight_start":62,"highlight_end":69}],"label":null,"suggested_replacement":"(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:56:62\n   |\nLL |     if let (Enum::UnitVariant, &Struct { a: 2, b: false }) = (e, &d) {}\n   |                                                              ^^^^^^^ help: try: `(e, &d) == (Enum::UnitVariant, &Struct { a: 2, b: false })`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1298,"byte_end":1299,"line_start":66,"line_end":66,"column_start":33,"column_end":34,"is_primary":true,"text":[{"text":"    if let NotStructuralEq::A = g {}","highlight_start":33,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1298,"byte_end":1299,"line_start":66,"line_end":66,"column_start":33,"column_end":34,"is_primary":true,"text":[{"text":"    if let NotStructuralEq::A = g {}","highlight_start":33,"highlight_end":34}],"label":null,"suggested_replacement":"g == NotStructuralEq::A","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:66:33\n   |\nLL |     if let NotStructuralEq::A = g {}\n   |                                 ^ help: try: `g == NotStructuralEq::A`\n\n"}
{"message":"this pattern matching can be expressed using equality","code":{"code":"clippy::equatable_if_let","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1387,"byte_end":1394,"line_start":68,"line_end":68,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    if let Some(NotStructuralEq::A) = Some(g) {}","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/equatable_if_let.rs","byte_start":1387,"byte_end":1394,"line_start":68,"line_end":68,"column_start":39,"column_end":46,"is_primary":true,"text":[{"text":"    if let Some(NotStructuralEq::A) = Some(g) {}","highlight_start":39,"highlight_end":46}],"label":null,"suggested_replacement":"Some(g) == Some(NotStructuralEq::A)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this pattern matching can be expressed using equality\n  --> tests/ui/equatable_if_let.rs:68:39\n   |\nLL |     if let Some(NotStructuralEq::A) = Some(g) {}\n   |                                       ^^^^^^^ help: try: `Some(g) == Some(NotStructuralEq::A)`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.0/src/lib.rs:105:22
