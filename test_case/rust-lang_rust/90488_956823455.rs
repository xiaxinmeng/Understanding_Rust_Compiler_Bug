plain
.................................................................................................... 2200/12373
.................................................................................................... 2300/12373
.................................................................................................... 2400/12373
.................................................................................................... 2500/12373
................................................................................F.F................. 2600/12373
.......................................................i..i......................................... 2800/12373
.................................................................................................... 2900/12373
........................................................................iiiii....................... 3000/12373
.................................................................................................... 3100/12373
---
.................................................................................................... 6400/12373
............................................ii.ii........i...i...................................... 6500/12373
.................................................................................................... 6600/12373
.i....i.........................................i................i.............i.................... 6700/12373
............................i..F.................................................................... 6800/12373
..............................................................i..................................... 7000/12373
..................................................................................ii................ 7100/12373
.......................................i............................................................ 7200/12373
.................................................................................................... 7300/12373
---

---- [ui] ui/attributes/key-value-expansion.rs stdout ----
diff of stderr:

20         ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
21                                                             &match (&"u8",) {
22                                                                  (arg0,) =>
-                                                                  [::core::fmt::ArgumentV1::new(arg0,
-                                                                                                ::core::fmt::Display::fmt)],
+                                                                  [::core::fmt::ArgumentV1::new_display(arg0)],
26     res
27 }.as_str()`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/key-value-expansion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args attributes/key-value-expansion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/key-value-expansion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/key-value-expansion/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unexpected token: `(7u32)`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:21:6
   |
LL | bug!((column!())); //~ ERROR unexpected token: `(7u32)`


error: unexpected token: `"bug" + "found"`
   |
   |
LL |         bug!("bug" + stringify!(found)); //~ ERROR unexpected token: `"bug" + "found"`
...
...
LL | bug!();
   |
   |
   = note: this error originates in the macro `bug` (in Nightly builds, run with -Z macro-backtrace for more info)
error: unexpected token: `{
    let res =
    let res =
        ::alloc::fmt::format(::core::fmt::Arguments::new_v1(&[""],
                                                            &match (&"u8",) {
                                                                 (arg0,) =>
                                                                 [::core::fmt::ArgumentV1::new_display(arg0)],
    res
}.as_str()`
  --> /checkout/src/test/ui/attributes/key-value-expansion.rs:48:23
   |
   |
LL |         doc_comment! {format!("{coor}", coor = stringify!($t1)).as_str()}
...
...
LL | some_macro!(u8);
   |
   = note: this error originates in the macro `some_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
---
1 error[E0277]: `impl Sized` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:9:22
+   --> $DIR/bound-suggestions.rs:9:5
3    |
4 LL |     println!("{:?}", t);
-    |                      ^ `impl Sized` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^ `impl Sized` cannot be formatted using `{:?}` because it doesn't implement `Debug`
7    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
8 help: consider further restricting this bound

11    |                            +++++++++++++++++
11    |                            +++++++++++++++++
12 
13 error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:15:22
15    |
16 LL |     println!("{:?}", t);
16 LL |     println!("{:?}", t);
-    |                      ^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
19    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
20 help: consider restricting type parameter `T`

23    |                    +++++++++++++++++
23    |                    +++++++++++++++++
24 
25 error[E0277]: `T` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:21:22
27    |
28 LL |     println!("{:?}", t);
28 LL |     println!("{:?}", t);
-    |                      ^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
31    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
32 help: consider further restricting this bound

35    |                            +++++++++++++++++
35    |                            +++++++++++++++++
36 
37 error[E0277]: `Y` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:27:30
39    |
40 LL |     println!("{:?} {:?}", x, y);
40 LL |     println!("{:?} {:?}", x, y);
-    |                              ^ `Y` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Y` cannot be formatted using `{:?}` because it doesn't implement `Debug`
43    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
44 help: consider further restricting type parameter `Y`

47    |                                                                   ~~~~~~~~~~~~~~~~~~~~
47    |                                                                   ~~~~~~~~~~~~~~~~~~~~
48 
49 error[E0277]: `X` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:33:22
51    |
52 LL |     println!("{:?}", x);
52 LL |     println!("{:?}", x);
-    |                      ^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
55    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
56 help: consider further restricting this bound

59    |                                                 +++++++++++++++++
59    |                                                 +++++++++++++++++
60 
61 error[E0277]: `X` doesn't implement `Debug`
-   --> $DIR/bound-suggestions.rs:39:22
63    |
64 LL |     println!("{:?}", x);
64 LL |     println!("{:?}", x);
-    |                      ^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
67    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
68 help: consider further restricting type parameter `X`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bound-suggestions/bound-suggestions.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args bound-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bound-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bound-suggestions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bound-suggestions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `impl Sized` doesn't implement `Debug`
  --> /checkout/src/test/ui/bound-suggestions.rs:9:5
   |
LL |     println!("{:?}", t);
   |     ^^^^^^^^^^^^^^^^^^^ `impl Sized` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
   |
   |
LL | fn test_impl(t: impl Sized + std::fmt::Debug) {


error[E0277]: `T` doesn't implement `Debug`
   |
LL |     println!("{:?}", t);
LL |     println!("{:?}", t);
   |     ^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider restricting type parameter `T`
   |
   |
LL | fn test_no_bounds<T: std::fmt::Debug>(t: T) {


error[E0277]: `T` doesn't implement `Debug`
   |
LL |     println!("{:?}", t);
LL |     println!("{:?}", t);
   |     ^^^^^^^^^^^^^^^^^^^ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
   |
   |
LL | fn test_one_bound<T: Sized + std::fmt::Debug>(t: T) {


error[E0277]: `Y` doesn't implement `Debug`
   |
LL |     println!("{:?} {:?}", x, y);
LL |     println!("{:?} {:?}", x, y);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Y` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `Y`
   |
   |
LL | fn test_no_bounds_where<X, Y>(x: X, y: Y) where X: std::fmt::Debug, Y: std::fmt::Debug {


error[E0277]: `X` doesn't implement `Debug`
   |
LL |     println!("{:?}", x);
LL |     println!("{:?}", x);
   |     ^^^^^^^^^^^^^^^^^^^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting this bound
   |
   |
LL | fn test_one_bound_where<X>(x: X) where X: Sized + std::fmt::Debug {


error[E0277]: `X` doesn't implement `Debug`
   |
LL |     println!("{:?}", x);
LL |     println!("{:?}", x);
   |     ^^^^^^^^^^^^^^^^^^^ `X` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider further restricting type parameter `X`
   |
   |
LL | fn test_many_bounds_where<X>(x: X) where X: Sized, X: Sized, X: std::fmt::Debug {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/bound-suggestions.rs:44:46
   |
   |
LL |     const SIZE: usize = core::mem::size_of::<Self>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:304:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider further restricting `Self`
   |
LL | trait Foo<T>: Sized {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/bound-suggestions.rs:49:46
   |
   |
LL |     const SIZE: usize = core::mem::size_of::<Self>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:304:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider further restricting `Self`
   |
LL | trait Bar: std::fmt::Display + Sized {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/bound-suggestions.rs:54:46
   |
   |
LL |     const SIZE: usize = core::mem::size_of::<Self>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:304:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider further restricting `Self`
   |
LL | trait Baz: Sized where Self: std::fmt::Display {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/bound-suggestions.rs:59:46
   |
   |
LL |     const SIZE: usize = core::mem::size_of::<Self>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:304:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider further restricting `Self`
   |
LL | trait Qux<T>: Sized where Self: std::fmt::Display {

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/bound-suggestions.rs:64:46
   |
   |
LL |     const SIZE: usize = core::mem::size_of::<Self>();
   |
note: required by a bound in `std::mem::size_of`
  --> /checkout/library/core/src/mem/mod.rs:304:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
help: consider further restricting `Self`
   |
LL | trait Bat<T>: std::fmt::Display + Sized {

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/consts/issue-66693-panic-in-array-len.rs stdout ----
diff of stderr:

- error: argument to `panic!()` in a const context must have type `&str`
+ error: `begin_panic` is not yet stable as a const fn
3    |
3    |
4 LL |     let _ = [0i32; panic!(2f32)];
5    |                    ^^^^^^^^^^^^
6    |
+    = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
7    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args consts/issue-66693-panic-in-array-len.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-66693-panic-in-array-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66693-panic-in-array-len" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66693-panic-in-array-len/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `begin_panic` is not yet stable as a const fn
   |
   |
LL |     let _ = [0i32; panic!(2f32)];
   |
   = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/issue-66693-panic-in-array-len.rs:10:21
   |
LL |     let _ = [false; panic!()];
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---

---- [ui] ui/consts/issue-66693.rs stdout ----
diff of stderr:

- error: argument to `panic!()` in a const context must have type `&str`
+ error: `begin_panic` is not yet stable as a const fn
3    |
3    |
4 LL | const _: () = panic!(1);
5    |               ^^^^^^^^^
6    |
+    = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
7    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
7    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
- error: argument to `panic!()` in a const context must have type `&str`
+ error: `begin_panic` is not yet stable as a const fn
11    |
11    |
12 LL | static _FOO: () = panic!(true);
13    |                   ^^^^^^^^^^^^
14    |
+    = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
15    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
15    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
16 
- error: argument to `panic!()` in a const context must have type `&str`
+ error: `begin_panic` is not yet stable as a const fn
19    |
20 LL |     panic!(&1);

21    |     ^^^^^^^^^^
---
To only update this specific test, also pass `--test-args consts/issue-66693.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-66693.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66693" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-66693/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `begin_panic` is not yet stable as a const fn
   |
   |
LL | const _: () = panic!(1);
   |
   = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)


error: `begin_panic` is not yet stable as a const fn
   |
   |
LL | static _FOO: () = panic!(true);
   |
   = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)


error: `begin_panic` is not yet stable as a const fn
   |
   |
LL |     panic!(&1); //~ ERROR: argument to `panic!()` in a const context must have type `&str`
   |
   = help: add `#![feature(const_panic_extra)]` to the crate attributes to enable
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

---

---- [ui] ui/fmt/ifmt-unimpl.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `str: UpperHex` is not satisfied
-   --> $DIR/ifmt-unimpl.rs:2:21
+   --> $DIR/ifmt-unimpl.rs:2:5
3    |
4 LL |     format!("{:X}", "3");
-    |                     ^^^ the trait `UpperHex` is not implemented for `str`
+    |     ^^^^^^^^^^^^^^^^^^^^ the trait `UpperHex` is not implemented for `str`
6    |
7    = note: required because of the requirements on the impl of `UpperHex` for `&str`
- note: required by `std::fmt::UpperHex::fmt`
+ note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
9   --> $SRC_DIR/core/src/fmt/mod.rs:LL:COL
10    |
- LL |     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     arg_new!(new_upper_hex, UpperHex);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
13    = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
15 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/ifmt-unimpl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/ifmt-unimpl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-unimpl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `str: UpperHex` is not satisfied
   |
   |
LL |     format!("{:X}", "3");
   |     ^^^^^^^^^^^^^^^^^^^^ the trait `UpperHex` is not implemented for `str`
   |
   = note: required because of the requirements on the impl of `UpperHex` for `&str`
note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
   |
   |
LL |     arg_new!(new_upper_hex, UpperHex);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
   = note: this error originates in the macro `$crate::__export::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
diff of stderr:

11    |         ^^^^^^^^^^^^^^^^^^^^^^^^
12 
13 warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:81:22
-    |
- LL |     println!("{:p}", &foo);
-    |                      ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:83:20
-    |
- LL |     print!("{:p}", &foo);
-    |                    ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:85:21
-    |
- LL |     format!("{:p}", &foo);
-    |                     ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:88:22
-    |
- LL |     println!("{:p}", &foo as *const _);
-    |                      ^^^^^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:90:22
-    |
- LL |     println!("{:p}", zst_ref);
-    |                      ^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:92:22
-    |
- LL |     println!("{:p}", cast_zst_ptr);
-    |                      ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:94:22
-    |
- LL |     println!("{:p}", coerced_zst_ptr);
-    |                      ^^^^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:97:22
-    |
- LL |     println!("{:p}", &fn_item);
-    |                      ^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:99:22
-    |
- LL |     println!("{:p}", indirect_ref);
-    |                      ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:102:22
-    |
- LL |     println!("{:p}", &nop);
-    |                      ^^^^ help: cast `nop` to obtain a function pointer: `nop as fn()`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:104:22
-    |
- LL |     println!("{:p}", &bar);
-    |                      ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:106:22
-    |
- LL |     println!("{:p}", &baz);
-    |                      ^^^^ help: cast `baz` to obtain a function pointer: `baz as fn(_, _) -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:108:22
-    |
- LL |     println!("{:p}", &unsafe_fn);
-    |                      ^^^^^^^^^^ help: cast `unsafe_fn` to obtain a function pointer: `unsafe_fn as unsafe fn()`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:110:22
-    |
- LL |     println!("{:p}", &c_fn);
-    |                      ^^^^^ help: cast `c_fn` to obtain a function pointer: `c_fn as extern "C" fn()`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:112:22
-    |
- LL |     println!("{:p}", &unsafe_c_fn);
-    |                      ^^^^^^^^^^^^ help: cast `unsafe_c_fn` to obtain a function pointer: `unsafe_c_fn as unsafe extern "C" fn()`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:114:22
-    |
- LL |     println!("{:p}", &variadic);
-    |                      ^^^^^^^^^ help: cast `variadic` to obtain a function pointer: `variadic as unsafe extern "C" fn(_, ...)`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:116:22
-    |
- LL |     println!("{:p}", &take_generic_ref::<u32>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_ref` to obtain a function pointer: `take_generic_ref::<u32> as fn(_)`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:118:22
-    |
- LL |     println!("{:p}", &take_generic_array::<u32, 4>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_array` to obtain a function pointer: `take_generic_array::<u32, 4_usize> as fn(_)`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:120:22
-    |
- LL |     println!("{:p}", &multiple_generic::<u32, f32>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic` to obtain a function pointer: `multiple_generic::<u32, f32> as fn(_, _)`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:122:22
-    |
- LL |     println!("{:p}", &multiple_generic_arrays::<u32, f32, 4, 8>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `multiple_generic_arrays` to obtain a function pointer: `multiple_generic_arrays::<u32, f32, 4_usize, 8_usize> as fn(_, _)`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:124:22
-    |
- LL |     println!("{:p}", &std::env::var::<String>);
-    |                      ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `var` to obtain a function pointer: `var::<String> as fn(_) -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:127:32
-    |
- LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
-    |                                ^^^^ help: cast `nop` to obtain a function pointer: `nop as fn()`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:127:38
-    |
- LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
-    |                                      ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
-   --> $DIR/function-item-references.rs:127:44
-    |
- LL |     println!("{:p} {:p} {:p}", &nop, &foo, &bar);
-    |                                            ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`
- 
- warning: taking a reference to a function item does not give a function pointer
159    |
159    |
160 LL |         std::mem::transmute::<_, usize>(&foo);

202 LL |     bound_by_ptr_trait_tuple((&foo, &bar));
203    |                              ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
- warning: 33 warnings emitted
+ warning: 9 warnings emitted
206 
207 
---
To only update this specific test, also pass `--test-args lint/function-item-references.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/function-item-references.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/function-item-references" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/function-item-references/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     Pointer::fmt(&zst_ref, f)
   |                  ^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/function-item-references.rs:3:9
   |
   |
LL | #![warn(function_item_references)]


warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |         std::mem::transmute::<_, usize>(&foo);
   |                                         ^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |         std::mem::transmute::<_, (usize, usize)>((&foo, &bar));
   |                                                  ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |         std::mem::transmute::<_, (usize, usize)>((&foo, &bar));
   |                                                  ^^^^^^^^^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |         std::mem::transmute::<_, usize>(&take_generic_ref::<u32>);
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^ help: cast `take_generic_ref` to obtain a function pointer: `take_generic_ref::<u32> as fn(_)`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     print_ptr(&bar);
   |               ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     bound_by_ptr_trait(&bar);
   |                        ^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     bound_by_ptr_trait_tuple((&foo, &bar));
   |                              ^^^^^^^^^^^^ help: cast `bar` to obtain a function pointer: `bar as fn(_) -> _`

warning: taking a reference to a function item does not give a function pointer
   |
   |
LL |     bound_by_ptr_trait_tuple((&foo, &bar));
   |                              ^^^^^^^^^^^^ help: cast `foo` to obtain a function pointer: `foo as fn() -> _`
warning: 9 warnings emitted


------------------------------------------
------------------------------------------


---- [ui] ui/on-unimplemented/no-debug.rs stdout ----
diff of stderr:

1 error[E0277]: `Foo` doesn't implement `Debug`
-   --> $DIR/no-debug.rs:10:27
3    |
3    |
4 LL |     println!("{:?} {:?}", Foo, Bar);
-    |                           ^^^ `Foo` cannot be formatted using `{:?}`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Foo` cannot be formatted using `{:?}`
7    = help: the trait `Debug` is not implemented for `Foo`
7    = help: the trait `Debug` is not implemented for `Foo`
8    = note: add `#[derive(Debug)]` to `Foo` or manually `impl Debug for Foo`
9    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
10 
11 error[E0277]: `Bar` doesn't implement `Debug`
-   --> $DIR/no-debug.rs:10:32
13    |
13    |
14 LL |     println!("{:?} {:?}", Foo, Bar);
-    |                                ^^^ `Bar` cannot be formatted using `{:?}` because it doesn't implement `Debug`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` cannot be formatted using `{:?}` because it doesn't implement `Debug`
17    = help: the trait `Debug` is not implemented for `Bar`
18    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

19 
19 
20 error[E0277]: `Foo` doesn't implement `std::fmt::Display`
-   --> $DIR/no-debug.rs:11:23
+   --> $DIR/no-debug.rs:11:5
22    |
23 LL |     println!("{} {}", Foo, Bar);
-    |                       ^^^ `Foo` cannot be formatted with the default formatter
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Foo` cannot be formatted with the default formatter
26    = help: the trait `std::fmt::Display` is not implemented for `Foo`
26    = help: the trait `std::fmt::Display` is not implemented for `Foo`
27    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
28    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
29 
29 
30 error[E0277]: `Bar` doesn't implement `std::fmt::Display`
-   --> $DIR/no-debug.rs:11:28
32    |
32    |
33 LL |     println!("{} {}", Foo, Bar);
-    |                            ^^^ `Bar` cannot be formatted with the default formatter
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` cannot be formatted with the default formatter
36    = help: the trait `std::fmt::Display` is not implemented for `Bar`
36    = help: the trait `std::fmt::Display` is not implemented for `Bar`
37    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug/no-debug.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args on-unimplemented/no-debug.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/on-unimplemented/no-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/on-unimplemented/no-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `Foo` doesn't implement `Debug`
  --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:10:5
   |
LL |     println!("{:?} {:?}", Foo, Bar);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Foo` cannot be formatted using `{:?}`
   = help: the trait `Debug` is not implemented for `Foo`
   = help: the trait `Debug` is not implemented for `Foo`
   = note: add `#[derive(Debug)]` to `Foo` or manually `impl Debug for Foo`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `Bar` doesn't implement `Debug`
  --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:10:5
   |
LL |     println!("{:?} {:?}", Foo, Bar);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   = help: the trait `Debug` is not implemented for `Bar`
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `Foo` doesn't implement `std::fmt::Display`
error[E0277]: `Foo` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:11:5
   |
LL |     println!("{} {}", Foo, Bar);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Foo` cannot be formatted with the default formatter
   = help: the trait `std::fmt::Display` is not implemented for `Foo`
   = help: the trait `std::fmt::Display` is not implemented for `Foo`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `Bar` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/on-unimplemented/no-debug.rs:11:5
   |
LL |     println!("{} {}", Foo, Bar);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Bar` cannot be formatted with the default formatter
   = help: the trait `std::fmt::Display` is not implemented for `Bar`
   = help: the trait `std::fmt::Display` is not implemented for `Bar`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.

---
1 error[E0277]: `Path` doesn't implement `std::fmt::Display`
-   --> $DIR/path-display.rs:5:20
+   --> $DIR/path-display.rs:5:5
3    |
4 LL |     println!("{}", path);
-    |                    ^^^^ `Path` cannot be formatted with the default formatter; call `.display()` on it
+    |     ^^^^^^^^^^^^^^^^^^^^ `Path` cannot be formatted with the default formatter; call `.display()` on it
7    = help: the trait `std::fmt::Display` is not implemented for `Path`
7    = help: the trait `std::fmt::Display` is not implemented for `Path`
8    = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display/path-display.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/path-display.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/path-display.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/path-display/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `Path` doesn't implement `std::fmt::Display`
  --> /checkout/src/test/ui/suggestions/path-display.rs:5:5
   |
LL |     println!("{}", path);
   |     ^^^^^^^^^^^^^^^^^^^^ `Path` cannot be formatted with the default formatter; call `.display()` on it
   = help: the trait `std::fmt::Display` is not implemented for `Path`
   = help: the trait `std::fmt::Display` is not implemented for `Path`
   = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12255 passed; 8 failed; 110 ignored; 0 measured; 0 filtered out; finished in 131.72s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:41
