plain
........................................................................................ 1848/13399
........................................................................................ 1936/13399
................................i....................................................... 2024/13399
........................................................................................ 2112/13399
.........F.F............................................................................ 2200/13399
........................................................................................ 2376/13399
........................................................................................ 2464/13399
........................................................................................ 2552/13399
........................................................................................ 2640/13399
---
---- [ui] src/test/ui/const-generics/generic_const_exprs/different-fn.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/eval-privacy/auxiliary"
stdout: none
stderr: none

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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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

