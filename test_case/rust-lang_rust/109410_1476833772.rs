plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
running 14691 tests
..........................................ii............................................ 88/14691
.................................................................................iiiiiii 176/14691
iiiiiiii.....................i..................i....................................... 264/14691
..............................................FF....F.FF.F.FFF...FF.F....F..FFF..F...... 352/14691
........................................................................................ 528/14691
........................................................................................ 616/14691
........................................................................................ 704/14691
........................................................................................ 792/14691
---
---- [ui] tests/ui/associated-inherent-types/assoc-inherent-use.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/assoc-inherent-use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/assoc-inherent-use" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/assoc-inherent-use/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(0, Some(0))`,
 right: `(1, Some(1))`: wrong number of generic parameters for DefId(0:6 ~ assoc_inherent_use[5c2b]::{impl#0}::Bar): [Foo]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7fef1def3b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7fef1df60bc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7fef1dee83f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7fef1def3941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7fef1def6b14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7fef1def67fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7fef1e9ef4f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fef1def7231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7fef1def6fa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7fef1def4006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7fef1def6c87 - rust_begin_unwind
  11:     0x7fef1dead2c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7fef1dead736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7fef1e7aecbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7fef1f171e63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7fef1f3541da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7fef1f351f15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7fef1f35d886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7fef1eecff6f - <rustc_hir_typeck[b7f5b175c883c7b2]::fn_ctxt::FnCtxt>::to_ty
  19:     0x7fef1f0b5f3f - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor>::declare
  20:     0x7fef1f0b65af - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_local
  21:     0x7fef1f0b4e94 - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_block
  22:     0x7fef1f09b571 - rustc_hir_typeck[b7f5b175c883c7b2]::check::check_fn
  23:     0x7fef1f0966ad - rustc_hir_typeck[b7f5b175c883c7b2]::typeck
  24:     0x7fef2085e0ac - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7fef204f7589 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck
  26:     0x7fef1efd26bb - rustc_data_structures[6ed9cf05cd26a8f9]::sync::par_for_each_in::<&[rustc_span[7ea47835a767f4a9]::def_id::LocalDefId], <rustc_middle[51b2e9d83909998]::hir::map::Map>::par_body_owners<rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies::{closure#0}>::{closure#0}>
  27:     0x7fef1f0940f3 - rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies
  28:     0x7fef20787644 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck_item_bodies, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  29:     0x7fef204f6ce3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck_item_bodies
  30:     0x7fef1f16bb87 - <rustc_session[3b49a9109e54b2fc]::session::Session>::time::<(), rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#7}>
  31:     0x7fef1f2ad306 - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  32:     0x7fef1eac3448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  33:     0x7fef20862b09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  34:     0x7fef204c3e23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  35:     0x7fef1e9f0f13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7fef1ea3d088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  37:     0x7fef1e9fd788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fef1ea2dba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  39:     0x7fef1ea3f5a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7fef1e9fb265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fef1df0347e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  42:     0x7fef1dc9db43 - <unknown>
  43:     0x7fef1dd2fa00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/assoc-inherent-private.rs stdout ----
---- [ui] tests/ui/associated-inherent-types/assoc-inherent-private.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/assoc-inherent-private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/assoc-inherent-private" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/assoc-inherent-private/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0624]: associated type `P` is private
  --> fake-test-src-base/associated-inherent-types/assoc-inherent-private.rs:10:10
LL |         type P = ();
   |         ------ associated type defined here
...
...
LL | type U = m::T::P; //~ ERROR associated type `P` is private

thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(0, Some(0))`,
 right: `(1, Some(1))`: wrong number of generic parameters for DefId(0:7 ~ assoc_inherent_private[979b]::m::{impl#0}::P): [m::T]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7f8783ac7b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7f8783b34bc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7f8783abc3f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7f8783ac7941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7f8783acab14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7f8783aca7fa - std::panicking::default_hook::hd0097697e449c588
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   6:     0x7f87845c34f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8783acb231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7f8783acafa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7f8783ac8006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7f8783acac87 - rust_begin_unwind
  11:     0x7f8783a812c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7f8783a81736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7f8784382cbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7f8784d45e63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7f8784f281da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7f8784f25f15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7f8784f31886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7f8784eb6fbe - rustc_hir_analysis[9f63bfe09a33ab11]::collect::type_of::type_of
  19:     0x7f8786434ee3 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::type_of, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  20:     0x7f8786095aa3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::type_of
  21:     0x7f8784e78b6f - <rustc_hir_analysis[9f63bfe09a33ab11]::collect::CollectItemTypesVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_item
  22:     0x7f8784e234a0 - <rustc_middle[51b2e9d83909998]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[9f63bfe09a33ab11]::collect::CollectItemTypesVisitor>
  23:     0x7f8784e76d9d - rustc_hir_analysis[9f63bfe09a33ab11]::collect::collect_mod_item_types
  24:     0x7f87863a0f29 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::collect_mod_item_types, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7f87860c9b29 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::collect_mod_item_types
  26:     0x7f8784e22d27 - <rustc_middle[51b2e9d83909998]::hir::map::Map>::for_each_module::<rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  27:     0x7f8784d40cac - <rustc_session[3b49a9109e54b2fc]::session::Session>::track_errors::<rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#0}, ()>
  28:     0x7f8784e811bb - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  29:     0x7f8784697448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  30:     0x7f8786436b09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  31:     0x7f8786097e23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  32:     0x7f87845c4f13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  33:     0x7f8784611088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  34:     0x7f87845d1788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f8784601ba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7f87846135a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7f87845cf265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f8783ad747e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  39:     0x7f8783871b43 - <unknown>
  40:     0x7f8783903a00 - <unknown>
  41:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [type_of] expanding type alias `U`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0624`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/dispatch-on-self-type-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/dispatch-on-self-type-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-1/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(0, Some(0))`,
 right: `(1, Some(1))`: wrong number of generic parameters for DefId(0:21 ~ dispatch_on_self_type_1[aac5]::{impl#3}::Type): [Select<Special, Special>]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7f359b4deb35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7f359b54bbc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7f359b4d33f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7f359b4de941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7f359b4e1b14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7f359b4e17fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7f359bfda4f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f359b4e2231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7f359b4e1fa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7f359b4df006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7f359b4e1c87 - rust_begin_unwind
  11:     0x7f359b4982c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7f359b498736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7f359bd99cbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7f359c75ce63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7f359c93f1da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7f359c93cf15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7f359c948886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7f359c4baf6f - <rustc_hir_typeck[b7f5b175c883c7b2]::fn_ctxt::FnCtxt>::to_ty
  19:     0x7f359c6a0f3f - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor>::declare
  20:     0x7f359c6a15af - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_local
  21:     0x7f359c69fe94 - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_block
  22:     0x7f359c686571 - rustc_hir_typeck[b7f5b175c883c7b2]::check::check_fn
  23:     0x7f359c6816ad - rustc_hir_typeck[b7f5b175c883c7b2]::typeck
  24:     0x7f359de490ac - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7f359dae2589 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck
  26:     0x7f359c5bd6bb - rustc_data_structures[6ed9cf05cd26a8f9]::sync::par_for_each_in::<&[rustc_span[7ea47835a767f4a9]::def_id::LocalDefId], <rustc_middle[51b2e9d83909998]::hir::map::Map>::par_body_owners<rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies::{closure#0}>::{closure#0}>
  27:     0x7f359c67f0f3 - rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies
  28:     0x7f359dd72644 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck_item_bodies, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  29:     0x7f359dae1ce3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck_item_bodies
  30:     0x7f359c756b87 - <rustc_session[3b49a9109e54b2fc]::session::Session>::time::<(), rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#7}>
  31:     0x7f359c898306 - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  32:     0x7f359c0ae448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  33:     0x7f359de4db09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  34:     0x7f359daaee23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  35:     0x7f359bfdbf13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7f359c028088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  37:     0x7f359bfe8788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f359c018ba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  39:     0x7f359c02a5a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f359bfe6265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f359b4ee47e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  42:     0x7f359b288b43 - <unknown>
  43:     0x7f359b31aa00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/dispatch-on-self-type-2.rs stdout ----
---- [ui] tests/ui/associated-inherent-types/dispatch-on-self-type-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/dispatch-on-self-type-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-2/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(0, Some(0))`,
 right: `(1, Some(1))`: wrong number of generic parameters for DefId(0:10 ~ dispatch_on_self_type_2[4937]::{impl#0}::Output): [Parameterized<(), ()>]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7f7ccf07db35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7f7ccf0eabc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7f7ccf0723f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7f7ccf07d941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7f7ccf080b14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7f7ccf0807fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7f7ccfb794f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7ccf081231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7f7ccf080fa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7f7ccf07e006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7f7ccf080c87 - rust_begin_unwind
  11:     0x7f7ccf0372c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7f7ccf037736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7f7ccf938cbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7f7cd02fbe63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7f7cd04de1da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7f7cd04dbf15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7f7cd04e7886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7f7cd0059f6f - <rustc_hir_typeck[b7f5b175c883c7b2]::fn_ctxt::FnCtxt>::to_ty
  19:     0x7f7cd023ff3f - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor>::declare
  20:     0x7f7cd02405af - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_local
  21:     0x7f7cd023ee94 - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_block
  22:     0x7f7cd0225571 - rustc_hir_typeck[b7f5b175c883c7b2]::check::check_fn
  23:     0x7f7cd02206ad - rustc_hir_typeck[b7f5b175c883c7b2]::typeck
  24:     0x7f7cd19e80ac - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7f7cd1681589 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck
  26:     0x7f7cd015c6bb - rustc_data_structures[6ed9cf05cd26a8f9]::sync::par_for_each_in::<&[rustc_span[7ea47835a767f4a9]::def_id::LocalDefId], <rustc_middle[51b2e9d83909998]::hir::map::Map>::par_body_owners<rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies::{closure#0}>::{closure#0}>
  27:     0x7f7cd021e0f3 - rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies
  28:     0x7f7cd1911644 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck_item_bodies, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  29:     0x7f7cd1680ce3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck_item_bodies
  30:     0x7f7cd02f5b87 - <rustc_session[3b49a9109e54b2fc]::session::Session>::time::<(), rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#7}>
  31:     0x7f7cd0437306 - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  32:     0x7f7ccfc4d448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  33:     0x7f7cd19ecb09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  34:     0x7f7cd164de23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  35:     0x7f7ccfb7af13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7f7ccfbc7088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  37:     0x7f7ccfb87788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f7ccfbb7ba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  39:     0x7f7ccfbc95a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f7ccfb85265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f7ccf08d47e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  42:     0x7f7ccee27b43 - <unknown>
  43:     0x7f7cceeb9a00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/dispatch-on-self-type-0.rs stdout ----
---- [ui] tests/ui/associated-inherent-types/dispatch-on-self-type-0.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/dispatch-on-self-type-0.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-0" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/dispatch-on-self-type-0/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(0, Some(0))`,
 right: `(1, Some(1))`: wrong number of generic parameters for DefId(0:10 ~ dispatch_on_self_type_0[437e]::{impl#1}::Projection): [Select<std::string::String>]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7f8032bc1b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7f8032c2ebc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7f8032bb63f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7f8032bc1941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7f8032bc4b14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7f8032bc47fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7f80336bd4f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f8032bc5231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7f8032bc4fa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7f8032bc2006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7f8032bc4c87 - rust_begin_unwind
  11:     0x7f8032b7b2c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7f8032b7b736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7f803347ccbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7f8033e3fe63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7f80340221da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7f803401ff15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7f803402b886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7f8033b9df6f - <rustc_hir_typeck[b7f5b175c883c7b2]::fn_ctxt::FnCtxt>::to_ty
  19:     0x7f8033d83f3f - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor>::declare
  20:     0x7f8033d845af - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_local
  21:     0x7f8033d82e94 - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_block
  22:     0x7f8033d69571 - rustc_hir_typeck[b7f5b175c883c7b2]::check::check_fn
  23:     0x7f8033d646ad - rustc_hir_typeck[b7f5b175c883c7b2]::typeck
  24:     0x7f803552c0ac - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7f80351c5589 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck
  26:     0x7f8033ca06bb - rustc_data_structures[6ed9cf05cd26a8f9]::sync::par_for_each_in::<&[rustc_span[7ea47835a767f4a9]::def_id::LocalDefId], <rustc_middle[51b2e9d83909998]::hir::map::Map>::par_body_owners<rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies::{closure#0}>::{closure#0}>
  27:     0x7f8033d620f3 - rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies
  28:     0x7f8035455644 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck_item_bodies, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  29:     0x7f80351c4ce3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck_item_bodies
  30:     0x7f8033e39b87 - <rustc_session[3b49a9109e54b2fc]::session::Session>::time::<(), rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#7}>
  31:     0x7f8033f7b306 - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  32:     0x7f8033791448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  33:     0x7f8035530b09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  34:     0x7f8035191e23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  35:     0x7f80336bef13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7f803370b088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  37:     0x7f80336cb788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7f80336fbba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  39:     0x7f803370d5a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f80336c9265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f8032bd147e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  42:     0x7f803296bb43 - <unknown>
  43:     0x7f80329fda00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



---- [ui] tests/ui/associated-inherent-types/generic-associated-types-bad.rs#item stdout ----

error in revision `item`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/generic-associated-types-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "item" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/generic-associated-types-bad.item" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/generic-associated-types-bad.item/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(1, Some(1))`,
 right: `(2, Some(2))`: wrong number of generic parameters for DefId(0:4 ~ generic_associated_types_bad[4434]::{impl#0}::Pr): [Ty, std::string::String]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7f53e7ec6b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7f53e7f33bc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7f53e7ebb3f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7f53e7ec6941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7f53e7ec9b14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7f53e7ec97fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7f53e89c24f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f53e7eca231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7f53e7ec9fa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7f53e7ec7006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7f53e7ec9c87 - rust_begin_unwind
  11:     0x7f53e7e802c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7f53e7e80736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7f53e8781cbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7f53e9144e63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7f53e93271da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7f53e9324f15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7f53e9330886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7f53e92b5fbe - rustc_hir_analysis[9f63bfe09a33ab11]::collect::type_of::type_of
  19:     0x7f53ea833ee3 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::type_of, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  20:     0x7f53ea494aa3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::type_of
  21:     0x7f53e92769f4 - <rustc_hir_analysis[9f63bfe09a33ab11]::collect::CollectItemTypesVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_item
  22:     0x7f53e92224a0 - <rustc_middle[51b2e9d83909998]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[9f63bfe09a33ab11]::collect::CollectItemTypesVisitor>
  23:     0x7f53e9275d9d - rustc_hir_analysis[9f63bfe09a33ab11]::collect::collect_mod_item_types
  24:     0x7f53ea79ff29 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::collect_mod_item_types, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7f53ea4c8b29 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::collect_mod_item_types
  26:     0x7f53e9221d27 - <rustc_middle[51b2e9d83909998]::hir::map::Map>::for_each_module::<rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  27:     0x7f53e913fcac - <rustc_session[3b49a9109e54b2fc]::session::Session>::track_errors::<rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#0}, ()>
  28:     0x7f53e92801bb - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  29:     0x7f53e8a96448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  30:     0x7f53ea835b09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  31:     0x7f53ea496e23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  32:     0x7f53e89c3f13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  33:     0x7f53e8a10088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  34:     0x7f53e89d0788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  35:     0x7f53e8a00ba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7f53e8a125a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7f53e89ce265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f53e7ed647e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  39:     0x7f53e7c70b43 - <unknown>
  40:     0x7f53e7d02a00 - <unknown>
  41:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (55a11147b 2023-03-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [type_of] computing type of `_`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] tests/ui/associated-inherent-types/generic-associated-types-bad.rs#region stdout ----
---- [ui] tests/ui/associated-inherent-types/generic-associated-types-bad.rs#region stdout ----

error in revision `region`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/associated-inherent-types/generic-associated-types-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "region" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/generic-associated-types-bad.region" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-inherent-types/generic-associated-types-bad.region/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `(1, Some(1))`,
 right: `(2, Some(2))`: wrong number of generic parameters for DefId(0:6 ~ generic_associated_types_bad[4434]::{impl#0}::Static): [Ty, &'a str]', /checkout/compiler/rustc_middle/src/ty/context.rs:1896:13
stack backtrace:
   0:     0x7fb95866cb35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3568d33f0bd005f2
   1:     0x7fb9586d9bc8 - core::fmt::write::hfa9dc75674c83f74
   2:     0x7fb9586613f1 - std::io::Write::write_fmt::hf28d90c59cf73831
   3:     0x7fb95866c941 - std::sys_common::backtrace::print::h1c0d8b977a208936
   4:     0x7fb95866fb14 - std::panicking::default_hook::{{closure}}::hddfa295ebcd1a8b9
   5:     0x7fb95866f7fa - std::panicking::default_hook::hd0097697e449c588
   6:     0x7fb9591684f5 - rustc_driver_impl[397b6771423a9e92]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb958670231 - std::panicking::rust_panic_with_hook::h48780f1da16eec90
   8:     0x7fb95866ffa9 - std::panicking::begin_panic_handler::{{closure}}::h0f5d7b980c4ccb18
   9:     0x7fb95866d006 - std::sys_common::backtrace::__rust_end_short_backtrace::h820f5925ec918d8b
  10:     0x7fb95866fc87 - rust_begin_unwind
  11:     0x7fb9586262c3 - core::panicking::panic_fmt::h5d97c33b59f696fe
  12:     0x7fb958626736 - core::panicking::assert_failed_inner::h32083f9c6cd4ca98
  13:     0x7fb958f27cbb - core[2b7d217fb62bdef4]::panicking::assert_failed::<(usize, core[2b7d217fb62bdef4]::option::Option<usize>), (usize, core[2b7d217fb62bdef4]::option::Option<usize>)>
  14:     0x7fb9598eae63 - <rustc_middle[51b2e9d83909998]::ty::context::TyCtxt>::mk_alias_ty::<&rustc_middle[51b2e9d83909998]::ty::list::List<rustc_middle[51b2e9d83909998]::ty::subst::GenericArg>>
  15:     0x7fb959acd1da - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty::{closure#0}
  16:     0x7fb959acaf15 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::associated_path_to_ty
  17:     0x7fb959ad6886 - <dyn rustc_hir_analysis[9f63bfe09a33ab11]::astconv::AstConv>::ast_ty_to_ty_inner
  18:     0x7fb959648f6f - <rustc_hir_typeck[b7f5b175c883c7b2]::fn_ctxt::FnCtxt>::to_ty
  19:     0x7fb95982ef3f - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor>::declare
  20:     0x7fb95982f5af - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_local
  21:     0x7fb95982de94 - <rustc_hir_typeck[b7f5b175c883c7b2]::gather_locals::GatherLocalsVisitor as rustc_hir[7799c8372af40a4e]::intravisit::Visitor>::visit_block
  22:     0x7fb959814571 - rustc_hir_typeck[b7f5b175c883c7b2]::check::check_fn
  23:     0x7fb95980f6ad - rustc_hir_typeck[b7f5b175c883c7b2]::typeck
  24:     0x7fb95afd70ac - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  25:     0x7fb95ac70589 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck
  26:     0x7fb95974b6bb - rustc_data_structures[6ed9cf05cd26a8f9]::sync::par_for_each_in::<&[rustc_span[7ea47835a767f4a9]::def_id::LocalDefId], <rustc_middle[51b2e9d83909998]::hir::map::Map>::par_body_owners<rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies::{closure#0}>::{closure#0}>
  27:     0x7fb95980d0f3 - rustc_hir_typeck[b7f5b175c883c7b2]::typeck_item_bodies
  28:     0x7fb95af00644 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::typeck_item_bodies, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  29:     0x7fb95ac6fce3 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::typeck_item_bodies
  30:     0x7fb9598e4b87 - <rustc_session[3b49a9109e54b2fc]::session::Session>::time::<(), rustc_hir_analysis[9f63bfe09a33ab11]::check_crate::{closure#7}>
  31:     0x7fb959a26306 - rustc_hir_analysis[9f63bfe09a33ab11]::check_crate
  32:     0x7fb95923c448 - rustc_interface[4cfbd9a8b1a27347]::passes::analysis
  33:     0x7fb95afdbb09 - rustc_query_system[7cf89328e3753ee6]::query::plumbing::try_execute_query::<rustc_query_impl[787292f363d2c747]::queries::analysis, rustc_query_impl[787292f363d2c747]::plumbing::QueryCtxt>
  34:     0x7fb95ac3ce23 - <rustc_query_impl[787292f363d2c747]::Queries as rustc_middle[51b2e9d83909998]::ty::query::QueryEngine>::analysis
  35:     0x7fb959169f13 - <rustc_middle[51b2e9d83909998]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  36:     0x7fb9591b6088 - <rustc_interface[4cfbd9a8b1a27347]::interface::Compiler>::enter::<rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}::{closure#2}, core[2b7d217fb62bdef4]::result::Result<core[2b7d217fb62bdef4]::option::Option<rustc_interface[4cfbd9a8b1a27347]::queries::Linker>, rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  37:     0x7fb959176788 - rustc_span[7ea47835a767f4a9]::with_source_map::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7fb9591a6ba7 - std[1de1d57a907b03a7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>
  39:     0x7fb9591b85a6 - std[1de1d57a907b03a7]::panicking::try::<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, core[2b7d217fb62bdef4]::panic::unwind_safe::AssertUnwindSafe<<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7fb959174265 - <<std[1de1d57a907b03a7]::thread::Builder>::spawn_unchecked_<rustc_interface[4cfbd9a8b1a27347]::util::run_in_thread_pool_with_globals<rustc_interface[4cfbd9a8b1a27347]::interface::run_compiler<core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>, rustc_driver_impl[397b6771423a9e92]::run_compiler::{closure#1}>::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[2b7d217fb62bdef4]::result::Result<(), rustc_span[7ea47835a767f4a9]::ErrorGuaranteed>>::{closure#1} as core[2b7d217fb62bdef4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7fb95867c47e - std::sys::unix::thread::Thread::new::thread_start::h4dfae64fcc091211
  42:     0x7fb958416b43 - <unknown>
  43:     0x7fb9584a8a00 - <unknown>
