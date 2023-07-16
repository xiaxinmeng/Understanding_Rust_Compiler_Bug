plain
warning: `tidy` is not installed; diffs will not be generated

running 509 tests
i.....................................................i............................................. 100/509
.......................................................................F...............FF........... 200/509
.................................................F.................................................. 300/509
.........i.......................................................................................... 500/509
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........
failures:
failures:

---- [rustdoc] rustdoc/intra-doc/extern-lang-item-impl.rs stdout ----

error: auxiliary build of "/checkout/src/test/rustdoc/intra-doc/auxiliary/extern-lang-item-impl-dep.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/rustdoc/intra-doc/auxiliary/extern-lang-item-impl-dep.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/extern-lang-item-impl/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/extern-lang-item-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0522]: definition of an unknown language item: `f64_runtime`
  --> /checkout/src/test/rustdoc/intra-doc/auxiliary/extern-lang-item-impl-dep.rs:18:5
   |
18 |     #[lang = "f64_runtime"]
   |     ^^^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `f64_runtime`
error[E0390]: cannot define inherent `impl` for primitive types
  --> /checkout/src/test/rustdoc/intra-doc/auxiliary/extern-lang-item-impl-dep.rs:19:10
   |
19 |     impl f64 {
---
---- [rustdoc] rustdoc/intra-doc/prim-methods-external-core.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-external-core/auxiliary/my-core/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-external-core" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/auxiliary/my-core.rs"
stdout: none
--- stderr -------------------------------
error[E0522]: definition of an unknown language item: `char`
 --> /checkout/src/test/rustdoc/intra-doc/auxiliary/my-core.rs:9:1
  |
9 | #[lang = "char"]
  | ^^^^^^^^^^^^^^^^ definition of unknown language item `char`
error[E0390]: cannot define inherent `impl` for primitive types
error[E0390]: cannot define inherent `impl` for primitive types
  --> /checkout/src/test/rustdoc/intra-doc/auxiliary/my-core.rs:10:6
10 | impl char {
   |      ^^^^
   |
   = help: consider using an extension trait instead
---
---- [rustdoc] rustdoc/intra-doc/prim-methods-local.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-local/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/prim-methods-local" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/prim-methods-local.rs"
stdout: none
--- stderr -------------------------------
error[E0522]: definition of an unknown language item: `char`
   |
   |
15 | #[lang = "char"]
   | ^^^^^^^^^^^^^^^^ definition of unknown language item `char`
error[E0390]: cannot define inherent `impl` for primitive types
  --> /checkout/src/test/rustdoc/intra-doc/prim-methods-local.rs:16:6
   |
16 | impl char {
---
---- [rustdoc] rustdoc/issue-23511.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23511" "--deny" "warnings" "/checkout/src/test/rustdoc/issue-23511.rs"
stdout: none
--- stderr -------------------------------
error[E0522]: definition of an unknown language item: `str_alloc`
  |
8 |     #[lang = "str_alloc"]
  |     ^^^^^^^^^^^^^^^^^^^^^ definition of unknown language item `str_alloc`

