plain

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

