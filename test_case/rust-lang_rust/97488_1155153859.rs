plain

---- [ui] src/test/ui/suggestions/issue-61963.rs stdout ----
diff of stderr:

30 LL - pub struct Foo {
31 LL + dyn pub struct Foo {
32    | 
+ help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
+    |
+ LL ~ <T: pub>
+ LL ~ T struct Foo {
33 
34 error: trait objects without an explicit `dyn` are deprecated
35   --> $DIR/issue-61963.rs:28:14


72 LL - pub struct Foo {
73 LL + dyn pub struct Foo {
74    | 
+ help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
+    |
+ LL ~ <T: pub>
+ LL ~ T struct Foo {
75 
76 error: trait objects without an explicit `dyn` are deprecated
77   --> $DIR/issue-61963.rs:18:1


86 LL - pub struct Foo {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
87 LL + dyn pub struct Foo {
88    | 
+ help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
+    |
+ LL ~ <T: pub>
+ LL ~ T struct Foo {
89 
90 error: trait objects without an explicit `dyn` are deprecated
91   --> $DIR/issue-61963.rs:18:1


100 LL - pub struct Foo {
101 LL + dyn pub struct Foo {
102    | 
+ help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
+    |
+ LL ~ <T: pub>
+ LL ~ T struct Foo {
103 
104 error: aborting due to 7 previous errors
105 

---
To only update this specific test, also pass `--test-args suggestions/issue-61963.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-61963.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-61963" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-61963/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:28:14
   |
   |
LL |     bar: Box<Bar>,
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:3:9
   |
   |
LL | #![deny(bare_trait_objects)]
   |         ^^^^^^^^^^^^^^^^^^
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {
   | 
help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
   |
LL ~ <T: pub>
LL ~ T struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:28:14
   |
   |
LL |     bar: Box<Bar>,
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:28:14
   |
   |
LL |     bar: Box<Bar>,
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {
   | 
help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
   |
LL ~ <T: pub>
LL ~ T struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {
   | 
help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
   |
LL ~ <T: pub>
LL ~ T struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {
   | 
help: use a blanket implementation to implement #[dom_struct] for all types that also implement "pub"
   |
LL ~ <T: pub>
LL ~ T struct Foo {

error: aborting due to 7 previous errors
------------------------------------------

