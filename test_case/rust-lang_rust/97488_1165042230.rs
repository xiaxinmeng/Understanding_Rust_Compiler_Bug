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
14 LL | impl <T: LocalTraitOne> LocalTraitTwo for T {}
24    |
24    |
25 LL - impl fmt::Display for LocalTraitOne {
26 LL + impl fmt::Display for dyn LocalTraitOne {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
28 
28 
29 error[E0782]: trait objects must include the `dyn` keyword
30   --> $DIR/suggest-blanket-impl-local-trait.rs:21:23
36    |
36    |
37 LL - impl fmt::Display for LocalTraitTwo + Send {
38 LL + impl fmt::Display for dyn LocalTraitTwo + Send {
+    |
40 
40 
41 error[E0782]: trait objects must include the `dyn` keyword
42   --> $DIR/suggest-blanket-impl-local-trait.rs:27:24
48    |
48    |
49 LL - impl LocalTraitOne for fmt::Display {}
50 LL + impl LocalTraitOne for dyn fmt::Display {}
+    |
52 
52 
53 error[E0782]: trait objects must include the `dyn` keyword
54   --> $DIR/suggest-blanket-impl-local-trait.rs:30:24
60    |
60    |
61 LL - impl LocalTraitOne for fmt::Display + Send {}
62 LL + impl LocalTraitOne for dyn fmt::Display + Send {}
+    |
64 
65 error: aborting due to 5 previous errors
66 
66 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-blanket-impl-local-trait/suggest-blanket-impl-local-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-blanket-impl-local-trait.rs`
error: 1 errors occurred comparing output.
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
LL | impl <T: LocalTraitOne> LocalTraitTwo for T {}
   |      ~~~          ++++++++++++++++++++++++~+++

error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:14:23
   |
LL | impl fmt::Display for LocalTraitOne {
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl fmt::Display for LocalTraitOne {
LL + impl fmt::Display for dyn LocalTraitOne {


error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:21:23
   |
LL | impl fmt::Display for LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl fmt::Display for LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword
LL + impl fmt::Display for dyn LocalTraitTwo + Send { //~ ERROR trait objects must include the `dyn` keyword


error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:27:24
   |
LL | impl LocalTraitOne for fmt::Display {}
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl LocalTraitOne for fmt::Display {}
LL + impl LocalTraitOne for dyn fmt::Display {}


error[E0782]: trait objects must include the `dyn` keyword
  --> /checkout/src/test/ui/suggestions/suggest-blanket-impl-local-trait.rs:30:24
   |
LL | impl LocalTraitOne for fmt::Display + Send {}
   |
   |
help: add `dyn` keyword before this trait
   |
LL - impl LocalTraitOne for fmt::Display + Send {}
LL + impl LocalTraitOne for dyn fmt::Display + Send {}

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0782`.
