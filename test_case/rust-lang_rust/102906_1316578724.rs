plain
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/binaryen.path'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/binaryen.url'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/doc/rust-by-example.path'. Skipping second one!
warning: e9a64996bedbc224d1edb401162d2c3cc15eb7c1:.gitmodules, multiple configurations found for 'submodule.src/doc/rust-by-example.url'. Skipping second one!
Searching for toolstate changes between 6d651a295e0e0c331153288b10b78344a4ede20b and 39eb36618231d71bb8e71be0318b04cfa485d548
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/cargo-miri-test-5593aacebc229b2c/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:rerun-if-env-changed=MIRITESTVAR
  cargo:rustc-env=MIRITESTVAR=testval
  --- stderr
  --- stderr
  error[E0463]: can't find crate for `doesnotexist`
   --> <anon>:1:1
    |
  1 | extern crate doesnotexist as probe;

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0463`.
  For more information about this error, try `rustc --explain E0463`.
  error[E0412]: cannot find type `doesnotexist` in this scope
   --> <anon>:1:18
    |
  1 | pub type Probe = doesnotexist;

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0412`.
  For more information about this error, try `rustc --explain E0412`.
  error[E0405]: cannot find trait `doesnotexist` in this scope
   --> <anon>:1:18
    |
  1 | pub trait Probe: doesnotexist + Sized {}

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0405`.
  For more information about this error, try `rustc --explain E0405`.
  error[E0432]: unresolved import `doesnotexist`
   --> <anon>:1:9
    |
  1 | pub use doesnotexist;
    |         ^^^^^^^^^^^^ no `doesnotexist` in the root
  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0432`.
  For more information about this error, try `rustc --explain E0432`.
  error[E0425]: cannot find value `doesnotexist` in this scope
   --> <anon>:1:28
    |
  1 | pub const PROBE: () = ((), doesnotexist).0;

  error: aborting due to previous error

  For more information about this error, try `rustc --explain E0425`.
  For more information about this error, try `rustc --explain E0425`.
  Block containg CatchPadInst must be jumped to only by its catchswitch.
    %catchpad = catchpad within %catchswitch [ptr null, i32 64, ptr null]
  Instruction does not dominate all uses!
    %catchswitch = catchswitch within none [label %terminate] unwind to caller
    %catchpad = catchpad within %catchswitch [ptr null, i32 64, ptr null]
  CatchSwitchInst needs to be in a function with a personality.
    %catchswitch = catchswitch within none [label %terminate] unwind to caller
  CatchPadInst needs to be in a function with a personality.
    %catchpad = catchpad within %catchswitch [ptr null, i32 64, ptr null]
  LLVM ERROR: Broken module found, compilation aborted!
  thread 'main' panicked at 'assertion failed: a.probe_expression(\"Box::new(0)\")', build.rs:38:5
     0: rust_begin_unwind
     1: core::panicking::panic_fmt
     2: core::panicking::panic
     3: build_script_build::main
