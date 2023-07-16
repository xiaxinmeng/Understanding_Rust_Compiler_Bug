plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: unreachable expression
    --> library/core/src/str/mod.rs:1589:65
     |
1589 |         SplitN(SplitNInternal { iter: self.split(pat).0, count: n })
     |                                       ---------------           ^ unreachable expression
     |                                       any code following this expression is unreachable
     |
     |
note: this expression has type `str::iter::Split<'_, P>`, which is uninhabited
     |
     |
1589 |         SplitN(SplitNInternal { iter: self.split(pat).0, count: n })
     |                                       ^^^^^^^^^^^^^^^
     = note: `-D unreachable-code` implied by `-D warnings`
error: unused variable: `n`
    --> library/core/src/str/mod.rs:1588:49
     |
     |
1588 |     pub fn splitn<'a, P: Pattern<'a>>(&'a self, n: usize, pat: P) -> SplitN<'a, P> {
     |                                                 ^ help: if this is intentional, prefix it with an underscore: `_n`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: unreachable expression
  --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:62:55
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
   | |                  ----------------------------------   ^^^^^ unreachable expression
   | |                  any code following this expression is unreachable
...  |
73 | |     }
74 | | }
74 | | }
   | |_- in this expansion of `impl_element!`
75 |
76 |   impl_element! { i8 }
   |
   |
note: this expression has type `masks::Mask<i8, LANES>`, which is uninhabited
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
...  |
73 | |     }
74 | | }
   | |_- in this expansion of `impl_element!`
   | |_- in this expansion of `impl_element!`
75 |
76 |   impl_element! { i8 }

error: unreachable expression
  --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:62:55
   |
   |
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
   | |                  ----------------------------------   ^^^^^ unreachable expression
   | |                  any code following this expression is unreachable
...  |
73 | |     }
74 | | }
74 | | }
   | |_- in this expansion of `impl_element!`
...
77 |   impl_element! { i16 }
   |
   |
note: this expression has type `masks::Mask<i16, LANES>`, which is uninhabited
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
...  |
73 | |     }
74 | | }
   | |_- in this expansion of `impl_element!`
   | |_- in this expansion of `impl_element!`
...
77 |   impl_element! { i16 }

error: unreachable expression
  --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:62:55
   |
   |
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
   | |                  ----------------------------------   ^^^^^ unreachable expression
   | |                  any code following this expression is unreachable
...  |
73 | |     }
74 | | }
74 | | }
   | |_- in this expansion of `impl_element!`
...
78 |   impl_element! { i32 }
   |
   |
note: this expression has type `masks::Mask<i32, LANES>`, which is uninhabited
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
...  |
73 | |     }
74 | | }
   | |_- in this expansion of `impl_element!`
   | |_- in this expansion of `impl_element!`
...
78 |   impl_element! { i32 }

error: unreachable expression
  --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:62:55
   |
   |
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
   | |                  ----------------------------------   ^^^^^ unreachable expression
   | |                  any code following this expression is unreachable
...  |
73 | |     }
74 | | }
74 | | }
   | |_- in this expansion of `impl_element!`
...
79 |   impl_element! { i64 }
   |
   |
note: this expression has type `masks::Mask<i64, LANES>`, which is uninhabited
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
...  |
73 | |     }
74 | | }
   | |_- in this expansion of `impl_element!`
   | |_- in this expansion of `impl_element!`
...
79 |   impl_element! { i64 }

error: unreachable expression
  --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:62:55
   |
   |
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
   | |                  ----------------------------------   ^^^^^ unreachable expression
   | |                  any code following this expression is unreachable
...  |
73 | |     }
74 | | }
74 | | }
   | |_- in this expansion of `impl_element!`
...
80 |   impl_element! { isize }
   |
   |
note: this expression has type `masks::Mask<isize, LANES>`, which is uninhabited
   |
