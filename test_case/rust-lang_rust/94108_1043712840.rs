plain

- error[E0631]: type mismatch in function arguments
-   --> $DIR/issue-60283.rs:17:13
-    |
- LL |     foo((), drop)
-    |     |       |
-    |     |       |
-    |     |       expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`
-    |     |       found signature of `fn(()) -> _`
-    |     required by a bound introduced by this call
- note: required by a bound in `foo`
-   --> $DIR/issue-60283.rs:12:16
-    |
-    |
- LL | pub fn foo<T, F>(_: T, _: F)
-    |        --- required by a bound in this
- ...
- LL |     F: for<'a> FnMut(<T as Trait<'a>>::Item),
-    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `foo`
- 
- error[E0277]: the size for values of type `<() as Trait<'_>>::Item` cannot be known at compilation time
-   --> $DIR/issue-60283.rs:17:13
-    |
- LL |     foo((), drop)
-    |     |
-    |     required by a bound introduced by this call
-    |
-    |
-    = help: the trait `Sized` is not implemented for `<() as Trait<'_>>::Item`
- note: required by a bound in `std::mem::drop`
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
-    |
- LL | pub fn drop<T>(_x: T) {}
-    |             ^ required by this bound in `std::mem::drop`
- help: consider further restricting the associated type
-    |
- LL | fn main() where <() as Trait<'_>>::Item: Sized {
- 
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0277, E0631.
---
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-60283.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-60283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-60283" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-60283/auxiliary"
------------------------------------------

------------------------------------------
stderr:
