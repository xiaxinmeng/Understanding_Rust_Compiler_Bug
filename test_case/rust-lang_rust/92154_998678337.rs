plain
.................................................................................................... 1500/12512
.................................................................................................... 1600/12512
i................................................................................................... 1700/12512
.................................................................................................... 1800/12512
............i..........................................................F............................ 1900/12512
......................................F..FF.....F............F.F............F.F......F..F......F.... 2000/12512
..........................................F.................F.................i..................F.. 2100/12512
.............F......F............................................................................... 2200/12512
.................................................................................................... 2400/12512
.............................................................................F...................... 2500/12512
.................................................................................................... 2600/12512
.................................................................................................... 2700/12512
---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/const-argument-if-length.full.stdout
To only update this specific test, also pass `--test-args const-generics/const-argument-if-length.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-argument-if-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-if-length.full/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0277]: the size for values of type `T` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:15:12
   |
LL | pub struct AtLeastByte<T: ?Sized> {
   |                        - this type parameter needs to be `std::marker::Sized`
LL |     value: T,
   |
   |
   = note: only the last field of a struct may have a dynamically sized type
   = help: change the field's type to have a statically known size
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL - pub struct AtLeastByte<T: ?Sized> {
LL + pub struct AtLeastByte<T> {
   | 
help: borrowed types always have a statically known size
LL |     value: &T,
   |            +
   |            +
help: the `Box` type always has a statically known size and allocates its contents in the heap
LL |     value: Box<T>,
   |            ++++ +

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/const-argument-if-length.rs:17:10
   |
LL |     pad: [u8; is_zst::<T>()],
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); is_zst::<T>()]:`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2/abstract-const-as-cast-2.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/abstract-const-as-cast-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2/auxiliary"
------------------------------------------
IN HEREE
IN HEREE
IN HEREE
IN HEREE

------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2.rs:6:25
   |
LL | struct Foo<const N: u8>([u8; N as usize])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N as usize]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2.rs:12:26
   |
   |
LL | struct Foo2<const N: u8>(Evaluatable::<{N as u128}>) where Evaluatable<{N as usize as u128 }>:;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); {N as u128}]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-2.rs:16:25
   |
   |
LL | struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 1) as usize]:;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); (N + 2) as usize]:`
error: aborting due to 3 previous errors


------------------------------------------
---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-consts-as-cast-5/abstract-consts-as-cast-5.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/abstract-consts-as-cast-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-consts-as-cast-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-consts-as-cast-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-consts-as-cast-5/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-consts-as-cast-5.rs:5:11
   |
LL |     bar::<{ N as usize as usize }>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N as usize as usize }]:`
error: aborting due to previous error


------------------------------------------
---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/abstract-const-as-cast-3.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/abstract-const-as-cast-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/auxiliary"
------------------------------------------
IN HEREE
IN HEREE
IN HEREE
---
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:17:5
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:14:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:17:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:20:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
note: required by a bound in `use_trait_impl::assert_impl`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:14:23
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:20:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:23:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12_u128`, found `13_u128`
   = note: expected type `12_u128`
              found type `13_u128`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:25:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13_u128`, found `14_u128`
   = note: expected type `13_u128`
              found type `14_u128`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:35:5
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as u128 }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:35:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as u128 }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as u128 }`
              found type `{ O as u128 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:38:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { O as u128 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1 }, { N as _ }>`
   |
   |
LL | impl<const O: usize> Trait for HasCastInTraitImpl<O, { O as u128 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `use_trait_impl_2::assert_impl`
   |
   |
LL |     fn assert_impl<T: Trait>() {}
   |                       ^^^^^ required by this bound in `use_trait_impl_2::assert_impl`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:38:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<{ N + 1 }, { N as _ }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N as _ }`, found `{ O as u128 }`
   |
   = note: expected type `{ N as _ }`
              found type `{ O as u128 }`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:41:5
   |
   |
LL |     assert_impl::<HasCastInTraitImpl<13, { 12 as u128 }>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `12_u128`, found `13_u128`
   = note: expected type `12_u128`
              found type `13_u128`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs:43:5
   |
LL |     assert_impl::<HasCastInTraitImpl<14, 13>>();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `13_u128`, found `14_u128`
   = note: expected type `13_u128`
              found type `14_u128`

error: aborting due to 12 previous errors
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/different-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/different-fn/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:10:5
   |
LL |     [0; size_of::<Foo<T>>()]
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `size_of::<T>()`, found `size_of::<Foo<T>>()`
   = note: expected type `size_of::<T>()`
   = note: expected type `size_of::<T>()`
              found type `size_of::<Foo<T>>()`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/different-fn.rs:10:9
   |
   |
LL |     [0; size_of::<Foo<T>>()]
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<Foo<T>>()]:`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/cross_crate_predicate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate_predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate_predicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate_predicate/auxiliary"
------------------------------------------
IN HEREE
IN HEREE
IN HEREE
---
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate_predicate.rs:7:13
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate_predicate.rs:7:13
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/cross_crate_predicate.rs:7:13
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: aborting due to 4 previous errors


------------------------------------------
---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/issue-62504.full.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-62504.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs:18:21
   |
LL |         ArrayHolder([0; Self::SIZE])
   |                     ^^^^^^^^^^^^^^^ expected `X`, found `Self::SIZE`
   = note: expected type `X`
              found type `Self::SIZE`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs:18:25
   |
LL |         ArrayHolder([0; Self::SIZE])
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::SIZE]:`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-83765/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs:30:5
   |
