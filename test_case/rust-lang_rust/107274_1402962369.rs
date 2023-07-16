plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7873766cb4a6a0bb00c54496556b38db3fffa5d6)
Complete job name: PR (x86_64-gnu-tools, false, 1, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c8e6a9e8b6251bbc8276cb78cabe1998deecbed7 and 3e255b7308c12a13d4763ace0a85a8afb2e38127
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
error: failed to evaluate generic const expression
    --> /checkout/library/alloc/src/vec/mod.rs:2921:10
     |
2921 |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
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

