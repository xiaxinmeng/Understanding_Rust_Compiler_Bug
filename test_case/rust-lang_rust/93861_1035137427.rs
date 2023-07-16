plain
3    |
4 LL | fn e() {
5    |        - while parsing this struct

10    = help: or use `(...)` if you meant to specify fn arguments
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
12 error[E0425]: cannot find value `p` in this scope
-   --> $DIR/issue-93835.rs:3:5
+   --> $DIR/issue-93835.rs:2:5
14    |
14    |
15 LL |     p:a<p:p<e=6>>

34    |     ~~~~~
35 
36 error[E0658]: associated const equality is incomplete
36 error[E0658]: associated const equality is incomplete
-   --> $DIR/issue-93835.rs:3:13
+   --> $DIR/issue-93835.rs:2:13
38    |
39 LL |     p:a<p:p<e=6>>


43    = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
45 error[E0658]: associated const equality is incomplete
-   --> $DIR/issue-93835.rs:3:13
+   --> $DIR/issue-93835.rs:2:13
47    |
47    |
48 LL |     p:a<p:p<e=6>>


52    = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
54 error[E0658]: associated type bounds are unstable
-   --> $DIR/issue-93835.rs:3:9
+   --> $DIR/issue-93835.rs:2:9
56    |
56    |
57 LL |     p:a<p:p<e=6>>


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835/issue-93835.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835/issue-93835.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-consts/issue-93835.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/issue-93835.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/issue-93835/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:8
   |
LL | fn e() {
   |        - while parsing this struct
LL |     p:a<p:p<e=6>>
   |        ^        ^
   |
   = help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   = help: or use `(...)` if you meant to specify fn arguments
error[E0425]: cannot find value `p` in this scope
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:5
   |
   |
LL |     p:a<p:p<e=6>>
   |
   |
help: you might have meant to write a `struct` literal
   |
LL ~ fn e() { SomeStruct {
LL |     p:a<p:p<e=6>>
LL |     //~^ ERROR comparison operators
LL |     //~| ERROR cannot find value
LL |     //~| ERROR associated const equality
LL |     //~| ERROR associated const equality
 ...
help: maybe you meant to write a path separator here
   |
LL |     p::a<p:p<e=6>>
help: maybe you meant to write an assignment here
   |
   |
LL |     let p:a<p:p<e=6>>

error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:13
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:13
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/associated-consts/issue-93835.rs:2:9
   |
   |
LL |     p:a<p:p<e=6>>
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0425, E0658.
For more information about an error, try `rustc --explain E0425`.