LL |     fn size(&self) -> [usize; DIM] {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs:32:24
   |
LL |         self.reference.size()
   |                        ^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::size`
   |
   |
LL |     fn size(&self) -> [usize; Self::DIM];
   |                               ^^^^^^^^^ required by this bound in `TensorSize::size`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-83765.rs:32:9
   |
LL |         self.reference.size()
LL |         self.reference.size()
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/needs_where_clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/needs_where_clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/needs_where_clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/needs_where_clause/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/needs_where_clause.rs:11:6
   |
LL |   b: [f32; complex_maths::<T>(N)],
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); complex_maths::<T>(N)]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/no_where_clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/no_where_clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_where_clause" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/no_where_clause/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/no_where_clause.rs:10:6
   |
LL |   b: [f32; complex_maths(N)],
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); complex_maths(N)]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-85848.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0277]: the trait bound `(): _Contains<&C>` is not satisfied
   |
   |
LL |     writes_to_specific_path(&cap);
   |     ----------------------- ^^^^ the trait `_Contains<&C>` is not implemented for `()`
   |     required by a bound introduced by this call
   |
   |
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24:29
   |
   |
LL |     writes_to_specific_path(&cap);
   |     ----------------------- ^^^^
   |     required by a bound introduced by this call
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { contains::<T, U>() }]:`
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/unify-op-with-fn-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/unify-op-with-fn-call.rs:28:12
   |
LL |     bar2::<{ std::ops::Add::add(N, N) }>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { std::ops::Add::add(N, N) }]:`
error: aborting due to previous error


------------------------------------------
---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739.full/issue-67739.full.stdout
To only update this specific test, also pass `--test-args const-generics/issues/issue-67739.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739.full/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-67739.rs:11:15
   |
LL |         [0u8; mem::size_of::<Self::Associated>()];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); mem::size_of::<Self::Associated>()]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-71202.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-71202.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-71202" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-71202/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-71202.rs:11:5
   |
LL | /     const ITEM_IS_COPY: [(); 1 - { //~ ERROR unconstrained generic constant
LL | |         trait NotCopy {
LL | |             const VALUE: bool = false;
LL | |         }
...  |
LL | |         <IsCopy<T>>::VALUE
LL | |     } as usize] = [];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); 1 - { //~ ERROR unconstrained generic constant
                   trait NotCopy {
                       const VALUE: bool = false;
           
           
                   impl<__Type: ?Sized> NotCopy for __Type {}
           
                   struct IsCopy<__Type: ?Sized>(PhantomData<__Type>);
           
                   impl<__Type> IsCopy<__Type>
                   where
                       __Type: Sized + Copy,
                       const VALUE: bool = true;
                   }
           
           
                   <IsCopy<T>>::VALUE
               } as usize]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-84659.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-84659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-84659/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-84659.rs:8:15
   |
LL |     type Baz: Bar<{ Self::N }>;
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { Self::N }]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-90455.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-90455.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-90455" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-90455/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-90455.rs:5:8
   |
LL |     n: [u64; num_limbs(N)],
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); num_limbs(N)]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
------------------------------------------
IN HEREE
IN HEREE
IN HEREE
---
------------------------------------------
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:44:5
   |
