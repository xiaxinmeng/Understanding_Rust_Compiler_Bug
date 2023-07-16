plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5dda74a48cd50de10539478c1e0b6699bfdab665 and a235a489eec40d894ddce83cdee3032055e38ef4
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

error[E0407]: method `enforce_number_no_provenance` is not a member of trait `Machine`
   --> src/tools/miri/src/machine.rs:534:5
    |
534 | /     fn enforce_number_no_provenance(ecx: &MiriEvalContext<'mir, 'tcx>) -> bool {
535 | |         !ecx.machine.allow_ptr_int_transmute
    | |_____^ not a member of trait `Machine`

error[E0407]: method `init_allocation_extra` is not a member of trait `Machine`
   --> src/tools/miri/src/machine.rs:633:5
---
This PR updated 'src/doc/reference', verifying if status is 'test-pass'...

We detected that this PR updated 'reference', but its tests failed.

If you do intend to update 'reference', please check the error messages above and
commit another update.

If you do NOT intend to update 'reference', please ensure you did not accidentally
change the submodule at 'src/doc/reference'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
