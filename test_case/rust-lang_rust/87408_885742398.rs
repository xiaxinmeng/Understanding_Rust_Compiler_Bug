plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
  |
  |
3 | use std::collections::TryReserveErrorDetail::*;
  |
  = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
  = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
706 |         if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.detail()) {
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
710 |         if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.detail()) {
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
716 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
724 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
732 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
740 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
753 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.detail()) {
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
756 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.detail()) {
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
760 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
767 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
775 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.detail()) {
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
797 |         if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
802 |         if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
809 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
816 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
823 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
830 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
842 |         if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
847 |         if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
853 |             if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
860 |             if let Err(AllocError { .. }) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
    |
    |
867 |         if let Err(CapacityOverflow) =
    |
    = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
    = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
  |
  |
3 | use std::collections::TryReserveErrorDetail::*;
  |
  = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
  = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1481 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP).map_err(|e| e.detail()) {
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1485 |         if let Err(CapacityOverflow) = empty_bytes.try_reserve(MAX_CAP).map_err(|e| e.detail()) {
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1491 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1499 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1507 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1515 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1528 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.detail()) {
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1531 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.detail()) {
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1535 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1542 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1550 |         if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_USIZE) {
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1560 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1565 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1571 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1578 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1586 |         if let Err(CapacityOverflow) = ten_u32s.try_reserve(MAX_USIZE - 20).map_err(|e| e.detail())
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1609 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1614 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1621 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1628 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1635 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1642 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1654 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1659 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1665 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1672 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1679 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1690 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1695 |         if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1701 |             if let Err(CapacityOverflow) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
     |
     |
1708 |             if let Err(AllocError { .. }) =
     |
     = note: see issue #48043 <https://github.com/rust-lang/rust/issues/48043> for more information
     = help: add `#![feature(try_reserve_detail)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'try_reserve_detail': Uncertain how much info should be exposed
