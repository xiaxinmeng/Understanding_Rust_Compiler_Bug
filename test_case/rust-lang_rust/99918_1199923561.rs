plain
.............ii......................................................................... 8624/13252
...................iiii................................................................. 8712/13252
........................................................i............................... 8800/13252
........i...................................................................i........... 8888/13252
...FFFF.........................................................F....................... 8976/13252
........................................................................................ 9152/13252
.......................................................................i................ 9240/13252
........................................................................................ 9328/13252
........................................................................................ 9416/13252
---

---- [ui] src/test/ui/fn/keyword-order.rs stdout ----
diff of stderr:

- error: `default` is not followed by an item
-   --> $DIR/keyword-order.rs:3:1
-    |
- LL | default pub const async unsafe extern fn err() {}
-    | ^^^^^^^ the `default` qualifier
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
9 error: expected item, found keyword `pub`
10   --> $DIR/keyword-order.rs:3:9
11    |


12 LL | default pub const async unsafe extern fn err() {}
14 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
16 
---
To only update this specific test, also pass `--test-args fn/keyword-order.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/keyword-order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/keyword-order" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/keyword-order/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected item, found keyword `pub`
   |
   |
LL | default pub const async unsafe extern fn err() {} //~ ERROR `default` is not followed by an item

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/default-unmatched.rs stdout ----
diff of stderr:

- error: `default` is not followed by an item
-   --> $DIR/default-unmatched.rs:3:5
- LL |     default do
-    |     ^^^^^^^ the `default` qualifier
-    |
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
9 error: expected item, found reserved keyword `do`
10   --> $DIR/default-unmatched.rs:3:13
11    |

---
To only update this specific test, also pass `--test-args parser/default-unmatched.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/default-unmatched.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected item, found reserved keyword `do`
   |
LL |     default do
   |             ^^ expected item


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/parser/default-unmatched-assoc.rs stdout ----
diff of stderr:

- error: `default` is not followed by an item
-   --> $DIR/default-unmatched-assoc.rs:5:5
- LL |     default do
-    |     ^^^^^^^ the `default` qualifier
-    |
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
9 error: non-item in item list
10   --> $DIR/default-unmatched-assoc.rs:5:13
11    |


18 LL | }
19    | - item list ends here
20 
- error: `default` is not followed by an item
-   --> $DIR/default-unmatched-assoc.rs:13:5
- LL |     default do
-    |     ^^^^^^^ the `default` qualifier
-    |
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
29 error: non-item in item list
30   --> $DIR/default-unmatched-assoc.rs:13:13
31    |

---
To only update this specific test, also pass `--test-args parser/default-unmatched-assoc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/default-unmatched-assoc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched-assoc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched-assoc/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/default-unmatched-assoc.rs:5:13
   |
LL | trait Foo {
   |           - item list starts here
   |           - item list starts here
LL |     default!(); //~ ERROR cannot find macro `default` in this scope
LL |     default do
   |             ^^ non-item starts here
LL | }
LL | }
   | - item list ends here
error: non-item in item list
  --> /checkout/src/test/ui/parser/default-unmatched-assoc.rs:13:13
   |
LL | impl S {
LL | impl S {
   |        - item list starts here
LL |     default!(); //~ ERROR cannot find macro `default` in this scope
LL |     default do
   |             ^^ non-item starts here
LL | }
LL | }
   | - item list ends here
error: cannot find macro `default` in this scope
  --> /checkout/src/test/ui/parser/default-unmatched-assoc.rs:4:5
   |
   |
LL |     default!(); //~ ERROR cannot find macro `default` in this scope

error: cannot find macro `default` in this scope
  --> /checkout/src/test/ui/parser/default-unmatched-assoc.rs:12:5
   |
   |
LL |     default!(); //~ ERROR cannot find macro `default` in this scope

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/default.rs stdout ----
diff of stderr:

- error: `default` is not followed by an item
-   --> $DIR/default.rs:23:5
-    |
- LL |     default pub fn foo<T: Default>() -> T { T::default() }
-    |     ^^^^^^^ the `default` qualifier
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
9 error: non-item in item list
10   --> $DIR/default.rs:23:13
11    |

---
To only update this specific test, also pass `--test-args parser/default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/default.rs:23:13
   |
   |
