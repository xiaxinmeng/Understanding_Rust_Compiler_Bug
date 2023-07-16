plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.71
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0221]: ambiguous associated type `Item` in bounds of `I`
    |
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [I::Item; N]
    |                                                                             ^^^^^^^ ambiguous associated type `Item`
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [<I as Iterator<>>::Item; N]
help: use fully qualified syntax to disambiguate
    |
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [<I as Iterator<>>::Item; N]


error[E0221]: ambiguous associated type `Item` in bounds of `A`
    |
    |
141 | / macro_rules! zip_impl_general_defaults {
142 | |     () => {
143 | |         default fn new(a: A, b: B) -> Self {
144 | |             Zip {
...   |
166 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                    ^^^^^^^ ambiguous associated type `Item`
195 | |     };
196 | | }
196 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
207 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>


error[E0221]: ambiguous associated type `Item` in bounds of `B`
    |
    |
141 | / macro_rules! zip_impl_general_defaults {
142 | |     () => {
143 | |         default fn new(a: A, b: B) -> Self {
144 | |             Zip {
...   |
166 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                             ^^^^^^^ ambiguous associated type `Item`
195 | |     };
196 | | }
196 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
207 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>


error[E0221]: ambiguous associated type `Item` in bounds of `A`
    |
    |
141 | / macro_rules! zip_impl_general_defaults {
142 | |     () => {
143 | |         default fn new(a: A, b: B) -> Self {
144 | |             Zip {
...   |
166 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                    ^^^^^^^ ambiguous associated type `Item`
195 | |     };
196 | | }
196 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
240 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>


error[E0221]: ambiguous associated type `Item` in bounds of `B`
    |
    |
141 | / macro_rules! zip_impl_general_defaults {
142 | |     () => {
143 | |         default fn new(a: A, b: B) -> Self {
144 | |             Zip {
...   |
166 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                             ^^^^^^^ ambiguous associated type `Item`
195 | |     };
196 | | }
196 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
240 |       zip_impl_general_defaults! {}
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
166 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>


error[E0221]: ambiguous associated type `Item` in bounds of `A`
    |
    |
331 |     fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    |                                        ^^^^^^^ ambiguous associated type `Item`
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
331 |     fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
331 |     fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>


error[E0221]: ambiguous associated type `Item` in bounds of `B`
    |
    |
331 |     fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    |                                                 ^^^^^^^ ambiguous associated type `Item`
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
help: use fully qualified syntax to disambiguate
    |
    |
331 |     fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
help: use fully qualified syntax to disambiguate
    |
    |
331 |     fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`', compiler/rustc_typeck/src/check/method/mod.rs:456:9
 right: `0`', compiler/rustc_typeck/src/check/method/mod.rs:456:9
stack backtrace:
   0:     0x7fb988d30d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha288dc719902e24d
   1:     0x7fb988d98648 - core::fmt::write::h42234c3e51154f4c
   2:     0x7fb988d21051 - std::io::Write::write_fmt::h74fbb9643da2d185
   3:     0x7fb988d34046 - std::panicking::default_hook::{{closure}}::h3bfe018301273550
   4:     0x7fb988d33c3d - std::panicking::default_hook::h4173afa32faa81d9
   5:     0x7fb989883a71 - rustc_driver[87b42345fa270eee]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb988d349e0 - std::panicking::rust_panic_with_hook::h59cc3082e9104592
   7:     0x7fb988d347f7 - std::panicking::begin_panic_handler::{{closure}}::h79b0ac1d2b9c8b15
   8:     0x7fb988d312a4 - std::sys_common::backtrace::__rust_end_short_backtrace::h72d3f8515fc7966b
   9:     0x7fb988d344e9 - rust_begin_unwind
  10:     0x7fb988ce80b3 - core::panicking::panic_fmt::h0eedb34d228802aa
  11:     0x7fb988d94f48 - core::panicking::assert_failed_inner::h555790e79e92093f
  12:     0x7fb989536b3b - core[10878fb91fc84a80]::panicking::assert_failed::<usize, usize>
  13:     0x7fb98a0dd474 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::construct_obligation_for_trait
  14:     0x7fb98a110ed7 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
  15:     0x7fb98a092ac5 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::try_overloaded_call_traits
  16:     0x7fb98a091fce - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_call
  17:     0x7fb98a0fc1e2 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  18:     0x7fb98a0a7678 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  19:     0x7fb98a0fb1c9 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  20:     0x7fb98a0c1741 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_argument_types::{closure#0}
  21:     0x7fb98a0bc195 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_argument_types
  22:     0x7fb98a0947c3 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  23:     0x7fb98a0926d6 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_call
  24:     0x7fb98a0fc1e2 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7fb98a0a7678 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7fb98a0fb1c9 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7fb98a0a8c6b - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7fb98a3fa567 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  29:     0x7fb98a0f3db9 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_closure
  30:     0x7fb98a0fb9bf - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  31:     0x7fb98a0a7678 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  32:     0x7fb98a0fb1c9 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  33:     0x7fb98a0c3c94 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  34:     0x7fb98a0fc4b0 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_kind
  35:     0x7fb98a0a7678 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  36:     0x7fb98a0fb1c9 - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  37:     0x7fb98a0a8c6b - <rustc_typeck[759fce67295582a0]::check::fn_ctxt::FnCtxt>::check_return_expr
  38:     0x7fb98a3fa567 - rustc_typeck[759fce67295582a0]::check::check::check_fn
  39:     0x7fb98a2ac420 - <rustc_infer[26ac34c435530b6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults, <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>::{closure#0}>
  40:     0x7fb98a3be2fe - <rustc_typeck[759fce67295582a0]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[759fce67295582a0]::check::typeck_with_fallback<rustc_typeck[759fce67295582a0]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>
  41:     0x7fb98a1d476e - rustc_typeck[759fce67295582a0]::check::typeck
  42:     0x7fb98af586d4 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::ty::context::TypeckResults>>
  43:     0x7fb98b0612c7 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::typeck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  44:     0x7fb98b3ca924 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::typeck
  45:     0x7fb98c071087 - <rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt>::typeck_opt_const_arg
  46:     0x7fb98a763b4c - rustc_mir_build[dd8be03e72f6550]::build::mir_built
  47:     0x7fb98af48ecc - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  48:     0x7fb98b063666 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_built, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  49:     0x7fb98b3b3857 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_built
  50:     0x7fb989f41878 - rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::unsafety_check_result
  51:     0x7fb989f3d7ec - <rustc_mir_transform[8cd1ea75711a0041]::check_unsafety::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  52:     0x7fb98af5a1b4 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::UnsafetyCheckResult>>
  53:     0x7fb98b036917 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::unsafety_check_result, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  54:     0x7fb98b3c32e4 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::unsafety_check_result
  55:     0x7fb989f02136 - rustc_mir_transform[8cd1ea75711a0041]::mir_const
  56:     0x7fb98af48ecc - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_middle[8d4dc3708b593ac1]::ty::WithOptConstParam<rustc_span[5c736203a6ab5594]::def_id::LocalDefId>, &rustc_data_structures[db21f27220b58c22]::steal::Steal<rustc_middle[8d4dc3708b593ac1]::mir::Body>>>
  57:     0x7fb98b0637a3 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_const, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  58:     0x7fb98b3b3da7 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_const
  59:     0x7fb989f02ec3 - rustc_mir_transform[8cd1ea75711a0041]::mir_promoted
  60:     0x7fb98b016808 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_promoted, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  61:     0x7fb98b3b63d7 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_promoted
  62:     0x7fb98aaa960a - rustc_borrowck[f218b4719d5fedf9]::mir_borrowck
  63:     0x7fb98aa76c0c - <rustc_borrowck[f218b4719d5fedf9]::provide::{closure#0} as core[10878fb91fc84a80]::ops::function::FnOnce<(rustc_middle[8d4dc3708b593ac1]::ty::context::TyCtxt, rustc_span[5c736203a6ab5594]::def_id::LocalDefId)>>::call_once
  64:     0x7fb98af59444 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::LocalDefId, &rustc_middle[8d4dc3708b593ac1]::mir::query::BorrowCheckResult>>
  65:     0x7fb98b016128 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::mir_borrowck, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  66:     0x7fb98b3cc954 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::mir_borrowck
  67:     0x7fb98a3ca85e - rustc_typeck[759fce67295582a0]::collect::type_of::type_of
  68:     0x7fb98af7219d - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<rustc_span[5c736203a6ab5594]::def_id::DefId, rustc_middle[8d4dc3708b593ac1]::ty::Ty>>
  69:     0x7fb98b061581 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::type_of, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  70:     0x7fb98b3adca9 - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::type_of
  71:     0x7fb989cde312 - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  72:     0x7fb989ccf471 - rustc_hir[78a3789577f2fa25]::intravisit::walk_ty::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  73:     0x7fb989cced98 - rustc_hir[78a3789577f2fa25]::intravisit::walk_fn::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  74:     0x7fb989cc9f55 - rustc_hir[78a3789577f2fa25]::intravisit::walk_impl_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  75:     0x7fb989cd4b3e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  76:     0x7fb989cde9fa - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  77:     0x7fb989cd471e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  78:     0x7fb989cde9fa - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  79:     0x7fb989cd471e - rustc_hir[78a3789577f2fa25]::intravisit::walk_item::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  80:     0x7fb989cde9fa - <rustc_privacy[de044dad0112bda]::EmbargoVisitor as rustc_hir[78a3789577f2fa25]::intravisit::Visitor>::visit_item
  81:     0x7fb989cd0cea - rustc_hir[78a3789577f2fa25]::intravisit::walk_mod::<rustc_privacy[de044dad0112bda]::EmbargoVisitor>
  82:     0x7fb989ce546b - rustc_privacy[de044dad0112bda]::privacy_access_levels
  83:     0x7fb98af9a0ec - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), &rustc_middle[8d4dc3708b593ac1]::middle::privacy::AccessLevels>>
  84:     0x7fb98b033575 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::privacy_access_levels, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  85:     0x7fb98b3d222e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::privacy_access_levels
  86:     0x7fb98a6d1267 - rustc_passes[d57e1aa5a6818bed]::stability::check_unused_or_stable_features
  87:     0x7fb9899a12ce - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}::{closure#2}::{closure#0}>
  88:     0x7fb9899a23be - <rustc_session[2a929b385c5bc398]::session::Session>::time::<(), rustc_interface[fc3bf7b819dbb0d8]::passes::analysis::{closure#0}>
  89:     0x7fb989994476 - rustc_interface[fc3bf7b819dbb0d8]::passes::analysis
  90:     0x7fb98af930ec - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::try_execute_query::<rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt, rustc_query_system[4cf300c5f65a2dfc]::query::caches::DefaultCache<(), core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>>
  91:     0x7fb98b0616a2 - rustc_query_system[4cf300c5f65a2dfc]::query::plumbing::get_query::<rustc_query_impl[b8302d1c2e1c865b]::queries::analysis, rustc_query_impl[b8302d1c2e1c865b]::plumbing::QueryCtxt>
  92:     0x7fb98b3ae20e - <rustc_query_impl[b8302d1c2e1c865b]::Queries as rustc_middle[8d4dc3708b593ac1]::ty::query::QueryEngine>::analysis
  93:     0x7fb98987395a - <rustc_interface[fc3bf7b819dbb0d8]::passes::QueryContext>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  94:     0x7fb9898194b6 - <rustc_interface[fc3bf7b819dbb0d8]::interface::Compiler>::enter::<rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}::{closure#2}, core[10878fb91fc84a80]::result::Result<core[10878fb91fc84a80]::option::Option<rustc_interface[fc3bf7b819dbb0d8]::queries::Linker>, rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  95:     0x7fb9897fb5b6 - rustc_span[5c736203a6ab5594]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_interface[fc3bf7b819dbb0d8]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#1}>
  96:     0x7fb98981a7af - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[5c736203a6ab5594]::SessionGlobals>>::set::<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  97:     0x7fb98986f449 - std[eba90a372f7a1edd]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>
  98:     0x7fb98982de01 - std[eba90a372f7a1edd]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  99:     0x7fb9898716c2 - <<std[eba90a372f7a1edd]::thread::Builder>::spawn_unchecked_<rustc_interface[fc3bf7b819dbb0d8]::util::run_in_thread_pool_with_globals<rustc_interface[fc3bf7b819dbb0d8]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>, rustc_driver[87b42345fa270eee]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[984494b0cf0e650]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
 100:     0x7fb988d413e3 - std::sys::unix::thread::Thread::new::thread_start::h6bc2e8f9e4e3d29f
 101:     0x7fb983291609 - start_thread
 102:     0x7fb988ba4163 - clone
 103:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (03f26b31f 2022-05-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#1 [mir_built] building MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#2 [unsafety_check_result] unsafety-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#3 [mir_const] processing MIR for `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#4 [mir_promoted] processing `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#5 [mir_borrowck] borrow-checking `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2`
#6 [type_of] computing type of `ops::try_trait::<impl at library/core/src/ops/try_trait.rs:379:1: 385:2>::wrap_mut_2::{opaque#1}`
#7 [privacy_access_levels] privacy access levels
#8 [analysis] running analysis passes on this crate
For more information about this error, try `rustc --explain E0221`.
error: could not compile `core` due to 7 previous errors
Build completed unsuccessfully in 0:03:25
