plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 1195bea5a7b73e079fa14b37ac7e375fc77d368a and cd70093632ab081ebc7bca7304a369a220e834b6
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

error[E0658]: unions in const fn are unstable
  --> library/core/src/ptr/metadata.rs:97:14
   |
97 |     unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
   |
   = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
   = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/ptr/metadata.rs:117:14
    |
117 |     unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.const_ptr }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/ptr/metadata.rs:134:14
    |
134 |     unsafe { PtrRepr { components: PtrComponents { data_address, metadata } }.mut_ptr }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable


error[E0658]: unions in const fn are unstable
   --> library/core/src/slice/mod.rs:111:18
    |
111 |         unsafe { crate::ptr::PtrRepr { const_ptr: self }.components.metadata }
    |
    = note: see issue #51909 <https://github.com/rust-lang/rust/issues/51909> for more information
    = help: add `#![feature(const_fn_union)]` to the crate attributes to enable

