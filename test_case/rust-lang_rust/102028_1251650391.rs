plain
    Checking addr2line v0.17.0
    Checking color-eyre v0.6.2
    Checking ui_test v0.3.1
    Checking libffi v3.0.1
error[E0658]: `let...else` statements are unstable
    |
    |
799 | /         let Some(start_fn) = this.tcx.lang_items().start_fn() else {
800 | |             // no_std situations
802 | |         };
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
720 | /             let Provenance::Concrete { alloc_id, .. } = ptr.provenance else {
721 | |                 panic!("extern_statics cannot contain wildcards")
    | |______________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
   |
66 | / ...                   let ty::Float(float_ty) = op.layout.ty.kind() else {
66 | / ...                   let ty::Float(float_ty) = op.layout.ty.kind() else {
67 | | ...                       span_bug!(this.cur_span(), "{} operand is not a float", intrinsic_name)
   | |________________________^
   |
   = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
   = help: add `#![feature(let_else)]` to the crate attributes to enable
   = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
   |
76 | / ...                   let ty::Float(float_ty) = op.layout.ty.kind() else {
76 | / ...                   let ty::Float(float_ty) = op.layout.ty.kind() else {
77 | | ...                       span_bug!(this.cur_span(), "{} operand is not a float", intrinsic_name)
   | |________________________^
   |
   = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
   = help: add `#![feature(let_else)]` to the crate attributes to enable
   = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
242 | /                     let ty::Float(float_ty) = dest.layout.ty.kind() else {
242 | /                     let ty::Float(float_ty) = dest.layout.ty.kind() else {
243 | |                         span_bug!(this.cur_span(), "{} operand is not a float", intrinsic_name)
    | |______________________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
469 | /                 let ty::Array(_, index_len) = index.layout.ty.kind() else {
470 | |                     span_bug!(this.cur_span(), "simd_shuffle index argument has non-array type {}", index.layout.ty)
    | |__________________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
602 | /     let ty::Float(float_ty) = left.layout.ty.kind() else {
603 | |         bug!("fmax operand is not a float")
    | |______^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
618 | /     let ty::Float(float_ty) = left.layout.ty.kind() else {
619 | |         bug!("fmin operand is not a float")
    | |______^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
243 | /         let Operation::Retag(op) = &mut self.operation else {
244 | |             unreachable!("start_grant must only be called during a retag, this is: {:?}", self.operation)
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
267 | /         let Operation::Retag(op) = &self.operation else {
268 | |             unreachable!("log_creation must only be called during a retag")
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
290 | /         let Operation::Retag(op) = &self.operation else {
291 | |             unreachable!("Protectors can only be created during a retag")
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
301 | /         let Some(created) = self.history
302 | |             .creations
303 | |             .iter()
304 | |             .rev()
344 | |                 return None;
345 | |             };
    | |______________^
    |
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
366 | /         let Operation::Retag(op) = &self.operation else {
367 | |             unreachable!("grant_error should only be called during a retag")
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
386 | /         let Operation::Access(op) = &self.operation  else {
387 | |             unreachable!("access_error should only be called during an access")
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
440 | /         let Operation::Dealloc(op) = &self.operation else {
441 | |             unreachable!("dealloc_error should only be called during a deallocation")
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
186 | /         let ProvenanceExtra::Concrete(tag) = tag else {
187 | |             // Handle the wildcard case.
188 | |             // Go search the stack for an exposed tag.
189 | |             if let Some(idx) =
...   |
206 | |             return if self.unknown_bottom.is_some() { Ok(None) } else { Err(()) };
    | |__________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
466 | /             let (Some(granting_idx), ProvenanceExtra::Concrete(_)) = (granting_idx, derived_from) else {
467 | |                 // The parent is a wildcard pointer or matched the unknown bottom.
468 | |                 // This is approximate. Nobody knows what happened, so forget everything.
469 | |                 // The new thing is SRW anyway, so we cannot push it "on top of the unkown part"
473 | |                 return Ok(());
474 | |             };
    | |______________^
    |
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
    |
    |
684 | /             let Some((alloc_id, base_offset, orig_tag)) = loc else {
686 | |             };
    | |______________^
    |
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
    = help: add `#![feature(let_else)]` to the crate attributes to enable

error[E0658]: `let...else` statements are unstable
  --> src/tools/miri/src/tag_gc.rs:69:17
   |
69 | /                 let LocalValue::Live(value) = local.value else {
71 | |             };
   | |______________^
   |
   = note: see issue #87335 <https://github.com/rust-lang/rust/issues/87335> for more information
