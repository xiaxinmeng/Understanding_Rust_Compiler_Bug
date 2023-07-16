plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 352/14652
........................................................................................ 440/14652
........................................................................................ 528/14652
........................................................................................ 616/14652
............................................F..........................................F 704/14652
......................................................................F................. 792/14652
..........................................................................i............. 968/14652
........................................................................................ 1056/14652
...i.................................................................................... 1144/14652
........................................................................................ 1232/14652
---
failures:

---- [ui] tests/ui/async-await/async-trait-fn.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/async-trait-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-trait-fn.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-trait-fn.next/auxiliary" "--edition=2018" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:5
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/async-await/async-trait-fn.rs:7:5
   |
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/async-await/async-trait-fn.rs:8:5
   |
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
   |     |
   |     |
   |     `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable

error[E0277]: the trait bound `impl Future<Output = ()>: Future` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |                    ^ the trait `Future` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#0}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |                    ^ required by this bound in `T::`

error[E0277]: the trait bound `impl Future<Output = ()>: Sized` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |                    ^ the trait `Sized` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#0}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`
   |                    ^ required by this bound in `T::`

error[E0277]: the trait bound `impl Future<Output = ()>: Future` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:7:25
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |                         ^ the trait `Future` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#1}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:7:25
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |                         ^ required by this bound in `T::`

error[E0277]: the trait bound `impl Future<Output = ()>: Sized` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:7:25
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |                         ^ the trait `Sized` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#1}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:7:25
   |
LL |     async fn bar(&self) {} //~ ERROR functions in traits cannot be declared `async`
   |                         ^ required by this bound in `T::`

error[E0277]: the trait bound `impl Future<Output = ()>: Future` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:8:20
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
   |                    ^ the trait `Future` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#2}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:8:20
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
   |                    ^ required by this bound in `T::`

error[E0277]: the trait bound `impl Future<Output = ()>: Sized` is not satisfied
  --> fake-test-src-base/async-await/async-trait-fn.rs:8:20
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
   |                    ^ the trait `Sized` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `T::{opaque#2}`
  --> fake-test-src-base/async-await/async-trait-fn.rs:8:20
   |
