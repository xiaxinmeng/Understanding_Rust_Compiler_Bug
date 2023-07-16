plain
........................................................................................ 1848/13229
........................................................................................ 1936/13229
.............i.......................................................................... 2024/13229
........................................................................................ 2112/13229
.....................................F.........FFF.F.F.................................. 2200/13229
........................................................................................ 2376/13229
........................................................................................ 2464/13229
........................................................................................ 2552/13229
........................................................................................ 2640/13229
---

94    |                   ^^^
95 help: provide the argument
96    |
- LL |     closure(/* value */);
-    |     ~~~~~~~~~~~~~~~~~~~~
+ LL |     closure(/* _ */);
99 
100 error: aborting due to 6 previous errors
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
101 
---
To only update this specific test, also pass `--test-args argument-suggestions/basic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/argument-suggestions/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/basic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/basic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:20:13
   |
   |
LL |     invalid(1.0); //~ ERROR mismatched types
   |     ------- ^^^ expected `u32`, found floating-point number
   |     arguments to this function are incorrect
   |
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:13:4
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:13:4
   |
LL | fn invalid(_i: u32) {}

error[E0061]: this function takes 0 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:21:5
   |
   |
LL |     extra(""); //~ ERROR this function takes
   |     ^^^^^ -- argument of type `&'static str` unexpected
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:14:4
   |
LL | fn extra() {}
LL | fn extra() {}
   |    ^^^^^
help: remove the extra argument
   |
LL |     extra(); //~ ERROR this function takes

error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:22:5
   |
   |
LL |     missing(); //~ ERROR this function takes
   |     ^^^^^^^-- an argument of type `u32` is missing
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:15:4
   |
   |
LL | fn missing(_i: u32) {}
help: provide the argument
   |
   |
LL |     missing(/* u32 */); //~ ERROR this function takes

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:23:5
   |
   |
LL |     swapped("", 1); //~ ERROR arguments to this function are incorrect
   |     ^^^^^^^ --  - expected `&str`, found `{integer}`
   |             expected `u32`, found `&'static str`
   |
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:16:4
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:16:4
   |
LL | fn swapped(_i: u32, _s: &str) {}
help: swap these arguments
   |
   |
LL |     swapped(1, ""); //~ ERROR arguments to this function are incorrect

error[E0308]: arguments to this function are incorrect
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:24:5
   |
   |
LL |     permuted(Y {}, Z {}, X {}); //~ ERROR arguments to this function are incorrect
   |     ^^^^^^^^ ----  ----  ---- expected `Z`, found `X`
   |              |     |
   |              |     expected `Y`, found `Z`
   |              expected `X`, found `Y`
note: function defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:17:4
   |
   |
LL | fn permuted(_x: X, _y: Y, _z: Z) {}
help: reorder these arguments
   |
   |
LL |     permuted(X {}, Y {}, Z {}); //~ ERROR arguments to this function are incorrect

error[E0057]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:27:5
   |
   |
LL |     closure(); //~ ERROR this function takes
   |     ^^^^^^^-- an argument is missing
note: closure defined here
  --> /checkout/src/test/ui/argument-suggestions/basic.rs:26:19
   |
   |
LL |     let closure = |x| x;
help: provide the argument
   |
   |
LL |     closure(/* _ */); //~ ERROR this function takes

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0057, E0061, E0308.
Some errors have detailed explanations: E0057, E0061, E0308.
For more information about an error, try `rustc --explain E0057`.
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs stdout ----
diff of stderr:

11    |                                                       ^^^^^ required by this bound in `use_dyn`
12 help: consider specifying the generic argument
13    |
- LL |     use_dyn::<N>(&());
+ LL |     use_dyn::<_>(&());
16 
17 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err/object-safety-ok-infer-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/object-safety-ok-infer-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/object-safety-ok-infer-err.rs:19:5
   |
   |
LL |     use_dyn(&());
   |     ^^^^^^^ cannot infer the value of the const parameter `N` declared on the function `use_dyn`
   |
note: required by a bound in `use_dyn`
   |
   |
