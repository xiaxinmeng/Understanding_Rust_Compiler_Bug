plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0da281b6068a7d889ae89a9bd8991284cc9b7535 and 43ec2478b29425aff7924d326cf66d68788ffc8e
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling libffi v3.0.1
error[E0308]: mismatched types
   --> src/tools/miri/src/stacked_borrows/mod.rs:717:41
    |
717 |                         if !ty.is_unpin(this.tcx.at(DUMMY_SP), this.param_env()) =>
    |                                -------- ^^^^^^^^^^^^^^^^^^^^^ expected struct `TyCtxt`, found struct `TyCtxtAt`
    |                                arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:884:12
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:884:12
    |
884 |     pub fn is_unpin(self, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> bool {
help: consider dereferencing the type
    |
    |
717 |                         if !ty.is_unpin(*this.tcx.at(DUMMY_SP), this.param_env()) =>

error[E0308]: mismatched types
   --> src/tools/miri/src/stacked_borrows/mod.rs:722:42
    |
    |
722 |                         if !ty.is_freeze(this.tcx.at(DUMMY_SP), this.param_env()) =>
    |                                --------- ^^^^^^^^^^^^^^^^^^^^^ expected struct `TyCtxt`, found struct `TyCtxtAt`
    |                                arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:844:12
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:844:12
    |
844 |     pub fn is_freeze(self, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> bool {
help: consider dereferencing the type
    |
    |
722 |                         if !ty.is_freeze(*this.tcx.at(DUMMY_SP), this.param_env()) =>

error[E0308]: mismatched types
   --> src/tools/miri/src/stacked_borrows/mod.rs:837:45
    |
    |
837 |                 if place.layout.ty.is_unpin(this.tcx.at(DUMMY_SP), this.param_env()) =>
    |                                    -------- ^^^^^^^^^^^^^^^^^^^^^ expected struct `TyCtxt`, found struct `TyCtxtAt`
    |                                    arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:884:12
   --> /checkout/compiler/rustc_middle/src/ty/util.rs:884:12
    |
884 |     pub fn is_unpin(self, tcx: TyCtxt<'tcx>, param_env: ty::ParamEnv<'tcx>) -> bool {
help: consider dereferencing the type
    |
    |
837 |                 if place.layout.ty.is_unpin(*this.tcx.at(DUMMY_SP), this.param_env()) =>

For more information about this error, try `rustc --explain E0308`.
error: could not compile `miri` due to 3 previous errors
error: could not compile `miri` due to 3 previous errors
thread 'main' panicked at 'in-tree tool', test.rs:495:14
Build completed unsuccessfully in 0:00:15
