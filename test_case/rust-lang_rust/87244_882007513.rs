plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0412]: cannot find type `Rc` in this scope
   --> compiler/rustc_middle/src/traits/mod.rs:338:15
    |
338 |     MatchImpl(Rc<ObligationCauseCode<'tcx>>, DefId),
    |               ^^ not found in this scope
help: consider importing one of these items
    |
    |
11  | use crate::ty::structural_impls::Rc;
11  | use std::rc::Rc;
    |

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:32
    |
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `40_usize`, found `::std::mem::size_of::<$ty>()`
7   | | }
    | |_- in this expansion of `static_assert_size!`
    | 
   ::: compiler/rustc_middle/src/traits/mod.rs:357:1
   ::: compiler/rustc_middle/src/traits/mod.rs:357:1
    |
357 |   static_assert_size!(ObligationCauseCode<'_>, 40);
    |
    |
    = note: expected array `[(); 40]`
               found array `[(); _]`
error: constant expression depends on a generic parameter
   --> /checkout/compiler/rustc_data_structures/src/macros.rs:5:37
    |
3   | / macro_rules! static_assert_size {
3   | / macro_rules! static_assert_size {
4   | |     ($ty:ty, $size:expr) => {
5   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
6   | |     };
7   | | }
    | |_- in this expansion of `static_assert_size!`
    | 
    | 
   ::: compiler/rustc_middle/src/traits/mod.rs:357:1
    |
357 |   static_assert_size!(ObligationCauseCode<'_>, 40);
    |
    = note: this may fail depending on what value the parameter takes

error: aborting due to 3 previous errors
