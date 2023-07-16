plain
.....................................................................i.................. 9416/13930
........................................................................................ 9504/13930
................................................................................i....... 9592/13930
........................................................................................ 9680/13930
.........................................................F....F.FF..FF.................. 9768/13930
........................................................................................ 9944/13930
........................................................................................ 10032/13930
........................................................................................ 10120/13930
........................................................................................ 10208/13930
---
....i.ii................................................................................ 13904/13930
..........................
failures:

---- [ui] src/test/ui/parser/suggest_misplaced_generics/enum.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the enum name
8    |
- LL | enum Foo<T> { Variant(T) }
-    |         ~~~
+ LL | enum<T> Foo<T> { Variant(T) }
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- enum Foo<T> { Variant(T) }
+ enum<T> Foo<T> { Variant(T) }
6 //~^ ERROR expected identifier, found `<`
7 //~| HELP place the generic parameter name after the enum name
8 //~| SUGGESTION  Foo<T>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/enum/enum.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/enum.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/enum" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/enum/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/enum.rs:5:5
   |
LL | enum<T> Foo { Variant(T) }
   |
   |
help: place the generic parameter name after the enum name
   |
LL | enum<T> Foo<T> { Variant(T) }

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the fn name
8    |
- LL | fn f<'a, B: 'a + std::ops::Add<Output = u32>>(_x: B) { }
-    |     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
+ LL | fn<'a, B: 'a + std::ops::Add<Output = u32>> f<'a, B: 'a + std::ops::Add<Output = u32>>(_x: B) { }
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- fn f<'a, B: 'a + std::ops::Add<Output = u32>>(_x: B) { }
+ fn<'a, B: 'a + std::ops::Add<Output = u32>> f<'a, B: 'a + std::ops::Add<Output = u32>>(_x: B) { }
6 //~^ ERROR expected identifier, found `<`
7 //~| HELP place the generic parameter name after the fn name
8 //~| SUGGESTION  f<'a, B: 'a + std::ops::Add<Output = u32>>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-complex-generics/fn-complex-generics.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/fn-complex-generics.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-complex-generics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-complex-generics/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/fn-complex-generics.rs:5:3
   |
LL | fn<'a, B: 'a + std::ops::Add<Output = u32>> f(_x: B) { }
   |
   |
help: place the generic parameter name after the fn name
   |
LL | fn<'a, B: 'a + std::ops::Add<Output = u32>> f<'a, B: 'a + std::ops::Add<Output = u32>>(_x: B) { }

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/suggest_misplaced_generics/fn-simple.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the fn name
8    |
- LL | fn id<T>(x: T) -> T { x }
-    |      ~~~
+ LL | fn<T> id<T>(x: T) -> T { x }
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- fn id<T>(x: T) -> T { x }
+ fn<T> id<T>(x: T) -> T { x }
6 //~^ ERROR expected identifier, found `<`
7 //~| HELP place the generic parameter name after the fn name
8 //~| SUGGESTION  id<T>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-simple/fn-simple.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/fn-simple.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/fn-simple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-simple" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/fn-simple/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/fn-simple.rs:5:3
   |
LL | fn<T> id(x: T) -> T { x }
   |
   |
help: place the generic parameter name after the fn name
   |
LL | fn<T> id<T>(x: T) -> T { x }

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/suggest_misplaced_generics/struct.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the struct name
8    |
- LL | struct Foo<T> { x: T }
-    |           ~~~
+ LL | struct<T> Foo<T> { x: T }
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- struct Foo<T> { x: T }
+ struct<T> Foo<T> { x: T }
6 //~^ ERROR expected identifier, found `<`
7 //~| HELP place the generic parameter name after the struct name
8 //~| SUGGESTION  Foo<T>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/struct/struct.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/struct.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/struct/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/struct.rs:5:7
   |
LL | struct<T> Foo { x: T }
   |
   |
help: place the generic parameter name after the struct name
   |
LL | struct<T> Foo<T> { x: T }

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/suggest_misplaced_generics/type.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the type name
- LL | type Foo<T> = T;
-    |         ~~~
-    |         ~~~
+ LL | type<T> Foo<T> = T;
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- type Foo<T> = T;
+ type<T> Foo<T> = T;
6 //~^ ERROR expected identifier, found `<`
7 //~| HELP place the generic parameter name after the type name
8 //~| SUGGESTION  Foo<T>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/type/type.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/type.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/type/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/type.rs:5:5
   |
LL | type<T> Foo = T;
   |
   |
help: place the generic parameter name after the type name
   |
LL | type<T> Foo<T> = T;

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/parser/suggest_misplaced_generics/trait.rs stdout ----

6    |
6    |
7 help: place the generic parameter name after the trait name
8    |
- LL | trait Foo<T> {
-    |          ~~~
+ LL | trait<T> Foo<T> {
11 
12 error: aborting due to previous error
13 

---

2 // run-rustfix
3 
4 #[allow(unused)]
- trait Foo<T> {
+ trait<T> Foo<T> {
6     //~^ ERROR expected identifier, found `<`
7     //~| HELP place the generic parameter name after the trait name
8     //~| SUGGESTION  Foo<T>

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/trait/trait.fixed
To only update this specific test, also pass `--test-args parser/suggest_misplaced_generics/trait.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/suggest_misplaced_generics/trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/suggest_misplaced_generics/trait/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `<`
  --> /checkout/src/test/ui/parser/suggest_misplaced_generics/trait.rs:5:6
   |
LL | trait<T> Foo {
   |
   |
help: place the generic parameter name after the trait name
   |
LL | trait<T> Foo<T> {

error: aborting due to previous error
------------------------------------------

