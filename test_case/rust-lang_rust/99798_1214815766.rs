plain
........................................................i......ii....................... 1760/13384
........................................................................................ 1848/13384
........................................................................................ 1936/13384
............................i........................................................... 2024/13384
.............................................................................F.......... 2112/13384
........F............................................F.........F........................ 2200/13384
........................................................................................ 2376/13384
........................................................................................ 2464/13384
........................................................................................ 2552/13384
........................................................................................ 2640/13384
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
LL | where
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:22
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:26
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-1.rs:14:13
   |
   |
LL |     let _ = Foo::<N>([1; N as usize]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
note: required by a bound in `Foo`
   |
   |
LL | struct Foo<const N: u8>([u8; N as usize])
LL | where
LL | where
LL |     [(); N as usize]:;
   |          ^^^^^^^^^^ required by this bound in `Foo`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     substs2::<{ L - 1 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { L - 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs:26:5
   |
   |
LL |     substs2::<{ L - 1 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { L - 1 }]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/subexprs_are_const_evalutable.rs:10:5
   |
   |
LL |     make_array::<{ N * 2 }>()
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N * 2 }]:`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/privacy/where-priv-type.rs stdout ----
diff of stderr:

52    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
53    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
54 
- error[E0446]: private type `fn(u8) -> u8 {my_const_fn}` in public interface
-   --> $DIR/where-priv-type.rs:80:5
-    |
- LL |     type AssocTy = Const<{ my_const_fn(U) }>;
-    |     ^^^^^^^^^^^^ can't leak private type
- ...
- LL | const fn my_const_fn(val: u8) -> u8 {
-    | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
- error: aborting due to 2 previous errors; 4 warnings emitted
+ error: aborting due to previous error; 4 warnings emitted
65 
66 For more information about this error, try `rustc --explain E0446`.
---
To only update this specific test, also pass `--test-args privacy/where-priv-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/where-priv-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/where-priv-type/auxiliary"
stdout: none
--- stderr -------------------------------
warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub struct S
   | ^^^^^^^^^^^^
   |
   |
   = note: `#[warn(private_in_public)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | pub enum E
   | ^^^^^^^^^^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

warning: private type `PrivTy` in public interface (error E0446)
   |
LL | / pub fn f()
LL | / pub fn f()
LL | | //~^ WARNING private type `PrivTy` in public interface
LL | | //~| WARNING hard error
LL | | where
LL | |     PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


error[E0446]: private type `PrivTy` in public interface
   |
LL | struct PrivTy;
LL | struct PrivTy;
   | ------------- `PrivTy` declared as private
LL | impl S
   | ^^^^^^ can't leak private type


warning: private type `PrivTy` in public interface (error E0446)
   |
LL | /     pub fn f()
LL | /     pub fn f()
LL | |     //~^ WARNING private type `PrivTy` in public interface
LL | |     //~| WARNING hard error
LL | |     where
LL | |         PrivTy:
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