LL | fn use_dyn<const N: usize>(v: &dyn Foo<N>) where [u8; N + 1]: Sized {
   |                                                       ^^^^^ required by this bound in `use_dyn`
help: consider specifying the generic argument
   |
LL |     use_dyn::<_>(&());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.
---

6    |
7 help: consider specifying the generic argument
8    |
- LL |     foo::<X>();
+ LL |     foo::<_>();
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/cannot-infer-const-args/cannot-infer-const-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/infer/cannot-infer-const-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/infer/cannot-infer-const-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/cannot-infer-const-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/cannot-infer-const-args/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/infer/cannot-infer-const-args.rs:6:5
   |
   |
LL |     foo(); //~ ERROR type annotations needed
   |     ^^^ cannot infer the value of the const parameter `X` declared on the function `foo`
help: consider specifying the generic argument
   |
   |
LL |     foo::<_>(); //~ ERROR type annotations needed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---

6    |
7 help: consider specifying the generic argument
8    |
- LL |     Foo.bar().bar().bar().bar().baz::<N>();
+ LL |     Foo.bar().bar().bar().bar().baz::<_>();
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/method-chain/method-chain.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/infer/method-chain.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/infer/method-chain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/method-chain" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/method-chain/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/infer/method-chain.rs:15:33
   |
   |
LL |     Foo.bar().bar().bar().bar().baz(); //~ ERROR type annotations needed
   |                                 ^^^ cannot infer the value of the const parameter `N` declared on the associated function `baz`
help: consider specifying the generic argument
   |
   |
LL |     Foo.bar().bar().bar().bar().baz::<_>(); //~ ERROR type annotations needed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---

6    |
7 help: consider specifying the generic arguments
8    |
- LL |     let _: [u8; 17] = foo::<17_usize, M>();
+ LL |     let _: [u8; 17] = foo::<17_usize, _>();
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/one-param-uninferred/one-param-uninferred.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/infer/one-param-uninferred.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/infer/one-param-uninferred.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/one-param-uninferred" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/one-param-uninferred/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/infer/one-param-uninferred.rs:9:23
   |
   |
LL |     let _: [u8; 17] = foo();
   |                       ^^^ cannot infer the value of the const parameter `M` declared on the function `foo`
help: consider specifying the generic arguments
   |
   |
LL |     let _: [u8; 17] = foo::<17_usize, _>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---

6    |
7 help: consider specifying the generic arguments
8    |
- LL |     Foo.foo::<A, B>();
+ LL |     Foo.foo::<_, _>();
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/uninferred-consts/uninferred-consts.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/infer/uninferred-consts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/infer/uninferred-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/uninferred-consts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/uninferred-consts/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/infer/uninferred-consts.rs:9:9
   |
   |
LL |     Foo.foo();
   |         ^^^ cannot infer the value of the const parameter `A` declared on the associated function `foo`
help: consider specifying the generic arguments
   |
   |
LL |     Foo.foo::<_, _>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---

6    |
7 help: consider specifying the generic arguments
8    |
- LL |         println!("{:?}", take_array_from_mut::<i32, N>(&mut arr, i));
+ LL |         println!("{:?}", take_array_from_mut::<i32, _>(&mut arr, i));
11 
12 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/issue-77092/issue-77092.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/infer/issue-77092.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/infer/issue-77092.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/issue-77092" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/infer/issue-77092/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/infer/issue-77092.rs:11:26
   |
   |
LL |         println!("{:?}", take_array_from_mut(&mut arr, i));
   |                          ^^^^^^^^^^^^^^^^^^^ cannot infer the value of the const parameter `N` declared on the function `take_array_from_mut`
help: consider specifying the generic arguments
   |
   |
LL |         println!("{:?}", take_array_from_mut::<i32, _>(&mut arr, i));

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---

11    |             ^^^
12 help: provide the argument
13    |
- LL |     let a = f(/* value */);
-    |             ~~~~~~~~~~~~~~
+ LL |     let a = f(/* _ */);
16 
17 error[E0057]: this function takes 1 argument but 2 arguments were supplied
18   --> $DIR/E0057.rs:5:13

---
To only update this specific test, also pass `--test-args error-codes/E0057.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0057.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0057" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0057/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0057.rs:3:13
   |
   |
LL |     let a = f(); //~ ERROR E0057
   |             ^-- an argument is missing
note: closure defined here
  --> /checkout/src/test/ui/error-codes/E0057.rs:2:13
   |
   |
LL |     let f = |x| x * 3;
help: provide the argument
   |
   |
LL |     let a = f(/* _ */); //~ ERROR E0057

error[E0057]: this function takes 1 argument but 2 arguments were supplied
  --> /checkout/src/test/ui/error-codes/E0057.rs:5:13
   |
   |
LL |     let c = f(2, 3); //~ ERROR E0057
   |             ^    - argument of type `{integer}` unexpected
note: closure defined here
  --> /checkout/src/test/ui/error-codes/E0057.rs:2:13
   |
   |
LL |     let f = |x| x * 3;
help: remove the extra argument
   |
   |
LL |     let c = f(2); //~ ERROR E0057

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0057`.
---

11    |    ^    ----
12 help: provide the argument
13    |
- LL |     f(&[f(/* value */)]);
-    |         ~~~~~~~~~~~~~~
+ LL |     f(&[f(/* _ */)]);
16 
17 error: aborting due to previous error
18 

---
To only update this specific test, also pass `--test-args hrtb/issue-58451.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-58451.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-58451" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-58451/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/hrtb/issue-58451.rs:12:9
   |
   |
LL |     f(&[f()]); //~ ERROR this function takes 1 argument
   |         ^-- an argument is missing
note: function defined here
  --> /checkout/src/test/ui/hrtb/issue-58451.rs:5:4
   |
   |
LL | fn f<I>(i: I)
   |    ^    ----
   |
   |
LL |     f(&[f(/* _ */)]); //~ ERROR this function takes 1 argument

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
For more information about this error, try `rustc --explain E0061`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-23041.rs stdout ----
diff of stderr:

4 LL |     b.downcast_ref::<fn(_)->_>();
5    |       ^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `downcast_ref`
- help: consider specifying the generic arguments
+ help: consider specifying the generic argument
8    |
8    |
9 LL |     b.downcast_ref::<fn(_) -> _>();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23041/issue-23041.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23041/issue-23041.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23041.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23041" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23041/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-23041.rs:6:7
   |
   |
LL |     b.downcast_ref::<fn(_)->_>(); //~ ERROR E0282
   |       ^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the associated function `downcast_ref`
help: consider specifying the generic argument
   |
   |
LL |     b.downcast_ref::<fn(_) -> _>(); //~ ERROR E0282

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-24013.rs stdout ----
diff of stderr:

4 LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};
5    |             ^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the function `swap`
- help: consider specifying the generic arguments
+ help: consider specifying the generic argument
8    |
8    |
9 LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013/issue-24013.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013/issue-24013.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-24013.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-24013.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-24013/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-24013.rs:5:13
   |
   |
LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};
   |             ^^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the function `swap`
help: consider specifying the generic argument
   |
   |
LL |     unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-3044.rs stdout ----
diff of stderr:

16    |
17 LL ~     needlesArr.iter().fold(|x, y| {
18 LL +
- LL ~     }, /* value */);
+ LL ~     }, /* _ */);
21 
22 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044/issue-3044.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3044.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3044.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-3044.rs:3:23
   |
   |
LL |       needlesArr.iter().fold(|x, y| {
   |  _______________________^^^^-
LL | |         //~^ ERROR this function takes 2 arguments but 1 argument was supplied
LL | |     });
   | |______- an argument is missing
note: associated function defined here
  --> /checkout/library/core/src/iter/traits/iterator.rs:2407:8
   |
   |
LL |     fn fold<B, F>(mut self, init: B, mut f: F) -> B
help: provide the argument
   |
   |
LL ~     needlesArr.iter().fold(|x, y| {
LL +         //~^ ERROR this function takes 2 arguments but 1 argument was supplied
LL ~     }, /* _ */);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
---
8               found reference `&_`
- help: a return type might be missing here
+ help: try adding a return type
10    |
- LL | fn g() -> _ {
-    |        ++++
+ LL | fn g() -> &_ {
13 help: consider removing the borrow
14    |
14    |
15 LL -     &panic!()

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485/diverging-tuple-parts-39485.stderr
To only update this specific test, also pass `--test-args never_type/diverging-tuple-parts-39485.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/diverging-tuple-parts-39485/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs:8:5
   |
   |
LL |     &panic!() //~ ERROR mismatched types
   |
   = note: expected unit type `()`
              found reference `&_`
help: try adding a return type
help: try adding a return type
   |
LL | fn g() -> &_ {
help: consider removing the borrow
   |
   |
LL -     &panic!() //~ ERROR mismatched types
LL +     panic!() //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/never_type/diverging-tuple-parts-39485.rs:12:5
   |
   |
LL | fn f() -> isize {
   |           ----- expected `isize` because of return type
LL |     (return 1, return 2) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^ expected `isize`, found tuple
   = note: expected type `isize`
   = note: expected type `isize`
             found tuple `(!, !)`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---

45    |     ^^
46 help: provide the argument
47    |
- LL |     let _ = Ok(/* value */);
-    |             ~~~~~~~~~~~~~~~
+ LL |     let _ = Ok(/* _ */);
50 
51 error[E0061]: this struct takes 1 argument but 0 arguments were supplied
52   --> $DIR/struct-enum-wrong-args.rs:9:13

---
To only update this specific test, also pass `--test-args typeck/struct-enum-wrong-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/struct-enum-wrong-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/struct-enum-wrong-args/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:6:13
   |
   |
LL |     let _ = Some(3, 2); //~ ERROR this enum variant takes
   |             ^^^^    - argument of type `{integer}` unexpected
note: tuple variant defined here
  --> /checkout/library/core/src/option.rs:526:5
   |
LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
LL |     Some(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^^^
help: remove the extra argument
   |
LL |     let _ = Some(3); //~ ERROR this enum variant takes

error[E0061]: this enum variant takes 1 argument but 3 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:7:13
   |
   |
LL |     let _ = Ok(3, 6, 2); //~ ERROR this enum variant takes
   |             ^^    -  - argument of type `{integer}` unexpected
   |                   argument of type `{integer}` unexpected
   |
note: tuple variant defined here
  --> /checkout/library/core/src/result.rs:508:5
  --> /checkout/library/core/src/result.rs:508:5
   |
LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^
help: remove the extra arguments
   |
LL |     let _ = Ok(3); //~ ERROR this enum variant takes

error[E0061]: this enum variant takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:8:13
   |
   |
LL |     let _ = Ok(); //~ ERROR this enum variant takes
   |             ^^-- an argument is missing
note: tuple variant defined here
  --> /checkout/library/core/src/result.rs:508:5
   |
LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
   |     ^^
help: provide the argument
   |
LL |     let _ = Ok(/* _ */); //~ ERROR this enum variant takes

error[E0061]: this struct takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:9:13
   |
   |
LL |     let _ = Wrapper(); //~ ERROR this struct takes
   |             ^^^^^^^-- an argument of type `i32` is missing
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
   |
   |
LL | struct Wrapper(i32);
help: provide the argument
   |
   |
LL |     let _ = Wrapper(/* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 1 argument but 2 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:10:13
   |
   |
LL |     let _ = Wrapper(5, 2); //~ ERROR this struct takes
   |             ^^^^^^^    - argument of type `{integer}` unexpected
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:2:8
   |
   |
LL | struct Wrapper(i32);
help: remove the extra argument
   |
   |
LL |     let _ = Wrapper(5); //~ ERROR this struct takes

error[E0061]: this struct takes 2 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:11:13
   |
   |
LL |     let _ = DoubleWrapper(); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^-- two arguments of type `i32` and `i32` are missing
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
   |
LL | struct DoubleWrapper(i32, i32);
help: provide the arguments
   |
   |
LL |     let _ = DoubleWrapper(/* i32 */, /* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:12:13
   |
   |
LL |     let _ = DoubleWrapper(5); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^--- an argument of type `i32` is missing
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
   |
LL | struct DoubleWrapper(i32, i32);
help: provide the argument
   |
   |
LL |     let _ = DoubleWrapper(5, /* i32 */); //~ ERROR this struct takes

error[E0061]: this struct takes 2 arguments but 3 arguments were supplied
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:13:13
   |
   |
LL |     let _ = DoubleWrapper(5, 2, 7); //~ ERROR this struct takes
   |             ^^^^^^^^^^^^^       - argument of type `{integer}` unexpected
note: tuple struct defined here
  --> /checkout/src/test/ui/typeck/struct-enum-wrong-args.rs:3:8
   |
   |
LL | struct DoubleWrapper(i32, i32);
help: remove the extra argument
   |
   |
LL |     let _ = DoubleWrapper(5, 2); //~ ERROR this struct takes

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0061`.
