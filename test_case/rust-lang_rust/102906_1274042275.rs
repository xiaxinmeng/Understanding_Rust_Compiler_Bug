plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 36c8e291a617ae6bd4b8ff13c54c82862eaf0eec and 87d24397e63c0eace8cdc99ddf5dd8e06efd1351
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    |                 |
    |                 expected enum `UnwindAction`, found enum `Option`
    |                 help: change the parameter type to match the trait: `UnwindAction`
    |
    = note: expected fn pointer `fn(&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>, &AssertKind<rustc_middle::mir::Operand<'tcx>>, UnwindAction) -> Result<_, _>`
               found fn pointer `fn(&mut rustc_const_eval::interpret::InterpCx<'mir, 'tcx, machine::MiriMachine<'mir, 'tcx>>, &AssertKind<rustc_middle::mir::Operand<'tcx>>, Option<BasicBlock>) -> Result<_, _>`
error[E0308]: mismatched types
   --> src/tools/miri/src/shims/panic.rs:216:25
    |
216 |                         unwind,
---
               found enum `Option<BasicBlock>`
note: associated function defined here
   --> src/tools/miri/src/shims/panic.rs:171:8
    |
171 |     fn start_panic(&mut self, msg: &str, unwind: mir::UnwindAction) -> InterpResult<'tcx> {

Some errors have detailed explanations: E0053, E0308.
For more information about an error, try `rustc --explain E0053`.
error: could not compile `miri` due to 3 previous errors
error: could not compile `miri` due to 3 previous errors
thread 'main' panicked at 'in-tree tool', test.rs:489:14
Build completed unsuccessfully in 0:00:17
