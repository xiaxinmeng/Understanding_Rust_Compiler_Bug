plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 38a76f33220c4b9d13dda1fa8f6c629c8a7bcc5d and f1dddda584935ae334e28d72949681ee09dd6a10
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling miniz_oxide v0.5.3
error: failed to evaluate generic const expression
    --> /checkout/library/alloc/src/vec/mod.rs:2864:10
     |
2864 |     [(); core::alloc::co_alloc_metadata_num_slots_with_preference::<A>(COOP_PREFERRED)]:,
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

