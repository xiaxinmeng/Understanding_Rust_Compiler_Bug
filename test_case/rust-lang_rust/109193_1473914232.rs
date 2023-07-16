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
........................................................................................ 352/14654
........................................................................................ 440/14654
........................................................................................ 528/14654
........................................................................................ 616/14654
............................................F..........................................F 704/14654
.............................................................F............F............. 792/14654
............................................................................i........... 968/14654
........................................................................................ 1056/14654
.....i.................................................................................. 1144/14654
........................................................................................ 1232/14654
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   |                    ^ required by this bound in `T::`

error: internal compiler error: compiler/rustc_hir_typeck/src/closure.rs:726:18: async fn generator return type not an inference variable: T::{opaque#0}
  --> fake-test-src-base/async-await/async-trait-fn.rs:6:20
   |
LL |     async fn foo() {} //~ ERROR functions in traits cannot be declared `async`

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:995:33
stack backtrace:
   0:     0x7f104ff6fb35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29071a543ba2f865
   0:     0x7f104ff6fb35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29071a543ba2f865
   1:     0x7f104ffdcbc8 - core::fmt::write::h0f1e63104fa59481
   2:     0x7f104ff643f1 - std::io::Write::write_fmt::h6badfbb0956a8bab
   3:     0x7f104ff6f941 - std::sys_common::backtrace::print::hc2e321a282a18f30
   4:     0x7f104ff72b14 - std::panicking::default_hook::{{closure}}::he838910dd1f5f90c
   5:     0x7f104ff727fa - std::panicking::default_hook::h3847350559c6c615
   6:     0x7f1050a69b95 - rustc_driver_impl[9cc2f217ea204821]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f104ff73231 - std::panicking::rust_panic_with_hook::he280f6605ee76208
   8:     0x7f10510930a3 - std[b94469896d417d60]::panicking::begin_panic::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>::{closure#0}
   9:     0x7f10510897a6 - std[b94469896d417d60]::sys_common::backtrace::__rust_end_short_backtrace::<std[b94469896d417d60]::panicking::begin_panic<rustc_errors[12be86b61f2e9c40]::ExplicitBug>::{closure#0}, !>
  10:     0x7f1050806676 - std[b94469896d417d60]::panicking::begin_panic::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>
  11:     0x7f10510483d6 - std[b94469896d417d60]::panic::panic_any::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>
  12:     0x7f1051045b94 - <rustc_errors[12be86b61f2e9c40]::HandlerInner>::span_bug::<rustc_span[be483264c2fca268]::span_encoding::Span, &alloc[60865e789aa515c4]::string::String>
  13:     0x7f1051045427 - <rustc_errors[12be86b61f2e9c40]::Handler>::span_bug::<rustc_span[be483264c2fca268]::span_encoding::Span, &alloc[60865e789aa515c4]::string::String>
  14:     0x7f1051053cd5 - rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}
  15:     0x7f1051053f7c - rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_opt::<rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f105104d696 - rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_context_opt::<rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_opt<rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f105104d149 - rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>
  18:     0x7f1050806717 - rustc_middle[cb1dd98d325a8ead]::util::bug::span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>
  19:     0x7f1050f9bc95 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  20:     0x7f1050f99c01 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  21:     0x7f1050f95223 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_closure
  22:     0x7f1050f9df5f - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7f1050f388cf - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7f1050f9d732 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7f1050f3a7ca - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7f1051112283 - rustc_hir_typeck[238f566e07f0839c]::check::check_fn
  27:     0x7f105110cf7d - rustc_hir_typeck[238f566e07f0839c]::typeck
  28:     0x7f10528d0e9c - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::typeck, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  29:     0x7f105256c109 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::typeck
  30:     0x7f10538b8940 - <rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt>::typeck_opt_const_arg
  31:     0x7f1051bd1260 - rustc_mir_build[2f40c5c2d4987112]::thir::cx::thir_body
  32:     0x7f10528eb6a5 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::thir_body, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  33:     0x7f1052540e81 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::thir_body
  34:     0x7f1051b42ec5 - rustc_mir_build[2f40c5c2d4987112]::build::mir_build
  35:     0x7f1051b424b0 - rustc_mir_build[2f40c5c2d4987112]::build::mir_built
  36:     0x7f10528e63ab - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_built, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  37:     0x7f10525444b3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_built
  38:     0x7f10514c6887 - rustc_mir_transform[f3670fd578099175]::check_unsafety::unsafety_check_result
  39:     0x7f10514c2f93 - <rustc_mir_transform[f3670fd578099175]::check_unsafety::provide::{closure#0} as core[bc873f7eb778192d]::ops::function::FnOnce<(rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt, rustc_span[be483264c2fca268]::def_id::LocalDefId)>>::call_once
  40:     0x7f105283c530 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::unsafety_check_result, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  41:     0x7f105255ffd9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7f10514f936f - rustc_mir_transform[f3670fd578099175]::mir_const
  43:     0x7f10528e78ed - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_const, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  44:     0x7f1052544db3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_const
  45:     0x7f10514f9d24 - rustc_mir_transform[f3670fd578099175]::mir_promoted
  46:     0x7f10527691c5 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_promoted, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  47:     0x7f1052548401 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_promoted
  48:     0x7f1051cd6888 - rustc_borrowck[c3e2d33d4fc924d4]::mir_borrowck
  49:     0x7f1051ca27c3 - <rustc_borrowck[c3e2d33d4fc924d4]::provide::{closure#0} as core[bc873f7eb778192d]::ops::function::FnOnce<(rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt, rustc_span[be483264c2fca268]::def_id::LocalDefId)>>::call_once
  50:     0x7f1052766067 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_borrowck, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  51:     0x7f105256f5e9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_borrowck
  52:     0x7f1051359f43 - rustc_hir_analysis[c6a445867f29e18b]::collect::type_of::find_opaque_ty_constraints_for_rpit
  53:     0x7f1051359276 - rustc_hir_analysis[c6a445867f29e18b]::collect::type_of::type_of
  54:     0x7f10528d3cd3 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::type_of, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  55:     0x7f1052536623 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::type_of
  56:     0x7f10511fb8ef - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_opaque
  57:     0x7f10511ffdb4 - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_item_type
  58:     0x7f105120c6aa - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_mod_item_types
  59:     0x7f1052819379 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::check_mod_item_types, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  60:     0x7f10525669b9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::check_mod_item_types
  61:     0x7f10512c4c27 - <rustc_middle[cb1dd98d325a8ead]::hir::map::Map>::for_each_module::<rustc_hir_analysis[c6a445867f29e18b]::check_crate::{closure#6}::{closure#0}>
  62:     0x7f10511e1ea2 - <rustc_session[ff95bf87dd92883b]::session::Session>::time::<(), rustc_hir_analysis[c6a445867f29e18b]::check_crate::{closure#6}>
  63:     0x7f1051322b44 - rustc_hir_analysis[c6a445867f29e18b]::check_crate
  64:     0x7f1050b3d948 - rustc_interface[bdfc87f8dad10d70]::passes::analysis
  65:     0x7f10528d58f9 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::analysis, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  66:     0x7f10525389a3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::analysis
  67:     0x7f1050a6b5d3 - <rustc_middle[cb1dd98d325a8ead]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  68:     0x7f1050ab7c78 - <rustc_interface[bdfc87f8dad10d70]::interface::Compiler>::enter::<rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}::{closure#2}, core[bc873f7eb778192d]::result::Result<core[bc873f7eb778192d]::option::Option<rustc_interface[bdfc87f8dad10d70]::queries::Linker>, rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  69:     0x7f1050a71d98 - rustc_span[be483264c2fca268]::with_source_map::<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7f1050aa8767 - std[b94469896d417d60]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  71:     0x7f1050aba1f6 - std[b94469896d417d60]::panicking::try::<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, core[bc873f7eb778192d]::panic::unwind_safe::AssertUnwindSafe<<std[b94469896d417d60]::thread::Builder>::spawn_unchecked_<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f1050a76725 - <<std[b94469896d417d60]::thread::Builder>::spawn_unchecked_<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#1} as core[bc873f7eb778192d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f104ff7f47e - std::sys::unix::thread::Thread::new::thread_start::h0dd2372df140ba88
  74:     0x7f104fd19b43 - <unknown>
  75:     0x7f104fdaba00 - <unknown>
  76:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (74de6dad8 2023-03-17) running on x86_64-unknown-linux-gnu


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
   0:     0x7fe5b21d6b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29071a543ba2f865
   0:     0x7fe5b21d6b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29071a543ba2f865
   1:     0x7fe5b2243bc8 - core::fmt::write::h0f1e63104fa59481
   2:     0x7fe5b21cb3f1 - std::io::Write::write_fmt::h6badfbb0956a8bab
   3:     0x7fe5b21d6941 - std::sys_common::backtrace::print::hc2e321a282a18f30
   4:     0x7fe5b21d9b14 - std::panicking::default_hook::{{closure}}::he838910dd1f5f90c
   5:     0x7fe5b21d97fa - std::panicking::default_hook::h3847350559c6c615
   6:     0x7fe5b2cd0b95 - rustc_driver_impl[9cc2f217ea204821]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe5b21da231 - std::panicking::rust_panic_with_hook::he280f6605ee76208
   8:     0x7fe5b32fa0a3 - std[b94469896d417d60]::panicking::begin_panic::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>::{closure#0}
   9:     0x7fe5b32f07a6 - std[b94469896d417d60]::sys_common::backtrace::__rust_end_short_backtrace::<std[b94469896d417d60]::panicking::begin_panic<rustc_errors[12be86b61f2e9c40]::ExplicitBug>::{closure#0}, !>
  10:     0x7fe5b2a6d676 - std[b94469896d417d60]::panicking::begin_panic::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>
  11:     0x7fe5b32af3d6 - std[b94469896d417d60]::panic::panic_any::<rustc_errors[12be86b61f2e9c40]::ExplicitBug>
  12:     0x7fe5b32acb94 - <rustc_errors[12be86b61f2e9c40]::HandlerInner>::span_bug::<rustc_span[be483264c2fca268]::span_encoding::Span, &alloc[60865e789aa515c4]::string::String>
  13:     0x7fe5b32ac427 - <rustc_errors[12be86b61f2e9c40]::Handler>::span_bug::<rustc_span[be483264c2fca268]::span_encoding::Span, &alloc[60865e789aa515c4]::string::String>
  14:     0x7fe5b32bacd5 - rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}
  15:     0x7fe5b32baf7c - rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_opt::<rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fe5b32b4696 - rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_context_opt::<rustc_middle[cb1dd98d325a8ead]::ty::context::tls::with_opt<rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt<rustc_span[be483264c2fca268]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fe5b32b4149 - rustc_middle[cb1dd98d325a8ead]::util::bug::opt_span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>
  18:     0x7fe5b2a6d717 - rustc_middle[cb1dd98d325a8ead]::util::bug::span_bug_fmt::<rustc_span[be483264c2fca268]::span_encoding::Span>
  19:     0x7fe5b3202c95 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::supplied_sig_of_closure
  20:     0x7fe5b3200c01 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::sig_of_closure_no_expectation
  21:     0x7fe5b31fc223 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_closure
  22:     0x7fe5b3204f5f - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_kind
  23:     0x7fe5b319f8cf - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  24:     0x7fe5b3204732 - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_expr_with_expectation
  25:     0x7fe5b31a17ca - <rustc_hir_typeck[238f566e07f0839c]::fn_ctxt::FnCtxt>::check_return_expr
  26:     0x7fe5b3379283 - rustc_hir_typeck[238f566e07f0839c]::check::check_fn
  27:     0x7fe5b3373f7d - rustc_hir_typeck[238f566e07f0839c]::typeck
  28:     0x7fe5b4b37e9c - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::typeck, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  29:     0x7fe5b47d3109 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::typeck
  30:     0x7fe5b5b1f940 - <rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt>::typeck_opt_const_arg
  31:     0x7fe5b3e38260 - rustc_mir_build[2f40c5c2d4987112]::thir::cx::thir_body
  32:     0x7fe5b4b526a5 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::thir_body, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  33:     0x7fe5b47a7e81 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::thir_body
  34:     0x7fe5b3da9ec5 - rustc_mir_build[2f40c5c2d4987112]::build::mir_build
  35:     0x7fe5b3da94b0 - rustc_mir_build[2f40c5c2d4987112]::build::mir_built
  36:     0x7fe5b4b4d3ab - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_built, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  37:     0x7fe5b47ab4b3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_built
  38:     0x7fe5b372d887 - rustc_mir_transform[f3670fd578099175]::check_unsafety::unsafety_check_result
  39:     0x7fe5b3729f93 - <rustc_mir_transform[f3670fd578099175]::check_unsafety::provide::{closure#0} as core[bc873f7eb778192d]::ops::function::FnOnce<(rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt, rustc_span[be483264c2fca268]::def_id::LocalDefId)>>::call_once
  40:     0x7fe5b4aa3530 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::unsafety_check_result, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  41:     0x7fe5b47c6fd9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::unsafety_check_result
  42:     0x7fe5b376036f - rustc_mir_transform[f3670fd578099175]::mir_const
  43:     0x7fe5b4b4e8ed - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_const, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  44:     0x7fe5b47abdb3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_const
  45:     0x7fe5b3760d24 - rustc_mir_transform[f3670fd578099175]::mir_promoted
  46:     0x7fe5b49d01c5 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_promoted, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  47:     0x7fe5b47af401 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_promoted
  48:     0x7fe5b3f3d888 - rustc_borrowck[c3e2d33d4fc924d4]::mir_borrowck
  49:     0x7fe5b3f097c3 - <rustc_borrowck[c3e2d33d4fc924d4]::provide::{closure#0} as core[bc873f7eb778192d]::ops::function::FnOnce<(rustc_middle[cb1dd98d325a8ead]::ty::context::TyCtxt, rustc_span[be483264c2fca268]::def_id::LocalDefId)>>::call_once
  50:     0x7fe5b49cd067 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::mir_borrowck, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  51:     0x7fe5b47d65e9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::mir_borrowck
  52:     0x7fe5b35c0f43 - rustc_hir_analysis[c6a445867f29e18b]::collect::type_of::find_opaque_ty_constraints_for_rpit
  53:     0x7fe5b35c0276 - rustc_hir_analysis[c6a445867f29e18b]::collect::type_of::type_of
  54:     0x7fe5b4b3acd3 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::type_of, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  55:     0x7fe5b479d623 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::type_of
  56:     0x7fe5b34628ef - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_opaque
  57:     0x7fe5b3466db4 - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_item_type
  58:     0x7fe5b34736aa - rustc_hir_analysis[c6a445867f29e18b]::check::check::check_mod_item_types
  59:     0x7fe5b4a80379 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::check_mod_item_types, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  60:     0x7fe5b47cd9b9 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::check_mod_item_types
  61:     0x7fe5b352bc27 - <rustc_middle[cb1dd98d325a8ead]::hir::map::Map>::for_each_module::<rustc_hir_analysis[c6a445867f29e18b]::check_crate::{closure#6}::{closure#0}>
  62:     0x7fe5b3448ea2 - <rustc_session[ff95bf87dd92883b]::session::Session>::time::<(), rustc_hir_analysis[c6a445867f29e18b]::check_crate::{closure#6}>
  63:     0x7fe5b3589b44 - rustc_hir_analysis[c6a445867f29e18b]::check_crate
  64:     0x7fe5b2da4948 - rustc_interface[bdfc87f8dad10d70]::passes::analysis
  65:     0x7fe5b4b3c8f9 - rustc_query_system[4202f8e02344ab4e]::query::plumbing::try_execute_query::<rustc_query_impl[d0ec4c716684c269]::queries::analysis, rustc_query_impl[d0ec4c716684c269]::plumbing::QueryCtxt>
  66:     0x7fe5b479f9a3 - <rustc_query_impl[d0ec4c716684c269]::Queries as rustc_middle[cb1dd98d325a8ead]::ty::query::QueryEngine>::analysis
  67:     0x7fe5b2cd25d3 - <rustc_middle[cb1dd98d325a8ead]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  68:     0x7fe5b2d1ec78 - <rustc_interface[bdfc87f8dad10d70]::interface::Compiler>::enter::<rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}::{closure#2}, core[bc873f7eb778192d]::result::Result<core[bc873f7eb778192d]::option::Option<rustc_interface[bdfc87f8dad10d70]::queries::Linker>, rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  69:     0x7fe5b2cd8d98 - rustc_span[be483264c2fca268]::with_source_map::<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7fe5b2d0f767 - std[b94469896d417d60]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>
  71:     0x7fe5b2d211f6 - std[b94469896d417d60]::panicking::try::<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, core[bc873f7eb778192d]::panic::unwind_safe::AssertUnwindSafe<<std[b94469896d417d60]::thread::Builder>::spawn_unchecked_<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7fe5b2cdd725 - <<std[b94469896d417d60]::thread::Builder>::spawn_unchecked_<rustc_interface[bdfc87f8dad10d70]::util::run_in_thread_pool_with_globals<rustc_interface[bdfc87f8dad10d70]::interface::run_compiler<core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>, rustc_driver_impl[9cc2f217ea204821]::run_compiler::{closure#1}>::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc873f7eb778192d]::result::Result<(), rustc_span[be483264c2fca268]::ErrorGuaranteed>>::{closure#1} as core[bc873f7eb778192d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7fe5b21e647e - std::sys::unix::thread::Thread::new::thread_start::h0dd2372df140ba88
  74:     0x7fe5b1f80b43 - <unknown>
  75:     0x7fe5b2012a00 - <unknown>
  76:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (74de6dad8 2023-03-17) running on x86_64-unknown-linux-gnu


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


---- [ui] tests/ui/async-await/in-trait/generics-mismatch.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/generics-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/generics-mismatch.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/generics-mismatch.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error[E0053]: method `foo` has an incompatible generic parameter for trait `Foo`
  --> fake-test-src-base/async-await/in-trait/generics-mismatch.rs:13:18
LL | trait Foo {
   |       ---
LL |     async fn foo<T>();
   |                  - expected type parameter
   |                  - expected type parameter
...
LL | impl Foo for () {
   | ---------------
LL |     async fn foo<const N: usize>() {}
   |                  ^^^^^^^^^^^^^^ found const parameter of type `usize`

error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:848:18: expected trait item, found HirId(DefId(0:13 ~ generics_mismatch[3425]::Foo::{opaque#0}).0) (unknown node)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7fe694bd5b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h29071a543ba2f865
   1:     0x7fe694c42bc8 - core::fmt::write::h0f1e63104fa59481
   1:     0x7fe694c42bc8 - core::fmt::write::h0f1e63104fa59481
   2:     0x7fe694bca3f1 - std::io::Write::write_fmt::h6badfbb0956a8bab
   3:     0x7fe694bd5941 - std::sys_common::backtrace::print::hc2e321a282a18f30
   4:     0x7fe694bd8b14 - std::panicking::default_hook::{{closure}}::he838910dd1f5f90c
   5:     0x7fe694bd87fa - std::panicking::default_hook::h3847350559c6c615
   6:     0x7fe6956cfb95 - rustc_driver_impl[9cc2f217ea204821]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe694bd9231 - std::panicking::rust_panic_with_hook::he280f6605ee76208
