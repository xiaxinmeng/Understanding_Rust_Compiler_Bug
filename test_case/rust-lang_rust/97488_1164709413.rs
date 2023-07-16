plain
...........................................................iii.......................... 13024/13101
.............................................................................
failures:

---- [ui] src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs stdout ----

8    |
8    |
9 LL - impl LocalTraitTwo for LocalTraitOne {}
10 LL + impl LocalTraitTwo for dyn LocalTraitOne {}
+    |
+    |
12 help: use a blanket implementation to implement LocalTraitTwo for all types that also implement LocalTraitOne
13    |
14 LL | impl <T: LocalTraitOne> for T {}
24    |
24    |
25 LL - impl fmt::Display for LocalTraitOne {
26 LL + impl fmt::Display for dyn LocalTraitOne {
+    |
28 
28 
29 error[E0782]: trait objects must include the `dyn` keyword
30   --> $DIR/suggest-blanket-impl-local-trait.rs:20:23
36    |
36    |
37 LL - impl fmt::Display for LocalTraitTwo + Send {
38 LL + impl fmt::Display for dyn LocalTraitTwo + Send {
+    |
40 
41 error: aborting due to 3 previous errors
42 
42 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait/suggest-blanket-impl-local-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-blanket-impl-local-trait.rs`
error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:11:24
   |
LL | impl LocalTraitTwo for LocalTraitOne {}
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl LocalTraitTwo for LocalTraitOne {}
LL + impl LocalTraitTwo for dyn LocalTraitOne {}
   |
help: use a blanket implementation to implement LocalTraitTwo for all types that also implement LocalTraitOne
   |
LL | impl <T: LocalTraitOne> for T {}
   |      ~~~          ++++++++++~+++

error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:14:23
   |
LL | impl fmt::Display for LocalTraitOne { //~ ERROR trait objects must include the `dyn` keyword
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl fmt::Display for LocalTraitOne { //~ ERROR trait objects must include the `dyn` keyword
LL + impl fmt::Display for dyn LocalTraitOne { //~ ERROR trait objects must include the `dyn` keyword


error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:20:23
   |
LL | impl fmt::Display for LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl fmt::Display for LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword
LL + impl fmt::Display for dyn LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0782`.
