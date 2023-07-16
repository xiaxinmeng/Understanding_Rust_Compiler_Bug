plain

running 12558 tests
.................................................................................................... 100/12558
...........................................iiiiiiiiiiii...........i.i................i...i.......... 200/12558
......F............................................................................................. 300/12558
................F................FF..............F......F........................................... 400/12558
.............................................FF..................................F.................. 500/12558
..F.......F.........F................i.............................................................. 600/12558
......i.i.............................................................................i............. 800/12558
.................................................................................................... 900/12558
.................................................................................................... 1000/12558
.................................................................................................... 1100/12558
---
........i.........i.........i...............................................................F....... 3700/12558
..................................................................................i................. 3800/12558
.................................................................................................... 3900/12558
......i............................................................................................. 4000/12558
..i.................i.......................i........................................F.............. 4100/12558
.....................................F.......F...................................................... 4200/12558
....................................................................F............................... 4400/12558
.................................................................F.................................. 4500/12558
.................................................................F.................................. 4500/12558
..............................F..............................F...................................... 4600/12558
................................................................................................i... 4800/12558
...........................................................i........................................ 4900/12558
.................................................................................................... 5000/12558
.................................................................................................... 5100/12558
---
.................................................................................................... 11000/12558
.................................................................................................... 11100/12558
...................................................................................ii............... 11200/12558
..............i..................................................................................... 11300/12558
......................................................F.F........................................... 11400/12558
.................................................................................................... 11600/12558
.................................................................................................... 11700/12558
...................................................................................i..........F..... 11800/12558
.............F...................................................................................... 11900/12558
---
.....................................................iii............................................ 12500/12558
..........................................................
failures:

---- [ui] ui/associated-consts/assoc-const-ty-mismatch.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const-ty-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: Could not find associated const on trait
   |
   = note: delayed at compiler/rustc_typeck/src/collect/type_of.rs:551:27

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_middle/src/ty/consts.rs:104:30

error: internal compiler error: TyKind::Error constructed but no error reported
---
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43

