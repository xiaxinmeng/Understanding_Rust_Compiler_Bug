plain
.................................................................................................... 9400/11719
.................................................................................................... 9500/11719
............................................................i......i................................ 9600/11719
.................................................................................................... 9700/11719
......iiiiiii..iiiiii.i............................................................................. 9800/11719
................................................................F................................... 10000/11719
.................................................................................................... 10100/11719
.................................................................................................... 10200/11719
.................................................................................................... 10300/11719
---
.................................................................................................... 10800/11719
.................................................................................................... 10900/11719
.................................................................................................... 11000/11719
............................ii...................................................................... 11100/11719
................................................................................F..F................ 11200/11719
.................................................................................................... 11400/11719
.................................................................................................... 11500/11719
.................................................................................................... 11600/11719
..........i.i....................................................................................... 11700/11719
..........i.i....................................................................................... 11700/11719
...................
failures:

---- [ui] ui/box/alloc-unstable-fail.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/box/alloc-unstable-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/alloc-unstable-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/alloc-unstable-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/const-generics/issues/issue-62878.rs#full stdout ----
diff of stderr:

4 LL | fn foo<const N: usize, const A: [u8; N]>() {}
5    |                                      ^ the type must not depend on the parameter `N`
- error[E0747]: type provided when a constant was expected
-   --> $DIR/issue-62878.rs:10:11
-   --> $DIR/issue-62878.rs:10:11
+ error: constant expression depends on a generic parameter
9    |
9    |
10 LL |     foo::<_, {[1]}>();
+    |              ^^^^^
12    |
12    |
-    = help: const arguments cannot yet be inferred with `_`
+    = note: this may fail depending on what value the parameter takes
15 error[E0308]: mismatched types
16   --> $DIR/issue-62878.rs:10:15

20 
---
25 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62878.full/issue-62878.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-62878.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62878.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62878.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62878.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0770]: the type of const parameters must not depend on other generic parameters
   |
   |
LL | fn foo<const N: usize, const A: [u8; N]>() {}
   |                                      ^ the type must not depend on the parameter `N`

error: constant expression depends on a generic parameter
   |
   |
LL |     foo::<_, {[1]}>();
   |
   = note: this may fail depending on what value the parameter takes

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-62878.rs:10:15
   |
LL |     foo::<_, {[1]}>();
   |               ^^^ expected `usize`, found array `[{integer}; 1]`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0770.
For more information about an error, try `rustc --explain E0308`.
---
---- [ui] ui/const-generics/min_const_generics/inferred_const.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/inferred_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/inferred_const" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/inferred_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `min_const_generics` has been stable since 1.51.0 and no longer requires an attribute to enable
   |
   |
LL | #![feature(min_const_generics)]
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
---

---- [ui] ui/did_you_mean/bad-assoc-ty.rs stdout ----
diff of stderr:

123    |          ^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
124 
125 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/bad-assoc-ty.rs:49:13
-    |
- LL | fn foo<X: K<_, _>>(x: X) {}
-    |             ^  ^ not allowed in type signatures
-    |             not allowed in type signatures
-    |
- help: use type parameters instead
-    |
-    |
- LL | fn foo<X: K<T, T>, T>(x: X) {}
-    |             ^  ^ ^^^
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
140    |
140    |
141 LL | fn bar<F>(_: F) where F: Fn() -> _ {}

223 LL |     fn foo<F, T>(_: F) where F: Fn() -> T {}
224    |             ^^^                         ^
- error: aborting due to 28 previous errors
+ error: aborting due to 27 previous errors
227 
228 Some errors have detailed explanations: E0121, E0223.
228 Some errors have detailed explanations: E0121, E0223.
229 For more information about an error, try `rustc --explain E0121`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/bad-assoc-ty.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/bad-assoc-ty.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:1:10
   |
LL | type A = [u8; 4]::AssocTy;
   |          ^^^^^^^^^^^^^^^^ help: try: `<[u8; 4]>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:5:10
   |
   |
LL | type B = [u8]::AssocTy;
   |          ^^^^^^^^^^^^^ help: try: `<[u8]>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:9:10
   |
   |
