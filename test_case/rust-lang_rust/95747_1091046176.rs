plain
..........i............................................................................. 7128/12902
........................................................................................ 7216/12902
.......................................................................ii............... 7304/12902
.................ii...........................................................i......... 7392/12902
..................F..F.................................................................. 7480/12902
........................................................................................ 7656/12902
........................................................................................ 7744/12902
........................................................................ii.............. 7832/12902
..i....i..ii............................................................................ 7920/12902
---
---- [ui] ui/macros/macro-match-nonterminal.rs stdout ----
diff of stderr:

3    |
4 LL |     ($a, $b) => {
+    |
+    |
+    = note: `#[deny(missing_fragment_specifier)]` on by default
+    = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>
6 
7 error: missing fragment specifier
8   --> $DIR/macro-match-nonterminal.rs:2:10
8   --> $DIR/macro-match-nonterminal.rs:2:10

10 LL |     ($a, $b) => {
12    |
12    |
-    = note: `#[deny(missing_fragment_specifier)]` on by default
15    = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-match-nonterminal/macro-match-nonterminal.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-match-nonterminal.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-match-nonterminal.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-match-nonterminal" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-match-nonterminal/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing fragment specifier
   |
   |
LL |     ($a, $b) => {
   |
   |
   = note: `#[deny(missing_fragment_specifier)]` on by default
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>

error: missing fragment specifier
  --> /checkout/src/test/ui/macros/macro-match-nonterminal.rs:2:10
  --> /checkout/src/test/ui/macros/macro-match-nonterminal.rs:2:10
   |
LL |     ($a, $b) => {
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>

---
---- [ui] ui/macros/macro-missing-fragment.rs stdout ----
diff of stderr:

3    |
4 LL |     ( $( any_token $field_rust_type )* ) => {};
+    |
+    |
+    = note: `#[deny(missing_fragment_specifier)]` on by default
+    = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args macros/macro-missing-fragment.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-missing-fragment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-missing-fragment" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-missing-fragment/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing fragment specifier
   |
   |
LL |     ( $( any_token $field_rust_type )* ) => {}; //~ ERROR missing fragment
   |
   |
   = note: `#[deny(missing_fragment_specifier)]` on by default
   = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>

error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] ui/parser/macro/issue-33569.rs stdout ----
diff of stderr:

15    |
16 LL |     { $+ } => {
+    |
+    |
+    = note: `#[deny(missing_fragment_specifier)]` on by default
+    = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>
18 
19 error: aborting due to 3 previous errors
20 
---
To only update this specific test, also pass `--test-args parser/macro/issue-33569.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/macro/issue-33569.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/issue-33569" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/macro/issue-33569/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `+`
   |
   |
LL |     { $+ } => { //~ ERROR expected identifier, found `+`


error: expected one of: `*`, `+`, or `?`
   |
   |
LL |         $(x)(y) //~ ERROR expected one of: `*`, `+`, or `?`

error: missing fragment specifier
  --> /checkout/src/test/ui/parser/macro/issue-33569.rs:2:8
   |
   |
LL |     { $+ } => { //~ ERROR expected identifier, found `+`
   |
   |
   = note: `#[deny(missing_fragment_specifier)]` on by default
   = note: for more information, see issue #40107 <https://github.com/rust-lang/rust/issues/40107>

error: aborting due to 3 previous errors
------------------------------------------
