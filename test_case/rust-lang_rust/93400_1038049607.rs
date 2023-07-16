plain
.................................................................................................... 12200/12632
.................................................................................................... 12300/12632
.................................................................................................... 12400/12632
.................................................................................................... 12500/12632
...........................iiiF..................................................................... 12600/12632
failures:

---- [ui] ui/variance/variance-unused-type-param.rs stdout ----
diff of stderr:
diff of stderr:

34    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
36 error[E0392]: parameter `T` is never used
-   --> $DIR/variance-unused-type-param.rs:19:24
+   --> $DIR/variance-unused-type-param.rs:22:24
38    |
38    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
39 LL | struct WithWhereBounds<T> where T: Sized {}
40    |                        ^ unused parameter

42    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
44 error[E0392]: parameter `T` is never used
-   --> $DIR/variance-unused-type-param.rs:19:27
+   --> $DIR/variance-unused-type-param.rs:25:27
46    |
46    |
47 LL | struct WithOutlivesBounds<T: 'static> {}
48    |                           ^ unused parameter

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/variance-unused-type-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:6:19
   |
LL | struct SomeStruct<A> { x: u32 }
   |                   ^ unused parameter
   |
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead

error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:9:15
   |
LL | enum SomeEnum<A> { Nothing }
   |               ^ unused parameter
   |
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `A` to be a const parameter, use `const A: usize` instead
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:13:15
   |
   |
LL | enum ListCell<T> {
   |               ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `T` to be a const parameter, use `const T: usize` instead
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:19:19
   |
   |
LL | struct WithBounds<T: Sized> {}
   |                   ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:22:24
   |
   |
LL | struct WithWhereBounds<T> where T: Sized {}
   |                        ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:25:27
   |
   |
LL | struct WithOutlivesBounds<T: 'static> {}
   |                           ^ unused parameter
   |
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0392`.

