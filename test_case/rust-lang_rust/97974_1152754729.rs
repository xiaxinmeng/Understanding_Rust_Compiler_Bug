plain
........................................................................................ 1848/13053
..........................................................................i............. 1936/13053
.............................................F.......................................... 2024/13053
........................................................................................ 2112/13053
............................F....F..............F.....F................................. 2200/13053
...F......F...F............................................F.........F....F............. 2288/13053
........F.......F...............F....................................................... 2376/13053
........................................................................................ 2552/13053
........................................................................................ 2640/13053
........................................................................................ 2728/13053
........................................................................................ 2816/13053
---

---- [ui] src/test/ui/const-generics/const-param-elided-lifetime.rs#min stdout ----
diff of stderr:

28 LL | fn bar<const N: &u8>() {}
29    |                 ^ explicit lifetime name needed here
30 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
33    |
33    |
34 LL | struct A<const N: &u8>;

37    = note: the only supported types are integers, `bool` and `char`
38    = help: more complex types are supported with `#![feature(adt_const_params)]`
39 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
42    |
42    |
43 LL | impl<const N: &u8> A<N> {

46    = note: the only supported types are integers, `bool` and `char`
47    = help: more complex types are supported with `#![feature(adt_const_params)]`
48 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
51    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
52 LL |     fn foo<const M: &u8>(&self) {}

55    = note: the only supported types are integers, `bool` and `char`
56    = help: more complex types are supported with `#![feature(adt_const_params)]`
57 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
60    |
60    |
61 LL | impl<const N: &u8> B for A<N> {}

64    = note: the only supported types are integers, `bool` and `char`
65    = help: more complex types are supported with `#![feature(adt_const_params)]`
66 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
69    |
69    |
70 LL | fn bar<const N: &u8>() {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.min/const-param-elided-lifetime.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-param-elided-lifetime.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-elided-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-elided-lifetime.min/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | struct A<const N: &u8>;
   |                   ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> A<N> {
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL |     fn foo<const M: &u8>(&self) {}
   |                     ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | impl<const N: &u8> B for A<N> {}
   |               ^ explicit lifetime name needed here

error[E0637]: `&` without an explicit lifetime name cannot be used here
   |
   |
LL | fn bar<const N: &u8>() {}
   |                 ^ explicit lifetime name needed here

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL | struct A<const N: &u8>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL | impl<const N: &u8> A<N> {
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL |     fn foo<const M: &u8>(&self) {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL | impl<const N: &u8> B for A<N> {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL | fn bar<const N: &u8>() {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0637`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/intrinsics-type_name-as-const-argument.rs#min stdout ----
diff of stderr:

7    = note: type parameters may not be used in const expressions
8    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
9 
- error: `&'static str` is forbidden as the type of a const generic parameter
+ error: `&str` is forbidden as the type of a const generic parameter
12    |
12    |
13 LL | trait Trait<const S: &'static str> {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/intrinsics-type_name-as-const-argument.min/intrinsics-type_name-as-const-argument.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/intrinsics-type_name-as-const-argument.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/intrinsics-type_name-as-const-argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/intrinsics-type_name-as-const-argument.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/intrinsics-type_name-as-const-argument.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |     T: Trait<{std::intrinsics::type_name::<T>()}>
   |                                            ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: `&str` is forbidden as the type of a const generic parameter
   |
   |
LL | trait Trait<const S: &'static str> {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-56445-1.rs#min stdout ----
diff of stderr:

6    |
7    = note: for more information, see issue #74052 <https://github.com/rust-lang/rust/issues/74052>
8 
- error: `&'static str` is forbidden as the type of a const generic parameter
+ error: `&str` is forbidden as the type of a const generic parameter
11    |
11    |
12 LL | struct Bug<'a, const S: &'a str>(PhantomData<&'a ()>);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-56445-1.min/issue-56445-1.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-56445-1.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-56445-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-56445-1.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-56445-1.min/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0771]: use of non-static lifetime `'a` in const generic
   |
   |
LL | struct Bug<'a, const S: &'a str>(PhantomData<&'a ()>);
   |
   = note: for more information, see issue #74052 <https://github.com/rust-lang/rust/issues/74052>


error: `&str` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Bug<'a, const S: &'a str>(PhantomData<&'a ()>);
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0771`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs#full stdout ----
diff of stderr:

- error[E0741]: `(dyn A + 'static)` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
+ error[E0741]: `dyn A` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
3    |
3    |
4 LL | fn test<const T: &'static dyn A>() {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.full/issue-63322-forbid-dyn.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-63322-forbid-dyn.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0741]: `dyn A` must be annotated with `#[derive(PartialEq, Eq)]` to be used as the type of a const parameter
   |
   |
LL | fn test<const T: &'static dyn A>() {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0741`.
For more information about this error, try `rustc --explain E0741`.
------------------------------------------


---- [ui] src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs#min stdout ----
diff of stderr:

- error: `&'static (dyn A + 'static)` is forbidden as the type of a const generic parameter
+ error: `&dyn A` is forbidden as the type of a const generic parameter
3    |
3    |
4 LL | fn test<const T: &'static dyn A>() {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.min/issue-63322-forbid-dyn.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-63322-forbid-dyn.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-63322-forbid-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-63322-forbid-dyn.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: `&dyn A` is forbidden as the type of a const generic parameter
   |
   |
LL | fn test<const T: &'static dyn A>() {
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-73491.rs#min stdout ----
diff of stderr:

- error: `[u32; _]` is forbidden as the type of a const generic parameter
+ error: `[u32; 1024]` is forbidden as the type of a const generic parameter
3    |
3    |
4 LL | fn hoge<const IN: [u32; LEN]>() {}

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73491.min/issue-73491.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-73491.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-73491.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73491.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-73491.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: `[u32; 1024]` is forbidden as the type of a const generic parameter
   |
   |
LL | fn hoge<const IN: [u32; LEN]>() {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-74101.rs#min stdout ----
diff of stderr:

- error: `[u8; _]` is forbidden as the type of a const generic parameter
+ error: `[u8; 3]` is forbidden as the type of a const generic parameter
3    |
3    |
4 LL | fn test<const N: [u8; 1 + 2]>() {}

7    = note: the only supported types are integers, `bool` and `char`
8    = help: more complex types are supported with `#![feature(adt_const_params)]`
9 
- error: `[u8; _]` is forbidden as the type of a const generic parameter
+ error: `[u8; 3]` is forbidden as the type of a const generic parameter
12    |
12    |
13 LL | struct Foo<const N: [u8; 1 + 2]>;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-74101.min/issue-74101.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-74101.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-74101.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-74101.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-74101.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: `[u8; 3]` is forbidden as the type of a const generic parameter
   |
   |
LL | fn test<const N: [u8; 1 + 2]>() {}
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `[u8; 3]` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Foo<const N: [u8; 1 + 2]>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/issues/issue-75047.rs#min stdout ----
diff of stderr:

- error: `[u8; _]` is forbidden as the type of a const generic parameter
+ error: `[u8; 42]` is forbidden as the type of a const generic parameter
3    |
3    |
4 LL | struct Foo<const N: [u8; Bar::<u32>::value()]>;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-75047.min/issue-75047.min.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-75047.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-75047.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-75047.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-75047.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: `[u8; 42]` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Foo<const N: [u8; Bar::<u32>::value()]>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/const-generics/min_const_generics/complex-types.rs stdout ----
diff of stderr:

25    = note: the only supported types are integers, `bool` and `char`
26    = help: more complex types are supported with `#![feature(adt_const_params)]`
27 
- error: `&'static u8` is forbidden as the type of a const generic parameter
+ error: `&u8` is forbidden as the type of a const generic parameter
30    |
30    |
31 LL | struct Faz<const N: &'static u8>;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-types/complex-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/min_const_generics/complex-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/complex-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-types/auxiliary"
stdout: none
--- stderr -------------------------------
error: `[u8; 0]` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Foo<const N: [u8; 0]>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `()` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Bar<const N: ()>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `No` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Fez<const N: No>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `&u8` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Faz<const N: &'static u8>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `!` is forbidden as the type of a const generic parameter
   |
   |
LL | struct Fiz<const N: !>;
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `()` is forbidden as the type of a const generic parameter
   |
   |
LL | enum Goo<const N: ()> { A, B }
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`

error: `()` is forbidden as the type of a const generic parameter
   |
   |
LL | union Boo<const N: ()> { a: () }
   |
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(adt_const_params)]`
