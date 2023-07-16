plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:8c19d39c6d9d7e831f6e393b2a871216393a5761)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: failed to evaluate generic const expression
    --> /checkout/library/alloc/src/vec/mod.rs:2920:10
     |
2920 |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
     |
     |
     = note: the crate this constant originates from uses `#![feature(generic_const_exprs)]`
help: consider enabling this feature
    -->  |/cargo/registry/src/github.com-1ecc6299db9ec823/miniz_oxide-0.5.3/src/lib.rs:1:1
1    | #![feature(generic_const_exprs)]
     |

error: could not compile `miniz_oxide` due to previous error
error: could not compile `miniz_oxide` due to previous error
warning: build failed, waiting for other jobs to finish...
error: failed to evaluate generic const expression
  --> /checkout/library/alloc/src/vec/spec_extend.rs:51:10
   |
51 |     [(); alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
   |
   |
   = note: the crate this constant originates from uses `#![feature(generic_const_exprs)]`
help: consider enabling this feature
   |
1  | #![feature(generic_const_exprs)]
   |

