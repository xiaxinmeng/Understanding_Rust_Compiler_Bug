plain

437    |                               not allowed in type signatures
438    |                               help: replace with an appropriate return type: `impl Iterator<Item = usize>`
439 
- error[E0121]: the placeholder `_` is not allowed within types on item signatures for constants
-   --> $DIR/typeck_type_placeholder_item.rs:229:10
+ error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
442    |
442    |
443 LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
-    |          ^ not allowed in type signatures
+    |                      ^^^^^^ `std::ops::Range<{integer}>` is not an iterator
445    |
- note: however, the inferred type `Map<Filter<Range<i32>, [closure@typeck_type_placeholder_item.rs:229:29]>, [closure@typeck_type_placeholder_item.rs:229:49]>` cannot be named
+    = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
+ note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
448    |
448    |
449 LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
-    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |              ^^^^^^^
451 
451 
+ error[E0277]: the trait bound `Filter<std::ops::Range<{integer}>, [closure@$DIR/typeck_type_placeholder_item.rs:229:29: 229:32]>: Iterator` is not satisfied
+    |
+    |
+ LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
+    |                                             ^^^ `Filter<std::ops::Range<{integer}>, [closure@$DIR/typeck_type_placeholder_item.rs:229:29: 229:32]>` is not an iterator
+    |
+    = help: the trait `~const Iterator` is not implemented for `Filter<std::ops::Range<{integer}>, [closure@$DIR/typeck_type_placeholder_item.rs:229:29: 229:32]>`
+ note: the trait `Iterator` is implemented for `Filter<std::ops::Range<{integer}>, [closure@$DIR/typeck_type_placeholder_item.rs:229:29: 229:32]>`, but that implementation is not `const`
+    |
+    |
+ LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
+ 
+ 
+ error[E0121]: the placeholder `_` is not allowed within types on item signatures for constants
+    |
+    |
+ LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
+    |          ^ not allowed in type signatures
+ 
452 error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
454    |

657    |              not allowed in type signatures
658    |              help: replace with the correct type: `i32`
---
To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found reserved identifier `_`
   |
   |
LL | struct BadStruct<_>(_);

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:157:16
   |
   |
LL | trait BadTrait<_> {}

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:167:19
   |
   |
LL | struct BadStruct1<_, _>(_);

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:167:22
   |
   |
LL | struct BadStruct1<_, _>(_);

error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:172:19
   |
   |
LL | struct BadStruct2<_, T>(_, T);

error: associated constant in `impl` without body
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:205:5
   |
   |
LL |     const C: _;
   |     ^^^^^^^^^^-
   |               |
   |               help: provide a definition for the constant: `= <expr>;`

