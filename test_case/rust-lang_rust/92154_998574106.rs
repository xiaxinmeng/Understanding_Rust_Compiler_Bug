plain
.................................................................................................... 1500/12512
.................................................................................................... 1600/12512
.i.................................................................................................. 1700/12512
.................................................................................................... 1800/12512
............i......................................................................................F 1900/12512
..........................................F......F..............F.............F..................... 2000/12512
..............................................................................iF.................... 2100/12512
..................F.....................FF.......................................................... 2200/12512
.................................................................................................... 2400/12512
.................................................................................................... 2500/12512
.................................................................................................... 2600/12512
.................................................................................................... 2700/12512
---
failures:

---- [ui] ui/const-generics/defaults/generic-expr-default-mismatched-types.rs stdout ----
normalized stdout:
Some("{ N + 2 }")


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-mismatched-types/generic-expr-default-mismatched-types.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-mismatched-types/generic-expr-default-mismatched-types.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/generic-expr-default-mismatched-types.rs`

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/generic-expr-default-mismatched-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-mismatched-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default-mismatched-types/auxiliary"
------------------------------------------
------------------------------------------
Some("{ N + 2 }")
------------------------------------------
stderr:
------------------------------------------
error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default-mismatched-types.rs:12:5
   |
LL |     Foo::<N, { N + 2 }>
   |     ^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ N + 2 }`
   |
   = note: expected type `{ N + 1 }`
              found type `{ N + 2 }`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/abstract-const-as-cast-3.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/abstract-const-as-cast-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/abstract-const-as-cast-3/auxiliary"
------------------------------------------
None
None
None
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
None

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



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/issue-62504.full.stdout
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-62504.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-62504.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-62504.full/auxiliary"
------------------------------------------
None

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
None
None
None
None

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
To only update this specific test, also pass `--test-args const-generics/issues/issue-72845.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-72845.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-72845" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-72845/auxiliary"
------------------------------------------
None

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0119]: conflicting implementations of trait `Foo`
  --> /checkout/src/test/ui/const-generics/issues/issue-72845.rs:44:1
   |
LL | impl<T: Spec1> Foo for T {
   | ------------------------ first implementation here
...
LL | impl<T: Spec2> Foo for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
------------------------------------------
None
None
None
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


---- [ui] ui/const-generics/mismatched-const-types-precedence.rs stdout ----
normalized stdout:
Some("{ 3 * { 2 + N + K } }")
Some("{ 3 * { 2 + N + K } }")
Some("{ 3 * { 2 + N } * K }")
Some("{ 3 * { 2 + N } * K }")
Some("{ 3 * { 2 + N } * K + L }")
Some("{ 3 * { 2 + N } * K + L }")


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence/mismatched-const-types-precedence.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence/mismatched-const-types-precedence.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/mismatched-const-types-precedence.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence/auxiliary"
------------------------------------------
------------------------------------------
Some("{ 3 * { 2 + N + K } }")
Some("{ 3 * { 2 + N + K } }")
Some("{ 3 * { 2 + N } * K }")
Some("{ 3 * { 2 + N } * K }")
Some("{ 3 * { 2 + N } * K + L }")
Some("{ 3 * { 2 + N } * K + L }")
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N + K } }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |                                                                           ^^^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:10
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N + K }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N + K`, found `{ 3 * { 2 + N + K } }`
   |
   = note: expected type `3 * 2 + N + K`
              found type `{ 3 * { 2 + N + K } }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N + K } }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |                                                                           ^^^^^^^^^^^ required by this bound in `foo`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:13:69
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |    ---                                                              ^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K }]:`
note: required by a bound in `foo2`
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |                                                                            ^^^^^^^^^ required by this bound in `foo2`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:10
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N * K }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N * K`, found `{ 3 * { 2 + N } * K }`
   |
   = note: expected type `3 * 2 + N * K`
              found type `{ 3 * { 2 + N } * K }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K }]:`
note: required by a bound in `foo2`
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |                                                                            ^^^^^^^^^ required by this bound in `foo2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:25:70
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |    ----                                                              ^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
note: required by a bound in `foo3`
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- required by a bound in this
LL |   -> [u32; 3 * M * {K + L}] {}
   |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:10
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N * K + L }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N * K + L`, found `{ 3 * { 2 + N } * K + L }`
   |
   = note: expected type `3 * 2 + N * K + L`
              found type `{ 3 * { 2 + N } * K + L }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
note: required by a bound in `foo3`
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- required by a bound in this
LL |   -> [u32; 3 * M * {K + L}] {}
   |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:38:6
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- implicitly returns `()` as its body has no tail or `return` expression
LL |   -> [u32; 3 * M * {K + L}] {}
   |      ^^^^^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/const-generics/mismatched-const-errors.rs stdout ----
normalized stdout:
Some("{ N + 2 + 1 }")
Some("{ N + 1 + 1 }")
Some("{ N + 1 + 1 }")
Some("{ N - 1 + 1 }")
Some("{ N + N + 1 }")


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/mismatched-const-errors.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/mismatched-const-errors.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/mismatched-const-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mismatched-const-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/auxiliary"
------------------------------------------
------------------------------------------
Some("{ N + 2 + 1 }")
Some("{ N + 1 + 1 }")
Some("{ N + 1 + 1 }")
Some("{ N - 1 + 1 }")
Some("{ N + N + 1 }")
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:9:30
   |
LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 2 + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 2}, N>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:9:30
   |
   |
LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N + 2 + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ N + 2 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:15:30
   |
   |
LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N + 1 }>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:15:30
   |
   |
LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ N + 1 + 1 }`
   |
   = note: expected type `{ N + 1 }`
              found type `{ N + 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:21:30
   |
   |
LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N - 1}>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:21:30
   |
   |
LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N - 1}`, found `{ N + 1 + 1 }`
   |
   = note: expected type `{ N - 1}`
              found type `{ N + 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:27:44
   |
   |
LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N - 1 + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N - 1}, N>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:27:44
   |
   |
LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N - 1 + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ N - 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:33:30
   |
   |
LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + N + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + N }, { 2 * N }>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:33:30
   |
   |
LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ N + N + 1 }`
   |
   = note: expected type `{ 2 * N }`
              found type `{ N + N + 1 }`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12384 passed; 9 failed; 119 ignored; 0 measured; 0 filtered out; finished in 154.53s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:52