LL |     fn size(&self) -> [usize;DIM] {self.size}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:51:5
   |
LL |     fn bget(&self,index:[usize;DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:78:5
   |
LL |     fn size(&self) -> [usize;DIM] {self.reference.size()}
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error[E0308]: method not compatible with trait
error[E0308]: method not compatible with trait
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:88:5
   |
LL |     fn bget(&self,index:[usize;DIM]) -> Option<Self::Element> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:54:18
   |
LL |         if !self.inbounds(index) {return None}
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::inbounds`
   |
   |
LL |     fn inbounds(&self,index : [usize;Self::DIM]) -> bool {
   |                                      ^^^^^^^^^ required by this bound in `TensorSize::inbounds`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:54:27
   |
   |
LL |         if !self.inbounds(index) {return None}
   |                           ^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:57:25
   |
LL |         let size = self.size();
   |                         ^^^^
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::size`
   |
   |
LL |     fn size(&self) -> [usize;Self::DIM];
   |                              ^^^^^^^^^ required by this bound in `TensorSize::size`

error[E0277]: the trait bound `[usize; _]: Default` is not satisfied
   |
   |
LL |         let newindex : [usize;T::DIM] = Default::default();
   |                                         ^^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[usize; _]`
   |
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | impl<'a,T : Broadcastable,const DIM : usize>  Broadcastable for LazyUpdim<'a,T,{T::DIM},DIM> where [usize; _]: Default

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:78:51
   |
   |
LL |     fn size(&self) -> [usize;DIM] {self.reference.size()}
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `TensorSize::size`
   |
   |
LL |     fn size(&self) -> [usize;Self::DIM];
   |                              ^^^^^^^^^ required by this bound in `TensorSize::size`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:78:36
   |
   |
LL |     fn size(&self) -> [usize;DIM] {self.reference.size()}
   |                                    ^^^^^^^^^^^^^^^^^^^^^ expected `DIM`, found `Self::DIM`
   = note: expected type `DIM`
              found type `Self::DIM`

error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:90:24
   |
LL |         self.reference.bget(index).map(&self.closure)
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); Self::DIM]:`
note: required by a bound in `Broadcastable::bget`
   |
   |
LL |     fn bget(&self, index:[usize;Self::DIM]) -> Option<Self::Element>;
   |                                 ^^^^^^^^^ required by this bound in `Broadcastable::bget`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issues/issue-83765.rs:90:29
   |
   |
LL |         self.reference.bget(index).map(&self.closure)
   |                             ^^^^^ expected `Self::DIM`, found `DIM`
   = note: expected type `Self::DIM`
              found type `DIM`

error: aborting due to 12 previous errors
---
To only update this specific test, also pass `--test-args consts/const-needs_drop-monomorphic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-needs_drop-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-needs_drop-monomorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-needs_drop-monomorphic/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0599]: no function or associated item named `assert` found for struct `Bool<{ std::mem::needs_drop::<T>() }>` in the current scope
   |
   |
LL | struct Bool<const B: bool> {}
   | -------------------------- function or associated item `assert` not found for this
...
LL |     Bool::<{ std::mem::needs_drop::<T>() }>::assert();
   |                                              ^^^^^^ function or associated item cannot be called on `Bool<{ std::mem::needs_drop::<T>() }>` due to unsatisfied trait bounds
error: unconstrained generic constant
  --> /checkout/src/test/ui/consts/const-needs_drop-monomorphic.rs:11:5
   |
   |
LL |     Bool::<{ std::mem::needs_drop::<T>() }>::assert();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { std::mem::needs_drop::<T>() }]:`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.

---
To only update this specific test, also pass `--test-args simd/array-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/array-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/array-trait/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/simd/array-trait.rs:23:23
   |
LL | pub struct T<S: Simd>([S::Lane; S::SIZE]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); S::SIZE]:`
error: aborting due to previous error


------------------------------------------
---
To only update this specific test, also pass `--test-args specialization/issue-51892.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-51892.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-51892" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-51892/auxiliary"
------------------------------------------
IN HEREE

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/specialization/issue-51892.rs:14:17
   |
LL |     type Type = [u8; std::mem::size_of::<<T as Trait>::Type>()];
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<<T as Trait>::Type>()]:`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12373 passed; 20 failed; 119 ignored; 0 measured; 0 filtered out; finished in 158.26s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:50
