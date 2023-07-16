plain
........................................................................................ 3344/13110
........................................................................................ 3432/13110
........................................................................................ 3520/13110
........................................................................................ 3608/13110
........................F......................................F........................ 3696/13110
........................................................................................ 3872/13110
.....................................i.................................................. 3960/13110
...................................................................................i.... 4048/13110
........................................................................................ 4136/13110
---
failures:

---- [ui] src/test/ui/error-codes/E0604.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0604" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0604/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'internal error: entered unreachable code: We're checking CastError::CastToChar.', compiler/rustc_typeck/src/check/cast.rs:366:34
stack backtrace:
   0:     0x7f7884ba197c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc395f9f80afa240a
   1:     0x7f7884c07f88 - core::fmt::write::hda02b3f7867ce775
   2:     0x7f7884b91801 - std::io::Write::write_fmt::hff7afe9550b31248
   3:     0x7f7884ba49ce - std::panicking::default_hook::{{closure}}::h8a234e098b625bf0
   4:     0x7f7884ba45fc - std::panicking::default_hook::haad06314ffadc53e
   5:     0x7f788573a081 - rustc_driver[946c1f1c45dc6ca4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7884ba522e - std::panicking::rust_panic_with_hook::h09d4f84b684ccdcc
   7:     0x7f7884ba5027 - std::panicking::begin_panic_handler::{{closure}}::hb1c6899353abfd94
   8:     0x7f7884ba1e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h417532c13275ba1b
   9:     0x7f7884ba4d09 - rust_begin_unwind
  10:     0x7f7884b59073 - core::panicking::panic_fmt::ha67b92d6a7eaae50
  11:     0x7f7886270995 - <rustc_typeck[5b95e64b79640036]::check::cast::CastCheck>::check
  12:     0x7f7885f20499 - <rustc_typeck[5b95e64b79640036]::check::fn_ctxt::FnCtxt>::check_casts
  13:     0x7f788611b11a - <rustc_infer[8236a63aaf999999]::infer::InferCtxtBuilder>::enter::<&rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults, <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>::{closure#0}>
  14:     0x7f788623774e - <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>
  15:     0x7f788603bbea - rustc_typeck[5b95e64b79640036]::check::typeck
  16:     0x7f78870ebf16 - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<rustc_span[ae298b9cf0e19e99]::def_id::LocalDefId, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>>
  17:     0x7f78872074e7 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  18:     0x7f7887040c24 - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck
  19:     0x7f78861dc9db - <rustc_middle[79f2da083fe9a345]::hir::map::Map>::par_body_owners::<rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies::{closure#0}>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  20:     0x7f788604065d - rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies
  21:     0x7f78871339aa - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), ()>>
  22:     0x7f78871cb045 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck_item_bodies, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  23:     0x7f78870406ce - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck_item_bodies
  24:     0x7f78860a34ca - <rustc_session[d554e3a5cb8222ae]::session::Session>::time::<(), rustc_typeck[5b95e64b79640036]::check_crate::{closure#7}>
  25:     0x7f788609fa88 - rustc_typeck[5b95e64b79640036]::check_crate
  26:     0x7f788580d0e1 - rustc_interface[7be494d712923013]::passes::analysis
  27:     0x7f788712806e - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>>
  28:     0x7f78872078c2 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::analysis, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  29:     0x7f78870245de - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::analysis
  30:     0x7f788571d894 - <rustc_interface[7be494d712923013]::passes::QueryContext>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  31:     0x7f78856c98ce - <rustc_interface[7be494d712923013]::interface::Compiler>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}, core[292f3ed7959c8b17]::result::Result<core[292f3ed7959c8b17]::option::Option<rustc_interface[7be494d712923013]::queries::Linker>, rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  32:     0x7f78856ab76b - rustc_span[ae298b9cf0e19e99]::with_source_map::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_interface[7be494d712923013]::interface::create_compiler_and_run<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#1}>
  33:     0x7f78856caa69 - <scoped_tls[1ac104e6e55e4698]::ScopedKey<rustc_span[ae298b9cf0e19e99]::SessionGlobals>>::set::<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  34:     0x7f7885720dd9 - std[a9307880c121f96b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  35:     0x7f78856def41 - std[a9307880c121f96b]::panicking::try::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, core[292f3ed7959c8b17]::panic::unwind_safe::AssertUnwindSafe<<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7f7885721ad2 - <<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1} as core[292f3ed7959c8b17]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f7884bb05b3 - std::sys::unix::thread::Thread::new::thread_start::h20ea5121ee985991
  38:     0x7f787f102609 - start_thread
  39:     0x7f7884a15163 - clone
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (635deced7 2022-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/error-festival.rs stdout ----
---- [ui] src/test/ui/error-festival.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-festival.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `y` in this scope
   |
LL |     y = 2;
   |     ^ help: a local variable with a similar name exists: `x`

---
   |
LL |     const FOO: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^

error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
   |
LL |     x += 2;
   |     -^^^^^
   |     |
   |     |
   |     cannot use `+=` on type `&str`

error[E0599]: no method named `z` found for reference `&str` in the current scope
   |
LL |     x.z();
   |       ^ method not found in `&str`


error[E0600]: cannot apply unary operator `!` to type `Question`
   |
   |
LL |     !Question::Yes;
   |     ^^^^^^^^^^^^^^ cannot apply unary operator `!`
   |
note: an implementation of `Not` might be missing for `Question`
   |
   |
LL | enum Question {
   | ^^^^^^^^^^^^^ must implement `Not`
  --> /checkout/library/core/src/ops/bit.rs:34:1
   |
LL | / pub trait Not {
LL | / pub trait Not {
LL | |     /// The resulting type after applying the `!` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn not(self) -> Self::Output;
LL | | }


thread 'rustc' panicked at 'internal error: entered unreachable code: We're checking CastError::CastToChar.', compiler/rustc_typeck/src/check/cast.rs:366:34
stack backtrace:
   0:     0x7faf56c3997c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc395f9f80afa240a
   1:     0x7faf56c9ff88 - core::fmt::write::hda02b3f7867ce775
   2:     0x7faf56c29801 - std::io::Write::write_fmt::hff7afe9550b31248
   3:     0x7faf56c3c9ce - std::panicking::default_hook::{{closure}}::h8a234e098b625bf0
   4:     0x7faf56c3c5fc - std::panicking::default_hook::haad06314ffadc53e
   5:     0x7faf577d2081 - rustc_driver[946c1f1c45dc6ca4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faf56c3d22e - std::panicking::rust_panic_with_hook::h09d4f84b684ccdcc
   7:     0x7faf56c3d027 - std::panicking::begin_panic_handler::{{closure}}::hb1c6899353abfd94
   8:     0x7faf56c39e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h417532c13275ba1b
   9:     0x7faf56c3cd09 - rust_begin_unwind
  10:     0x7faf56bf1073 - core::panicking::panic_fmt::ha67b92d6a7eaae50
  11:     0x7faf58308995 - <rustc_typeck[5b95e64b79640036]::check::cast::CastCheck>::check
  12:     0x7faf57fb8499 - <rustc_typeck[5b95e64b79640036]::check::fn_ctxt::FnCtxt>::check_casts
  13:     0x7faf581b311a - <rustc_infer[8236a63aaf999999]::infer::InferCtxtBuilder>::enter::<&rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults, <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>::{closure#0}>
  14:     0x7faf582cf74e - <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>
  15:     0x7faf580d3bea - rustc_typeck[5b95e64b79640036]::check::typeck
  16:     0x7faf59183f16 - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<rustc_span[ae298b9cf0e19e99]::def_id::LocalDefId, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>>
  17:     0x7faf5929f4e7 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  18:     0x7faf590d8c24 - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck
  19:     0x7faf582749db - <rustc_middle[79f2da083fe9a345]::hir::map::Map>::par_body_owners::<rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies::{closure#0}>
  20:     0x7faf580d865d - rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies
  21:     0x7faf591cb9aa - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), ()>>
  22:     0x7faf59263045 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck_item_bodies, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  23:     0x7faf590d86ce - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck_item_bodies
  24:     0x7faf5813b4ca - <rustc_session[d554e3a5cb8222ae]::session::Session>::time::<(), rustc_typeck[5b95e64b79640036]::check_crate::{closure#7}>
  25:     0x7faf58137a88 - rustc_typeck[5b95e64b79640036]::check_crate
  26:     0x7faf578a50e1 - rustc_interface[7be494d712923013]::passes::analysis
  27:     0x7faf591c006e - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>>
  28:     0x7faf5929f8c2 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::analysis, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  29:     0x7faf590bc5de - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::analysis
  30:     0x7faf577b5894 - <rustc_interface[7be494d712923013]::passes::QueryContext>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  31:     0x7faf577618ce - <rustc_interface[7be494d712923013]::interface::Compiler>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}, core[292f3ed7959c8b17]::result::Result<core[292f3ed7959c8b17]::option::Option<rustc_interface[7be494d712923013]::queries::Linker>, rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  32:     0x7faf5774376b - rustc_span[ae298b9cf0e19e99]::with_source_map::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_interface[7be494d712923013]::interface::create_compiler_and_run<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#1}>
  33:     0x7faf57762a69 - <scoped_tls[1ac104e6e55e4698]::ScopedKey<rustc_span[ae298b9cf0e19e99]::SessionGlobals>>::set::<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  34:     0x7faf577b8dd9 - std[a9307880c121f96b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  35:     0x7faf57776f41 - std[a9307880c121f96b]::panicking::try::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, core[292f3ed7959c8b17]::panic::unwind_safe::AssertUnwindSafe<<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7faf577b9ad2 - <<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1} as core[292f3ed7959c8b17]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7faf56c485b3 - std::sys::unix::thread::Thread::new::thread_start::h20ea5121ee985991
  38:     0x7faf5119a609 - start_thread
  39:     0x7faf56aad163 - clone
  40:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (635deced7 2022-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0368, E0425, E0599, E0600, E0603.
For more information about an error, try `rustc --explain E0368`.
For more information about an error, try `rustc --explain E0368`.
------------------------------------------


---- [ui] src/test/ui/mismatched_types/cast-rfc0401.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/cast-rfc0401.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/cast-rfc0401/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0606]: casting `*const U` as `*const V` is invalid
   |
   |
LL |     u as *const V //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0606]: casting `*const U` as `*const str` is invalid
   |
   |
LL |     u as *const str //~ ERROR is invalid
   |
   |
   = note: vtable kinds may not match

error[E0609]: no field `f` on type `fn() {main}`
   |
   |
LL |     let _ = main.f as *const u32; //~ ERROR no field


error[E0605]: non-primitive cast: `*const u8` as `&u8`
   |
   |
LL |     let _ = v as &u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^ invalid cast
help: consider borrowing the value
   |
   |
LL -     let _ = v as &u8; //~ ERROR non-primitive cast
LL +     let _ = &*v; //~ ERROR non-primitive cast


error[E0605]: non-primitive cast: `*const u8` as `E`
   |
   |
LL |     let _ = v as E; //~ ERROR non-primitive cast
   |             ^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `*const u8` as `fn()`
   |
   |
LL |     let _ = v as fn(); //~ ERROR non-primitive cast
   |             ^^^^^^^^^ invalid cast

error[E0605]: non-primitive cast: `*const u8` as `(u32,)`
   |
   |
LL |     let _ = v as (u32,); //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0605]: non-primitive cast: `Option<&*const u8>` as `*const u8`
   |
   |
LL |     let _ = Some(&v) as *const u8; //~ ERROR non-primitive cast
   |             ^^^^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

error[E0606]: casting `*const u8` as `f32` is invalid
   |
   |
LL |     let _ = v as f32; //~ ERROR is invalid


error[E0606]: casting `fn() {main}` as `f64` is invalid
   |
   |
LL |     let _ = main as f64; //~ ERROR is invalid


error[E0606]: casting `&*const u8` as `usize` is invalid
   |
   |
LL |     let _ = &v as usize; //~ ERROR is invalid
   |
   = help: cast through a raw pointer first


error[E0606]: casting `f32` as `*const u8` is invalid
   |
   |
LL |     let _ = f as *const u8; //~ ERROR is invalid

error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:39:13
   |
   |
LL |     let _ = 3_i32 as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^^ help: compare with zero instead: `3_i32 != 0`
error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/mismatched_types/cast-rfc0401.rs:40:13
   |
   |
LL |     let _ = E::A as bool; //~ ERROR cannot cast
   |             ^^^^^^^^^^^^ unsupported cast

thread 'rustc' panicked at 'internal error: entered unreachable code: We're checking CastError::CastToChar.', compiler/rustc_typeck/src/check/cast.rs:366:34
stack backtrace:
   0:     0x7f1324a4297c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc395f9f80afa240a
   1:     0x7f1324aa8f88 - core::fmt::write::hda02b3f7867ce775
   2:     0x7f1324a32801 - std::io::Write::write_fmt::hff7afe9550b31248
   3:     0x7f1324a459ce - std::panicking::default_hook::{{closure}}::h8a234e098b625bf0
   4:     0x7f1324a455fc - std::panicking::default_hook::haad06314ffadc53e
   5:     0x7f13255db081 - rustc_driver[946c1f1c45dc6ca4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1324a4622e - std::panicking::rust_panic_with_hook::h09d4f84b684ccdcc
   7:     0x7f1324a46027 - std::panicking::begin_panic_handler::{{closure}}::hb1c6899353abfd94
   8:     0x7f1324a42e94 - std::sys_common::backtrace::__rust_end_short_backtrace::h417532c13275ba1b
   9:     0x7f1324a45d09 - rust_begin_unwind
  10:     0x7f13249fa073 - core::panicking::panic_fmt::ha67b92d6a7eaae50
  11:     0x7f1326111995 - <rustc_typeck[5b95e64b79640036]::check::cast::CastCheck>::check
  12:     0x7f1325dc1499 - <rustc_typeck[5b95e64b79640036]::check::fn_ctxt::FnCtxt>::check_casts
  13:     0x7f1325fbc11a - <rustc_infer[8236a63aaf999999]::infer::InferCtxtBuilder>::enter::<&rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults, <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>::{closure#0}>
  14:     0x7f13260d874e - <rustc_typeck[5b95e64b79640036]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[5b95e64b79640036]::check::typeck_with_fallback<rustc_typeck[5b95e64b79640036]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>
  15:     0x7f1325edcbea - rustc_typeck[5b95e64b79640036]::check::typeck
  16:     0x7f1326f8cf16 - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<rustc_span[ae298b9cf0e19e99]::def_id::LocalDefId, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>>
  17:     0x7f13270a84e7 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  18:     0x7f1326ee1c24 - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck
  19:     0x7f1325edcd34 - rustc_typeck[5b95e64b79640036]::check::typeck
  20:     0x7f1326f8cf16 - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<rustc_span[ae298b9cf0e19e99]::def_id::LocalDefId, &rustc_middle[79f2da083fe9a345]::ty::context::TypeckResults>>
  21:     0x7f13270a84e7 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  22:     0x7f1326ee1c24 - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck
  23:     0x7f132607d9db - <rustc_middle[79f2da083fe9a345]::hir::map::Map>::par_body_owners::<rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies::{closure#0}>
  24:     0x7f1325ee165d - rustc_typeck[5b95e64b79640036]::check::typeck_item_bodies
  25:     0x7f1326fd49aa - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), ()>>
  26:     0x7f132706c045 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::typeck_item_bodies, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  27:     0x7f1326ee16ce - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::typeck_item_bodies
  28:     0x7f1325f444ca - <rustc_session[d554e3a5cb8222ae]::session::Session>::time::<(), rustc_typeck[5b95e64b79640036]::check_crate::{closure#7}>
  29:     0x7f1325f40a88 - rustc_typeck[5b95e64b79640036]::check_crate
  30:     0x7f13256ae0e1 - rustc_interface[7be494d712923013]::passes::analysis
  31:     0x7f1326fc906e - rustc_query_system[c78069d595a8230e]::query::plumbing::try_execute_query::<rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt, rustc_query_system[c78069d595a8230e]::query::caches::DefaultCache<(), core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>>
  32:     0x7f13270a88c2 - rustc_query_system[c78069d595a8230e]::query::plumbing::get_query::<rustc_query_impl[265d686f657d61ca]::queries::analysis, rustc_query_impl[265d686f657d61ca]::plumbing::QueryCtxt>
  33:     0x7f1326ec55de - <rustc_query_impl[265d686f657d61ca]::Queries as rustc_middle[79f2da083fe9a345]::ty::query::QueryEngine>::analysis
  34:     0x7f13255be894 - <rustc_interface[7be494d712923013]::passes::QueryContext>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  35:     0x7f132556a8ce - <rustc_interface[7be494d712923013]::interface::Compiler>::enter::<rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}::{closure#2}, core[292f3ed7959c8b17]::result::Result<core[292f3ed7959c8b17]::option::Option<rustc_interface[7be494d712923013]::queries::Linker>, rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  36:     0x7f132554c76b - rustc_span[ae298b9cf0e19e99]::with_source_map::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_interface[7be494d712923013]::interface::create_compiler_and_run<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7f132556ba69 - <scoped_tls[1ac104e6e55e4698]::ScopedKey<rustc_span[ae298b9cf0e19e99]::SessionGlobals>>::set::<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  38:     0x7f13255c1dd9 - std[a9307880c121f96b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>
  39:     0x7f132557ff41 - std[a9307880c121f96b]::panicking::try::<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, core[292f3ed7959c8b17]::panic::unwind_safe::AssertUnwindSafe<<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f13255c2ad2 - <<std[a9307880c121f96b]::thread::Builder>::spawn_unchecked_<rustc_interface[7be494d712923013]::util::run_in_thread_pool_with_globals<rustc_interface[7be494d712923013]::interface::run_compiler<core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>, rustc_driver[946c1f1c45dc6ca4]::run_compiler::{closure#1}>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#0}, core[292f3ed7959c8b17]::result::Result<(), rustc_errors[df5968e2760cc5cf]::ErrorGuaranteed>>::{closure#1} as core[292f3ed7959c8b17]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f1324a515b3 - std::sys::unix::thread::Thread::new::thread_start::h20ea5121ee985991
  42:     0x7f131efa3609 - start_thread
  43:     0x7f13248b6163 - clone
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (635deced7 2022-05-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck] type-checking `main::{closure#0}`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
error: aborting due to 14 previous errors

Some errors have detailed explanations: E0054, E0605, E0606, E0609.
For more information about an error, try `rustc --explain E0054`.