LL | type C = (u8)::AssocTy;
   |          ^^^^^^^^^^^^^ help: try: `<(u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:13:10
   |
   |
LL | type D = (u8, u8)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^ help: try: `<(u8, u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:17:10
   |
   |
LL | type E = _::AssocTy;
   |          ^^^^^^^^^^ help: try: `<_>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:21:19
   |
   |
LL | type F = &'static (u8)::AssocTy;
   |                   ^^^^^^^^^^^^^ help: try: `<(u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:27:10
   |
   |
LL | type G = dyn 'static + (Send)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `<dyn 'static + (Send)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:44:10
   |
   |
LL | type I = ty!()::AssocTy;
   |          ^^^^^^^^^^^^^^ help: try: `<ty!()>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:37:19
   |
   |
LL |     ($ty: ty) => ($ty::AssocTy);
   |                   ^^^^^^^^^^^^ help: try: `<$ty>::AssocTy`
...
LL | type J = ty!(u8);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0223]: ambiguous associated type
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:1:10
   |
LL | type A = [u8; 4]::AssocTy;
   |          ^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<[u8; 4] as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:5:10
   |
   |
LL | type B = [u8]::AssocTy;
   |          ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<[u8] as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:9:10
   |
   |
LL | type C = (u8)::AssocTy;
   |          ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:13:10
   |
   |
LL | type D = (u8, u8)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(u8, u8) as Trait>::AssocTy`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:17:10
   |
LL | type E = _::AssocTy;
   |          ^ not allowed in type signatures
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:21:19
   |
   |
LL | type F = &'static (u8)::AssocTy;
   |                   ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:27:10
   |
   |
LL | type G = dyn 'static + (Send)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(dyn Send + 'static) as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:33:10
   |
   |
LL | type H = Fn(u8) -> (u8)::Output;
   |          ^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(dyn Fn(u8) -> u8 + 'static) as Trait>::Output`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:37:19
   |
   |
LL |     ($ty: ty) => ($ty::AssocTy);
   |                   ^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
...
LL | type J = ty!(u8);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0223]: ambiguous associated type
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:44:10
   |
LL | type I = ty!()::AssocTy;
   |          ^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:52:34
   |
LL | fn bar<F>(_: F) where F: Fn() -> _ {}
   |                                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn bar<F, T>(_: F) where F: Fn() -> T {}
   |         ^^^                         ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:55:19
   |
LL | fn baz<F: Fn() -> _>(_: F) {}
   |                   ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn baz<F: Fn() -> T, T>(_: F) {}


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:58:33
   |
LL | struct L<F>(F) where F: Fn() -> _;
   |                                 ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct L<F, T>(F) where F: Fn() -> T;
   |           ^^^                      ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:60:30
   |
LL | struct M<F> where F: Fn() -> _ {
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct M<F, T> where F: Fn() -> T {
   |           ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:64:28
   |
LL | enum N<F> where F: Fn() -> _ {
   |                            ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | enum N<F, T> where F: Fn() -> T {
   |         ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:69:29
   |
LL | union O<F> where F: Fn() -> _ {
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | union O<F, T> where F: Fn() -> T {
   |          ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:74:29
   |
LL | trait P<F> where F: Fn() -> _ {
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | trait P<F, T> where F: Fn() -> T {
   |          ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:79:38
   |
LL |     fn foo<F>(_: F) where F: Fn() -> _ {}
   |                                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn foo<F, T>(_: F) where F: Fn() -> T {}
   |             ^^^                         ^
error: aborting due to 27 previous errors

Some errors have detailed explanations: E0121, E0223.
For more information about an error, try `rustc --explain E0121`.
For more information about an error, try `rustc --explain E0121`.

------------------------------------------


---- [ui] ui/inference/cannot-infer-partial-try-return.rs stdout ----
diff of stderr:

1 error[E0282]: type annotations needed for the closure `fn() -> Result<(), QualifiedError<_>>`
-   --> $DIR/cannot-infer-partial-try-return.rs:19:9
3    |
3    |
+ LL |     let x = || -> Result<_, QualifiedError<_>> {
4 LL |         infallible()?;
-    |         ^^^^^^^^^^^^^ cannot infer type of error for `?` operator
-    |         ^^^^^^^^^^^^^ cannot infer type of error for `?` operator
+    |         ------------- cannot infer type for type parameter `E` declared on the struct `QualifiedError`
6    |
-    = note: `?` implicitly converts the error value into `QualifiedError<_>` using its implementation of `From<Infallible>`
8 help: give this closure an explicit return type without `_` placeholders
9    |
10 LL |     let x = || -> Result<(), QualifiedError<_>> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/cannot-infer-partial-try-return.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/cannot-infer-partial-try-return.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-partial-try-return.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed for the closure `fn() -> Result<(), QualifiedError<_>>`
   |
   |
LL |     let x = || -> Result<_, QualifiedError<_>> {
   |                             ^^^^^^^^^^^^^^^^^
LL |         infallible()?; //~ERROR type annotations needed
   |         ------------- cannot infer type for type parameter `E` declared on the struct `QualifiedError`
   |
help: give this closure an explicit return type without `_` placeholders
   |
LL |     let x = || -> Result<(), QualifiedError<_>> {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
---
diff of stderr:

8   --> $DIR/issue-47486.rs:3:11
9    |
10 LL |     [0u8; std::mem::size_of::<_>()];
-    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
+    |           ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T` declared on the function `size_of`
13 error: aborting due to 2 previous errors
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47486/issue-47486.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47486.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47486.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47486" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47486/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-47486.rs:2:10
   |
LL |     () < std::mem::size_of::<_>(); //~ ERROR: mismatched types
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ expected `()`, found `usize`
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-47486.rs:3:11
   |
   |
LL |     [0u8; std::mem::size_of::<_>()]; //~ ERROR: type annotations needed
   |           ^^^^^^^^^^^^^^^^^^^^^^ cannot infer type for type parameter `T` declared on the function `size_of`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0308.
For more information about an error, try `rustc --explain E0282`.
For more information about an error, try `rustc --explain E0282`.

------------------------------------------


---- [ui] ui/parser/issue-14303-fncall.rs stdout ----
diff of stderr:

- error[E0747]: type provided when a lifetime was expected
+ error[E0747]: inferred provided when a lifetime was expected
3    |
3    |
4 LL |         .collect::<Vec<S<_, 'a>>>();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall/issue-14303-fncall.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall/issue-14303-fncall.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-fncall.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-fncall.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0747]: inferred provided when a lifetime was expected
   |
   |
LL |         .collect::<Vec<S<_, 'a>>>();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0747`.
---
diff of stderr:

274    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
275 
276 error: type `priv_parent_substs::Priv` is private
-   --> $DIR/associated-item-privacy-trait.rs:119:30
-    |
- LL |         let _: <Pub as PubTr<_>>::AssocTy;
-    |                              ^ private type
- ...
- LL |     priv_parent_substs::mac!();
-    |
-    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
- 
- 
- error: type `priv_parent_substs::Priv` is private
289    |
289    |
290 LL |         let _: <Priv as PubTr<_>>::AssocTy;
328    |
329    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
330 
- error: aborting due to 30 previous errors
- error: aborting due to 30 previous errors
+ error: aborting due to 29 previous errors
332 
333 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/associated-item-privacy-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/associated-item-privacy-trait.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
   |
   |
LL |         let value = <Pub as PrivTr>::method;
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private
   |
   |
LL |         Pub.method();
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: associated constant `<Pub as PrivTr>::CONST` is private
   |
   |
LL |         <Pub as PrivTr>::CONST;
   |         ^^^^^^^^^^^^^^^^^^^^^^ private associated constant
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: associated type `<Pub as PrivTr>::AssocTy` is private
   |
   |
LL |         let _: <Pub as PrivTr>::AssocTy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ private associated type
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         pub type InSignatureTy = <Pub as PrivTr>::AssocTy;
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         pub trait InSignatureTr: PrivTr {}
   |                                  ^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         impl PrivTr for u8 {}
   |              ^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method;
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         Pub.method(loop {});
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method::<Priv>;
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         Pub.method::<Priv>();
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr<_>>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Pub.method();
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Priv as PubTr<_>>::method;
...
...
LL |     priv_parent_substs::mac!();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Priv.method();
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Pub as PubTr>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Pub as PubTr<_>>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Priv as PubTr<_>>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let _: <Priv as PubTr<_>>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         pub type InSignatureTy1 = <Pub as PubTr>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         pub type InSignatureTy2 = <Priv as PubTr<Pub>>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         impl PubTr for u8 {}
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 29 previous errors
---
- error[E0282]: type annotations needed for `Option<_>`
+ error[E0282]: type annotations needed for `Option<T>`
2   --> $DIR/issue-42234-unknown-receiver-type.rs:7:7
3    |
4 LL |     let x: Option<_> = None;

-    |         - consider giving `x` the explicit type `Option<_>`, where the type parameter `T` is specified
+    |         - consider giving `x` the explicit type `Option<T>`, where the type parameter `T` is specified
6 LL |     x.unwrap().method_that_could_exist_on_some_type();
7    |       ^^^^^^ cannot infer type for type parameter `T`

12   --> $DIR/issue-42234-unknown-receiver-type.rs:13:10
13    |
13    |
14 LL |         .sum::<_>()
-    |          ^^^ cannot infer type
+    |          ^^^ cannot infer type for type parameter `S` declared on the associated function `sum`
16    |
17    = note: type must be known at this point


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/issue-42234-unknown-receiver-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/issue-42234-unknown-receiver-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-42234-unknown-receiver-type.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-42234-unknown-receiver-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed for `Option<T>`
  --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:7:7
   |
LL |     let x: Option<_> = None;
   |         - consider giving `x` the explicit type `Option<T>`, where the type parameter `T` is specified
LL |     x.unwrap().method_that_could_exist_on_some_type();
   |       ^^^^^^ cannot infer type for type parameter `T`
   |
   = note: type must be known at this point
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/span/issue-42234-unknown-receiver-type.rs:13:10
   |
   |
LL |         .sum::<_>() //~ ERROR type annotations needed
   |          ^^^ cannot infer type for type parameter `S` declared on the associated function `sum`
   |
   = note: type must be known at this point
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/typeck/typeck_type_placeholder_item.rs#min_tait stdout ----
diff of stderr:

204    |               help: replace `_` with the correct type: `i32`
205 
206 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:80:15
-    |
- LL |     static C: Option<_> = Some(42);
-    |               ^^^^^^^^^ not allowed in type signatures
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
214    |
214    |
215 LL |     fn fn_test() -> _ { 5 }
358    |                  ^  ^
359 
359 
360 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:163:15
-    |
- LL | impl BadTrait<_> for BadStruct<_> {}
-    |               ^                ^ not allowed in type signatures
-    |               not allowed in type signatures
-    |
- help: use type parameters instead
-    |
-    |
- LL | impl<T> BadTrait<T> for BadStruct<T> {}
-    |     ^^^          ^                ^
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:166:34
-    |
- LL | fn impl_trait() -> impl BadTrait<_> {
-    |                                  ^ not allowed in type signatures
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
381    |
381    |
382 LL | struct BadStruct1<_, _>(_);
399    |                   ^     ^
400 
400 
401 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:180:14
-    |
- LL | type X = Box<_>;
-    |              ^ not allowed in type signatures
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:186:21
-    |
- LL | type Y = impl Trait<_>;
-    |                     ^ not allowed in type signatures
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
415    |
415    |
416 LL | fn value() -> Option<&'static _> {

420    |               help: replace with the correct return type: `Option<&'static u8>`
421 
422 error[E0121]: the type placeholder `_` is not allowed within types on item signatures
-   --> $DIR/typeck_type_placeholder_item.rs:225:10
-    |
- LL | const _: Option<_> = map(value);
-    |          |
-    |          not allowed in type signatures
-    |          not allowed in type signatures
-    |          help: replace `_` with the correct type: `Option<u8>`
- 
- error[E0121]: the type placeholder `_` is not allowed within types on item signatures
433    |
433    |
434 LL |     fn method_test1(&self, x: _);
632    |              not allowed in type signatures
632    |              not allowed in type signatures
633    |              help: replace `_` with the correct type: `i32`
- error: aborting due to 69 previous errors
+ error: aborting due to 63 previous errors
636 
637 Some errors have detailed explanations: E0121, E0282, E0403.
637 Some errors have detailed explanations: E0121, E0282, E0403.
638 For more information about an error, try `rustc --explain E0121`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait/typeck_type_placeholder_item.min_tait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`

error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:158:18
   |
LL | struct BadStruct<_>(_);
   |                  ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:161:16
   |
   |
LL | trait BadTrait<_> {}
   |                ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:171:19
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:171:22
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                      ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:176:19
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                   ^ expected identifier, found reserved identifier

error: associated constant in `impl` without body
   |
LL |     const C: _;
   |     ^^^^^^^^^^-
   |               |
   |               |
   |               help: provide a definition for the constant: `= <expr>;`

error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   -  ^ already used
   |                   first use of `_`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test() -> _ { 5 }
   |              |
   |              not allowed in type signatures
   |              help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test2() -> (_, _) { (5, 5) }
   |               -^--^-
   |               ||  |
   |               ||  not allowed in type signatures
   |               |not allowed in type signatures
   |               help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST3: _ = "test";
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST4: _ = 145;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST5: (_, _) = (1, 2);
   |               ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6(_: _) { }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6<T>(_: T) { }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_b<T>(_: _, _: T) { }
   |                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_b<T, U>(_: U, _: T) { }
   |             ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }
   |                         ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test7(x: _) { let _x: usize = x; }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test7<T>(x: T) { let _x: usize = x; }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      |
   |                      not allowed in type signatures
   |                      help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test8<T>(_f: fn() -> T) { }
   |         ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test11(x: &usize) -> &_ {
   |                         -^
   |                         ||
   |                         |not allowed in type signatures
   |                         help: replace with the correct return type: `&'static &'static usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | unsafe fn test12(x: *const usize) -> *const *const _ {
   |                                      |             |
   |                                      |             not allowed in type signatures
   |                                      |             not allowed in type signatures
   |                                      help: replace with the correct return type: `*const *const usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |     a: _,
   |        ^ not allowed in type signatures
   |        ^ not allowed in type signatures
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (_, _),
   |         ^  ^ not allowed in type signatures
   |         not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | struct Test10<T> {
LL |     a: T,
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (T, T),


error: missing type for `static` item
   |
LL |     static A = 42;
LL |     static A = 42;
   |            ^ help: provide a type for the item: `A: i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static B: _ = 42;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test() -> _ { 5 }
   |                     |
   |                     not allowed in type signatures
   |                     help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test2() -> (_, _) { (5, 5) }
   |                      -^--^-
   |                      ||  |
   |                      ||  not allowed in type signatures
   |                      |not allowed in type signatures
   |                      help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST3: _ = "test";
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST4: _ = 145;
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST5: (_, _) = (1, 2);
   |                      ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test6(_: _) { }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test6<T>(_: T) { }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test7(x: _) { let _x: usize = x; }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             |
   |                             not allowed in type signatures
   |                             help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test8<T>(_f: fn() -> T) { }
   |                ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |         a: _,
   |            ^ not allowed in type signatures
   |            ^ not allowed in type signatures
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (_, _),
   |             ^  ^ not allowed in type signatures
   |             not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL |     struct FnTest10<T> {
LL |         a: T,
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (T, T),

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:132:18
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                  ^ cannot infer type

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                            ^  ^ not allowed in type signatures
   |                            not allowed in type signatures


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
   |                             -^--^-
   |                             ||  |
   |                             ||  not allowed in type signatures
   |                             |not allowed in type signatures
   |                             help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
   |                           ------^-
---
test result: FAILED. 11612 passed; 11 failed; 96 ignored; 0 measured; 0 filtered out; finished in 134.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:20
