plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between eecde5850cade0c058dc12330081329b31a826c7 and ab7c07e351d341b00b5e50c8e15710b91392eda6
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling shell-escape v0.1.5
   Compiling parking_lot v0.11.2
   Compiling measureme v10.1.0
   Compiling libffi v3.0.1
error[E0599]: no method named `misc_cast` found for mutable reference `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>` in the current scope
    |
    |
440 | ...                   this.misc_cast(&op, dest.layout.ty)?,
    |                            ^^^^^^^^^ method not found in `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>`

error[E0599]: no method named `misc_cast` found for mutable reference `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>` in the current scope
    |
    |
443 | ...                   this.misc_cast(&op, dest.layout.ty)?,
    |                            ^^^^^^^^^ method not found in `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>`

error[E0599]: no method named `misc_cast` found for mutable reference `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>` in the current scope
    |
    |
446 | ...                   this.misc_cast(&op, dest.layout.ty)?,
    |                            ^^^^^^^^^ method not found in `&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `miri` due to 3 previous errors
error: could not compile `miri` due to 3 previous errors
thread 'main' panicked at 'in-tree tool', test.rs:489:14
Build completed unsuccessfully in 0:00:14
