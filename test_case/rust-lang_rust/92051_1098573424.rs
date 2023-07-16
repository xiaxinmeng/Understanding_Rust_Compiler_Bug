plain
failures:

---- [ui] src/test/ui/destructuring-assignment/slice_destructure.rs stdout ----
normalized stderr:
warning: variable `a` is assigned to, but never used
  --> $DIR/slice_destructure.rs:4:12
   |
LL |   let (mut a, mut b);
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: `#[warn(unused_variables)]` on by default
   = note: consider using `_a` instead

warning: variable `b` is assigned to, but never used
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> $DIR/slice_destructure.rs:4:19
   |
LL |   let (mut a, mut b);
   |
   |
   = note: consider using `_b` instead
warning: value assigned to `a` is never read
  --> $DIR/slice_destructure.rs:5:4
   |
   |
LL |   [a, b] = [0, 1];
   |
   = note: `#[warn(unused_assignments)]` on by default
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

warning: value assigned to `b` is never read
  --> $DIR/slice_destructure.rs:5:7
   |
LL |   [a, b] = [0, 1];
   |
   |
   = help: maybe it is overwritten before being read?
warning: variable does not need to be mutable
  --> $DIR/slice_destructure.rs:4:8
   |
   |
LL |   let (mut a, mut b);
   |        |
   |        help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> $DIR/slice_destructure.rs:4:15
   |
LL |   let (mut a, mut b);
   |               |
   |               help: remove this `mut`

warning: variable does not need to be mutable
---
To only update this specific test, also pass `--test-args destructuring-assignment/slice_destructure.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/destructuring-assignment/slice_destructure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/slice_destructure/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/destructuring-assignment/slice_destructure/auxiliary"
stdout: none
--- stderr -------------------------------
warning: variable `a` is assigned to, but never used
   |
   |
LL |   let (mut a, mut b);
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: `#[warn(unused_variables)]` on by default
   = note: consider using `_a` instead

warning: variable `b` is assigned to, but never used
   |
   |
LL |   let (mut a, mut b);
   |
   |
   = note: consider using `_b` instead
warning: value assigned to `a` is never read
  --> /checkout/src/test/ui/destructuring-assignment/slice_destructure.rs:5:4
   |
   |
LL |   [a, b] = [0, 1];
   |
   = note: `#[warn(unused_assignments)]` on by default
   = note: `#[warn(unused_assignments)]` on by default
   = help: maybe it is overwritten before being read?

warning: value assigned to `b` is never read
   |
   |
LL |   [a, b] = [0, 1];
   |
   |
   = help: maybe it is overwritten before being read?
warning: variable does not need to be mutable
  --> /checkout/src/test/ui/destructuring-assignment/slice_destructure.rs:4:8
   |
   |
LL |   let (mut a, mut b);
   |        |
   |        help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> /checkout/src/test/ui/destructuring-assignment/slice_destructure.rs:4:15
   |
LL |   let (mut a, mut b);
   |               |
   |               help: remove this `mut`

warning: variable does not need to be mutable
