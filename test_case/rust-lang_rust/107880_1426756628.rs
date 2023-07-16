plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
.......................................................
test result: ok. 667 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 12.46s

   Doc-tests core
error[E0252]: the name `vcvtq_s32_f32` is defined multiple times
   |
73 | pub use crate::core_arch::arm_shared::*;
   |         -------------------------------
   |         |
   |         |
   |         previous import of the value `vcvtq_s32_f32` here
   |         you can use `as` to change the binding name of the import
...
81 | pub use neon::*;
   |         ^^^^^^^ `vcvtq_s32_f32` reimported here
   |
   = note: `vcvtq_s32_f32` must be defined only once in the value namespace of this module

error[E0252]: the name `vcvtq_u32_f32` is defined multiple times
   |
73 | pub use crate::core_arch::arm_shared::*;
   |         -------------------------------
   |         |
   |         |
   |         previous import of the value `vcvtq_u32_f32` here
   |         you can use `as` to change the binding name of the import
...
81 | pub use neon::*;
   |         ^^^^^^^ `vcvtq_u32_f32` reimported here
   |
   = note: `vcvtq_u32_f32` must be defined only once in the value namespace of this module
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0252`.
For more information about this error, try `rustc --explain E0252`.
error: doctest failed, to rerun pass `-p core --doc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core --test /checkout/library/core/src/lib.rs --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/build/compiler_builtins-23df797a2ec042c2/out --test-args --quiet --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-fa992565d2130c71.rlib --extern rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand-4711c5294469b9e2.rlib --extern rand_xorshift=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librand_xorshift-ef0660ce0acc5afc.rlib -C embed-bitcode=no -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.69.0-nightly
  (c17c9c917
  2023-02-11)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --error-format human` (exit status: 1)
