plain
    Checking cranelift-object v0.75.0 (https://github.com/bytecodealliance/wasmtime.git?branch=main#c71ad949)
    Checking rustc_codegen_cranelift v0.1.0 (/checkout/compiler/rustc_codegen_cranelift)
    Finished release [optimized] target(s) in 29.15s
Checking stage0 gcc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Updating git repository `https://github.com/antoyo/gccjit.rs`
---
   Compiling target-lexicon v0.10.0
    Checking libc v0.1.12
    Checking ar v0.8.0
   Compiling indexmap v1.6.2
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#c736ae6c)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#c736ae6c)
    Checking object v0.25.3
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0407]: method `set_frame_pointer_elimination` is not a member of trait `MiscMethods`
    |
    |
382 | /     fn set_frame_pointer_elimination(&self, _llfn: RValue<'gcc>) {
383 | |         // TODO
384 | |         //attributes::set_frame_pointer_elimination(self, llfn)
385 | |     }
    | |_____^ not a member of trait `MiscMethods`

error[E0050]: method `load` has 3 parameters but the declaration in trait `load` has 4
    |
    |
920 |     fn load(&mut self, ptr: RValue<'gcc>, _align: Align) -> RValue<'gcc> {
    |
    |
    = note: `load` from trait: `fn(&mut Self, <Self as BackendTypes>::Type, <Self as BackendTypes>::Value, Align) -> <Self as BackendTypes>::Value`

error[E0050]: method `volatile_load` has 2 parameters but the declaration in trait `rustc_codegen_ssa::traits::BuilderMethods::volatile_load` has 3
    |
    |
935 |     fn volatile_load(&mut self, ptr: RValue<'gcc>) -> RValue<'gcc> {
    |
    |
    = note: `volatile_load` from trait: `fn(&mut Self, <Self as BackendTypes>::Type, <Self as BackendTypes>::Value) -> <Self as BackendTypes>::Value`

error[E0050]: method `atomic_load` has 4 parameters but the declaration in trait `rustc_codegen_ssa::traits::BuilderMethods::atomic_load` has 5
    |
    |
942 |     fn atomic_load(&mut self, ptr: RValue<'gcc>, order: AtomicOrdering, size: Size) -> RValue<'gcc> {
    |
    |
    = note: `atomic_load` from trait: `fn(&mut Self, <Self as BackendTypes>::Type, <Self as BackendTypes>::Value, AtomicOrdering, rustc_target::abi::Size) -> <Self as BackendTypes>::Value`
error[E0046]: not all trait items implemented, missing: `const_data_from_alloc`
  --> src/common.rs:92:1
   |
   |
92 | impl<'gcc, 'tcx> ConstMethods<'tcx> for CodegenCx<'gcc, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `const_data_from_alloc` in implementation
   |
   = help: implement the missing item: `fn const_data_from_alloc(&self, _: &Allocation) -> <Self as BackendTypes>::Value { todo!() }`
error[E0046]: not all trait items implemented, missing: `set_frame_pointer_type`
   --> src/context.rs:287:1
    |
    |
287 | impl<'gcc, 'tcx> MiscMethods<'tcx> for CodegenCx<'gcc, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `set_frame_pointer_type` in implementation
    |
    = help: implement the missing item: `fn set_frame_pointer_type(&self, _: <Self as BackendTypes>::Function) { todo!() }`
error[E0061]: this function takes 3 arguments but 4 arguments were supplied
   --> src/lib.rs:121:13
    |
    |
121 |             link_binary::<crate::archive::ArArchiveBuilder<'_>>(
122 |                 sess,
    |                 ----
123 |                 &codegen_results,
    |                 ----------------
    |                 ----------------
124 |                 outputs,
    |                 -------
125 |                 &codegen_results.crate_info.local_crate_name.as_str(),
    |
note: function defined here
   --> /checkout/compiler/rustc_codegen_ssa/src/back/link.rs:52:8
    |
    |
52  | pub fn link_binary<'a, B: ArchiveBuilder<'a>>(

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> src/builder.rs:105:35
    |
    |
105 |         let previous_value = self.atomic_load(dst, load_ordering.clone(), Size::from_bytes(size));
    |                                   |
    |                                   expected 4 arguments
    |
note: associated function defined here
---

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
   --> src/builder.rs:991:37
    |
991 |                     let load = self.load(place.llval, place.align);
    |                                     |
    |                                     expected 3 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:140:8
    |
140 |     fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: Align) -> Self::Value;

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> src/builder.rs:1004:37
     |
     |
1004 |                     let load = self.load(llptr, align);
     |                                     |
     |                                     expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:140:8
     |
140  |     fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: Align) -> Self::Value;

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> src/builder.rs:1291:28
     |
     |
1291 |             let val = self.load(src, src_align);
     |                            |
     |                            expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:140:8
     |
140  |     fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: Align) -> Self::Value;

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
    --> src/builder.rs:1309:28
     |
     |
1309 |             let val = self.load(src, src_align);
     |                            |
     |                            expected 3 arguments
     |
note: associated function defined here
note: associated function defined here
    --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:140:8
     |
140  |     fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: Align) -> Self::Value;


error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   --> src/common.rs:253:13
253 |             Scalar::Ptr(ptr) => {
    |             ^^^^^^^^^^^^^^^^ expected 2 fields, found 1
    | 
   ::: /checkout/compiler/rustc_middle/src/mir/interpret/value.rs:135:5
   ::: /checkout/compiler/rustc_middle/src/mir/interpret/value.rs:135:5
    |
135 |     Ptr(Pointer<Tag>, u8),
    |     --------------------- tuple variant defined here
    |
help: use `_` to explicitly ignore each field
253 |             Scalar::Ptr(ptr, _) => {
    |                            ^^^

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/consts.rs:420:19
    |
420 |     for &(offset, ((), alloc_id)) in alloc.relocations().iter() {
    |                   ^^^^^^^^^^^^^^     -------------------------- this expression has type `&(rustc_target::abi::Size, AllocId)`
    |                   expected struct `AllocId`, found tuple
    |
    = note: expected struct `AllocId`
                found tuple `(_, _)`
                found tuple `(_, _)`

error[E0277]: the trait bound `rustc_mir::interpret::Scalar: From<rustc_mir::interpret::Pointer<_>>` is not satisfied
    |
    |
444 |             interpret::Pointer::new(alloc_id, Size::from_bytes(ptr_offset)).into(),
    |                                                                             ^^^^ the trait `From<rustc_mir::interpret::Pointer<_>>` is not implemented for `rustc_mir::interpret::Scalar`
    = help: the following implementations were found:
              <rustc_mir::interpret::Scalar<Tag> as From<ScalarInt>>
              <rustc_mir::interpret::Scalar<Tag> as From<ScalarInt>>
              <rustc_mir::interpret::Scalar<Tag> as From<rustc_apfloat::ieee::IeeeFloat<rustc_apfloat::ieee::DoubleS>>>
              <rustc_mir::interpret::Scalar<Tag> as From<rustc_apfloat::ieee::IeeeFloat<rustc_apfloat::ieee::SingleS>>>
    = note: required because of the requirements on the impl of `Into<rustc_mir::interpret::Scalar>` for `rustc_mir::interpret::Pointer<_>`
error[E0599]: no method named `all_crate_nums` found for struct `TyCtxt<'tcx>` in the current scope
  --> src/debuginfo.rs:78:38
   |
   |
78 |         for crate_num in self.cx.tcx.all_crate_nums(()).iter().copied().chain(iter::once(LOCAL_CRATE)) {

error[E0061]: this function takes 2 arguments but 1 argument was supplied
   --> src/intrinsic/mod.rs:163:37
    |
    |
163 |                     let load = self.volatile_load(ptr);
    |                                     |
    |                                     expected 2 arguments
    |
note: associated function defined here
note: associated function defined here
   --> /checkout/compiler/rustc_codegen_ssa/src/traits/builder.rs:141:8
    |
141 |     fn volatile_load(&mut self, ty: Self::Type, ptr: Self::Value) -> Self::Value;

error: aborting due to 17 previous errors

Some errors have detailed explanations: E0023, E0046, E0050, E0061, E0277, E0308, E0407, E0599.