error: internal compiler error: no MIR body is available for DefId(0:12 ~ assoc_const_ty_mismatch[6aa3]::{impl#1}::T)
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:19:3
LL |   type T = usize;
   |   ^^^^^^^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_const_eval/src/const_eval/eval_queries.rs:293:22
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43


error: internal compiler error: cannot relate constants of different types: usize != [type error]
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:559:29

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:721:18

error: internal compiler error: mir_const_qualif: MIR had errors
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_mir_transform/src/lib.rs:186:18


error: internal compiler error: PromoteTemps: MIR had errors
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22


error: internal compiler error: broken MIR in DefId(0:17 ~ assoc_const_ty_mismatch[6aa3]::foo2::{constant#0}) ("return type"): bad type [type error]
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:796:20

error: internal compiler error: broken MIR in DefId(0:17 ~ assoc_const_ty_mismatch[6aa3]::foo2::{constant#0}) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20: 24:26 (#0), scope: scope[0] } }): bad type [type error]
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:541:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:796:20

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
   |
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/consts.rs:245:43

error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
  --> /checkout/src/test/ui/associated-consts/assoc-const-ty-mismatch.rs:24:20
   |
LL | fn foo2<F: FooTy<T=3usize>>() {}
   |
   = note: delayed at compiler/rustc_middle/src/ty/consts.rs:104:30


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1173:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (b9cc7777a 2022-01-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs stdout ----
diff of stderr:

4 LL | fn b() { blue_car(ModelT); }
5    |          ^^^^^^^^ type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
6    |
- note: expected this to be `Blue`
+ note: expected struct `Blue`, found struct `Black`
9    |
9    |
10 LL | impl Vehicle for ModelT { type Color = Black; }

21 LL | fn c() { black_car(ModelU); }
22    |          ^^^^^^^^^ type mismatch resolving `<ModelU as Vehicle>::Color == Black`
23    |
- note: expected this to be `Black`
+ note: expected struct `Black`, found struct `Blue`
26    |
26    |
27 LL | impl Vehicle for ModelU { type Color = Blue; }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/associated-types-binding-to-type-defined-in-supertrait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-binding-to-type-defined-in-supertrait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-binding-to-type-defined-in-supertrait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
   |
   |
LL | fn b() { blue_car(ModelT); } //~ ERROR type mismatch
   |          ^^^^^^^^ type mismatch resolving `<ModelT as Vehicle>::Color == Blue`
   |
note: expected struct `Blue`, found struct `Black`
   |
   |
LL | impl Vehicle for ModelT { type Color = Black; }
   |                                        ^^^^^
note: required by a bound in `blue_car`
   |
   |
LL | fn blue_car<C:Car<Color=Blue>>(c: C) {
   |                   ^^^^^^^^^^ required by this bound in `blue_car`

error[E0271]: type mismatch resolving `<ModelU as Vehicle>::Color == Black`
   |
   |
LL | fn c() { black_car(ModelU); } //~ ERROR type mismatch
   |          ^^^^^^^^^ type mismatch resolving `<ModelU as Vehicle>::Color == Black`
   |
note: expected struct `Black`, found struct `Blue`
   |
   |
LL | impl Vehicle for ModelU { type Color = Blue; }
   |                                        ^^^^
note: required by a bound in `black_car`
   |
   |
LL | fn black_car<C:Car<Color=Black>>(c: C) {
   |                    ^^^^^^^^^^^ required by this bound in `black_car`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/associated-types/associated-types-eq-hr.rs stdout ----
diff of stderr:

4 LL |     foo::<UintStruct>();
5    |     ^^^^^^^^^^^^^^^^^ type mismatch resolving `for<'x> <UintStruct as TheTrait<&'x isize>>::A == &'x isize`
6    |
- note: expected this to be `&isize`
+ note: expected `isize`, found `usize`
9    |
9    |
10 LL |     type A = &'a usize;
11    |              ^^^^^^^^^
-    = note: expected reference `&isize`
-               found reference `&usize`
14 note: required by a bound in `foo`
14 note: required by a bound in `foo`
15   --> $DIR/associated-types-eq-hr.rs:45:36
16    |

26 LL |     bar::<IntStruct>();
27    |     ^^^^^^^^^^^^^^^^ type mismatch resolving `for<'x> <IntStruct as TheTrait<&'x isize>>::A == &'x usize`
28    |
- note: expected this to be `&usize`
+ note: expected `usize`, found `isize`
31    |
31    |
32 LL |     type A = &'a isize;
33    |              ^^^^^^^^^
-    = note: expected reference `&usize`
-               found reference `&isize`
36 note: required by a bound in `bar`
---
To only update this specific test, also pass `--test-args associated-types/associated-types-eq-hr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-eq-hr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-hr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-hr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `for<'x> <UintStruct as TheTrait<&'x isize>>::A == &'x isize`
   |
   |
LL |     foo::<UintStruct>(); //~ ERROR type mismatch
   |     ^^^^^^^^^^^^^^^^^ type mismatch resolving `for<'x> <UintStruct as TheTrait<&'x isize>>::A == &'x isize`
note: expected `isize`, found `usize`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:26:14
   |
   |
LL |     type A = &'a usize;
note: required by a bound in `foo`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:45:36
   |
LL | fn foo<T>()
LL | fn foo<T>()
   |    --- required by a bound in this
LL | where
LL |     T: for<'x> TheTrait<&'x isize, A = &'x isize>,
   |                                    ^^^^^^^^^^^^^ required by this bound in `foo`

error[E0271]: type mismatch resolving `for<'x> <IntStruct as TheTrait<&'x isize>>::A == &'x usize`
   |
   |
LL |     bar::<IntStruct>(); //~ ERROR type mismatch
   |     ^^^^^^^^^^^^^^^^ type mismatch resolving `for<'x> <IntStruct as TheTrait<&'x isize>>::A == &'x usize`
note: expected `usize`, found `isize`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:14:14
   |
   |
LL |     type A = &'a isize;
note: required by a bound in `bar`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-hr.rs:52:36
   |
   |
LL | fn bar<T>()
   |    --- required by a bound in this
LL | where
LL |     T: for<'x> TheTrait<&'x isize, A = &'x usize>,
   |                                    ^^^^^^^^^^^^^ required by this bound in `bar`

error: implementation of `TheTrait` is not general enough
   |
   |
LL |     tuple_one::<Tuple>();
   |     ^^^^^^^^^^^^^^^^^^ implementation of `TheTrait` is not general enough
   |
   = note: `Tuple` must implement `TheTrait<(&'0 isize, &'1 isize)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `TheTrait<(&'2 isize, &'2 isize)>`, for some specific lifetime `'2`

error: implementation of `TheTrait` is not general enough
   |
   |
LL |     tuple_one::<Tuple>();
   |     ^^^^^^^^^^^^^^^^^^ implementation of `TheTrait` is not general enough
   |
   = note: `Tuple` must implement `TheTrait<(&'0 isize, &'1 isize)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `TheTrait<(&'2 isize, &'2 isize)>`, for some specific lifetime `'2`

error: implementation of `TheTrait` is not general enough
   |
   |
LL |     tuple_two::<Tuple>();
   |     ^^^^^^^^^^^^^^^^^^ implementation of `TheTrait` is not general enough
   |
   = note: `Tuple` must implement `TheTrait<(&'0 isize, &'1 isize)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `TheTrait<(&'2 isize, &'2 isize)>`, for some specific lifetime `'2`

error: implementation of `TheTrait` is not general enough
   |
   |
LL |     tuple_two::<Tuple>();
   |     ^^^^^^^^^^^^^^^^^^ implementation of `TheTrait` is not general enough
   |
   = note: `Tuple` must implement `TheTrait<(&'0 isize, &'1 isize)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `TheTrait<(&'2 isize, &'2 isize)>`, for some specific lifetime `'2`

error: implementation of `TheTrait` is not general enough
   |
   |
LL |     tuple_four::<Tuple>();
   |     ^^^^^^^^^^^^^^^^^^^ implementation of `TheTrait` is not general enough
   |
   = note: `Tuple` must implement `TheTrait<(&'0 isize, &'1 isize)>`, for any two lifetimes `'0` and `'1`...
   = note: ...but it actually implements `TheTrait<(&'2 isize, &'2 isize)>`, for some specific lifetime `'2`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/associated-types/associated-types-eq-3.rs stdout ----
diff of stderr:

19 LL |     foo1(a);
20    |     ^^^^ type mismatch resolving `<isize as Foo>::A == Bar`
- note: expected this to be `Bar`
- note: expected this to be `Bar`
+ note: expected struct `Bar`, found `usize`
24    |
24    |
25 LL |     type A = usize;

36 LL |     baz(&a);
37    |         ^^ type mismatch resolving `<isize as Foo>::A == Bar`
- note: expected this to be `Bar`
- note: expected this to be `Bar`
+ note: expected struct `Bar`, found `usize`
41    |
41    |
42 LL |     type A = usize;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/associated-types-eq-3.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-eq-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-eq-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-eq-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/associated-types/associated-types-eq-3.rs:23:18
   |
LL |     let _: Bar = x.boo();
   |            ---   ^^^^^^^ expected struct `Bar`, found associated type
   |            expected due to this
   |
   = note:       expected struct `Bar`
           found associated type `<I as Foo>::A`
           found associated type `<I as Foo>::A`
help: consider constraining the associated type `<I as Foo>::A` to `Bar`
   |
LL | fn foo2<I: Foo<A = Bar>>(x: I) {


error[E0271]: type mismatch resolving `<isize as Foo>::A == Bar`
   |
LL |     foo1(a);
LL |     foo1(a);
   |     ^^^^ type mismatch resolving `<isize as Foo>::A == Bar`
note: expected struct `Bar`, found `usize`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-3.rs:12:14
   |
   |
LL |     type A = usize;
note: required by a bound in `foo1`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-3.rs:18:16
   |
   |
LL | fn foo1<I: Foo<A=Bar>>(x: I) {
   |                ^^^^^ required by this bound in `foo1`

error[E0271]: type mismatch resolving `<isize as Foo>::A == Bar`
   |
   |
LL |     baz(&a);
   |         ^^ type mismatch resolving `<isize as Foo>::A == Bar`
note: expected struct `Bar`, found `usize`
  --> /checkout/src/test/ui/associated-types/associated-types-eq-3.rs:12:14
   |
   |
LL |     type A = usize;
   |              ^^^^^
   = note: required for the cast to the object type `dyn Foo<A = Bar>`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0271, E0308.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/associated-types/associated-types-issue-20346.rs stdout ----
diff of stderr:

7 LL |     is_iterator_of::<Option<T>, _>(&adapter);
8    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Adapter<I> as Iterator>::Item == Option<T>`
- note: expected this to be `Option<T>`
- note: expected this to be `Option<T>`
+ note: expected enum `Option`, found type parameter `T`
12    |
13 LL |     type Item = T;

14    |                 ^
14    |                 ^
-    = note: expected enum `Option<T>`
+    = note: expected type `Option<T>`
16               found type `T`
17 note: required by a bound in `is_iterator_of`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-issue-20346/associated-types-issue-20346.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-issue-20346/associated-types-issue-20346.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-issue-20346.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-issue-20346.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-issue-20346" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-issue-20346/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Adapter<I> as Iterator>::Item == Option<T>`
   |
   |
LL | fn test_adapter<T, I: Iterator<Item=Option<T>>>(it: I) {
   |                 - this type parameter
...
LL |     is_iterator_of::<Option<T>, _>(&adapter); //~ ERROR type mismatch
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Adapter<I> as Iterator>::Item == Option<T>`
   |
note: expected enum `Option`, found type parameter `T`
   |
LL |     type Item = T;
   |                 ^
   = note: expected type `Option<T>`
   = note: expected type `Option<T>`
              found type `T`
note: required by a bound in `is_iterator_of`
   |
   |
LL | fn is_iterator_of<A, I: Iterator<Item=A>>(_: &I) {}
   |                                  ^^^^^^ required by this bound in `is_iterator_of`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/associated-types/associated-types-multiple-types-one-trait.rs stdout ----
diff of stderr:

4 LL |     want_y(t);
5    |     ^^^^^^ expected `i32`, found associated type
-    = note:         expected type `i32`
-    = note:         expected type `i32`
-            found associated type `<T as Foo>::Y`
+    = note: expected type `i32`
+               found type `<T as Foo>::Y`
9 note: required by a bound in `want_y`
11    |


22 LL |     want_x(t);
23    |     ^^^^^^ expected `u32`, found associated type
-    = note:         expected type `u32`
-    = note:         expected type `u32`
-            found associated type `<T as Foo>::X`
+    = note: expected type `u32`
+               found type `<T as Foo>::X`
27 note: required by a bound in `want_x`
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait/associated-types-multiple-types-one-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-multiple-types-one-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-multiple-types-one-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-multiple-types-one-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<T as Foo>::Y == i32`
   |
   |
LL |     want_y(t); //~ ERROR type mismatch
   |     ^^^^^^ expected `i32`, found associated type
   = note: expected type `i32`
   = note: expected type `i32`
              found type `<T as Foo>::Y`
note: required by a bound in `want_y`
   |
   |
LL | fn want_y<T:Foo<Y=i32>>(t: &T) { }
   |                 ^^^^^ required by this bound in `want_y`
help: consider constraining the associated type `<T as Foo>::Y` to `i32`
   |
LL | fn have_x_want_y<T:Foo<X=u32, Y = i32>>(t: &T)


error[E0271]: type mismatch resolving `<T as Foo>::X == u32`
   |
   |
LL |     want_x(t); //~ ERROR type mismatch
   |     ^^^^^^ expected `u32`, found associated type
   = note: expected type `u32`
   = note: expected type `u32`
              found type `<T as Foo>::X`
note: required by a bound in `want_x`
   |
   |
LL | fn want_x<T:Foo<X=u32>>(t: &T) { }
   |                 ^^^^^ required by this bound in `want_x`
help: consider constraining the associated type `<T as Foo>::X` to `u32`
   |
LL | fn have_y_want_x<T:Foo<Y=i32, X = u32>>(t: &T)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/associated-types/hr-associated-type-projection-1.rs stdout ----
diff of stderr:

4 LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<'_, T> for T {
5    |      - this type parameter      ^^^^^^^^^^^^^^^^^ expected associated type, found type parameter `T`
-    = note: expected associated type `<T as Deref>::Target`
-                found type parameter `T`
+    = note: expected type `<T as Deref>::Target`
+               found type `T`
+               found type `T`
9 help: consider further restricting this bound
10    |
11 LL | impl<T: Copy + std::ops::Deref + Deref<Target = T>> UnsafeCopy<'_, T> for T {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-projection-1/hr-associated-type-projection-1.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-projection-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-projection-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-projection-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-projection-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
   |
   |
LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<'_, T> for T {
   |      - this type parameter      ^^^^^^^^^^^^^^^^^ expected associated type, found type parameter `T`
   = note: expected type `<T as Deref>::Target`
              found type `T`
help: consider further restricting this bound
   |
   |
LL | impl<T: Copy + std::ops::Deref + Deref<Target = T>> UnsafeCopy<'_, T> for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/associated-types/impl-trait-return-missing-constraint.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<impl Bar as Foo>::Item == i32`
3    |
3    |
- LL | fn bar() -> impl Bar {
- ...
- ...
7 LL | fn baz() -> impl Bar<Item = i32> {
8    |             ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found associated type

-    = note:         expected type `i32`
-    = note:         expected type `i32`
-            found associated type `<impl Bar as Foo>::Item`
+    = note: expected type `i32`
+               found type `<impl Bar as Foo>::Item`
12 help: consider constraining the associated type `<impl Bar as Foo>::Item` to `i32`
13    |
14 LL | fn bar() -> impl Bar<Item = i32> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-trait-return-missing-constraint/impl-trait-return-missing-constraint.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/impl-trait-return-missing-constraint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-trait-return-missing-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-trait-return-missing-constraint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-trait-return-missing-constraint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<impl Bar as Foo>::Item == i32`
   |
   |
LL | fn baz() -> impl Bar<Item = i32> {
   |             ^^^^^^^^^^^^^^^^^^^^ expected `i32`, found associated type
   = note: expected type `i32`
   = note: expected type `i32`
              found type `<impl Bar as Foo>::Item`
help: consider constraining the associated type `<impl Bar as Foo>::Item` to `i32`
   |
LL | fn bar() -> impl Bar<Item = i32> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/associated-types/issue-44153.rs stdout ----
diff of stderr:

4 LL |     <() as Visit>::visit();
5    |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<() as Array>::Element == &()`
6    |
- note: expected this to be `&()`
+ note: expected `&()`, found `()`
9    |
10 LL |     type Element = ();



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-44153/issue-44153.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-44153.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-44153.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-44153" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-44153/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<() as Array>::Element == &()`
   |
   |
LL |     <() as Visit>::visit(); //~ ERROR: type mismatch resolving
   |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<() as Array>::Element == &()`
   |
note: expected `&()`, found `()`
   |
LL |     type Element = ();
   |                    ^^
   |                    ^^
note: required because of the requirements on the impl of `Visit` for `()`
   |
   |
LL | impl<'a> Visit for () where

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/associated-types/issue-72806.rs stdout ----
diff of stderr:

4 LL |     type Sibling = Foo2;
5    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == char`
- note: expected this to be `char`
---
24    |
-    = note:    expected unit type `()`
-            found associated type `<B as Trait>::Associated`
+    = note: expected type `()`
+               found type `<B as Trait>::Associated`
27    = help: consider constraining the associated type `<B as Trait>::Associated` to `()`
29 note: required by a bound in `accepts_trait`

38 LL |     accepts_trait(c);
39    |     ^^^^^^^^^^^^^ expected `()`, found associated type
---
58    |
-    = note:    expected unit type `()`
-            found associated type `<D as Trait>::Associated`
+    = note: expected type `()`
+               found type `<D as Trait>::Associated`
61    = help: consider constraining the associated type `<D as Trait>::Associated` to `()`
63 note: required by a bound in `accepts_trait`


72 LL |     accepts_generic_trait(e);
74    |
-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<E as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<E as GenericTrait<()>>::Associated`
77 note: required by a bound in `accepts_generic_trait`
79    |


90 LL |     accepts_generic_trait(f);
92    |
-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<F as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<F as GenericTrait<()>>::Associated`
95 note: required by a bound in `accepts_generic_trait`
97    |


108 LL |     accepts_generic_trait(g);
110    |
-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<G as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<G as GenericTrait<()>>::Associated`
113    = help: consider constraining the associated type `<G as GenericTrait<()>>::Associated` to `()`
114    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
115 note: required by a bound in `accepts_generic_trait`

121 error[E0271]: type mismatch resolving `<impl Trait as Trait>::Associated == ()`
123    |
123    |
- LL | fn returns_opaque() -> impl Trait + 'static {
- ...
- ...
127 LL |     accepts_trait(returns_opaque());
129    |

-    = note:    expected unit type `()`
-            found associated type `<impl Trait as Trait>::Associated`
-            found associated type `<impl Trait as Trait>::Associated`
+    = note: expected type `()`
+               found type `<impl Trait as Trait>::Associated`
132 note: required by a bound in `accepts_trait`
133   --> $DIR/issue-87261.rs:43:27
134    |

142 error[E0271]: type mismatch resolving `<impl DerivedTrait as Trait>::Associated == ()`
144    |
144    |
- LL | fn returns_opaque_derived() -> impl DerivedTrait + 'static {
- ...
- ...
148 LL |     accepts_trait(returns_opaque_derived());
150    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl DerivedTrait as Trait>::Associated`
+    = note: expected type `()`
+               found type `<impl DerivedTrait as Trait>::Associated`
153 note: required by a bound in `accepts_trait`
155    |


163 error[E0271]: type mismatch resolving `<impl Foo + Trait as Trait>::Associated == ()`
165    |
165    |
- LL | fn returns_opaque_foo() -> impl Trait + Foo {
- ...
- ...
169 LL |     accepts_trait(returns_opaque_foo());
171    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl Foo + Trait as Trait>::Associated`
+    = note: expected type `()`
+               found type `<impl Foo + Trait as Trait>::Associated`
174 note: required by a bound in `accepts_trait`
176    |


184 error[E0271]: type mismatch resolving `<impl Foo + DerivedTrait as Trait>::Associated == ()`
186    |
186    |
- LL | fn returns_opaque_derived_foo() -> impl DerivedTrait + Foo {
- ...
- ...
190 LL |     accepts_trait(returns_opaque_derived_foo());
192    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl Foo + DerivedTrait as Trait>::Associated`
+    = note: expected type `()`
+               found type `<impl Foo + DerivedTrait as Trait>::Associated`
195    = help: consider constraining the associated type `<impl Foo + DerivedTrait as Trait>::Associated` to `()`
197 note: required by a bound in `accepts_trait`


203 error[E0271]: type mismatch resolving `<impl GenericTrait<()> as GenericTrait<()>>::Associated == ()`
205    |
205    |
- LL | fn returns_opaque_generic() -> impl GenericTrait<()> + 'static {
- ...
- ...
209 LL |     accepts_generic_trait(returns_opaque_generic());
211    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl GenericTrait<()> as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<impl GenericTrait<()> as GenericTrait<()>>::Associated`
214 note: required by a bound in `accepts_generic_trait`
216    |


224 error[E0271]: type mismatch resolving `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
226    |
226    |
- LL | fn returns_opaque_generic_foo() -> impl GenericTrait<()> + Foo {
- ...
- ...
230 LL |     accepts_generic_trait(returns_opaque_generic_foo());
232    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated`
235 note: required by a bound in `accepts_generic_trait`
237    |


245 error[E0271]: type mismatch resolving `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
247    |
247    |
- LL | fn returns_opaque_generic_duplicate() -> impl GenericTrait<()> + GenericTrait<u8> {
- ...
- ...
251 LL |     accepts_generic_trait(returns_opaque_generic_duplicate());
253    |

-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated`
+    = note: expected type `()`
+               found type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated`
256    = help: consider constraining the associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
257    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
258 note: required by a bound in `accepts_generic_trait`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/issue-87261.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/issue-87261.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/issue-87261.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/issue-87261/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<A as Trait>::Associated == ()`
   |
LL |     accepts_trait(a);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note: expected type `()`
              found type `<A as Trait>::Associated`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<A as Trait>::Associated` to `()`
   |
LL |     A: Trait<Associated = ()> + 'static,


error[E0271]: type mismatch resolving `<B as Trait>::Associated == ()`
   |
LL |     accepts_trait(b);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note: expected type `()`
              found type `<B as Trait>::Associated`
   = help: consider constraining the associated type `<B as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<C as Trait>::Associated == ()`
   |
LL |     accepts_trait(c);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note: expected type `()`
              found type `<C as Trait>::Associated`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<C as Trait>::Associated` to `()`
   |
LL |     C: Trait<Associated = ()> + Foo,


error[E0271]: type mismatch resolving `<D as Trait>::Associated == ()`
   |
LL |     accepts_trait(d);
   |     ^^^^^^^^^^^^^ expected `()`, found associated type
   |
   |
   = note: expected type `()`
              found type `<D as Trait>::Associated`
   = help: consider constraining the associated type `<D as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<E as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(e);
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<E as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<E as GenericTrait<()>>::Associated` to `()`
   |
LL |     E: GenericTrait<(), Associated = ()> + 'static,


error[E0271]: type mismatch resolving `<F as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(f);
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<F as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<F as GenericTrait<()>>::Associated` to `()`
   |
LL |     F: GenericTrait<(), Associated = ()> + Foo,


error[E0271]: type mismatch resolving `<G as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(g);
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<G as GenericTrait<()>>::Associated`
   = help: consider constraining the associated type `<G as GenericTrait<()>>::Associated` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`

error[E0271]: type mismatch resolving `<impl Trait as Trait>::Associated == ()`
   |
   |
LL |     accepts_trait(returns_opaque());
   |
   = note: expected type `()`
              found type `<impl Trait as Trait>::Associated`
note: required by a bound in `accepts_trait`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl Trait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque() -> impl Trait<Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl DerivedTrait as Trait>::Associated == ()`
   |
   |
LL |     accepts_trait(returns_opaque_derived());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl DerivedTrait as Trait>::Associated`
note: required by a bound in `accepts_trait`
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl DerivedTrait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque_derived() -> impl DerivedTrait<Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl Foo + Trait as Trait>::Associated == ()`
   |
   |
LL |     accepts_trait(returns_opaque_foo());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl Foo + Trait as Trait>::Associated`
note: required by a bound in `accepts_trait`
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`
help: consider constraining the associated type `<impl Foo + Trait as Trait>::Associated` to `()`
   |
LL | fn returns_opaque_foo() -> impl Trait<Associated = ()> + Foo {


error[E0271]: type mismatch resolving `<impl Foo + DerivedTrait as Trait>::Associated == ()`
   |
   |
LL |     accepts_trait(returns_opaque_derived_foo());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl Foo + DerivedTrait as Trait>::Associated`
   = help: consider constraining the associated type `<impl Foo + DerivedTrait as Trait>::Associated` to `()`
note: required by a bound in `accepts_trait`
  --> /checkout/src/test/ui/associated-types/issue-87261.rs:43:27
   |
   |
LL | fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
   |                           ^^^^^^^^^^^^^^^ required by this bound in `accepts_trait`

error[E0271]: type mismatch resolving `<impl GenericTrait<()> as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(returns_opaque_generic());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl GenericTrait<()> as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<impl GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
   |
LL | fn returns_opaque_generic() -> impl GenericTrait<(), Associated = ()> + 'static {


error[E0271]: type mismatch resolving `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(returns_opaque_generic_foo());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated`
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
help: consider constraining the associated type `<impl Foo + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
   |
LL | fn returns_opaque_generic_foo() -> impl GenericTrait<(), Associated = ()> + Foo {


error[E0271]: type mismatch resolving `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated == ()`
   |
   |
LL |     accepts_generic_trait(returns_opaque_generic_duplicate());
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated`
   = help: consider constraining the associated type `<impl GenericTrait<u8> + GenericTrait<()> as GenericTrait<()>>::Associated` to `()`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
note: required by a bound in `accepts_generic_trait`
   |
   |
LL | fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}
   |                                              ^^^^^^^^^^^^^^^ required by this bound in `accepts_generic_trait`
error: aborting due to 14 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/associated-types/point-at-type-on-obligation-failure.rs stdout ----
diff of stderr:

4 LL |     type Sibling = Foo2;
5    |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
- note: expected this to be `()`
+ note: expected `()`, found `u32`
8   --> $DIR/point-at-type-on-obligation-failure.rs:18:15
9    |
---
To only update this specific test, also pass `--test-args associated-types/point-at-type-on-obligation-failure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/point-at-type-on-obligation-failure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
   |
LL |     type Sibling = Foo2;
LL |     type Sibling = Foo2;
   |                    ^^^^ type mismatch resolving `<Foo2 as Bar2>::Ok == ()`
note: expected `()`, found `u32`
  --> /checkout/src/test/ui/associated-types/point-at-type-on-obligation-failure.rs:18:15
   |
LL |     type Ok = u32;
LL |     type Ok = u32;
   |               ^^^
note: required by a bound in `Bar::Sibling`
   |
   |
LL |     type Sibling: Bar2<Ok=Self::Ok>;
   |                        ^^^^^^^^^^^ required by this bound in `Bar::Sibling`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.

---
To only update this specific test, also pass `--test-args error-codes/E0271.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0271.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0271" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0271/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<i8 as Trait>::AssociatedType == u32`
  --> /checkout/src/test/ui/error-codes/E0271.rs:10:5
   |
LL |     foo(3_i8); //~ ERROR E0271
   |     ^^^ type mismatch resolving `<i8 as Trait>::AssociatedType == u32`
note: expected `u32`, found `&str`
  --> /checkout/src/test/ui/error-codes/E0271.rs:7:43
   |
LL | impl Trait for i8 { type AssociatedType = &'static str; }
LL | impl Trait for i8 { type AssociatedType = &'static str; }
   |                                           ^^^^^^^^^^^^
note: required by a bound in `foo`
  --> /checkout/src/test/ui/error-codes/E0271.rs:3:32
   |
LL | fn foo<T>(t: T) where T: Trait<AssociatedType=u32> {
   |                                ^^^^^^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-associated_const_equality.rs stdout ----
diff of stderr:

7    = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
8    = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
- error: associated const equality is incomplete
-   --> $DIR/feature-gate-associated_const_equality.rs:10:28
-    |
-    |
- LL | fn foo<A: TraitWAssocConst<A=32>>() {}
-    |                            ^^^^ cannot yet relate associated const
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
17 
18 For more information about this error, try `rustc --explain E0658`.
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-associated_const_equality.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_const_equality.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_const_equality/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: associated const equality is incomplete
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_const_equality.rs:10:28
   |
LL | fn foo<A: TraitWAssocConst<A=32>>() {}
   |
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = note: see issue #92827 <https://github.com/rust-lang/rust/issues/92827> for more information
   = help: add `#![feature(associated_const_equality)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.

---

19    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found enum `Result`
20    |
21    = note: expected type `i32`
-               found enum `Result<{integer}, _>`
+               found type `Result<{integer}, _>`
24 error: aborting due to 2 previous errors
25 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/type-mismatch-signature-deduction/type-mismatch-signature-deduction.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/type-mismatch-signature-deduction.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/type-mismatch-signature-deduction" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/type-mismatch-signature-deduction/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs:13:9
   |
LL |         5 //~ ERROR mismatched types [E0308]
   |         ^ expected enum `Result`, found integer
   |
   = note: expected type `Result<{integer}, _>`
              found type `{integer}`
note: return type inferred to be `Result<{integer}, _>` here
   |
LL |             return Ok(6);
   |                    ^^^^^


error[E0271]: type mismatch resolving `<[generator@/checkout/src/test/ui/generator/type-mismatch-signature-deduction.rs:6:5: 14:6] as Generator>::Return == i32`
   |
   |
LL | fn foo() -> impl Generator<Return = i32> { //~ ERROR type mismatch
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found enum `Result`
   = note: expected type `i32`
   = note: expected type `i32`
              found type `Result<{integer}, _>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0308.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-68656-unsized-values.rs stdout ----
diff of stderr:

6 LL |     type Item<'a> = T;
7    |                     ^ expected type parameter `T`, found associated type
-    = note: expected type parameter `T`
-              found associated type `<T as Deref>::Target`
+    = note: expected type `T`
+               found type `<T as Deref>::Target`
+               found type `<T as Deref>::Target`
11 note: required by a bound in `UnsafeCopy::Item`
13    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/issue-68656-unsized-values.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-68656-unsized-values.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-68656-unsized-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-68656-unsized-values/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<T as Deref>::Target == T`
   |
   |
LL | impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {
   |      - this type parameter
LL |     type Item<'a> = T;
   |                     ^ expected type parameter `T`, found associated type
   = note: expected type `T`
              found type `<T as Deref>::Target`
              found type `<T as Deref>::Target`
note: required by a bound in `UnsafeCopy::Item`
   |
   |
LL |     type Item<'a>: std::ops::Deref<Target = T>;
   |                                    ^^^^^^^^^^ required by this bound in `UnsafeCopy::Item`
   |
   |
LL | impl<T: Copy + std::ops::Deref + Deref<Target = T>> UnsafeCopy<T> for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/generic-associated-types/issue-74684-2.rs stdout ----
diff of stderr:

4 LL |     bug(Box::new(x));
5    |     ^^^ type mismatch resolving `<{integer} as Fun>::F<'_> == [u8]`
- note: expected this to be `[u8]`
- note: expected this to be `[u8]`
+ note: expected slice `[u8]`, found `i32`
9    |
9    |
10 LL |     type F<'a> = i32;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-2/issue-74684-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-74684-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-74684-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-74684-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<{integer} as Fun>::F<'_> == [u8]`
   |
   |
LL |     bug(Box::new(x));
   |     ^^^ type mismatch resolving `<{integer} as Fun>::F<'_> == [u8]`
   |
note: expected slice `[u8]`, found `i32`
   |
   |
LL |     type F<'a> = i32;
note: required by a bound in `bug`
  --> /checkout/src/test/ui/generic-associated-types/issue-74684-2.rs:13:28
   |
   |
LL | fn bug<'a, T: ?Sized + Fun<F<'a> = [u8]>>(t: Box<T>) -> &'static T::F<'a> {
   |                            ^^^^^^^^^^^^ required by this bound in `bug`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] ui/hrtb/issue-62203-hrtb-ice.rs stdout ----
diff of stderr:

4 LL |     let v = Unit2.m(
5    |                   ^ type mismatch resolving `for<'r> <L<[closure@$DIR/issue-62203-hrtb-ice.rs:42:17: 42:39]> as T0<'r, (&'r u8,)>>::O == <_ as Ty<'r>>::V`
6    |
- note: expected this to be `<_ as Ty<'_>>::V`
+ note: expected associated type, found struct `Unit4`
9    |
9    |
10 LL |     type O = T::Output;
11    |              ^^^^^^^^^
11    |              ^^^^^^^^^
-    = note: expected associated type `<_ as Ty<'_>>::V`
-                        found struct `Unit4`
+    = note: expected type `<_ as Ty<'_>>::V`
+               found type `Unit4`
14    = help: consider constraining the associated type `<_ as Ty<'_>>::V` to `Unit4` or calling a method that returns `<_ as Ty<'_>>::V`
16 note: required by a bound in `T1::m`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-62203-hrtb-ice/issue-62203-hrtb-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-62203-hrtb-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-62203-hrtb-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-62203-hrtb-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `for<'r> <L<[closure@/checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:42:17: 42:39]> as T0<'r, (&'r u8,)>>::O == <_ as Ty<'r>>::V`
   |
   |
LL |     let v = Unit2.m(
   |                   ^ type mismatch resolving `for<'r> <L<[closure@/checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:42:17: 42:39]> as T0<'r, (&'r u8,)>>::O == <_ as Ty<'r>>::V`
note: expected associated type, found struct `Unit4`
  --> /checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:21:14
   |
   |
LL |     type O = T::Output;
   |              ^^^^^^^^^
   = note: expected type `<_ as Ty<'_>>::V`
              found type `Unit4`
   = help: consider constraining the associated type `<_ as Ty<'_>>::V` to `Unit4` or calling a method that returns `<_ as Ty<'_>>::V`
note: required by a bound in `T1::m`
  --> /checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:27:51
   |
   |
LL |     fn m<'a, B: Ty<'a>, F>(&self, f: F) -> Unit1
   |        - required by a bound in this
LL |     where
LL |         F: for<'r> T0<'r, (<Self as Ty<'r>>::V,), O = <B as Ty<'r>>::V>,
   |                                                   ^^^^^^^^^^^^^^^^^^^^ required by this bound in `T1::m`

error[E0271]: type mismatch resolving `for<'r> <[closure@/checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:42:17: 42:39] as FnOnce<((&'r u8,),)>>::Output == Unit3`
   |
   |
LL |       let v = Unit2.m(
   |                     - required by a bound introduced by this call
LL |           //~^ ERROR type mismatch
LL | /         L {
LL | |         //~^ ERROR type mismatch
LL | |             f : |x| { drop(x); Unit4 }
LL | |         });
   | |_________^ expected struct `Unit3`, found struct `Unit4`
   |
note: required because of the requirements on the impl of `for<'r> T0<'r, (&'r u8,)>` for `L<[closure@/checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:42:17: 42:39]>`
   |
   |
LL | impl<'a, A, T> T0<'a, A> for L<T>
note: required by a bound in `T1::m`
  --> /checkout/src/test/ui/hrtb/issue-62203-hrtb-ice.rs:27:12
   |
   |
LL |     fn m<'a, B: Ty<'a>, F>(&self, f: F) -> Unit1
   |        - required by a bound in this
LL |     where
LL |         F: for<'r> T0<'r, (<Self as Ty<'r>>::V,), O = <B as Ty<'r>>::V>,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `T1::m`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/impl-trait/bound-normalization-fail.rs stdout ----
diff of stderr:

4 LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output = T::Assoc> {
5    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
6    |
- note: expected this to be `<T as impl_trait::Trait>::Assoc`
+ note: expected associated type, found `()`
9    |
10 LL |     type Output = T;

11    |                   ^
11    |                   ^
-    = note: expected associated type `<T as impl_trait::Trait>::Assoc`
-                     found unit type `()`
+    = note: expected type `<T as impl_trait::Trait>::Assoc`
+               found type `()`
14 help: consider constraining the associated type `<T as impl_trait::Trait>::Assoc` to `()`
15    |
16 LL |     fn foo_fail<T: Trait<Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {

28 LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output = T::Assoc> {
29    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
30    |
- note: expected this to be `<T as lifetimes::Trait<'static>>::Assoc`
+ note: expected associated type, found `()`
33    |
34 LL |     type Output = T;

35    |                   ^
35    |                   ^
-    = note: expected associated type `<T as lifetimes::Trait<'static>>::Assoc`
-                     found unit type `()`
+    = note: expected type `<T as lifetimes::Trait<'static>>::Assoc`
+               found type `()`
38 help: consider constraining the associated type `<T as lifetimes::Trait<'static>>::Assoc` to `()`
39    |
40 LL |     fn foo2_fail<'a, T: Trait<'a, Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/bound-normalization-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/bound-normalization-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bound-normalization-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bound-normalization-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
   |
   |
LL |     fn foo_fail<T: Trait>() -> impl FooLike<Output = T::Assoc> {
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo<()> as FooLike>::Output == <T as impl_trait::Trait>::Assoc`
note: expected associated type, found `()`
  --> /checkout/src/test/ui/impl-trait/bound-normalization-fail.rs:14:19
   |
LL |     type Output = T;
LL |     type Output = T;
   |                   ^
   = note: expected type `<T as impl_trait::Trait>::Assoc`
              found type `()`
help: consider constraining the associated type `<T as impl_trait::Trait>::Assoc` to `()`
   |
LL |     fn foo_fail<T: Trait<Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {


error[E0760]: `impl Trait` return type cannot contain a projection or `Self` that references lifetimes from a parent scope
   |
   |
LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output = T::Assoc> {


error[E0271]: type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
   |
   |
LL |     fn foo2_fail<'a, T: Trait<'a>>() -> impl FooLike<Output = T::Assoc> {
   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Foo<()> as FooLike>::Output == <T as lifetimes::Trait<'static>>::Assoc`
note: expected associated type, found `()`
  --> /checkout/src/test/ui/impl-trait/bound-normalization-fail.rs:14:19
   |
LL |     type Output = T;
LL |     type Output = T;
   |                   ^
   = note: expected type `<T as lifetimes::Trait<'static>>::Assoc`
              found type `()`
help: consider constraining the associated type `<T as lifetimes::Trait<'static>>::Assoc` to `()`
   |
LL |     fn foo2_fail<'a, T: Trait<'a, Assoc = ()>>() -> impl FooLike<Output = T::Assoc> {

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0271, E0760.
---

---- [ui] ui/impl-trait/issues/issue-70877.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<Bar as Iterator>::Item == Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
3    |
3    |
- LL | type FooRet = impl std::fmt::Debug;
- ...
- ...
7 LL | type Foo = impl Iterator<Item = FooItem>;
8    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Bar as Iterator>::Item == Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`


- note: expected this to be `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
+ note: expected enum `Option`, found opaque type
12    |
12    |
13 LL |     type Item = FooItem;
14    |                 ^^^^^^^
14    |                 ^^^^^^^
-    = note: expected struct `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
-               found struct `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> impl Debug + 'static)>`
+    = note: expected type `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
+               found type `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> impl Debug + 'static)>`
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-70877/issue-70877.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-70877.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-70877.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-70877" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-70877/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Bar as Iterator>::Item == Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
   |
   |
LL | type Foo = impl Iterator<Item = FooItem>; //~ ERROR: type mismatch
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<Bar as Iterator>::Item == Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
note: expected enum `Option`, found opaque type
  --> /checkout/src/test/ui/impl-trait/issues/issue-70877.rs:13:17
   |
   |
LL |     type Item = FooItem;
   |                 ^^^^^^^
   = note: expected type `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> Option<String> + 'static)>`
              found type `Box<(dyn for<'r> Fn(&'r (dyn ToString + 'r)) -> impl Debug + 'static)>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/impl-trait/projection-mismatch-in-impl-where-clause.rs stdout ----
diff of stderr:

4 LL | fn test() -> impl Test {
5    |              ^^^^^^^^^ type mismatch resolving `<() as Super>::Assoc == ()`
- note: expected this to be `()`
+ note: expected `()`, found `u8`
8   --> $DIR/projection-mismatch-in-impl-where-clause.rs:6:18
9    |
---
To only update this specific test, also pass `--test-args impl-trait/projection-mismatch-in-impl-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/projection-mismatch-in-impl-where-clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/projection-mismatch-in-impl-where-clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<() as Super>::Assoc == ()`
   |
LL | fn test() -> impl Test {
LL | fn test() -> impl Test {
   |              ^^^^^^^^^ type mismatch resolving `<() as Super>::Assoc == ()`
note: expected `()`, found `u8`
  --> /checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs:6:18
   |
LL |     type Assoc = u8;
LL |     type Assoc = u8;
   |                  ^^
note: required because of the requirements on the impl of `Test` for `()`
  --> /checkout/src/test/ui/impl-trait/projection-mismatch-in-impl-where-clause.rs:11:9
   |
LL | impl<T> Test for T where T: Super<Assoc = ()> {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
---
To only update this specific test, also pass `--test-args issues/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31173.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31173/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]> as Iterator>::Item == &_`
   |
LL |         .cloned()
   |          ^^^^^^ expected reference, found `u8`
   |
   |
   = note: expected type `&_`
              found type `u8`
note: required by a bound in `cloned`
  --> /checkout/library/core/src/iter/traits/iterator.rs:3053:32
   |
LL |         Self: Sized + Iterator<Item = &'a T>,
   |                                ^^^^^^^^^^^^ required by this bound in `cloned`

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>`, but its trait bounds were not satisfied
   |
   |
LL |         .collect(); //~ ERROR the method
   |          ^^^^^^^ method cannot be called on `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/iter/adapters/cloned.rs:17:1
   |
   |
LL | pub struct Cloned<I> {
   | -------------------- doesn't satisfy `_: Iterator`
  ::: /checkout/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
LL | pub struct TakeWhile<I, P> {
   | -------------------------- doesn't satisfy `<_ as Iterator>::Item = &_`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@/checkout/src/test/ui/issues/issue-31173.rs:6:39: 9:6]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
For more information about an error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/issues/issue-33941.rs stdout ----
diff of stderr:

4 LL |     for _ in HashMap::new().iter().cloned() {}
6    |
-    = note: expected reference `&_`
-    = note: expected reference `&_`
-                   found tuple `(&_, &_)`
+    = note: expected type `&_`
+               found type `(&_, &_)`
9 note: required by a bound in `cloned`
10   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL


18 LL |     for _ in HashMap::new().iter().cloned() {}
20    |
20    |
-    = note:  expected tuple `(&_, &_)`
-            found reference `&_`
+    = note: expected type `(&_, &_)`
+               found type `&_`
23    = note: required because of the requirements on the impl of `Iterator` for `Cloned<std::collections::hash_map::Iter<'_, _, _>>`
24    = note: required because of the requirements on the impl of `IntoIterator` for `Cloned<std::collections::hash_map::Iter<'_, _, _>>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941/issue-33941.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941/issue-33941.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-33941.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-33941.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-33941/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<std::collections::hash_map::Iter<'_, _, _> as Iterator>::Item == &_`
   |
   |
LL |     for _ in HashMap::new().iter().cloned() {} //~ ERROR type mismatch
   |
   = note: expected type `&_`
   = note: expected type `&_`
              found type `(&_, &_)`
note: required by a bound in `cloned`
   |
   |
LL |         Self: Sized + Iterator<Item = &'a T>,
   |                                ^^^^^^^^^^^^ required by this bound in `cloned`

error[E0271]: type mismatch resolving `<std::collections::hash_map::Iter<'_, _, _> as Iterator>::Item == &_`
   |
   |
LL |     for _ in HashMap::new().iter().cloned() {} //~ ERROR type mismatch
   |
   |
   = note: expected type `(&_, &_)`
              found type `&_`
   = note: required because of the requirements on the impl of `Iterator` for `Cloned<std::collections::hash_map::Iter<'_, _, _>>`
   = note: required because of the requirements on the impl of `IntoIterator` for `Cloned<std::collections::hash_map::Iter<'_, _, _>>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/issues/issue-39970.rs stdout ----
diff of stderr:

4 LL |     <() as Visit>::visit();
5    |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `for<'a> <() as Array<'a>>::Element == ()`
- note: expected this to be `()`
+ note: expected `()`, found `&()`
8   --> $DIR/issue-39970.rs:10:20
9    |
9    |
10 LL |     type Element = &'a ();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970/issue-39970.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-39970.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39970.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39970/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `for<'a> <() as Array<'a>>::Element == ()`
   |
   |
LL |     <() as Visit>::visit();
   |     ^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `for<'a> <() as Array<'a>>::Element == ()`
note: expected `()`, found `&()`
  --> /checkout/src/test/ui/issues/issue-39970.rs:10:20
   |
   |
LL |     type Element = &'a ();
   |                    ^^^^^^
note: required because of the requirements on the impl of `Visit` for `()`
   |
   |
LL | impl Visit for () where

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
For more information about this error, try `rustc --explain E0271`.

------------------------------------------


---- [ui] ui/issues/issue-67039-unsound-pin-partialeq.rs stdout ----
diff of stderr:

4 LL |     let _ = Pin::new(Apple) == Rc::pin(Apple);
5    |                             ^^ expected struct `Apple`, found struct `Rc`
-    = note: expected struct `Apple`
-    = note: expected struct `Apple`
-               found struct `Rc<Apple>`
+    = note: expected type `Apple`
+               found type `Rc<Apple>`
9    = note: required because of the requirements on the impl of `PartialEq<Pin<Rc<Apple>>>` for `Pin<Apple>`
11 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq/issue-67039-unsound-pin-partialeq.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-67039-unsound-pin-partialeq.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-67039-unsound-pin-partialeq.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67039-unsound-pin-partialeq/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<Rc<Apple> as Deref>::Target == Rc<Apple>`
  --> /checkout/src/test/ui/issues/issue-67039-unsound-pin-partialeq.rs:25:29
   |
LL |     let _ = Pin::new(Apple) == Rc::pin(Apple);
   |                             ^^ expected struct `Apple`, found struct `Rc`
   |
   = note: expected type `Apple`
              found type `Rc<Apple>`
   = note: required because of the requirements on the impl of `PartialEq<Pin<Rc<Apple>>>` for `Pin<Apple>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.

---
15 error: aborting due to previous error


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback/fallback-closure-wrap.fallback.stderr
To only update this specific test, also pass `--test-args never_type/fallback-closure-wrap.rs`


error in revision `fallback`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/fallback-closure-wrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "fallback" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/fallback-closure-wrap.fallback/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<[closure@/checkout/src/test/ui/never_type/fallback-closure-wrap.rs:18:40: 21:6] as FnOnce<()>>::Output == ()`
   |
   |
LL |       let error = Closure::wrap(Box::new(move || {
   |  _______________________________^
LL | |         //[fallback]~^ ERROR type mismatch resolving
LL | |         panic!("Can't connect to server.");
LL | |     }) as Box<dyn FnMut()>);
   | |______^ expected `()`, found `!`
   = note: expected type `()`
              found type `!`
   = note: required for the cast to the object type `dyn FnMut()`

---

---- [ui] ui/traits/associated_type_bound/check-trait-object-bounds-6.rs stdout ----
diff of stderr:

4 LL |     is_obj(x)
5    |     ^^^^^^ type mismatch resolving `<i32 as Is>::T == i64`
- note: expected this to be `i64`
+ note: expected `i64`, found `i32`
8   --> $DIR/check-trait-object-bounds-6.rs:9:14
9    |
9    |
10 LL |     type T = U;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-6/check-trait-object-bounds-6.stderr
To only update this specific test, also pass `--test-args traits/associated_type_bound/check-trait-object-bounds-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<i32 as Is>::T == i64`
   |
LL |     is_obj(x)
LL |     is_obj(x)
   |     ^^^^^^ type mismatch resolving `<i32 as Is>::T == i64`
note: expected `i64`, found `i32`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-6.rs:9:14
   |
LL |     type T = U;
LL |     type T = U;
   |              ^
note: required by a bound in `is_obj`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-6.rs:17:23
   |
LL | fn is_obj<T: ?Sized + Obj>(_: &T) {}
   |                       ^^^ required by this bound in `is_obj`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/traits/associated_type_bound/check-trait-object-bounds-5.rs stdout ----
diff of stderr:

4 LL |     is_obj(x)
5    |     ^^^^^^ type mismatch resolving `<i32 as Is>::T == i64`
- note: expected this to be `i64`
+ note: expected `i64`, found `i32`
8   --> $DIR/check-trait-object-bounds-5.rs:9:14
9    |
9    |
10 LL |     type T = U;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-5/check-trait-object-bounds-5.stderr
To only update this specific test, also pass `--test-args traits/associated_type_bound/check-trait-object-bounds-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<i32 as Is>::T == i64`
   |
LL |     is_obj(x)
LL |     is_obj(x)
   |     ^^^^^^ type mismatch resolving `<i32 as Is>::T == i64`
note: expected `i64`, found `i32`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-5.rs:9:14
   |
LL |     type T = U;
LL |     type T = U;
   |              ^
note: required by a bound in `is_obj`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-5.rs:20:23
   |
LL | fn is_obj<T: ?Sized + Obj>(_: &T) {}
   |                       ^^^ required by this bound in `is_obj`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.


------------------------------------------


---- [ui] ui/type-alias-impl-trait/issue-63355.rs stdout ----
diff of stderr:

1 error[E0271]: type mismatch resolving `<() as Bar>::Foo == ()`
3    |
- LL | pub type FooImpl = impl Foo;
-    |                    -------- the found opaque type
-    |                    -------- the found opaque type
6 LL | pub type BarImpl = impl Bar<Foo = FooImpl>;
7    |                    ^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<() as Bar>::Foo == ()`

- note: expected this to be `()`
+ note: expected `()`, found opaque type
10   --> $DIR/issue-63355.rs:24:16
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-63355.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-63355.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63355" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-63355/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<() as Bar>::Foo == ()`
   |
   |
LL | pub type BarImpl = impl Bar<Foo = FooImpl>;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^ type mismatch resolving `<() as Bar>::Foo == ()`
note: expected `()`, found opaque type
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-63355.rs:24:16
   |
LL |     type Foo = FooImpl;
---
---- [ui] ui/type-alias-impl-trait/issue-89686.rs stdout ----
diff of stderr:

3    |
4 LL | type G<'a, T> = impl Future<Output = ()>;
- ...
- ...
- LL |         async move { self.f().await }
-    |                    ------------------ the found `async` block
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |
-    = note:    expected unit type `()`
-    = note:    expected unit type `()`
-            found associated type `<impl Future<Output = [async output]> as Future>::Output`
+    = note: expected type `()`
+               found type `<impl Future<Output = [async output]> as Future>::Output`
17    = help: consider constraining the associated type `<impl Future<Output = [async output]> as Future>::Output` to `()`
19 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686/issue-89686.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-89686.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-89686.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0271]: type mismatch resolving `<impl Future<Output = [async output]> as Future>::Output == ()`
   |
   |
LL | type G<'a, T> = impl Future<Output = ()>;
   |
   = note: expected type `()`
   = note: expected type `()`
              found type `<impl Future<Output = [async output]> as Future>::Output`
   = help: consider constraining the associated type `<impl Future<Output = [async output]> as Future>::Output` to `()`

error[E0277]: the trait bound `T: Trait` is not satisfied
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-89686.rs:7:17
   |
   |
LL | type G<'a, T> = impl Future<Output = ()>;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | type G<'a, T: Trait> = impl Future<Output = ()>;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0277.
---
test result: FAILED. 12407 passed; 30 failed; 121 ignored; 0 measured; 0 filtered out; finished in 119.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:44