55 | / macro_rules! impl_element {
55 | / macro_rules! impl_element {
56 | |     { $ty:ty } => {
57 | |         impl Sealed for $ty {
58 | |             fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
...  |
62 | |                 (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
...  |
73 | |     }
74 | | }
   | |_- in this expansion of `impl_element!`
   | |_- in this expansion of `impl_element!`
...
80 |   impl_element! { isize }

error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:362:29
    |
    |
362 |         Mask::splat(self) & rhs
    |         -----------------   ^^^ unreachable expression
    |         any code following this expression is unreachable
    |
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
    |
362 |         Mask::splat(self) & rhs

error: unused variable: `rhs`
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:361:21
    |
    |
361 |     fn bitand(self, rhs: Mask<T, LANES>) -> Mask<T, LANES> {
    |                     ^^^ help: if this is intentional, prefix it with an underscore: `_rhs`
error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:401:29
    |
401 |         Mask::splat(self) | rhs
401 |         Mask::splat(self) | rhs
    |         -----------------   ^^^ unreachable expression
    |         |
    |         any code following this expression is unreachable
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
401 |         Mask::splat(self) | rhs
    |         ^^^^^^^^^^^^^^^^^


error: unused variable: `rhs`
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:400:20
    |
400 |     fn bitor(self, rhs: Mask<T, LANES>) -> Mask<T, LANES> {
    |                    ^^^ help: if this is intentional, prefix it with an underscore: `_rhs`
error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:440:29
    |
    |
440 |         Mask::splat(self) ^ rhs
    |         -----------------   ^^^ unreachable expression
    |         any code following this expression is unreachable
    |
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
    |
440 |         Mask::splat(self) ^ rhs

error: unused variable: `rhs`
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:439:21
    |
    |
439 |     fn bitxor(self, rhs: Mask<T, LANES>) -> Self::Output {
    |                     ^^^ help: if this is intentional, prefix it with an underscore: `_rhs`
error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:475:10
    |
    |
475 |         *self &= Self::splat(rhs);
    |          ^^^^    ---------------- any code following this expression is unreachable
    |          unreachable expression
    |
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
    |
475 |         *self &= Self::splat(rhs);

error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:497:10
    |
    |
497 |         *self |= Self::splat(rhs);
    |          ^^^^    ---------------- any code following this expression is unreachable
    |          unreachable expression
    |
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
    |
497 |         *self |= Self::splat(rhs);

error: unreachable expression
   --> library/core/src/../../portable-simd/crates/core_simd/src/masks.rs:519:10
    |
    |
519 |         *self ^= Self::splat(rhs);
    |          ^^^^    ---------------- any code following this expression is unreachable
    |          unreachable expression
    |
    |
note: this expression has type `masks::Mask<T, LANES>`, which is uninhabited
    |
    |
519 |         *self ^= Self::splat(rhs);


thread 'rustc' panicked at 'assertion failed: layout.abi.is_uninhabited()', compiler/rustc_ty_utils/src/layout_sanity_check.rs:16:9
stack backtrace:
   0:     0x7f10a1a91ba5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h328e207c0fd2205c
   1:     0x7f10a1afe138 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f10a1a85841 - std::io::Write::write_fmt::hf1d0b0080eeddb6d
   3:     0x7f10a1a919b5 - std::sys_common::backtrace::print::h031dcf236c316efb
   4:     0x7f10a1a94b74 - std::panicking::default_hook::{{closure}}::h9f518d39880907e1
   5:     0x7f10a1a94862 - std::panicking::default_hook::h689a8190a575275d
   6:     0x7f10a24fe185 - rustc_driver_impl[df7b5f2916424563]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f10a1a952a9 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
   8:     0x7f10a1a94fe2 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
   9:     0x7f10a1a92056 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
  10:     0x7f10a1a94cf2 - rust_begin_unwind
  11:     0x7f10a1a4e0b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f10a1a4e14d - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f10a28e3f49 - rustc_ty_utils[4b6d69ec75974a3]::layout_sanity_check::sanity_check_layout
  14:     0x7f10a291b71f - rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of
  15:     0x7f10a3fda750 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e14e60d8f45db920]::queries::layout_of, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 24usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 24usize]>>
  16:     0x7f10a429e274 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::layout_of, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  17:     0x7f10a4167d6c - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::layout_of
  18:     0x7f10a2972ccb - <rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutCx<rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt> as rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutOf>::spanned_layout_of
  19:     0x7f10a296ff55 - <core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_fold::<(), <core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_fold<(), core[54ab13d2a06817e1]::iter::traits::iterator::Iterator::try_for_each::call<rustc_abi[dfc517f7ac714473]::Layout, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_abi[dfc517f7ac714473]::Layout>, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_abi[dfc517f7ac714473]::Layout>::Break>::{closure#0}, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_abi[dfc517f7ac714473]::Layout>>::{closure#0}, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_abi[dfc517f7ac714473]::Layout>>>
  20:     0x7f10a28ff2a3 - <core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::next
  21:     0x7f10a2961fa5 - <alloc[492b206f37fb556b]::vec::Vec<rustc_abi[dfc517f7ac714473]::Layout> as alloc[492b206f37fb556b]::vec::spec_from_iter::SpecFromIter<rustc_abi[dfc517f7ac714473]::Layout, core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>>>>::from_iter
  22:     0x7f10a28fc2dd - core[54ab13d2a06817e1]::iter::adapters::try_process::<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}>>, rustc_abi[dfc517f7ac714473]::Layout, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>, <core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::FieldDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}::{closure#0}> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_collect<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>::{closure#0}, rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>
  23:     0x7f10a296fe8a - <core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_fold::<(), <core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_fold<(), core[54ab13d2a06817e1]::iter::traits::iterator::Iterator::try_for_each::call<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>::Break>::{closure#0}, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>>::{closure#0}, core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<core[54ab13d2a06817e1]::ops::control_flow::ControlFlow<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>>>
  24:     0x7f10a28ff1f1 - <core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::next
  25:     0x7f10a29612bd - <alloc[492b206f37fb556b]::vec::Vec<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>> as alloc[492b206f37fb556b]::vec::spec_from_iter::SpecFromIter<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>, core[54ab13d2a06817e1]::iter::adapters::GenericShunt<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}>>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>>>>::from_iter
  26:     0x7f10a28fc22e - core[54ab13d2a06817e1]::iter::adapters::try_process::<core[54ab13d2a06817e1]::iter::adapters::by_ref_sized::ByRefSized<core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}>>, rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::convert::Infallible, rustc_middle[b8dd7aca70be3dc8]::ty::layout::LayoutError>, <core[54ab13d2a06817e1]::iter::adapters::map::Map<core[54ab13d2a06817e1]::slice::iter::Iter<rustc_middle[b8dd7aca70be3dc8]::ty::VariantDef>, rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached::{closure#5}> as core[54ab13d2a06817e1]::iter::traits::iterator::Iterator>::try_collect<rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::VariantIdx, rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>>::{closure#0}, rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::VariantIdx, rustc_index[7d04d2f01c7184d9]::vec::IndexVec<rustc_abi[dfc517f7ac714473]::FieldIdx, rustc_abi[dfc517f7ac714473]::Layout>>>
  27:     0x7f10a291661a - rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of_uncached
  28:     0x7f10a291b668 - rustc_ty_utils[4b6d69ec75974a3]::layout::layout_of
  29:     0x7f10a3fda750 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e14e60d8f45db920]::queries::layout_of, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 24usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 24usize]>>
  30:     0x7f10a429e274 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::layout_of, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  31:     0x7f10a4167d6c - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::layout_of
  32:     0x7f10a304ac5b - <rustc_mir_transform[80d65ac27e373d0e]::const_prop::CanConstProp>::check
  33:     0x7f10a30eb94f - <rustc_mir_transform[80d65ac27e373d0e]::const_prop_lint::ConstProp as rustc_mir_transform[80d65ac27e373d0e]::pass_manager::MirLint>::run_lint
  34:     0x7f10a30652a8 - rustc_mir_transform[80d65ac27e373d0e]::pass_manager::run_passes_inner
  35:     0x7f10a2f9dbe4 - rustc_mir_transform[80d65ac27e373d0e]::run_analysis_to_runtime_passes
  36:     0x7f10a2f9d4d9 - rustc_mir_transform[80d65ac27e373d0e]::mir_drops_elaborated_and_const_checked
  37:     0x7f10a3fd94d7 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e14e60d8f45db920]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  38:     0x7f10a428f024 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  39:     0x7f10a413274c - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  40:     0x7f10a25eb446 - <rustc_session[53b0b4d7ab1aba8c]::session::Session>::time::<(), rustc_interface[b42dd50c7ade7b0f]::passes::analysis::{closure#3}>
  41:     0x7f10a25b8138 - rustc_interface[b42dd50c7ade7b0f]::passes::analysis
  42:     0x7f10a3fd9f0c - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>
  43:     0x7f10a4297fd6 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  44:     0x7f10a4128b8a - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::analysis
  45:     0x7f10a2541330 - <rustc_middle[b8dd7aca70be3dc8]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  46:     0x7f10a25537f1 - <rustc_interface[b42dd50c7ade7b0f]::interface::Compiler>::enter::<rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[b42dd50c7ade7b0f]::queries::Linker>, rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  47:     0x7f10a25706f0 - rustc_span[f084618c3ebd7322]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_interface[b42dd50c7ade7b0f]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  48:     0x7f10a25089b2 - <scoped_tls[b5fd76ccba344725]::ScopedKey<rustc_span[f084618c3ebd7322]::SessionGlobals>>::set::<rustc_interface[b42dd50c7ade7b0f]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  49:     0x7f10a2538b6a - std[ea8c57938de3e3e8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b42dd50c7ade7b0f]::util::run_in_thread_pool_with_globals<rustc_interface[b42dd50c7ade7b0f]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  50:     0x7f10a2501e48 - std[ea8c57938de3e3e8]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[b42dd50c7ade7b0f]::util::run_in_thread_pool_with_globals<rustc_interface[b42dd50c7ade7b0f]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  51:     0x7f10a2511fa6 - <<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[b42dd50c7ade7b0f]::util::run_in_thread_pool_with_globals<rustc_interface[b42dd50c7ade7b0f]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[df7b5f2916424563]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  52:     0x7f10a1aa14ee - std::sys::unix::thread::Thread::new::thread_start::h29b0cb2325153421
  53:     0x7f10a183eb43 - <unknown>
  54:     0x7f10a18d0a00 - <unknown>
  55:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (62d95f37a 2023-04-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [layout_of] computing layout of `cell::Cell<isize>`
#1 [layout_of] computing layout of `cell::RefCell<T>`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `cell::<impl at library/core/src/cell.rs:770:1: 770:19>::new`
#3 [analysis] running analysis passes on this crate
error: could not compile `core` due to 16 previous errors
Build completed unsuccessfully in 0:03:25