LL |     async fn baz() { //~ ERROR functions in traits cannot be declared `async`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                    ^ required by this bound in `T::`

error: internal compiler error: compiler/rustc_hir_typeck/src/closure.rs:726:18: async fn generator return type not an inference variable: T::{opaque#0}
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:995:33
stack backtrace:
stack backtrace:
   0:     0x7fa99f18faf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c40ce509a840afd
   1:     0x7fa99f1fccc8 - core::fmt::write::he50351aba0fb4be0
   2:     0x7fa99f1841d1 - std::io::Write::write_fmt::h36271bd9955d1c92
   3:     0x7fa99f18f901 - std::sys_common::backtrace::print::haf84653e0b627020
   4:     0x7fa99f192ad4 - std::panicking::default_hook::{{closure}}::h53cb5542507c1b44
   5:     0x7fa99f1927ba - std::panicking::default_hook::hf81e15db920f2d9d
   6:     0x7fa99fc8c735 - rustc_driver_impl[6be8ea1e08e0fb50]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa99f1931f1 - std::panicking::rust_panic_with_hook::ha2e4cc89b1c0c549
   8:     0x7fa9a031fe53 - std[30eaf278f3c3e719]::panicking::begin_panic::<rustc_errors[656126943353edb0]::ExplicitBug>::{closure#0}
   9:     0x7fa9a0318a56 - std[30eaf278f3c3e719]::sys_common::backtrace::__rust_end_short_backtrace::<std[30eaf278f3c3e719]::panicking::begin_panic<rustc_errors[656126943353edb0]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa99fa26a46 - std[30eaf278f3c3e719]::panicking::begin_panic::<rustc_errors[656126943353edb0]::ExplicitBug>
  11:     0x7fa9a0272bb6 - std[30eaf278f3c3e719]::panic::panic_any::<rustc_errors[656126943353edb0]::ExplicitBug>
  12:     0x7fa9a0270264 - <rustc_errors[656126943353edb0]::HandlerInner>::span_bug::<rustc_span[baedd86fa81739d3]::span_encoding::Span, &alloc[ee6e51a699c5592e]::string::String>
  13:     0x7fa9a026faf7 - <rustc_errors[656126943353edb0]::Handler>::span_bug::<rustc_span[baedd86fa81739d3]::span_encoding::Span, &alloc[ee6e51a699c5592e]::string::String>
  14:     0x7fa9a027e3a5 - rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}
  15:     0x7fa9a027e64c - rustc_middle[91136591d5a6877e]::ty::context::tls::with_opt::<rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fa9a0277d76 - rustc_middle[91136591d5a6877e]::ty::context::tls::with_context_opt::<rustc_middle[91136591d5a6877e]::ty::context::tls::with_opt<rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fa9a0274719 - rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>
  18:     0x7fa99fa26ae7 - rustc_middle[91136591d5a6877e]::util::bug::span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>
  19:     0x7fa9a01c59c5 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  20:     0x7fa9a01c3931 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  21:     0x7fa9a01bef53 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_closure
  22:     0x7fa9a01c7c8f - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7fa9a01625ff - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7fa9a01c7462 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7fa9a01644fa - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7fa9a033cc53 - rustc_hir_typeck[cae46fd94b06e5d1]::check::check_fn
  27:     0x7fa9a033794d - rustc_hir_typeck[cae46fd94b06e5d1]::typeck
  28:     0x7fa9a1b0b768 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::typeck, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  29:     0x7fa9a17a53f9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::typeck
  30:     0x7fa9a2afbc50 - <rustc_middle[91136591d5a6877e]::ty::context::TyCtxt>::typeck_opt_const_arg
  31:     0x7fa9a0e037e0 - rustc_mir_build[1827ab6a00c511f2]::thir::cx::thir_body
  32:     0x7fa9a1b26005 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::thir_body, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  33:     0x7fa9a177a171 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::thir_body
  34:     0x7fa9a0d74868 - rustc_mir_build[1827ab6a00c511f2]::build::mir_build
  35:     0x7fa9a0d73dc0 - rustc_mir_build[1827ab6a00c511f2]::build::mir_built
  36:     0x7fa9a1b20ce0 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_built, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  37:     0x7fa9a177d7a3 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_built
  38:     0x7fa9a06f4267 - rustc_mir_transform[9208b1abc04ee969]::check_unsafety::unsafety_check_result
  39:     0x7fa9a06f0973 - <rustc_mir_transform[9208b1abc04ee969]::check_unsafety::provide::{closure#0} as core[92af54eb94feda89]::ops::function::FnOnce<(rustc_middle[91136591d5a6877e]::ty::context::TyCtxt, rustc_span[baedd86fa81739d3]::def_id::LocalDefId)>>::call_once
  40:     0x7fa9a1a76801 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::unsafety_check_result, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  41:     0x7fa9a17992c9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7fa9a07264af - rustc_mir_transform[9208b1abc04ee969]::mir_const
  43:     0x7fa9a1b2221d - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_const, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  44:     0x7fa9a177e0a3 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_const
  45:     0x7fa9a0726e64 - rustc_mir_transform[9208b1abc04ee969]::mir_promoted
  46:     0x7fa9a19a2ea5 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_promoted, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  47:     0x7fa9a17816f1 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_promoted
  48:     0x7fa9a0f09568 - rustc_borrowck[b90f2c2133d73860]::mir_borrowck
  49:     0x7fa9a0ed59a3 - <rustc_borrowck[b90f2c2133d73860]::provide::{closure#0} as core[92af54eb94feda89]::ops::function::FnOnce<(rustc_middle[91136591d5a6877e]::ty::context::TyCtxt, rustc_span[baedd86fa81739d3]::def_id::LocalDefId)>>::call_once
  50:     0x7fa9a199fd3a - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_borrowck, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  51:     0x7fa9a17a88d9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_borrowck
  52:     0x7fa9a0584f53 - rustc_hir_analysis[c27c4bc4b8cae064]::collect::type_of::find_opaque_ty_constraints_for_rpit
  53:     0x7fa9a0584286 - rustc_hir_analysis[c27c4bc4b8cae064]::collect::type_of::type_of
  54:     0x7fa9a1b0e571 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::type_of, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  55:     0x7fa9a176f913 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::type_of
  56:     0x7fa9a04264df - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_opaque
  57:     0x7fa9a042a9a4 - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_item_type
  58:     0x7fa9a043729a - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_mod_item_types
  59:     0x7fa9a1a53558 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::check_mod_item_types, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  60:     0x7fa9a179fca9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::check_mod_item_types
  61:     0x7fa9a04efd47 - <rustc_middle[91136591d5a6877e]::hir::map::Map>::for_each_module::<rustc_hir_analysis[c27c4bc4b8cae064]::check_crate::{closure#6}::{closure#0}>
  62:     0x7fa9a040c9c2 - <rustc_session[2f2abdb76d55bcea]::session::Session>::time::<(), rustc_hir_analysis[c27c4bc4b8cae064]::check_crate::{closure#6}>
  63:     0x7fa9a054dcd4 - rustc_hir_analysis[c27c4bc4b8cae064]::check_crate
  64:     0x7fa99fd62fe8 - rustc_interface[66e9e8b95de75cbf]::passes::analysis
  65:     0x7fa9a1b1019f - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::analysis, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  66:     0x7fa9a1771c93 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::analysis
  67:     0x7fa99fc8e153 - <rustc_middle[91136591d5a6877e]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  68:     0x7fa99fcdafd8 - <rustc_interface[66e9e8b95de75cbf]::interface::Compiler>::enter::<rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}::{closure#2}, core[92af54eb94feda89]::result::Result<core[92af54eb94feda89]::option::Option<rustc_interface[66e9e8b95de75cbf]::queries::Linker>, rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  69:     0x7fa99fc9acd8 - rustc_span[baedd86fa81739d3]::with_source_map::<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7fa99fccbac7 - std[30eaf278f3c3e719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  71:     0x7fa99fcdd4a6 - std[30eaf278f3c3e719]::panicking::try::<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, core[92af54eb94feda89]::panic::unwind_safe::AssertUnwindSafe<<std[30eaf278f3c3e719]::thread::Builder>::spawn_unchecked_<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7fa99fc985d5 - <<std[30eaf278f3c3e719]::thread::Builder>::spawn_unchecked_<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#1} as core[92af54eb94feda89]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7fa99f19f4de - std::sys::unix::thread::Thread::new::thread_start::h4777aea7a802d235
  74:     0x7fa99ef39b43 - <unknown>
  75:     0x7fa99efcba00 - <unknown>
  76:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (9e18aac4a 2023-03-16) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `T::foo`
#1 [thir_body] building THIR for `T::foo`
#2 [mir_built] building MIR for `T::foo`
#3 [unsafety_check_result] unsafety-checking `T::foo`
#4 [mir_const] preparing `T::foo` for borrow checking
#5 [mir_promoted] processing MIR for `T::foo`
#6 [mir_borrowck] borrow-checking `T::foo`
#7 [type_of] computing type of `T::foo::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0277, E0706.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui/async-await/edition-deny-async-fns-2015.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/edition-deny-async-fns-2015.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/edition-deny-async-fns-2015.next/auxiliary" "--edition=2015" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:5:1
   |
LL | async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:7:12
   |
LL | fn baz() { async fn foo() {} } //~ ERROR `async fn` is not permitted in Rust 2015
   |            ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:9:1
   |
LL | async fn async_baz() { //~ ERROR `async fn` is not permitted in Rust 2015
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:10:5
   |
LL |     async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:16:5
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:5
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:38:9
   |
LL |         async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:28:9
   |
LL |         async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |         ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:33:13
   |
LL |             async fn bar() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |             ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0706]: functions in traits cannot be declared `async`
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:5
   |
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |     |
   |     |
   |     `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable

error[E0277]: the trait bound `impl Future<Output = ()>: Future` is not satisfied
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:20
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |                    ^ the trait `Future` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `Bar::{opaque#0}`
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:20
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |                    ^ required by this bound in `Bar::`

error[E0277]: the trait bound `impl Future<Output = ()>: Sized` is not satisfied
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:20
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |                    ^ the trait `Sized` is not implemented for `impl Future<Output = ()>`
   |
note: required by a bound in `Bar::{opaque#0}`
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:20
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015
   |                    ^ required by this bound in `Bar::`

error: internal compiler error: compiler/rustc_hir_typeck/src/closure.rs:726:18: async fn generator return type not an inference variable: Bar::{opaque#0}
  --> fake-test-src-base/async-await/edition-deny-async-fns-2015.rs:20:20
   |
LL |     async fn foo() {} //~ ERROR `async fn` is not permitted in Rust 2015

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:995:33
stack backtrace:
   0:     0x7f91f311aaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c40ce509a840afd
   0:     0x7f91f311aaf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c40ce509a840afd
   1:     0x7f91f3187cc8 - core::fmt::write::he50351aba0fb4be0
   2:     0x7f91f310f1d1 - std::io::Write::write_fmt::h36271bd9955d1c92
   3:     0x7f91f311a901 - std::sys_common::backtrace::print::haf84653e0b627020
   4:     0x7f91f311dad4 - std::panicking::default_hook::{{closure}}::h53cb5542507c1b44
   5:     0x7f91f311d7ba - std::panicking::default_hook::hf81e15db920f2d9d
   6:     0x7f91f3c17735 - rustc_driver_impl[6be8ea1e08e0fb50]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f91f311e1f1 - std::panicking::rust_panic_with_hook::ha2e4cc89b1c0c549
   8:     0x7f91f42aae53 - std[30eaf278f3c3e719]::panicking::begin_panic::<rustc_errors[656126943353edb0]::ExplicitBug>::{closure#0}
   9:     0x7f91f42a3a56 - std[30eaf278f3c3e719]::sys_common::backtrace::__rust_end_short_backtrace::<std[30eaf278f3c3e719]::panicking::begin_panic<rustc_errors[656126943353edb0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f91f39b1a46 - std[30eaf278f3c3e719]::panicking::begin_panic::<rustc_errors[656126943353edb0]::ExplicitBug>
  11:     0x7f91f41fdbb6 - std[30eaf278f3c3e719]::panic::panic_any::<rustc_errors[656126943353edb0]::ExplicitBug>
  12:     0x7f91f41fb264 - <rustc_errors[656126943353edb0]::HandlerInner>::span_bug::<rustc_span[baedd86fa81739d3]::span_encoding::Span, &alloc[ee6e51a699c5592e]::string::String>
  13:     0x7f91f41faaf7 - <rustc_errors[656126943353edb0]::Handler>::span_bug::<rustc_span[baedd86fa81739d3]::span_encoding::Span, &alloc[ee6e51a699c5592e]::string::String>
  14:     0x7f91f42093a5 - rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}
  15:     0x7f91f420964c - rustc_middle[91136591d5a6877e]::ty::context::tls::with_opt::<rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f91f4202d76 - rustc_middle[91136591d5a6877e]::ty::context::tls::with_context_opt::<rustc_middle[91136591d5a6877e]::ty::context::tls::with_opt<rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt<rustc_span[baedd86fa81739d3]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f91f41ff719 - rustc_middle[91136591d5a6877e]::util::bug::opt_span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>
  18:     0x7f91f39b1ae7 - rustc_middle[91136591d5a6877e]::util::bug::span_bug_fmt::<rustc_span[baedd86fa81739d3]::span_encoding::Span>
  19:     0x7f91f41509c5 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  20:     0x7f91f414e931 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  21:     0x7f91f4149f53 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_closure
  22:     0x7f91f4152c8f - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7f91f40ed5ff - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7f91f4152462 - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7f91f40ef4fa - <rustc_hir_typeck[cae46fd94b06e5d1]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7f91f42c7c53 - rustc_hir_typeck[cae46fd94b06e5d1]::check::check_fn
  27:     0x7f91f42c294d - rustc_hir_typeck[cae46fd94b06e5d1]::typeck
  28:     0x7f91f5a96768 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::typeck, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  29:     0x7f91f57303f9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::typeck
  30:     0x7f91f6a86c50 - <rustc_middle[91136591d5a6877e]::ty::context::TyCtxt>::typeck_opt_const_arg
  31:     0x7f91f4d8e7e0 - rustc_mir_build[1827ab6a00c511f2]::thir::cx::thir_body
  32:     0x7f91f5ab1005 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::thir_body, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  33:     0x7f91f5705171 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::thir_body
  34:     0x7f91f4cff868 - rustc_mir_build[1827ab6a00c511f2]::build::mir_build
  35:     0x7f91f4cfedc0 - rustc_mir_build[1827ab6a00c511f2]::build::mir_built
  36:     0x7f91f5aabce0 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_built, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  37:     0x7f91f57087a3 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_built
  38:     0x7f91f467f267 - rustc_mir_transform[9208b1abc04ee969]::check_unsafety::unsafety_check_result
  39:     0x7f91f467b973 - <rustc_mir_transform[9208b1abc04ee969]::check_unsafety::provide::{closure#0} as core[92af54eb94feda89]::ops::function::FnOnce<(rustc_middle[91136591d5a6877e]::ty::context::TyCtxt, rustc_span[baedd86fa81739d3]::def_id::LocalDefId)>>::call_once
  40:     0x7f91f5a01801 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::unsafety_check_result, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  41:     0x7f91f57242c9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7f91f46b14af - rustc_mir_transform[9208b1abc04ee969]::mir_const
  43:     0x7f91f5aad21d - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_const, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  44:     0x7f91f57090a3 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_const
  45:     0x7f91f46b1e64 - rustc_mir_transform[9208b1abc04ee969]::mir_promoted
  46:     0x7f91f592dea5 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_promoted, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  47:     0x7f91f570c6f1 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_promoted
  48:     0x7f91f4e94568 - rustc_borrowck[b90f2c2133d73860]::mir_borrowck
  49:     0x7f91f4e609a3 - <rustc_borrowck[b90f2c2133d73860]::provide::{closure#0} as core[92af54eb94feda89]::ops::function::FnOnce<(rustc_middle[91136591d5a6877e]::ty::context::TyCtxt, rustc_span[baedd86fa81739d3]::def_id::LocalDefId)>>::call_once
  50:     0x7f91f592ad3a - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::mir_borrowck, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  51:     0x7f91f57338d9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::mir_borrowck
  52:     0x7f91f450ff53 - rustc_hir_analysis[c27c4bc4b8cae064]::collect::type_of::find_opaque_ty_constraints_for_rpit
  53:     0x7f91f450f286 - rustc_hir_analysis[c27c4bc4b8cae064]::collect::type_of::type_of
  54:     0x7f91f5a99571 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::type_of, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  55:     0x7f91f56fa913 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::type_of
  56:     0x7f91f43b14df - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_opaque
  57:     0x7f91f43b59a4 - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_item_type
  58:     0x7f91f43c229a - rustc_hir_analysis[c27c4bc4b8cae064]::check::check::check_mod_item_types
  59:     0x7f91f59de558 - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::check_mod_item_types, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  60:     0x7f91f572aca9 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::check_mod_item_types
  61:     0x7f91f447ad47 - <rustc_middle[91136591d5a6877e]::hir::map::Map>::for_each_module::<rustc_hir_analysis[c27c4bc4b8cae064]::check_crate::{closure#6}::{closure#0}>
  62:     0x7f91f43979c2 - <rustc_session[2f2abdb76d55bcea]::session::Session>::time::<(), rustc_hir_analysis[c27c4bc4b8cae064]::check_crate::{closure#6}>
  63:     0x7f91f44d8cd4 - rustc_hir_analysis[c27c4bc4b8cae064]::check_crate
  64:     0x7f91f3cedfe8 - rustc_interface[66e9e8b95de75cbf]::passes::analysis
  65:     0x7f91f5a9b19f - rustc_query_system[d323406f7de09c3f]::query::plumbing::try_execute_query::<rustc_query_impl[157bda5756f75dab]::queries::analysis, rustc_query_impl[157bda5756f75dab]::plumbing::QueryCtxt>
  66:     0x7f91f56fcc93 - <rustc_query_impl[157bda5756f75dab]::Queries as rustc_middle[91136591d5a6877e]::ty::query::QueryEngine>::analysis
  67:     0x7f91f3c19153 - <rustc_middle[91136591d5a6877e]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  68:     0x7f91f3c65fd8 - <rustc_interface[66e9e8b95de75cbf]::interface::Compiler>::enter::<rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}::{closure#2}, core[92af54eb94feda89]::result::Result<core[92af54eb94feda89]::option::Option<rustc_interface[66e9e8b95de75cbf]::queries::Linker>, rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  69:     0x7f91f3c25cd8 - rustc_span[baedd86fa81739d3]::with_source_map::<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7f91f3c56ac7 - std[30eaf278f3c3e719]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>
  71:     0x7f91f3c684a6 - std[30eaf278f3c3e719]::panicking::try::<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, core[92af54eb94feda89]::panic::unwind_safe::AssertUnwindSafe<<std[30eaf278f3c3e719]::thread::Builder>::spawn_unchecked_<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f91f3c235d5 - <<std[30eaf278f3c3e719]::thread::Builder>::spawn_unchecked_<rustc_interface[66e9e8b95de75cbf]::util::run_in_thread_pool_with_globals<rustc_interface[66e9e8b95de75cbf]::interface::run_compiler<core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>, rustc_driver_impl[6be8ea1e08e0fb50]::run_compiler::{closure#1}>::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[92af54eb94feda89]::result::Result<(), rustc_span[baedd86fa81739d3]::ErrorGuaranteed>>::{closure#1} as core[92af54eb94feda89]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f91f312a4de - std::sys::unix::thread::Thread::new::thread_start::h4777aea7a802d235
  74:     0x7f91f2ec4b43 - <unknown>
  75:     0x7f91f2f56a00 - <unknown>
  76:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (9e18aac4a 2023-03-16) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `Bar::foo`
#1 [thir_body] building THIR for `Bar::foo`
#2 [mir_built] building MIR for `Bar::foo`
#3 [unsafety_check_result] unsafety-checking `Bar::foo`
#4 [mir_const] preparing `Bar::foo` for borrow checking
#5 [mir_promoted] processing MIR for `Bar::foo`
#6 [mir_borrowck] borrow-checking `Bar::foo`
#7 [type_of] computing type of `Bar::foo::{opaque#0}`
#8 [check_mod_item_types] checking item types in top-level module
#9 [analysis] running analysis passes on this crate
error: aborting due to 13 previous errors

Some errors have detailed explanations: E0277, E0670, E0706.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] tests/ui/async-await/in-trait/return-type-suggestion.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/return-type-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/return-type-suggestion.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/return-type-suggestion.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/async-await/in-trait/return-type-suggestion.rs:5:12
   |
LL | #![feature(async_fn_in_trait)]
   |
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = note: `#[warn(incomplete_features)]` on by default


error[E0277]: the trait bound `impl Future<Output = ()>: Future` is not satisfied
  --> fake-test-src-base/async-await/in-trait/return-type-suggestion.rs:9:18
LL |     async fn e() {
   |                  ^ the trait `Future` is not implemented for `impl Future<Output = ()>`
   |
   |
note: required by a bound in `A::{opaque#0}`
  --> fake-test-src-base/async-await/in-trait/return-type-suggestion.rs:9:18
LL |     async fn e() {
   |                  ^ required by this bound in `A::`


error[E0277]: the trait bound `impl Future<Output = ()>: Sized` is not satisfied
  --> fake-test-src-base/async-await/in-trait/return-type-suggestion.rs:9:18
LL |     async fn e() {
   |                  ^ the trait `Sized` is not implemented for `impl Future<Output = ()>`
