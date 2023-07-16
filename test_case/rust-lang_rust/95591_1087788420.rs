plain
.............ii................i....i..ii........................................................... 7900/12890
.................................................................................................... 8000/12890
.................................................................................................... 8100/12890
.................................................................................................... 8200/12890
..............................F...F...............................................i..ii............. 8300/12890
....................................iiii............................................................ 8500/12890
..........................................................i.......................................i. 8600/12890
..................................................................i................................. 8700/12890
.................................................................................................... 8800/12890
---
failures:

---- [ui] ui/nll/user-annotations/constant-in-expr-normalize.rs#base stdout ----
normalized stderr:
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     <() as Foo<'a>>::C
   |
   |
   = note: ...the reference is valid for the static lifetime...
note: ...but the borrowed content is only valid for the lifetime `'a` as defined here
   |
   |
LL | fn foo<'a>(_: &'a u32) -> &'static u32 {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0312`.
For more information about this error, try `rustc --explain E0312`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.base/constant-in-expr-normalize.base.stderr
To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-normalize.rs`


error in revision `base`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-normalize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "base" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.base" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.base/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
   |
   |
LL |     <() as Foo<'a>>::C //~ ERROR
   |
   |
   = note: ...the reference is valid for the static lifetime...
note: ...but the borrowed content is only valid for the lifetime `'a` as defined here
   |
   |
LL | fn foo<'a>(_: &'a u32) -> &'static u32 {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0312`.
---
normalized stderr:
error: lifetime may not live long enough
  --> $DIR/constant-in-expr-normalize.rs:22:5
   |
LL | fn foo<'a>(_: &'a u32) -> &'static u32 {
   |        -- lifetime `'a` defined here
LL |     <() as Foo<'a>>::C
   |     ^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: aborting due to previous error





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.nll/constant-in-expr-normalize.nll.stderr
To only update this specific test, also pass `--test-args nll/user-annotations/constant-in-expr-normalize.rs`


error in revision `nll`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/constant-in-expr-normalize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.nll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "borrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/constant-in-expr-normalize.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a>(_: &'a u32) -> &'static u32 {
   |        -- lifetime `'a` defined here
LL |     <() as Foo<'a>>::C //~ ERROR
   |     ^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: aborting due to previous error
------------------------------------------


