plain
.................................................................................................... 2500/12091
.................................................................................................... 2600/12091
.................................................................................................... 2700/12091
.............................................................................i..i................... 2800/12091
.............................................................................F......FF.F............ 2900/12091
....F.....................................................................................iiiii..... 3000/12091
.................................................................................................... 3200/12091
.................................................................................................... 3300/12091
.................................................................................................... 3400/12091
.................................................................................................... 3500/12091
---
diff of stderr:

97    |     A
98    |
99    = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
101 error[E0369]: binary operation `!=` cannot be applied to type `A`

107    |     A
108    |
108    |
109    = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`
110 
111 error[E0369]: binary operation `<` cannot be applied to type `A`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/issue-28837.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/issue-28837.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-28837.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-28837.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-28837/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: cannot add `A` to `A`
   |
   |
LL |     a + a; //~ ERROR cannot add `A` to `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Add` might be missing for `A`

error[E0369]: cannot subtract `A` from `A`
   |
   |
LL |     a - a; //~ ERROR cannot subtract `A` from `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Sub` might be missing for `A`

error[E0369]: cannot multiply `A` by `A`
   |
   |
LL |     a * a; //~ ERROR cannot multiply `A` by `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Mul` might be missing for `A`

error[E0369]: cannot divide `A` by `A`
   |
   |
LL |     a / a; //~ ERROR cannot divide `A` by `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Div` might be missing for `A`

error[E0369]: cannot mod `A` by `A`
   |
   |
LL |     a % a; //~ ERROR cannot mod `A` by `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Rem` might be missing for `A`

error[E0369]: no implementation for `A & A`
   |
   |
LL |     a & a; //~ ERROR no implementation for `A & A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::BitAnd` might be missing for `A`

error[E0369]: no implementation for `A | A`
   |
   |
LL |     a | a; //~ ERROR no implementation for `A | A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::BitOr` might be missing for `A`

error[E0369]: no implementation for `A << A`
   |
   |
LL |     a << a; //~ ERROR no implementation for `A << A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Shl` might be missing for `A`

error[E0369]: no implementation for `A >> A`
   |
   |
LL |     a >> a; //~ ERROR no implementation for `A >> A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::ops::Shr` might be missing for `A`

error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == a; //~ ERROR binary operation `==` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`

error[E0369]: binary operation `!=` cannot be applied to type `A`
   |
   |
LL |     a != a; //~ ERROR binary operation `!=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`

error[E0369]: binary operation `<` cannot be applied to type `A`
   |
   |
LL |     a < a; //~ ERROR binary operation `<` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`

error[E0369]: binary operation `<=` cannot be applied to type `A`
   |
   |
LL |     a <= a; //~ ERROR binary operation `<=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`

error[E0369]: binary operation `>` cannot be applied to type `A`
   |
   |
LL |     a > a; //~ ERROR binary operation `>` cannot be applied to type `A`
   |     - ^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`

error[E0369]: binary operation `>=` cannot be applied to type `A`
   |
   |
LL |     a >= a; //~ ERROR binary operation `>=` cannot be applied to type `A`
   |     - ^^ - A
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialOrd` might be missing for `A`
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0369`.

---
diff of stderr:

8    |      ^^^^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `Error`
20    |      ^^^^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/derives-span-PartialEq-enum-struct-variant.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum-struct-variant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum-struct-variant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      x: Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum-struct-variant.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      x: Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.

---
diff of stderr:

8    |     ^^^^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `Error`
20    |     ^^^^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/derives-span-PartialEq-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs:8:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct Struct {
LL |     x: Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-struct.rs:8:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct Struct {
LL |     x: Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.

---
diff of stderr:

8    |      ^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `Error`
20    |      ^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/derives-span-PartialEq-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-enum.rs:9:6
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
...
LL |      Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.

---
diff of stderr:

8    |     ^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `Error`
20    |     ^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/derives-span-PartialEq-tuple-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derives-span-PartialEq-tuple-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-PartialEq-tuple-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-PartialEq-tuple-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-tuple-struct.rs:8:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct Struct(
LL |     Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `Error`
  --> /checkout/src/test/ui/derives/derives-span-PartialEq-tuple-struct.rs:8:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct Struct(
LL |     Error //~ ERROR
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `Error`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `Error`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0369`.

---
diff of stderr:

8    |     ^^^^^^^^^^^^^^
9    |
10    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `NoCloneOrEq`
11    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
12 
13 error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
20    |     ^^^^^^^^^^^^^^
21    |
21    |
22    = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `NoCloneOrEq`
23    = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error[E0277]: the trait bound `NoCloneOrEq: Clone` is not satisfied

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/deriving-no-inner-impl-error-message.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/deriving-no-inner-impl-error-message.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-no-inner-impl-error-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `NoCloneOrEq`
  --> /checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs:5:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct E {
LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `NoCloneOrEq`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0369]: binary operation `!=` cannot be applied to type `NoCloneOrEq`
  --> /checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs:5:5
   |
LL | #[derive(PartialEq)]
   |          --------- in this derive macro expansion
LL | struct E {
LL |     x: NoCloneOrEq //~ ERROR binary operation `==` cannot be applied to type `NoCloneOrEq`
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `NoCloneOrEq`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `NoCloneOrEq`
   = note: this error originates in the derive macro `PartialEq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the trait bound `NoCloneOrEq: Clone` is not satisfied
  --> /checkout/src/test/ui/derives/deriving-no-inner-impl-error-message.rs:10:5
LL | #[derive(Clone)]
   |          ----- in this derive macro expansion
LL | struct C {
LL | struct C {
LL |     x: NoCloneOrEq
   |     ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `NoCloneOrEq`
note: required by `clone`
  --> /checkout/library/core/src/clone.rs:121:5
   |
LL |     fn clone(&self) -> Self;
LL |     fn clone(&self) -> Self;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
---
diff of stderr:

7    |     A
8    |
9    = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
+    = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/issue-62375.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-62375.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-62375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-62375/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: binary operation `==` cannot be applied to type `A`
   |
   |
LL |     a == A::Value;
   |     - ^^ -------- fn(()) -> A {A::Value}
   |     A
   |
   |
   = note: an implementation of `std::cmp::PartialEq` might be missing for `A`
   = note: add `#[derive(PartialEq)]` or manually implement `PartialEq` for `A`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.

---
test result: FAILED. 11981 passed; 7 failed; 103 ignored; 0 measured; 0 filtered out; finished in 122.37s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:13
