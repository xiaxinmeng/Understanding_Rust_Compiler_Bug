plain
...........................................................iii.......................... 13640/13720
................................................................................
failures:

---- [ui] src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs stdout ----

31    |       ----- this trait cannot be made into an object...
31    |       ----- this trait cannot be made into an object...
32 LL |     fn ptr(self: Ptr<Self>);
33    |                  ^^^^^^^^^ ...because method `ptr`'s `self` parameter cannot be dispatched on
- note: required because of the requirements on the impl of `CoerceUnsized<Ptr<dyn Trait>>` for `Ptr<{integer}>`
+ note: required for `Ptr<{integer}>` to implement `CoerceUnsized<Ptr<dyn Trait>>`
35   --> $DIR/feature-gate-dispatch-from-dyn-missing-impl.rs:20:40
36    |
37 LL | impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T> {}

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl/feature-gate-dispatch-from-dyn-missing-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0038]: the trait `Trait` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs:32:25
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |     fn ptr(self: Ptr<Self>);
   |                  --------- help: consider changing method `ptr`'s `self` parameter to be `&self`: `&Self`
...
LL |     Ptr(Box::new(4)) as Ptr<dyn Trait>;
   |                         ^^^^^^^^^^^^^^ `Trait` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs:25:18
   |
LL | trait Trait {
LL | trait Trait {
   |       ----- this trait cannot be made into an object...
LL |     fn ptr(self: Ptr<Self>);
   |                  ^^^^^^^^^ ...because method `ptr`'s `self` parameter cannot be dispatched on
error[E0038]: the trait `Trait` cannot be made into an object
  --> /checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs:32:5
   |
   |
LL |     fn ptr(self: Ptr<Self>);
   |                  --------- help: consider changing method `ptr`'s `self` parameter to be `&self`: `&Self`
...
LL |     Ptr(Box::new(4)) as Ptr<dyn Trait>;
   |     ^^^^^^^^^^^^^^^^ `Trait` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs:25:18
   |
LL | trait Trait {
LL | trait Trait {
   |       ----- this trait cannot be made into an object...
LL |     fn ptr(self: Ptr<Self>);
   |                  ^^^^^^^^^ ...because method `ptr`'s `self` parameter cannot be dispatched on
note: required for `Ptr<{integer}>` to implement `CoerceUnsized<Ptr<dyn Trait>>`
  --> /checkout/src/test/ui/feature-gates/feature-gate-dispatch-from-dyn-missing-impl.rs:20:40
   |
LL | impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T> {}
   |                                        ^^^^^^^^^^^^^^^^^^^^^     ^^^^^^
   = note: required by cast to type `Ptr<dyn Trait>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0038`.
------------------------------------------
