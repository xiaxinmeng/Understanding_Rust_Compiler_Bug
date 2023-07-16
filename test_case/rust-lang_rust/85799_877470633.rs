plain
.................................................................................................... 1400/12071
.......................................................................i.....i...............i...... 1500/12071
.................................................................................................... 1600/12071
.........................................................................................i.......... 1700/12071
..............................................................F.F................................... 1800/12071
.................................i.................................................................. 2000/12071
.................................................................................................... 2100/12071
.................................................................................................... 2200/12071
.................................................................................................... 2300/12071
---
.................................................................................................... 11500/12071
.................................................................................................... 11600/12071
...............................................................................................F.... 11700/12071
.................................................................................................... 11800/12071
.........................FFF..FF...F.FF..FF..F..F................................................... 11900/12071
.......................................................................
failures:

---- [ui] ui/const-generics/const-argument-if-length.rs#min stdout ----
---- [ui] ui/const-generics/const-argument-if-length.rs#min stdout ----
diff of stderr:

17    |
18    = note: only the last field of a struct may have a dynamically sized type
19    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | pub struct AtLeastByte<T> {
-    |                        --
24 help: borrowed types always have a statically known size
26 LL |     value: &T,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.min/const-argument-if-length.min.stderr
To only update this specific test, also pass `--test-args const-generics/const-argument-if-length.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-argument-if-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |     pad: [u8; is_zst::<T>()],
   |                        ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:16:12
   |
   |
LL | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     value: &T,
   |            ^
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ^^^^ ^

error: aborting due to 2 previous errors
---
---- [ui] ui/const-generics/const-argument-if-length.rs#full stdout ----
diff of stderr:

11    |
12 LL | pub const fn size_of<T>() -> usize {
13    |                      ^ required by this bound in `std::mem::size_of`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | pub const fn is_zst<T>() -> usize {
18 
19 error[E0277]: the size for values of type `T` cannot be known at compilation time
20   --> $DIR/const-argument-if-length.rs:16:12


26    |
27    = note: only the last field of a struct may have a dynamically sized type
28    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | pub struct AtLeastByte<T> {
-    |                        --
33 help: borrowed types always have a statically known size
35 LL |     value: &T,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/const-argument-if-length.full.stderr
To only update this specific test, also pass `--test-args const-generics/const-argument-if-length.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-argument-if-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:7:28
   |
LL | pub const fn is_zst<T: ?Sized>() -> usize {
   |                     - this type parameter needs to be `std::marker::Sized`
LL |     if std::mem::size_of::<T>() == 0 {
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/library/core/src/mem/mod.rs:301:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:16:12
   |
   |
LL | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     value: &T,
   |            ^
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ^^^^ ^

error: aborting due to 2 previous errors
---
diff of stderr:

7    |                       ^ doesn't have a size known at compile-time
8    |
9    = note: required for the cast to the object type `dyn Foo`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn test1<T: Foo>(t: &T) {
14 
15 error[E0277]: the size for values of type `T` cannot be known at compilation time
16   --> $DIR/dst-object-from-unsized-type.rs:13:23


21    |                       ^ doesn't have a size known at compile-time
22    |
23    = note: required for the cast to the object type `dyn Foo`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn test2<T: Foo>(t: &T) {
28 
29 error[E0277]: the size for values of type `str` cannot be known at compilation time
30   --> $DIR/dst-object-from-unsized-type.rs:18:28



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-object-from-unsized-type/dst-object-from-unsized-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dst/dst-object-from-unsized-type.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dst/dst-object-from-unsized-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-object-from-unsized-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dst/dst-object-from-unsized-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/dst/dst-object-from-unsized-type.rs:8:23
   |
LL | fn test1<T: ?Sized + Foo>(t: &T) {
   |          - this type parameter needs to be `std::marker::Sized`
LL |     let u: &dyn Foo = t;
   |
   = note: required for the cast to the object type `dyn Foo`

error[E0277]: the size for values of type `T` cannot be known at compilation time
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/dst/dst-object-from-unsized-type.rs:13:23
   |
LL | fn test2<T: ?Sized + Foo>(t: &T) {
   |          - this type parameter needs to be `std::marker::Sized`
LL |     let v: &dyn Foo = t as &dyn Foo;
   |
   = note: required for the cast to the object type `dyn Foo`

error[E0277]: the size for values of type `str` cannot be known at compilation time
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/dst/dst-object-from-unsized-type.rs:18:28
   |
LL |     let _: &[&dyn Foo] = &["hi"];
   |
   = help: the trait `Sized` is not implemented for `str`
   = note: required for the cast to the object type `dyn Foo`


error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/src/test/ui/dst/dst-object-from-unsized-type.rs:23:23
   |
LL |     let _: &dyn Foo = x as &dyn Foo;
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: required for the cast to the object type `dyn Foo`

---
---- [ui] ui/packed/issue-27060-2.rs stdout ----
diff of stderr:

8    |
9    = note: the last field of a packed struct may only have a dynamically sized type if it does not need drop to be run
10    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | pub struct Bad<T> {
-    |                --
15 help: borrowed types always have a statically known size
17 LL |     data: &T,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060-2/issue-27060-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args packed/issue-27060-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/packed/issue-27060-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/packed/issue-27060-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/packed/issue-27060-2.rs:3:11
   |
LL | pub struct Bad<T: ?Sized> {
   |                - this type parameter needs to be `std::marker::Sized`
LL |     data: T, //~ ERROR the size for values of type
   |
   |
   = note: the last field of a packed struct may only have a dynamically sized type if it does not need drop to be run
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     data: &T, //~ ERROR the size for values of type
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     data: Box<T>, //~ ERROR the size for values of type
   |           ^^^^ ^
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/suggestions/adt-param-with-implicit-sized-bound.rs stdout ----
diff of stderr:

18    |          ^  - ...if indirection were used here: `Box<T>`
19    |          |
20    |          this could be changed to `T: ?Sized`...
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | struct Struct5<T>{
25 
26 error[E0277]: the size for values of type `Self` cannot be known at compilation time
27   --> $DIR/adt-param-with-implicit-sized-bound.rs:2:19

---
To only update this specific test, also pass `--test-args suggestions/adt-param-with-implicit-sized-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/adt-param-with-implicit-sized-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/adt-param-with-implicit-sized-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:25:9
   |
LL | struct Struct5<T: ?Sized>{
   |                - this type parameter needs to be `std::marker::Sized`
LL |     _t: X<T>, //~ ERROR E0277
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:18:10
   |
   |
LL | struct X<T>(T);
   |          ^ required by this bound in `X`
help: you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`
   |
   |
LL | struct X<T>(T);
   |          ^  - ...if indirection were used here: `Box<T>`
   |          |
   |          this could be changed to `T: ?Sized`...
error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:2:19
   |
   |
LL |     fn func1() -> Struct1<Self>; //~ ERROR E0277
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:8:16
   |
   |
LL | struct Struct1<T>{
   |                ^ required by this bound in `Struct1`
help: consider further restricting `Self`
   |
LL |     fn func1() -> Struct1<Self> where Self: Sized; //~ ERROR E0277
   |                                 ^^^^^^^^^^^^^^^^^
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Struct1<T: ?Sized>{

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:3:23
   |
   |
LL |     fn func2<'a>() -> Struct2<'a, Self>; //~ ERROR E0277
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:11:20
   |
   |
LL | struct Struct2<'a, T>{
   |                    ^ required by this bound in `Struct2`
help: consider further restricting `Self`
   |
LL |     fn func2<'a>() -> Struct2<'a, Self> where Self: Sized; //~ ERROR E0277
   |                                         ^^^^^^^^^^^^^^^^^
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Struct2<'a, T: ?Sized>{

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:4:19
   |
   |
LL |     fn func3() -> Struct3<Self>; //~ ERROR E0277
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:14:16
   |
   |
LL | struct Struct3<T>{
   |                ^ required by this bound in `Struct3`
help: you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`
   |
   |
LL | struct Struct3<T>{
   |                ^ this could be changed to `T: ?Sized`...
LL |     _t: T,
   |         - ...if indirection were used here: `Box<T>`
help: consider further restricting `Self`
   |
LL |     fn func3() -> Struct3<Self> where Self: Sized; //~ ERROR E0277

error[E0277]: the size for values of type `Self` cannot be known at compilation time
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:5:19
   |
   |
LL |     fn func4() -> Struct4<Self>; //~ ERROR E0277
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/suggestions/adt-param-with-implicit-sized-bound.rs:20:16
   |
   |
LL | struct Struct4<T>{
   |                ^ required by this bound in `Struct4`
help: consider further restricting `Self`
   |
LL |     fn func4() -> Struct4<Self> where Self: Sized; //~ ERROR E0277
   |                                 ^^^^^^^^^^^^^^^^^
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Struct4<T: ?Sized>{

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/trait-bounds/unsized-bound.rs stdout ----
diff of stderr:

49    |
50 LL | trait Trait<A> {}
51    |             ^ required by this bound in `Trait`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<A, B: ?Sized, C> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
-    |                    --
56 help: consider relaxing the implicit `Sized` restriction
57    |
58 LL | trait Trait<A: ?Sized> {}
81    |         this type parameter needs to be `std::marker::Sized`
82    |
82    |
83    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<A, B, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
88 
89 error[E0277]: the size for values of type `B` cannot be known at compilation time
90   --> $DIR/unsized-bound.rs:10:28


100    |
101 LL | trait Trait2<A> {}
102    |              ^ required by this bound in `Trait2`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<A: ?Sized, B> Trait2<(A, B)> for (A, B) {}
-    |                 --
107 help: consider relaxing the implicit `Sized` restriction
108    |
109 LL | trait Trait2<A: ?Sized> {}
118    |      this type parameter needs to be `std::marker::Sized`
119    |
119    |
120    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<A, B: ?Sized> Trait2<(A, B)> for (A, B) {}
125 
125 
126 error[E0277]: the size for values of type `A` cannot be known at compilation time

158    |
158    |
159 LL | trait Trait4<A> {}
160    |              ^ required by this bound in `Trait4`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<A> Trait4<A> for A {}
-    |      --
165 help: consider relaxing the implicit `Sized` restriction
166    |
167 LL | trait Trait4<A: ?Sized> {}
202    |
202    |
203 LL | trait Trait6<A, B> {}
204    |              ^ required by this bound in `Trait6`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<X, Y> Trait6<X, Y> for X {}
-    |      --
209 help: consider relaxing the implicit `Sized` restriction
210    |
211 LL | trait Trait6<A: ?Sized, B> {}
246    |
246    |
247 LL | trait Trait8<A, B> {}
248    |                 ^ required by this bound in `Trait8`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<X, Y> Trait8<X, Y> for X {}
-    |         --
253 help: consider relaxing the implicit `Sized` restriction
254    |
255 LL | trait Trait8<A, B: ?Sized> {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound/unsized-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args trait-bounds/unsized-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/unsized-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:2:12
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, B: ?Sized, {}
   |         -  ^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:1:13
   |
LL | trait Trait<A> {}
   |             ^ required by this bound in `Trait`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, {}
   |                                                   --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, B: ?Sized, {}
   |      -                       ^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where B: ?Sized, {}

error[E0277]: the size for values of type `C` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:5:31
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |                    -          ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                    this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B, C)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:1:13
   |
LL | trait Trait<A> {}
   |             ^ required by this bound in `Trait`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |      -                                             ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C)  {}

error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:5:52
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |         -                                          ^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:10:28
   |
   |
LL | impl<A: ?Sized, B: ?Sized> Trait2<(A, B)> for (A, B) {}
   |                 -          ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:9:14
   |
LL | trait Trait2<A> {}
   |              ^ required by this bound in `Trait2`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait2<A: ?Sized> {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A: ?Sized, B: ?Sized> Trait2<(A, B)> for (A, B) {}
   |      -                                        ^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type

error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A> Trait3<A> for A where A: ?Sized {}
   |      -  ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:13:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:13:14
   |
LL | trait Trait3<A> {}
   |              ^ required by this bound in `Trait3`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A> Trait3<A> for A  {}
   |                        --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait3<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A: ?Sized> Trait4<A> for A {}
   |      -          ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:16:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:16:14
   |
LL | trait Trait4<A> {}
   |              ^ required by this bound in `Trait4`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait4<A: ?Sized> {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:20:12
   |
   |
LL | impl<X, Y> Trait5<X, Y> for X where X: ?Sized {}
   |      -     ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:19:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:19:14
   |
LL | trait Trait5<A, B> {}
   |              ^ required by this bound in `Trait5`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X, Y> Trait5<X, Y> for X  {}
   |                              --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait5<A: ?Sized, B> {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:23:20
   |
   |
LL | impl<X: ?Sized, Y> Trait6<X, Y> for X {}
   |      -             ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:22:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:22:14
   |
LL | trait Trait6<A, B> {}
   |              ^ required by this bound in `Trait6`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait6<A: ?Sized, B> {}

error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:26:12
   |
   |
LL | impl<X, Y> Trait7<X, Y> for X where Y: ?Sized {}
   |         -  ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:25:17
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:25:17
   |
LL | trait Trait7<A, B> {}
   |                 ^ required by this bound in `Trait7`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X, Y> Trait7<X, Y> for X  {}
   |                              --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait7<A, B: ?Sized> {}

error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:29:20
   |
   |
LL | impl<X, Y: ?Sized> Trait8<X, Y> for X {}
   |         -          ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:28:17
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:28:17
   |
LL | trait Trait8<A, B> {}
   |                 ^ required by this bound in `Trait8`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait8<A, B: ?Sized> {}

error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/traits/suggest-where-clause.rs stdout ----
diff of stderr:

12    |
13 LL | pub const fn size_of<T>() -> usize {
14    |                      ^ required by this bound in `std::mem::size_of`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn check<T: Iterator, U>() {
19 
20 error[E0277]: the size for values of type `U` cannot be known at compilation time
21   --> $DIR/suggest-where-clause.rs:10:5


36    |
37 LL | pub const fn size_of<T>() -> usize {
38    |                      ^ required by this bound in `std::mem::size_of`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn check<T: Iterator, U>() {
43 
43 
44 error[E0277]: the trait bound `u64: From<T>` is not satisfied
45   --> $DIR/suggest-where-clause.rs:15:5

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause/suggest-where-clause.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-where-clause/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `U` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/suggest-where-clause.rs:7:20
   |
LL | fn check<T: Iterator, U: ?Sized>() {
   |                       - this type parameter needs to be `std::marker::Sized`
LL |     // suggest a where-clause, if needed
LL |     mem::size_of::<U>();
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/library/core/src/mem/mod.rs:301:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
error[E0277]: the size for values of type `U` cannot be known at compilation time
  --> /checkout/src/test/ui/traits/suggest-where-clause.rs:10:5
   |
   |
LL | fn check<T: Iterator, U: ?Sized>() {
   |                       - this type parameter needs to be `std::marker::Sized`
...
LL |     mem::size_of::<Misc<U>>();
   |
   |
note: required because it appears within the type `Misc<U>`
   |
   |
LL | struct Misc<T:?Sized>(T);
note: type parameters have an implicit `Sized` obligation
  --> /checkout/library/core/src/mem/mod.rs:301:22
   |
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`

error[E0277]: the trait bound `u64: From<T>` is not satisfied
   |
   |
LL |     <u64 as From<T>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `From<T>` is not implemented for `u64`
   = note: required by `from`
   = note: required by `from`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | fn check<T: Iterator, U: ?Sized>() where u64: From<T> {


error[E0277]: the trait bound `u64: From<<T as Iterator>::Item>` is not satisfied
   |
   |
LL |     <u64 as From<<T as Iterator>::Item>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<<T as Iterator>::Item>` is not implemented for `u64`
   = note: required by `from`
   = note: required by `from`
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | fn check<T: Iterator, U: ?Sized>() where u64: From<<T as Iterator>::Item> {


error[E0277]: the trait bound `Misc<_>: From<T>` is not satisfied
   |
   |
LL |     <Misc<_> as From<T>>::from;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `From<T>` is not implemented for `Misc<_>`
   = note: required by `from`


error[E0277]: the size for values of type `[T]` cannot be known at compilation time
   |
   |
LL |     mem::size_of::<[T]>();
   |
   = help: the trait `Sized` is not implemented for `[T]`
note: type parameters have an implicit `Sized` obligation
  --> /checkout/library/core/src/mem/mod.rs:301:22
  --> /checkout/library/core/src/mem/mod.rs:301:22
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`

error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
   |
   |
LL |     mem::size_of::<[&U]>();
   |
   = help: the trait `Sized` is not implemented for `[&U]`
note: type parameters have an implicit `Sized` obligation
  --> /checkout/library/core/src/mem/mod.rs:301:22
  --> /checkout/library/core/src/mem/mod.rs:301:22
   |
LL | pub const fn size_of<T>() -> usize {
   |                      ^ required by this bound in `std::mem::size_of`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/union/union-sized-field.rs stdout ----
diff of stderr:

8    |
9    = note: no field of a union may have a dynamically sized type
10    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | union Foo<T> {
-    |           --
15 help: borrowed types always have a statically known size
17 LL |     value: &T,

31    |
31    |
32    = note: only the last field of a struct may have a dynamically sized type
33    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | struct Foo2<T> {
-    |             --
38 help: borrowed types always have a statically known size
40 LL |     value: &T,

54    |
54    |
55    = note: no field of an enum variant may have a dynamically sized type
56    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum Foo3<T> {
-    |           --
61 help: borrowed types always have a statically known size
63 LL |     Value(&T),


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-sized-field/union-sized-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args union/union-sized-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-sized-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-sized-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-sized-field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/union/union-sized-field.rs:4:12
   |
LL | union Foo<T: ?Sized> {
   |           - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: no field of a union may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     value: &T,
   |            ^
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ^^^^ ^

error[E0277]: the size for values of type `T` cannot be known at compilation time
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/union/union-sized-field.rs:9:12
   |
LL | struct Foo2<T: ?Sized> {
   |             - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     value: &T,
   |            ^
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ^^^^ ^

error[E0277]: the size for values of type `T` cannot be known at compilation time
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/union/union-sized-field.rs:15:11
   |
LL | enum Foo3<T: ?Sized> {
   |           - this type parameter needs to be `std::marker::Sized`
LL |     Value(T),
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     Value(&T),
   |           ^
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     Value(Box<T>),
   |           ^^^^ ^

error: aborting due to 3 previous errors
---
diff of stderr:

7    |      this type parameter needs to be `std::marker::Sized`
8    |
9    = help: unsized fn params are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f<T>(t: T) {}
-    |      --
14 help: function arguments must have a statically known size, borrowed types always have a known size
15    |
16 LL | fn f<T: ?Sized>(t: &T) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-fn-arg/unsized-fn-arg.stderr
diff of fixed:
diff of fixed:

2 #![crate_type="lib"]
3 #![allow(unused)]
4 
- fn f<T>(t: &T) {}
+ fn f<T: ?Sized>(t: &T) {}
6 //~^ ERROR the size for values of type `T` cannot be known at compilation time


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-fn-arg/unsized-fn-arg.fixed
To only update this specific test, also pass `--test-args unsized/unsized-fn-arg.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-fn-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-fn-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-fn-arg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-fn-arg.rs:5:17
   |
LL | fn f<T: ?Sized>(t: T) {}
   |      -          ^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   = help: unsized fn params are gated as an unstable feature
   = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL | fn f<T: ?Sized>(t: &T) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/unsized/unsized-bare-typaram.rs stdout ----
diff of stderr:

11    |
12 LL | fn bar<T: Sized>() { }
13    |        ^ required by this bound in `bar`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn foo<T>() { bar::<T>() }
18 
19 error: aborting due to previous error
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-bare-typaram/unsized-bare-typaram.stderr
To only update this specific test, also pass `--test-args unsized/unsized-bare-typaram.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-bare-typaram.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-bare-typaram" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-bare-typaram/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-bare-typaram.rs:2:29
   |
LL | fn foo<T: ?Sized>() { bar::<T>() }
   |        -                    ^ doesn't have a size known at compile-time
   |        this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-bare-typaram.rs:1:8
  --> /checkout/src/test/ui/unsized/unsized-bare-typaram.rs:1:8
   |
LL | fn bar<T: Sized>() { }
   |        ^ required by this bound in `bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-enum.rs stdout ----
diff of stderr:

18    |          ^            - ...if indirection were used here: `Box<U>`
19    |          |
20    |          this could be changed to `U: ?Sized`...
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn foo2<T>() { not_sized::<Foo<T>>() }
25 
26 error: aborting due to previous error
27 

---
To only update this specific test, also pass `--test-args unsized/unsized-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum.rs:6:36
   |
LL | fn foo2<T: ?Sized>() { not_sized::<Foo<T>>() }
   |         -                          ^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-enum.rs:4:10
  --> /checkout/src/test/ui/unsized/unsized-enum.rs:4:10
   |
LL | enum Foo<U> { FooSome(U), FooNone }
   |          ^ required by this bound in `Foo`
help: you could relax the implicit `Sized` bound on `U` if it were used through indirection like `&U` or `Box<U>`
   |
   |
LL | enum Foo<U> { FooSome(U), FooNone }
   |          ^            - ...if indirection were used here: `Box<U>`
   |          |
   |          this could be changed to `U: ?Sized`...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-enum2.rs stdout ----
diff of stderr:

9    |
10    = note: no field of an enum variant may have a dynamically sized type
11    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum E<W, X: ?Sized, Y: ?Sized, Z: ?Sized> {
-    |        --
16 help: borrowed types always have a statically known size
17    |
18 LL |     VA(&W),
33    |
33    |
34    = note: no field of an enum variant may have a dynamically sized type
35    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum E<W: ?Sized, X, Y: ?Sized, Z: ?Sized> {
-    |                   --
40 help: borrowed types always have a statically known size
41    |
42 LL |     VB{x: &X},
57    |
57    |
58    = note: no field of an enum variant may have a dynamically sized type
59    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum E<W: ?Sized, X: ?Sized, Y, Z: ?Sized> {
-    |                              --
64 help: borrowed types always have a statically known size
65    |
66 LL |     VC(isize, &Y),
81    |
81    |
82    = note: no field of an enum variant may have a dynamically sized type
83    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z> {
-    |                                         --
88 help: borrowed types always have a statically known size
89    |
90 LL |     VD{u: isize, x: &Z},

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-enum2/unsized-enum2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized-enum2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-enum2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-enum2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-enum2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `W` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:23:8
   |
LL | enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {
   |        - this type parameter needs to be `std::marker::Sized`
LL |     // parameter
LL |     VA(W),
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VA(&W),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VA(Box<W>),
   |        ^^^^ ^
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:25:11
   |
   |
LL | enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {
   |                   - this type parameter needs to be `std::marker::Sized`
...
LL |     VB{x: X},
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VB{x: &X},
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VB{x: Box<X>},
   |           ^^^^ ^
error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:27:15
   |
   |
LL | enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {
   |                              - this type parameter needs to be `std::marker::Sized`
...
LL |     VC(isize, Y),
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VC(isize, &Y),
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VC(isize, Box<Y>),
   |               ^^^^ ^

error[E0277]: the size for values of type `Z` cannot be known at compilation time
   |
   |
LL | enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {
   |                                         - this type parameter needs to be `std::marker::Sized`
...
LL |     VD{u: isize, x: Z},
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VD{u: isize, x: &Z},
   |                     ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VD{u: isize, x: Box<Z>},
   |                     ^^^^ ^
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:33:8
   |
   |
LL |     VE([u8]),
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VE(&[u8]),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VE(Box<[u8]>),
   |        ^^^^    ^
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:35:11
   |
   |
LL |     VF{x: str},
   |
   = help: the trait `Sized` is not implemented for `str`
   = help: the trait `Sized` is not implemented for `str`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VF{x: &str},
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VF{x: Box<str>},
   |           ^^^^   ^

error[E0277]: the size for values of type `[f32]` cannot be known at compilation time
   |
   |
LL |     VG(isize, [f32]),
   |
   = help: the trait `Sized` is not implemented for `[f32]`
   = help: the trait `Sized` is not implemented for `[f32]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VG(isize, &[f32]),
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VG(isize, Box<[f32]>),
   |               ^^^^     ^
error[E0277]: the size for values of type `[u32]` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-enum2.rs:39:21
   |
   |
LL |     VH{u: isize, x: [u32]},
   |
   = help: the trait `Sized` is not implemented for `[u32]`
   = help: the trait `Sized` is not implemented for `[u32]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VH{u: isize, x: &[u32]},
   |                     ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VH{u: isize, x: Box<[u32]>},
   |                     ^^^^     ^

error[E0277]: the size for values of type `(dyn Foo + 'static)` cannot be known at compilation time
   |
   |
LL |     VM(dyn Foo),
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn Foo + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VM(&dyn Foo),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VM(Box<dyn Foo>),
   |        ^^^^       ^

error[E0277]: the size for values of type `(dyn Bar + 'static)` cannot be known at compilation time
   |
   |
LL |     VN{x: dyn Bar},
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn Bar + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VN{x: &dyn Bar},
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VN{x: Box<dyn Bar>},
   |           ^^^^       ^

error[E0277]: the size for values of type `(dyn FooBar + 'static)` cannot be known at compilation time
   |
   |
LL |     VO(isize, dyn FooBar),
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn FooBar + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VO(isize, &dyn FooBar),
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VO(isize, Box<dyn FooBar>),
   |               ^^^^          ^

error[E0277]: the size for values of type `(dyn BarFoo + 'static)` cannot be known at compilation time
   |
   |
LL |     VP{u: isize, x: dyn BarFoo},
   |
   |
   = help: the trait `Sized` is not implemented for `(dyn BarFoo + 'static)`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VP{u: isize, x: &dyn BarFoo},
   |                     ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VP{u: isize, x: Box<dyn BarFoo>},
   |                     ^^^^          ^

error[E0277]: the size for values of type `[i8]` cannot be known at compilation time
   |
   |
LL |     VQ(<&'static [i8] as Deref>::Target),
   |
   = help: the trait `Sized` is not implemented for `[i8]`
   = help: the trait `Sized` is not implemented for `[i8]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VQ(&<&'static [i8] as Deref>::Target),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VQ(Box<<&'static [i8] as Deref>::Target>),
   |        ^^^^                                ^

error[E0277]: the size for values of type `[char]` cannot be known at compilation time
   |
   |
LL |     VR{x: <&'static [char] as Deref>::Target},
   |
   = help: the trait `Sized` is not implemented for `[char]`
   = help: the trait `Sized` is not implemented for `[char]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VR{x: &<&'static [char] as Deref>::Target},
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VR{x: Box<<&'static [char] as Deref>::Target>},
   |           ^^^^                                  ^

error[E0277]: the size for values of type `[f64]` cannot be known at compilation time
   |
   |
LL |     VS(isize, <&'static [f64] as Deref>::Target),
   |
   = help: the trait `Sized` is not implemented for `[f64]`
   = help: the trait `Sized` is not implemented for `[f64]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VS(isize, &<&'static [f64] as Deref>::Target),
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VS(isize, Box<<&'static [f64] as Deref>::Target>),
   |               ^^^^                                 ^

error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
   |
   |
LL |     VT{u: isize, x: <&'static [i32] as Deref>::Target},
   |
   = help: the trait `Sized` is not implemented for `[i32]`
   = help: the trait `Sized` is not implemented for `[i32]`
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VT{u: isize, x: &<&'static [i32] as Deref>::Target},
   |                     ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VT{u: isize, x: Box<<&'static [i32] as Deref>::Target>},
   |                     ^^^^                                 ^

error[E0277]: the size for values of type `(dyn PathHelper1 + 'static)` cannot be known at compilation time
   |
   |
LL |     VI(Path1),
   |
   |
   = help: within `Path1`, the trait `Sized` is not implemented for `(dyn PathHelper1 + 'static)`
note: required because it appears within the type `Path1`
   |
   |
LL | struct Path1(dyn PathHelper1);
   |        ^^^^^
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VI(&Path1),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VI(Box<Path1>),
   |        ^^^^     ^

error[E0277]: the size for values of type `(dyn PathHelper2 + 'static)` cannot be known at compilation time
   |
   |
LL |     VJ{x: Path2},
   |
   |
   = help: within `Path2`, the trait `Sized` is not implemented for `(dyn PathHelper2 + 'static)`
note: required because it appears within the type `Path2`
   |
   |
LL | struct Path2(dyn PathHelper2);
   |        ^^^^^
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VJ{x: &Path2},
   |           ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VJ{x: Box<Path2>},
   |           ^^^^     ^

error[E0277]: the size for values of type `(dyn PathHelper3 + 'static)` cannot be known at compilation time
   |
   |
LL |     VK(isize, Path3),
   |
   |
   = help: within `Path3`, the trait `Sized` is not implemented for `(dyn PathHelper3 + 'static)`
note: required because it appears within the type `Path3`
   |
   |
LL | struct Path3(dyn PathHelper3);
   |        ^^^^^
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VK(isize, &Path3),
   |               ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VK(isize, Box<Path3>),
   |               ^^^^     ^

error[E0277]: the size for values of type `(dyn PathHelper4 + 'static)` cannot be known at compilation time
   |
   |
LL |     VL{u: isize, x: Path4},
   |
   |
   = help: within `Path4`, the trait `Sized` is not implemented for `(dyn PathHelper4 + 'static)`
note: required because it appears within the type `Path4`
   |
   |
LL | struct Path4(dyn PathHelper4);
   |        ^^^^^
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     VL{u: isize, x: &Path4},
   |                     ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     VL{u: isize, x: Box<Path4>},
   |                     ^^^^     ^
error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-inherent-impl-self-type.rs stdout ----
diff of stderr:

18    |           ^  - ...if indirection were used here: `Box<Y>`
---
To only update this specific test, also pass `--test-args unsized/unsized-inherent-impl-self-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-inherent-impl-self-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-inherent-impl-self-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-inherent-impl-self-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-inherent-impl-self-type.rs:7:17
   |
LL | impl<X: ?Sized> S5<X> {
   |      -          ^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-inherent-impl-self-type.rs:5:11
  --> /checkout/src/test/ui/unsized/unsized-inherent-impl-self-type.rs:5:11
   |
LL | struct S5<Y>(Y);
   |           ^ required by this bound in `S5`
help: you could relax the implicit `Sized` bound on `Y` if it were used through indirection like `&Y` or `Box<Y>`
   |
   |
LL | struct S5<Y>(Y);
   |           ^  - ...if indirection were used here: `Box<Y>`
   |           |
   |           this could be changed to `Y: ?Sized`...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-struct.rs stdout ----
diff of stderr:

18    |            ^          - ...if indirection were used here: `Box<T>`
19    |            |
20    |            this could be changed to `T: ?Sized`...
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn foo2<T>() { not_sized::<Foo<T>>() }
25 
26 error[E0277]: the size for values of type `T` cannot be known at compilation time
27   --> $DIR/unsized-struct.rs:13:24


41    |
42 LL | fn is_sized<T:Sized>() { }
43    |             ^ required by this bound in `is_sized`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn bar2<T>() { is_sized::<Bar<T>>() }
48 
49 error: aborting due to 2 previous errors
50 

---
To only update this specific test, also pass `--test-args unsized/unsized-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-struct.rs:6:36
   |
LL | fn foo2<T: ?Sized>() { not_sized::<Foo<T>>() }
   |         -                          ^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-struct.rs:4:12
  --> /checkout/src/test/ui/unsized/unsized-struct.rs:4:12
   |
LL | struct Foo<T> { data: T }
   |            ^ required by this bound in `Foo`
help: you could relax the implicit `Sized` bound on `T` if it were used through indirection like `&T` or `Box<T>`
   |
   |
LL | struct Foo<T> { data: T }
   |            ^          - ...if indirection were used here: `Box<T>`
   |            |
   |            this could be changed to `T: ?Sized`...
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-struct.rs:13:24
   |
   |
LL | fn bar2<T: ?Sized>() { is_sized::<Bar<T>>() }
   |         -              ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
   |
note: required because it appears within the type `Bar<T>`
   |
   |
LL | struct Bar<T: ?Sized> { data: T }
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-struct.rs:1:13
   |
   |
LL | fn is_sized<T:Sized>() { }
   |             ^ required by this bound in `is_sized`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-trait-impl-self-type.rs stdout ----
diff of stderr:

18    |           ^  - ...if indirection were used here: `Box<Y>`
19    |           |
20    |           this could be changed to `Y: ?Sized`...
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<X> T3<X> for S5<X> {
25 
26 error: aborting due to previous error
27 

---
To only update this specific test, also pass `--test-args unsized/unsized-trait-impl-self-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-trait-impl-self-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-trait-impl-self-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-trait-impl-self-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-self-type.rs:10:27
   |
LL | impl<X: ?Sized> T3<X> for S5<X> {
   |      -                    ^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-self-type.rs:8:11
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-self-type.rs:8:11
   |
LL | struct S5<Y>(Y);
   |           ^ required by this bound in `S5`
help: you could relax the implicit `Sized` bound on `Y` if it were used through indirection like `&Y` or `Box<Y>`
   |
   |
LL | struct S5<Y>(Y);
   |           ^  - ...if indirection were used here: `Box<Y>`
   |           |
   |           this could be changed to `Y: ?Sized`...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized-trait-impl-trait-arg.rs stdout ----
diff of stderr:

11    |
12 LL | trait T2<Z> {
13    |          ^ required by this bound in `T2`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<X> T2<X> for S4<X> {
-    |      --
18 help: consider relaxing the implicit `Sized` restriction
19    |
20 LL | trait T2<Z: ?Sized> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-trait-impl-trait-arg/unsized-trait-impl-trait-arg.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized-trait-impl-trait-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized-trait-impl-trait-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-trait-impl-trait-arg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized-trait-impl-trait-arg/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-trait-arg.rs:8:17
   |
LL | impl<X: ?Sized> T2<X> for S4<X> {
   |      -          ^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-trait-arg.rs:4:10
  --> /checkout/src/test/ui/unsized/unsized-trait-impl-trait-arg.rs:4:10
   |
LL | trait T2<Z> {
   |          ^ required by this bound in `T2`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait T2<Z: ?Sized> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/unsized/unsized5.rs stdout ----
diff of stderr:

8    |
9    = note: only the last field of a struct may have a dynamically sized type
10    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | struct S1<X> {
-    |           --
15 help: borrowed types always have a statically known size
17 LL |     f1: &X,

32    |
32    |
33    = note: only the last field of a struct may have a dynamically sized type
34    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | struct S2<X> {
-    |           --
39 help: borrowed types always have a statically known size
40    |
41 LL |     g: &X,
91    |
91    |
92    = note: no field of an enum variant may have a dynamically sized type
93    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum E<X> {
-    |        --
98 help: borrowed types always have a statically known size
99    |
100 LL |     V1(&X, isize),
114    |
114    |
115    = note: no field of an enum variant may have a dynamically sized type
116    = help: change the field's type to have a statically known size
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | enum F<X> {
-    |        --
121 help: borrowed types always have a statically known size
122    |
123 LL |     V2{f1: &X, f: isize},

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized5/unsized5.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:4:9
   |
LL | struct S1<X: ?Sized> {
   |           - this type parameter needs to be `std::marker::Sized`
LL |     f1: X,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     f1: &X,
   |         ^
   |         ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     f1: Box<X>,
   |         ^^^^ ^

error[E0277]: the size for values of type `X` cannot be known at compilation time
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:10:8
   |
LL | struct S2<X: ?Sized> {
   |           - this type parameter needs to be `std::marker::Sized`
LL |     f: isize,
LL |     g: X,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     g: &X,
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     g: Box<X>,
   |        ^^^^ ^
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:15:8
   |
LL |     f: str,
LL |     f: str,
   |        ^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `str`
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     f: &str,
   |        ^
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     f: Box<str>,
   |        ^^^^   ^

error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:20:8
   |
LL |     f: [u8],
   |        ^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `[u8]`
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
LL |     f: &[u8],
   |        ^
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     f: Box<[u8]>,
   |        ^^^^    ^

error[E0277]: the size for values of type `X` cannot be known at compilation time
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:25:8
   |
LL | enum E<X: ?Sized> {
   |        - this type parameter needs to be `std::marker::Sized`
LL |     V1(X, isize),
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     V1(&X, isize),
   |        ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     V1(Box<X>, isize),
   |        ^^^^ ^
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized5.rs:29:12
   |
   |
LL | enum F<X: ?Sized> {
   |        - this type parameter needs to be `std::marker::Sized`
LL |     V2{f1: X, f: isize},
   |
   |
   = note: no field of an enum variant may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: borrowed types always have a statically known size
   |
LL |     V2{f1: &X, f: isize},
   |            ^
help: the `Box` type always has a statically known size and allocates its contents in the heap
   |
LL |     V2{f1: Box<X>, f: isize},
   |            ^^^^ ^
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/unsized/unsized3.rs stdout ----
diff of stderr:

11    |
12 LL | fn f2<X>(x: &X) {
13    |       ^ required by this bound in `f2`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f1<X>(x: &X) {
-    |       --
18 help: consider relaxing the implicit `Sized` restriction
19    |
20 LL | fn f2<X: ?Sized>(x: &X) {
33    |
33    |
34 LL | fn f4<X: T>(x: &X) {
35    |       ^ required by this bound in `f4`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f3<X: T>(x: &X) {
-    |         --
40 help: consider relaxing the implicit `Sized` restriction
41    |
42 LL | fn f4<X: T + ?Sized>(x: &X) {
60    |
60    |
61 LL | fn f5<Y>(x: &Y) {}
62    |       ^ required by this bound in `f5`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f8<X>(x1: &S<X>, x2: &S<X>) {
-    |       --
67 help: consider relaxing the implicit `Sized` restriction
68    |
69 LL | fn f5<Y: ?Sized>(x: &Y) {}

83 LL | struct S<X: ?Sized> {
84    |        ^
85    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f9<X>(x1: Box<S<X>>) {
90 
91 error[E0277]: the size for values of type `X` cannot be known at compilation time
92   --> $DIR/unsized3.rs:45:9


103    |        ^
104    = note: required because it appears within the type `({integer}, S<X>)`
105    = note: tuples must have a statically known size to be initialized
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f10<X>(x1: Box<S<X>>) {
110 
111 error[E0277]: the size for values of type `X` cannot be known at compilation time
112   --> $DIR/unsized3.rs:45:8


127    |
128 LL | fn f5<Y>(x: &Y) {}
129    |       ^ required by this bound in `f5`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f10<X>(x1: Box<S<X>>) {
-    |        --
134 help: consider relaxing the implicit `Sized` restriction
135    |
136 LL | fn f5<Y: ?Sized>(x: &Y) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized3/unsized3.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:7:13
   |
LL | fn f1<X: ?Sized>(x: &X) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     f2::<X>(x);
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized3.rs:10:7
   |
   |
LL | fn f2<X>(x: &X) {
   |       ^ required by this bound in `f2`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn f2<X: ?Sized>(x: &X) {

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:18:13
   |
   |
LL | fn f3<X: ?Sized + T>(x: &X) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     f4::<X>(x);
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized3.rs:21:7
   |
   |
LL | fn f4<X: T>(x: &X) {
   |       ^ required by this bound in `f4`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn f4<X: T + ?Sized>(x: &X) {

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:33:8
   |
   |
LL | fn f8<X: ?Sized>(x1: &S<X>, x2: &S<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     f5(x1);
   |        ^^ doesn't have a size known at compile-time
note: required because it appears within the type `S<X>`
  --> /checkout/src/test/ui/unsized/unsized3.rs:28:8
   |
   |
LL | struct S<X: ?Sized> {
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized3.rs:24:7
   |
   |
LL | fn f5<Y>(x: &Y) {}
   |       ^ required by this bound in `f5`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn f5<Y: ?Sized>(x: &Y) {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:40:8
   |
   |
LL | fn f9<X: ?Sized>(x1: Box<S<X>>) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     f5(&(*x1, 34));
   |
note: required because it appears within the type `S<X>`
  --> /checkout/src/test/ui/unsized/unsized3.rs:28:8
   |
   |
LL | struct S<X: ?Sized> {
   |        ^
   = note: only the last element of a tuple may have a dynamically sized type
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:45:9
   |
   |
LL | fn f10<X: ?Sized>(x1: Box<S<X>>) {
   |        - this type parameter needs to be `std::marker::Sized`
LL |     f5(&(32, *x1));
   |
note: required because it appears within the type `S<X>`
  --> /checkout/src/test/ui/unsized/unsized3.rs:28:8
   |
   |
LL | struct S<X: ?Sized> {
   |        ^
   = note: required because it appears within the type `({integer}, S<X>)`
   = note: tuples must have a statically known size to be initialized
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized3.rs:45:8
   |
   |
LL | fn f10<X: ?Sized>(x1: Box<S<X>>) {
   |        - this type parameter needs to be `std::marker::Sized`
LL |     f5(&(32, *x1));
   |
note: required because it appears within the type `S<X>`
  --> /checkout/src/test/ui/unsized/unsized3.rs:28:8
   |
   |
LL | struct S<X: ?Sized> {
   |        ^
   = note: required because it appears within the type `({integer}, S<X>)`
note: type parameters have an implicit `Sized` obligation
   |
   |
LL | fn f5<Y>(x: &Y) {}
   |       ^ required by this bound in `f5`
help: consider relaxing the implicit `Sized` restriction
   |
LL | fn f5<Y: ?Sized>(x: &Y) {}

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0277`.
---
---- [ui] ui/unsized/unsized7.rs stdout ----
diff of stderr:

11    |
12 LL | trait T1<Z: T> {
13    |          ^ required by this bound in `T1`
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | impl<X: T> T1<X> for S3<X> {
-    |        --
18 help: consider relaxing the implicit `Sized` restriction
19    |
20 LL | trait T1<Z: T + ?Sized> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized7/unsized7.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized7.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized7.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized7" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized7/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized7.rs:12:21
   |
LL | impl<X: ?Sized + T> T1<X> for S3<X> {
   |      -              ^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/unsized/unsized7.rs:7:10
  --> /checkout/src/test/ui/unsized/unsized7.rs:7:10
   |
LL | trait T1<Z: T> {
   |          ^ required by this bound in `T1`
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait T1<Z: T + ?Sized> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---

9    |
10    = note: all local variables must have a statically known size
11    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f1<W: ?Sized, X: ?Sized, Y, Z: ?Sized>(x: &X) {
16 
17 error[E0277]: the size for values of type `X` cannot be known at compilation time
18   --> $DIR/unsized6.rs:7:12


24    |            ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
25    |
26    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f1<W: ?Sized, X, Y: ?Sized, Z: ?Sized>(x: &X) {
31 
31 
32 error[E0277]: the size for values of type `Z` cannot be known at compilation time

39    |            ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
40    |
40    |
41    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f1<W: ?Sized, X: ?Sized, Y: ?Sized, Z>(x: &X) {
46 
47 error[E0277]: the size for values of type `X` cannot be known at compilation time
48   --> $DIR/unsized6.rs:15:9


54    |
55    = note: all local variables must have a statically known size
56    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f2<X, Y: ?Sized>(x: &X) {
61 
62 error[E0277]: the size for values of type `Y` cannot be known at compilation time
63   --> $DIR/unsized6.rs:17:12


69    |            ^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
70    |
71    = note: only the last element of a tuple may have a dynamically sized type
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f2<X: ?Sized, Y>(x: &X) {
76 
77 error[E0277]: the size for values of type `X` cannot be known at compilation time
78   --> $DIR/unsized6.rs:22:9


84    |
85    = note: all local variables must have a statically known size
86    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f3<X>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
91 
92 error[E0277]: the size for values of type `X` cannot be known at compilation time
93   --> $DIR/unsized6.rs:24:9


100    |
101    = note: all local variables must have a statically known size
102    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f3<X>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
107 
108 error[E0277]: the size for values of type `X` cannot be known at compilation time
109   --> $DIR/unsized6.rs:26:10


116    |
117    = note: all local variables must have a statically known size
118    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f3<X>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
123 
124 error[E0277]: the size for values of type `X` cannot be known at compilation time
125   --> $DIR/unsized6.rs:30:9


131    |
132    = note: all local variables must have a statically known size
133    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f4<X: T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
138 
139 error[E0277]: the size for values of type `X` cannot be known at compilation time
140   --> $DIR/unsized6.rs:32:9


147    |
148    = note: all local variables must have a statically known size
149    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f4<X: T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
154 
155 error[E0277]: the size for values of type `X` cannot be known at compilation time
156   --> $DIR/unsized6.rs:34:10


163    |
164    = note: all local variables must have a statically known size
165    = help: unsized locals are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn f4<X: T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
170 
171 error[E0277]: the size for values of type `X` cannot be known at compilation time
172   --> $DIR/unsized6.rs:38:18


177    |       this type parameter needs to be `std::marker::Sized`
178    |
179    = help: unsized fn params are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn g1<X>(x: X) {}
-    |       --
184 help: function arguments must have a statically known size, borrowed types always have a known size
185    |
186 LL | fn g1<X: ?Sized>(x: &X) {}
195    |       this type parameter needs to be `std::marker::Sized`
196    |
197    = help: unsized fn params are gated as an unstable feature
197    = help: unsized fn params are gated as an unstable feature
- help: consider removing the `?Sized` bound to make the type parameter `Sized`
-    |
- LL | fn g2<X: T>(x: X) {}
-    |         --
202 help: function arguments must have a statically known size, borrowed types always have a known size
203    |
204 LL | fn g2<X: ?Sized + T>(x: &X) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized6/unsized6.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/unsized6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/unsized6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized6" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/unsized6/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:9:9
   |
LL | fn f1<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized>(x: &X) {
   |                             - this type parameter needs to be `std::marker::Sized`
LL |     let y: Y;
   |         ^ doesn't have a size known at compile-time
   |
   = note: all local variables must have a statically known size
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:7:12
   |
LL | fn f1<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized>(x: &X) {
   |                  - this type parameter needs to be `std::marker::Sized`
LL |     let _: W; // <-- this is OK, no bindings created, no initializer.
LL |     let _: (isize, (X, isize));
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type

error[E0277]: the size for values of type `Z` cannot be known at compilation time
   |
   |
LL | fn f1<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized>(x: &X) {
   |                                        - this type parameter needs to be `std::marker::Sized`
...
LL |     let y: (isize, (Z, usize));
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:15:9
   |
   |
LL | fn f2<X: ?Sized, Y: ?Sized>(x: &X) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     let y: X;
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:17:12
   |
LL | fn f2<X: ?Sized, Y: ?Sized>(x: &X) {
   |                  - this type parameter needs to be `std::marker::Sized`
...
LL |     let y: (isize, (Y, isize));
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:22:9
   |
   |
LL | fn f3<X: ?Sized>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     let y: X = *x1;
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:24:9
   |
LL | fn f3<X: ?Sized>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
...
LL |     let y = *x2;
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:26:10
   |
LL | fn f3<X: ?Sized>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
...
LL |     let (y, z) = (*x3, 4);
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:30:9
   |
LL | fn f4<X: ?Sized + T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
LL |     let y: X = *x1;
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:32:9
   |
LL | fn f4<X: ?Sized + T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
...
LL |     let y = *x2;
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:34:10
   |
LL | fn f4<X: ?Sized + T>(x1: Box<X>, x2: Box<X>, x3: Box<X>) {
   |       - this type parameter needs to be `std::marker::Sized`
...
LL |     let (y, z) = (*x3, 4);
   |
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature


error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:38:18
   |
LL | fn g1<X: ?Sized>(x: X) {}
   |       -          ^ doesn't have a size known at compile-time
   |       this type parameter needs to be `std::marker::Sized`
   |
   = help: unsized fn params are gated as an unstable feature
   = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL | fn g1<X: ?Sized>(x: &X) {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/unsized/unsized6.rs:40:22
   |
   |
LL | fn g2<X: ?Sized + T>(x: X) {}
   |       -              ^ doesn't have a size known at compile-time
   |       this type parameter needs to be `std::marker::Sized`
   |
   = help: unsized fn params are gated as an unstable feature
   = help: unsized fn params are gated as an unstable feature
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL | fn g2<X: ?Sized + T>(x: &X) {}

error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 11951 passed; 20 failed; 100 ignored; 0 measured; 0 filtered out; finished in 110.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:14