error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   -  ^ already used
   |                   first use of `_`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn test() -> _ { 5 }
   |              |
   |              not allowed in type signatures
   |              help: replace with the correct return type: `i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn test2() -> (_, _) { (5, 5) }
   |               -^--^-
   |               ||  |
   |               ||  not allowed in type signatures
   |               |not allowed in type signatures
   |               help: replace with the correct return type: `(i32, i32)`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static TEST3: _ = "test";
   |               |
   |               not allowed in type signatures
   |               help: replace with the correct type: `&str`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static TEST4: _ = 145;
   |               |
   |               not allowed in type signatures
   |               help: replace with the correct type: `i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static TEST5: (_, _) = (1, 2);
   |               ^^^^^^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test6(_: _) { }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6<T>(_: T) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test6_b<T>(_: _, _: T) { }
   |                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_b<T, U>(_: U, _: T) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test7(x: _) { let _x: usize = x; }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test7<T>(x: T) { let _x: usize = x; }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      |
   |                      not allowed in type signatures
   |                      help: use type parameters instead: `T`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test8<T>(_f: fn() -> T) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn test11(x: &usize) -> &_ {
   |                         -^
   |                         ||
   |                         |not allowed in type signatures
   |                         help: replace with the correct return type: `&'static &'static usize`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | unsafe fn test12(x: *const usize) -> *const *const _ {
   |                                      |             |
   |                                      |             not allowed in type signatures
   |                                      |             not allowed in type signatures
   |                                      help: replace with the correct return type: `*const *const usize`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for structs
   |
LL |     a: _,
   |        ^ not allowed in type signatures
   |        ^ not allowed in type signatures
LL |     //~^ ERROR the placeholder `_` is not allowed within types on item signatures for structs
LL |     b: (_, _),
   |         ^  ^ not allowed in type signatures
   |         not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL ~ struct Test10<T> {
LL ~     a: T,
LL |     //~^ ERROR the placeholder `_` is not allowed within types on item signatures for structs
LL ~     b: (T, T),

error: missing type for `static` item
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:73:13
   |
   |
LL |     static A = 42;
   |             ^ help: provide a type for the static variable: `: i32`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static B: _ = 42;
   |               |
   |               not allowed in type signatures
   |               help: replace with the correct type: `i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static C: Option<_> = Some(42);
   |               ^^^^^^^^^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     fn fn_test() -> _ { 5 }
   |                     |
   |                     not allowed in type signatures
   |                     help: replace with the correct return type: `i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     fn fn_test2() -> (_, _) { (5, 5) }
   |                      -^--^-
   |                      ||  |
   |                      ||  not allowed in type signatures
   |                      |not allowed in type signatures
   |                      help: replace with the correct return type: `(i32, i32)`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static FN_TEST3: _ = "test";
   |                      |
   |                      not allowed in type signatures
   |                      help: replace with the correct type: `&str`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static FN_TEST4: _ = 145;
   |                      |
   |                      not allowed in type signatures
   |                      help: replace with the correct type: `i32`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static FN_TEST5: (_, _) = (1, 2);
   |                      ^^^^^^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn fn_test6(_: _) { }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test6<T>(_: T) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn fn_test7(x: _) { let _x: usize = x; }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             |
   |                             not allowed in type signatures
   |                             help: use type parameters instead: `T`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test8<T>(_f: fn() -> T) { }


error[E0121]: the placeholder `_` is not allowed within types on item signatures for structs
   |
LL |         a: _,
   |            ^ not allowed in type signatures
   |            ^ not allowed in type signatures
LL |         //~^ ERROR the placeholder `_` is not allowed within types on item signatures for structs
LL |         b: (_, _),
   |             ^  ^ not allowed in type signatures
   |             not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL ~     struct FnTest10<T> {
LL ~         a: T,
LL |         //~^ ERROR the placeholder `_` is not allowed within types on item signatures for structs
LL ~         b: (T, T),

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:128:18
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                  ^ cannot infer type

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                            ^  ^ not allowed in type signatures
   |                            not allowed in type signatures


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
   |                             -^--^-
   |                             ||  |
   |                             ||  not allowed in type signatures
   |                             |not allowed in type signatures
   |                             help: replace with the correct return type: `(i32, i32)`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
   |                           ------^-
   |                           |     not allowed in type signatures
   |                           |     not allowed in type signatures
   |                           help: replace with the correct return type: `(i32, i32)`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for structs
   |
   |
LL | struct BadStruct<_>(_);
   |                     ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct<T>(T);
   |                  ~  ~

error[E0121]: the placeholder `_` is not allowed within types on item signatures for implementations
   |
   |
LL | impl BadTrait<_> for BadStruct<_> {}
   |               ^                ^ not allowed in type signatures
   |               not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | impl<T> BadTrait<T> for BadStruct<T> {}


error[E0121]: the placeholder `_` is not allowed within types on item signatures for opaque types
   |
   |
LL | fn impl_trait() -> impl BadTrait<_> {
   |                                  ^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for structs
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct1<T, _>(T);


error[E0121]: the placeholder `_` is not allowed within types on item signatures for structs
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct2<U, T>(U, T);


error[E0121]: the placeholder `_` is not allowed within types on item signatures for type aliases
   |
   |
LL | type X = Box<_>;
   |              ^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for opaque types
   |
   |
LL | type Y = impl Trait<_>;
   |                     ^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn value() -> Option<&'static _> {
   |               ----------------^-
   |               |               not allowed in type signatures
   |               |               not allowed in type signatures
   |               help: replace with the correct return type: `Option<&'static u8>`

error[E0121]: the placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const _: Option<_> = map(value);
   |          |
   |          not allowed in type signatures
   |          help: replace with the correct type: `Option<u8>`


error[E0121]: the placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn evens_squared(n: usize) -> _ {
   |                               |
   |                               not allowed in type signatures
   |                               help: replace with an appropriate return type: `impl Iterator<Item = usize>`


error[E0277]: the trait bound `std::ops::Range<{integer}>: Iterator` is not satisfied
   |
   |
LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
   |                      ^^^^^^ `std::ops::Range<{integer}>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `std::ops::Range<{integer}>`
note: the trait `Iterator` is implemented for `std::ops::Range<{integer}>`, but that implementation is not `const`
   |
   |
LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);


error[E0277]: the trait bound `Filter<std::ops::Range<{integer}>, [closure@/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:229:29: 229:32]>: Iterator` is not satisfied
   |
   |
LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
   |                                             ^^^ `Filter<std::ops::Range<{integer}>, [closure@/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:229:29: 229:32]>` is not an iterator
   |
   = help: the trait `~const Iterator` is not implemented for `Filter<std::ops::Range<{integer}>, [closure@/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:229:29: 229:32]>`
note: the trait `Iterator` is implemented for `Filter<std::ops::Range<{integer}>, [closure@/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:229:29: 229:32]>`, but that implementation is not `const`
   |
   |
LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);


error[E0121]: the placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const _: _ = (1..10).filter(|x| x % 2 == 0).map(|x| x * x);
   |          ^ not allowed in type signatures

error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn method_test1(&self, x: _);
   |                               ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn method_test1<T>(&self, x: T);


error[E0121]: the placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn method_test2(&self, x: _) -> _;
   |                               ^     ^ not allowed in type signatures
   |                               not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL |     fn method_test2<T>(&self, x: T) -> T;
