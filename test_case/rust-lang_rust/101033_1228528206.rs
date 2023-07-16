plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 983f4daddf238d114c4adc4751c5528fc6695a5a and dc57dbd1d7cb807b5b3579cc79b9a621203ada3f
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling miri v0.1.0 (/checkout/src/tools/miri)
error[E0432]: unresolved import `rustc_const_eval::interpret::ScalarMaybeUninit`
  --> src/tools/miri/src/concurrency/weak_memory.rs:81:54
   |
81 |     alloc_range, AllocRange, InterpResult, MPlaceTy, ScalarMaybeUninit,
   |                                                      ^^^^^^^^^^^^^^^^^ no `ScalarMaybeUninit` in `interpret`
error[E0407]: method `force_int_for_alignment_check` is not a member of trait `Machine`
   --> src/tools/miri/src/machine.rs:560:5
    |
560 |       fn force_int_for_alignment_check(ecx: &MiriEvalContext<'mir, 'tcx>) -> bool {
560 |       fn force_int_for_alignment_check(ecx: &MiriEvalContext<'mir, 'tcx>) -> bool {
    |       ^  ----------------------------- help: there is an associated function with a similar name: `use_addr_for_alignment_check`
    | |
    | |
561 | |         ecx.machine.check_alignment == AlignmentCheck::Int
    | |_____^ not a member of trait `Machine`

error[E0407]: method `enforce_number_init` is not a member of trait `Machine`
   --> src/tools/miri/src/machine.rs:570:5
---

error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:577:40
    |
577 |         mutex_set_kind(this, mutex_op, ScalarMaybeUninit::Uninit)?;
    |                                        ^^^^^^^^^^^^^^^^^ use of undeclared type `ScalarMaybeUninit`
error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:578:38
    |
    |
578 |         mutex_set_id(this, mutex_op, ScalarMaybeUninit::Uninit)?;
    |                                      ^^^^^^^^^^^^^^^^^ use of undeclared type `ScalarMaybeUninit`
error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:701:40
    |
    |
701 |         rwlock_set_id(this, rwlock_op, ScalarMaybeUninit::Uninit)?;
    |                                        ^^^^^^^^^^^^^^^^^ use of undeclared type `ScalarMaybeUninit`
error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:765:46
    |
765 |         condattr_set_clock_id(this, attr_op, ScalarMaybeUninit::Uninit)?;
765 |         condattr_set_clock_id(this, attr_op, ScalarMaybeUninit::Uninit)?;
    |                                              ^^^^^^^^^^^^^^^^^ use of undeclared type `ScalarMaybeUninit`

error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:913:36
    |
913 |         cond_set_id(this, cond_op, ScalarMaybeUninit::Uninit)?;
    |                                    ^^^^^^^^^^^^^^^^^ use of undeclared type `ScalarMaybeUninit`
error[E0433]: failed to resolve: use of undeclared type `ScalarMaybeUninit`
   --> src/tools/miri/src/shims/unix/sync.rs:914:42
    |
914 |         cond_set_clock_id(this, cond_op, ScalarMaybeUninit::Uninit)?;
---

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:458:26
    |
458 |         value: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:472:29
    |
    |
472 |     ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                             ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:490:14
    |
490 |         val: ScalarMaybeUninit<Provenance>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:544:14
    |
    |
544 |         new: ScalarMaybeUninit<Provenance>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:546:29
    |
    |
546 |     ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                             ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/concurrency/data_race.rs:607:14
    |
607 |         new: ScalarMaybeUninit<Provenance>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/helpers.rs:683:29
    |
    |
683 |     ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                             ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/helpers.rs:693:26
    |
693 |         value: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/machine.rs:152:21
    |
    |
152 | static_assert_size!(ScalarMaybeUninit<Provenance>, 32);

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
  --> src/tools/miri/src/shims/unix/sync.rs:41:25
   |
   |
41 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
   |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
  --> src/tools/miri/src/shims/unix/sync.rs:48:21
   |
48 |     kind: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
  --> src/tools/miri/src/shims/unix/sync.rs:65:25
   |
   |
65 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
   |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
  --> src/tools/miri/src/shims/unix/sync.rs:78:21
   |
78 |     kind: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
  --> src/tools/miri/src/shims/unix/sync.rs:93:25
   |
   |
93 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
   |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:100:19
    |
100 |     id: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:149:25
    |
    |
149 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:156:19
    |
156 |     id: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:204:25
    |
    |
204 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:211:25
    |
211 |     clock_id: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:233:25
    |
    |
233 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:240:19
    |
240 |     id: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:282:25
    |
    |
282 | ) -> InterpResult<'tcx, ScalarMaybeUninit<Provenance>> {
    |                         ^^^^^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `ScalarMaybeUninit` in this scope
   --> src/tools/miri/src/shims/unix/sync.rs:289:25
    |
289 |     clock_id: impl Into<ScalarMaybeUninit<Provenance>>,

error[E0046]: not all trait items implemented, missing: `use_addr_for_alignment_check`
   --> src/tools/miri/src/machine.rs:535:1
    |
    |
535 | impl<'mir, 'tcx> Machine<'mir, 'tcx> for Evaluator<'mir, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `use_addr_for_alignment_check` in implementation
    |
    = help: implement the missing item: `fn use_addr_for_alignment_check(_: &rustc_const_eval::interpret::InterpCx<'mir, 'tcx, Self>) -> bool { todo!() }`
error[E0599]: no method named `to_scalar_or_uninit` found for struct `rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:531:17
    |
531 |             val.to_scalar_or_uninit(),
---

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/concurrency/data_race.rs:574:18
    |
574 |         let lt = this.binary_op(mir::BinOp::Lt, &old, &rhs)?.to_scalar()?.to_bool()?;
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0599]: no method named `to_scalar_or_uninit` found for reference `&rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:587:21
   --> src/tools/miri/src/concurrency/data_race.rs:587:21
    |
587 |             new_val.to_scalar_or_uninit(),
    |                     ^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `to_scalar_pair`
error[E0599]: no method named `to_scalar_or_uninit` found for struct `rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:590:17
    |
590 |             old.to_scalar_or_uninit(),
590 |             old.to_scalar_or_uninit(),
    |                 ^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `to_scalar_pair`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/concurrency/data_race.rs:627:31
    |
627 |         let cmpxchg_success = eq.to_scalar()?.to_bool()?
    |                               ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0599]: no method named `to_scalar_or_uninit` found for struct `rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:634:17
   --> src/tools/miri/src/concurrency/data_race.rs:634:17
    |
634 |             old.to_scalar_or_uninit(),
    |                 ^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `to_scalar_pair`

error[E0599]: no method named `to_scalar_or_uninit` found for struct `rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:644:63
    |
644 |             this.buffered_atomic_rmw(new, place, success, old.to_scalar_or_uninit())?;
    |                                                               ^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `to_scalar_pair`
error[E0599]: no method named `to_scalar_or_uninit` found for struct `rustc_const_eval::interpret::ImmTy<'_, machine::Provenance>` in the current scope
   --> src/tools/miri/src/concurrency/data_race.rs:651:67
    |
    |
651 |             this.perform_read_on_buffered_latest(place, fail, old.to_scalar_or_uninit())?;
    |                                                                   ^^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `to_scalar_pair`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/helpers.rs:120:19
    |
120 |         const_val.check_init()
---

error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:35:48
   |
35 |                     Immediate::Scalar(l) => (l.check_init()?.to_bits(size)?, 0),
   |                                                ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:37:29
   |
   |
37 |                         (l1.check_init()?.to_bits(size)?, l2.check_init()?.to_bits(size)?),
   |                             ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:37:62
   |
   |
37 |                         (l1.check_init()?.to_bits(size)?, l2.check_init()?.to_bits(size)?),
   |                                                              ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:41:48
   |
   |
41 |                     Immediate::Scalar(r) => (r.check_init()?.to_bits(size)?, 0),
   |                                                ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:43:29
   |
   |
43 |                         (r1.check_init()?.to_bits(size)?, r2.check_init()?.to_bits(size)?),
   |                             ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
  --> src/tools/miri/src/operator.rs:43:62
   |
   |
43 |                         (r1.check_init()?.to_bits(size)?, r2.check_init()?.to_bits(size)?),
   |                                                              ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/operator.rs:60:27
   |
   |
60 |                 let ptr = left.to_scalar()?.to_pointer(self)?;
   |                           ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/operator.rs:61:30
  --> src/tools/miri/src/operator.rs:61:30
   |
61 |                 let offset = right.to_scalar()?.to_machine_isize(self)?;
   |                              ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/operator.rs:74:27
  --> src/tools/miri/src/operator.rs:74:27
   |
74 |                 let ptr = left.to_scalar()?.to_pointer(self)?;
   |                           ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/operator.rs:78:21
  --> src/tools/miri/src/operator.rs:78:21
   |
78 |                     right.to_scalar()?.to_machine_usize(self)?,
   |                     ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/intrinsics/simd.rs:63:46
  --> src/tools/miri/src/shims/intrinsics/simd.rs:63:46
   |
63 |                         Op::MirOp(mir_op) => this.unary_op(mir_op, &op)?.to_scalar()?,
   |                                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/intrinsics/simd.rs:69:38
  --> src/tools/miri/src/shims/intrinsics/simd.rs:69:38
   |
69 | ...                   let op = op.to_scalar()?;
   |                                ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/intrinsics/simd.rs:82:60
  --> src/tools/miri/src/shims/intrinsics/simd.rs:82:60
   |
82 | ...                   let f = f32::from_bits(op.to_scalar()?.to_u32()?);
   |                                              ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/intrinsics/simd.rs:93:60
  --> src/tools/miri/src/shims/intrinsics/simd.rs:93:60
   |
93 | ...                   let f = f64::from_bits(op.to_scalar()?.to_u64()?);
   |                                              ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:185:49
   --> src/tools/miri/src/shims/intrinsics/simd.rs:185:49
    |
185 | ...                   let r_val = right.to_scalar()?.to_bits(right.layout.size)?;
    |                                   ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:204:39
---

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:205:48
    |
205 | ...                   let offset_count = right.to_scalar()?.to_machine_isize(this)?;
    |                                          ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:235:29
   --> src/tools/miri/src/shims/intrinsics/simd.rs:235:29
    |
235 |                     let a = this.read_immediate(&this.mplace_index(&a, i)?.into())?.to_scalar()?;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:236:29
   --> src/tools/miri/src/shims/intrinsics/simd.rs:236:29
    |
236 |                     let b = this.read_immediate(&this.mplace_index(&b, i)?.into())?.to_scalar()?;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:237:29
   --> src/tools/miri/src/shims/intrinsics/simd.rs:237:29
    |
237 |                     let c = this.read_immediate(&this.mplace_index(&c, i)?.into())?.to_scalar()?;
    |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:318:36
   --> src/tools/miri/src/shims/intrinsics/simd.rs:318:36
    |
318 | ...                   if this.binary_op(BinOp::Ge, &res, &op)?.to_scalar()?.to_bool()? {
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:330:36
   --> src/tools/miri/src/shims/intrinsics/simd.rs:330:36
    |
330 | ...                   if this.binary_op(BinOp::Le, &res, &op)?.to_scalar()?.to_bool()? {
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/intrinsics/simd.rs:399:22
   --> src/tools/miri/src/shims/intrinsics/simd.rs:399:22
    |
399 |                     .check_init()?
    |                      ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:449:57
    |
449 | ...                   this.float_to_int_unchecked(op.to_scalar()?.to_f32()?, dest.layout.ty)?.into(),
    |                                                   ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:451:57
   --> src/tools/miri/src/shims/intrinsics/simd.rs:451:57
    |
451 | ...                   this.float_to_int_unchecked(op.to_scalar()?.to_f64()?, dest.layout.ty)?.into(),
    |                                                   ^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:478:42
   --> src/tools/miri/src/shims/intrinsics/simd.rs:478:42
    |
478 |                       let src_index: u64 = this
    |  __________________________________________^
479 | |                         .read_immediate(&this.operand_index(index, i)?)?
480 | |                         .to_scalar()?
    | |_____________________________________^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:578:15
   --> src/tools/miri/src/shims/intrinsics/simd.rs:578:15
    |
578 |     let val = elem.to_scalar()?.to_int(elem.layout.size)?;
    |               ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:602:16
---

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:603:17
    |
603 |     let right = right.to_scalar()?;
    |                 ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:618:16
---

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/simd.rs:619:17
    |
619 |     let right = right.to_scalar()?;
    |                 ^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/mod.rs:229:52
   --> src/tools/miri/src/shims/intrinsics/mod.rs:229:52
    |
229 |                         ty::Float(FloatTy::F32) => x.to_scalar()?.to_f32()?.is_finite(),
    |
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<_>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/mod.rs:230:52
    |
230 |                         ty::Float(FloatTy::F64) => x.to_scalar()?.to_f64()?.is_finite(),
    |
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<_>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/mod.rs:348:53
    |
348 |                         this.float_to_int_unchecked(val.to_scalar()?.to_f32()?, dest.layout.ty)?,
    |                                                     ^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
   --> src/tools/miri/src/shims/intrinsics/mod.rs:350:53
   --> src/tools/miri/src/shims/intrinsics/mod.rs:350:53
    |
350 |                         this.float_to_int_unchecked(val.to_scalar()?.to_f64()?, dest.layout.ty)?,
    |                                                     ^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
    = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/unix/foreign_items.rs:285:50
---

error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/unix/foreign_items.rs:302:73
    |
302 |                 this.machine.tls.store_tls(key, active_thread, new_data.check_init()?, &*this.tcx)?;
    |                                                                         ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/unix/foreign_items.rs:467:56
    |
467 |                 let errnum = this.read_scalar(errnum)?.check_init()?;
---

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/unix/linux/sync.rs:36:23
   |
36 |     let addr_scalar = addr.to_scalar()?;
   |                       ^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `rustc_const_eval::interpret::Scalar<machine::Provenance>`
   = help: the trait `std::ops::Try` is not implemented for `rustc_const_eval::interpret::Scalar<machine::Provenance>`

error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
  --> src/tools/miri/src/shims/unix/macos/foreign_items.rs:25:35
---

error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/unix/macos/foreign_items.rs:177:73
    |
177 |                 this.pthread_setname_np(thread, this.read_scalar(name)?.check_init()?)?;
    |                                                                         ^^^^^^^^^^ method not found in `rustc_const_eval::interpret::Scalar<machine::Provenance>`
error[E0599]: no method named `check_init` found for enum `rustc_const_eval::interpret::Scalar` in the current scope
   --> src/tools/miri/src/shims/unix/macos/foreign_items.rs:186:52
    |
186 |                 let addr = this.read_scalar(addr)?.check_init()?;
---

---- compile_test stdout ----
diff of stderr:

-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = &ipaddr {}
-   |     -------^^^^^---------- help: try this: `if ipaddr.is_ipv4()`
-   |
-   = note: `-D clippy::redundant-pattern-matching` implied by `-D warnings`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     let _ = if let V4(_) = V4(Ipv4Addr::LOCALHOST) {
-   |             -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     let _ = if let V4(_) = gen_ipaddr() {
-   |             -------^^^^^--------------- help: try this: `if gen_ipaddr().is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     } else if let V6(_) = gen_ipaddr() {
-   |            -------^^^^^--------------- help: try this: `if gen_ipaddr().is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     -------^^^^^-------------------------- help: try this: `if V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL |     while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL |     while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
-   |     ----------^^^^^-------------------------- help: try this: `while V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-
-error: redundant pattern matching, consider using `is_ipv4()`
-   |
-   |
-LL | /     match V4(Ipv4Addr::LOCALHOST) {
-LL | |         V4(_) => true,
-LL | |         V6(_) => false,
-LL | |     };
-   | |_____^ help: try this: `V4(Ipv4Addr::LOCALHOST).is_ipv4()`
-
-error: redundant pattern matching, consider using `is_ipv6()`
-   |
-   |
-LL | /     match V6(Ipv6Addr::LOCALHOST) {
-LL | |         V4(_) => false,
-LL | |         V6(_) => true,
-LL | |     };
-   | |_____^ help: try this: `V6(Ipv6Addr::LOCALHOST).is_ipv6()`
-error: aborting due to 18 previous errors
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::all)]
 #![warn(clippy::redundant_pattern_matching)]
 #![allow(unused_must_use, clippy::needless_bool, clippy::match_like_matches_macro)]
 use std::net::{
 use std::net::{
     IpAddr::{self, V4, V6},
     Ipv4Addr, Ipv6Addr,
 
 fn main() {
 fn main() {
     let ipaddr: IpAddr = V4(Ipv4Addr::LOCALHOST);
-    if ipaddr.is_ipv4() {}
+    if let V4(_) = &ipaddr {}
 
-    if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    while V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    while V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
     if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
 
     if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
 
     if let V4(ipaddr) = V4(Ipv4Addr::LOCALHOST) {
         println!("{}", ipaddr);
 
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv4();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv6();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv6();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv4();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    let _ = if V4(Ipv4Addr::LOCALHOST).is_ipv4() {
+    let _ = if let V4(_) = V4(Ipv4Addr::LOCALHOST) {
     } else {
         false
     };
 
 
     ipaddr_const();
 
-    let _ = if gen_ipaddr().is_ipv4() {
+    let _ = if let V4(_) = gen_ipaddr() {
         1
-    } else if gen_ipaddr().is_ipv6() {
+    } else if let V6(_) = gen_ipaddr() {
     } else {
         3
     };
 }
 }
 
 fn gen_ipaddr() -> IpAddr {
     V4(Ipv4Addr::LOCALHOST)
 
 const fn ipaddr_const() {
 const fn ipaddr_const() {
-    if V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    if let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    if V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    if let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    while V4(Ipv4Addr::LOCALHOST).is_ipv4() {}
+    while let V4(_) = V4(Ipv4Addr::LOCALHOST) {}
 
-    while V6(Ipv6Addr::LOCALHOST).is_ipv6() {}
+    while let V6(_) = V6(Ipv6Addr::LOCALHOST) {}
 
-    V4(Ipv4Addr::LOCALHOST).is_ipv4();
+    match V4(Ipv4Addr::LOCALHOST) {
+        V4(_) => true,
+        V6(_) => false,
 
 
-    V6(Ipv6Addr::LOCALHOST).is_ipv6();
+    match V6(Ipv6Addr::LOCALHOST) {
+        V4(_) => false,
+        V6(_) => true,
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.fixed
To only update this specific test, also pass `--test-args redundant_pattern_matching_ipaddr.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/redundant_pattern_matching_ipaddr.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-b0e96f2e9d30bd37.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-11c942eb60796e9d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-04f014bd62aa87c5.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-507b29393c1a728f.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7dc368fb32eb8aae.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-56bce9bcc023120a.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-3e103f3c7cb1e342.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-36709515b9cb16b6.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/redundant_pattern_matching_ipaddr.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
