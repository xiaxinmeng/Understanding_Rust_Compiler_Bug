plain
..........i....ii....................................................................... 1760/13113
........................................................................................ 1848/13113
...........................................................i............................ 1936/13113
........................................................................................ 2024/13113
................................F...........F...........................F............... 2112/13113
.........................................FF............................................. 2288/13113
........................................................................................ 2376/13113
........................................................................................ 2464/13113
........................................................................................ 2552/13113
---

---- [ui] src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs#full stdout ----
diff of stderr:

16    = note: type parameters may not be used in const expressions
17    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
18 
- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/dependence_lint.rs:9:9
-    |
- LL |     [0; size_of::<*mut T>()]; // lint on stable, error with `generic_const_exprs`
-    |
-    = note: `#[warn(const_evaluatable_unchecked)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
- 
- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/dependence_lint.rs:16:9
-    |
- LL |     [0; if false { size_of::<T>() } else { 3 }]; // lint on stable, error with gce
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
- 
---
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.full/dependence_lint.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/dependence_lint.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/dependence_lint.full/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; size_of::<*mut T>()]; // error on stable, error with gce
   |                                ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; if true { size_of::<T>() } else { 3 }]; // error on stable, error with gce
   |                                     ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/function-call.rs stdout ----
diff of stderr:

- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/function-call.rs:14:17
-    |
- LL |     let _ = [0; foo::<T>()];
-    |
-    = note: `#[warn(const_evaluatable_unchecked)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/function-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/function-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/function-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/function-call/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/no_dependence.rs stdout ----
normalized stderr:
normalized stderr:
warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |      two_args::<N, 2>() // no lint
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

warning: cannot use constants which depend on generic parameters in types
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

warning: 2 warnings emitted
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/no_dependence.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/no_dependence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_dependence" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_dependence/auxiliary"
stdout: none
--- stderr -------------------------------
warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |      two_args::<N, 2>() // no lint
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

warning: cannot use constants which depend on generic parameters in types
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

warning: 2 warnings emitted
warning: 2 warnings emitted
------------------------------------------


---- [ui] src/test/ui/const-generics/min_const_generics/complex-expression.rs stdout ----
diff of stderr:

61    = note: type parameters may not be used in const expressions
62    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
63 
- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/complex-expression.rs:37:17
-    |
- LL |     let _ = [0; size_of::<*mut T>() + 1];
-    |
-    = note: `#[warn(const_evaluatable_unchecked)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
---
To only update this specific test, also pass `--test-args const-generics/min_const_generics/complex-expression.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/complex-expression.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-expression" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-expression/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL | struct Break0<const N: usize>([u8; { N + 1 }]);
   |                                      ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct Break1<const N: usize>([u8; { { N } }]);
   |                                        ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; N + 1];
   |                 ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _ = [0; N + 1];
   |                 ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct BreakTy0<T>(T, [u8; { size_of::<*mut T>() }]);
   |                                             ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct BreakTy1<T>(T, [u8; { { size_of::<*mut T>() } }]);
   |                                               ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; size_of::<*mut T>() + 1];
   |                                ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
error: aborting due to 7 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked.rs stdout ----
diff of stderr:

- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/const-evaluatable-unchecked.rs:5:9
-    |
- LL |     [0; std::mem::size_of::<*mut T>()];
-    |
-    = note: `#[warn(const_evaluatable_unchecked)]` on by default
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
- 
- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/const-evaluatable-unchecked.rs:16:21
-    |
- LL |         let _ = [0; Self::ASSOC];
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
- 
- 
- warning: cannot use constants which depend on generic parameters in types
-   --> $DIR/const-evaluatable-unchecked.rs:28:21
-    |
- LL |         let _ = [0; Self::ASSOC];
-    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
- 
---
To only update this specific test, also pass `--test-args const-generics/min_const_generics/const-evaluatable-unchecked.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/const-generics/generic_const_exprs/dependence_lint.rs#full