LL | impl Foo for u32 { //~ ERROR not all trait items implemented, missing: `foo`
   |                  - item list starts here
LL |     default pub fn foo<T: Default>() -> T { T::default() }
   |             ^^^ non-item starts here
LL | }
LL | }
   | - item list ends here
error[E0449]: unnecessary visibility qualifier
  --> /checkout/src/test/ui/parser/default.rs:17:5
   |
   |
LL |     pub default fn foo<T: Default>() -> T { //~ ERROR unnecessary visibility qualifier
   |     ^^^ `pub` not permitted here because it's implied
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
  --> /checkout/src/test/ui/parser/default.rs:3:12
   |
LL | #![feature(specialization)]
---

error[E0046]: not all trait items implemented, missing: `foo`
  --> /checkout/src/test/ui/parser/default.rs:22:1
   |
LL |     fn foo<T: Default>() -> T;
   |     -------------------------- `foo` from trait
...
LL | impl Foo for u32 { //~ ERROR not all trait items implemented, missing: `foo`
   | ^^^^^^^^^^^^^^^^ missing `foo` in implementation
error: aborting due to 3 previous errors; 1 warning emitted

Some errors have detailed explanations: E0046, E0449.
For more information about an error, try `rustc --explain E0046`.
For more information about an error, try `rustc --explain E0046`.
------------------------------------------


---- [ui] src/test/ui/parser/default-unmatched-extern.rs stdout ----
diff of stderr:

- error: `default` is not followed by an item
-   --> $DIR/default-unmatched-extern.rs:5:5
- LL |     default do
-    |     ^^^^^^^ the `default` qualifier
-    |
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
9 error: non-item in item list
10   --> $DIR/default-unmatched-extern.rs:5:13
11    |

---
To only update this specific test, also pass `--test-args parser/default-unmatched-extern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/default-unmatched-extern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched-extern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/default-unmatched-extern/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/default-unmatched-extern.rs:5:13
   |
LL | extern "C" {
   |            - item list starts here
   |            - item list starts here
LL |     default!(); //~ ERROR cannot find macro `default` in this scope
LL |     default do
   |             ^^ non-item starts here
LL | }
LL | }
   | - item list ends here
error: cannot find macro `default` in this scope
  --> /checkout/src/test/ui/parser/default-unmatched-extern.rs:4:5
   |
   |
LL |     default!(); //~ ERROR cannot find macro `default` in this scope

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/parser/impl-parsing.rs stdout ----
diff of stderr:

22 LL | impl ?Sized for .. {}
24 
24 
- error: `default` is not followed by an item
-   --> $DIR/impl-parsing.rs:9:1
-    |
- LL | default unsafe FAIL
-    | ^^^^^^^ the `default` qualifier
-    |
-    = note: only `fn`, `const`, `type`, or `impl` items may be prefixed by `default`
33 error: expected item, found keyword `unsafe`
34   --> $DIR/impl-parsing.rs:9:9
35    |


36 LL | default unsafe FAIL
38 
- error: aborting due to 6 previous errors
+ error: aborting due to 5 previous errors
40 
---
To only update this specific test, also pass `--test-args parser/impl-parsing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/impl-parsing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-parsing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/impl-parsing/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing `for` in a trait impl
   |
   |
LL | impl Trait Type {} //~ ERROR missing `for` in a trait impl
   |           ^ help: add `for` here
error: missing `for` in a trait impl
  --> /checkout/src/test/ui/parser/impl-parsing.rs:5:11
   |
   |
LL | impl Trait .. {} //~ ERROR missing `for` in a trait impl
   |           ^ help: add `for` here
error: expected a trait, found type
  --> /checkout/src/test/ui/parser/impl-parsing.rs:6:6
   |
   |
LL | impl ?Sized for Type {} //~ ERROR expected a trait, found type

error: expected a trait, found type
  --> /checkout/src/test/ui/parser/impl-parsing.rs:7:6
   |
   |
LL | impl ?Sized for .. {} //~ ERROR expected a trait, found type

error: expected item, found keyword `unsafe`
  --> /checkout/src/test/ui/parser/impl-parsing.rs:9:9
   |
   |
LL | default unsafe FAIL //~ ERROR expected item, found keyword `unsafe`

error: aborting due to 5 previous errors
------------------------------------------

