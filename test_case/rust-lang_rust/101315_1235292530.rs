plain
  python3-oauthlib python3-pyparsing python3-secretstorage python3-six
  python3-software-properties python3-wadllib python3-zipp systemd
  systemd-sysv systemd-timesyncd unattended-upgrades
Suggested packages:
  default-dbus-session-bus | dbus-session-bus dbus-user-session
  pinentry-gnome3 tor parcimonie xloadimage scdaemon isoquery
  gstreamer1.0-tools iw | wireless-tools appstream pinentry-doc
  python3-apt-dbg python-apt-doc python-blinker-doc python-cryptography-doc
  python3-cryptography-vectors python-dbus-doc python3-crypto gir1.2-secret-1
  gnome-keyring libkf5wallet-bin python3-keyrings.alt python3-testresources
  python-pyparsing-doc python-secretstorage-doc systemd-container libfido2-1
  libtss2-esys-3.0.2-0 libtss2-mu0 libtss2-rc0 bsd-mailx default-mta
  | mail-transport-agent needrestart powermgmt-base
  apt-transport-https dbus dirmngr distro-info-data dmsetup gir1.2-glib-2.0
  gir1.2-packagekitglib-1.0 gnupg gnupg-l10n gnupg-utils gpg gpg-agent
  gpg-wks-client gpg-wks-server gpgconf gpgsm iso-codes libapparmor1
  libappstream4 libargon2-1 libassuan0 libcap2-bin libcryptsetup12 libdbus-1-3
---
Successfully built 1b5ba0b23e2c
Successfully tagged rust-ci:latest
Built container sha256:1b5ba0b23e2c7b7fb2d34c8ae279489f8061139e7da3f35b5e5b94d2016cfb82
Uploading finished image to https://ci-caches.rust-lang.org/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78
upload failed: - to s3://rust-lang-ci-sccache2/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
.......................................................iii.............................. 13376/13450
..........................................................................
failures:

---- [ui] src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/closure-malformed-projection-input-issue-99665.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-99665" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/closure-malformed-projection-input-issue-99665/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:809:36: cannot convert `RePlaceholder(Placeholder { universe: U1, name: BrNamed(DefId(0:26 ~ closure_malformed_projection_input_issue_99665[9dd7]::fail::'_), '_) })` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1465:9
stack backtrace:
stack backtrace:
   0:     0x7f52723cf12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbcfcef02ebd4ef5f
   1:     0x7f5272437c68 - core::fmt::write::heb777e1f8e8d5428
   2:     0x7f52723bf9a1 - std::io::Write::write_fmt::h581f1449f8b0a279
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   3:     0x7f52723d211e - std::panicking::default_hook::{{closure}}::h1c499c0cbaac4018
   4:     0x7f52723d1de7 - std::panicking::default_hook::h2fc073321f2cb032
   5:     0x7f5272d85724 - rustc_driver[afe8487019944c47]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f52723d28d1 - std::panicking::rust_panic_with_hook::hb6cede1d780318d4
   7:     0x7f5275862b33 - std[6bfb6720db0ddf9e]::panicking::begin_panic::<rustc_errors[4884e65d37508f9c]::ExplicitBug>::{closure#0}
   8:     0x7f5275861d86 - std[6bfb6720db0ddf9e]::sys_common::backtrace::__rust_end_short_backtrace::<std[6bfb6720db0ddf9e]::panicking::begin_panic<rustc_errors[4884e65d37508f9c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5272d1d656 - std[6bfb6720db0ddf9e]::panicking::begin_panic::<rustc_errors[4884e65d37508f9c]::ExplicitBug>
  10:     0x7f527585ddc6 - std[6bfb6720db0ddf9e]::panic::panic_any::<rustc_errors[4884e65d37508f9c]::ExplicitBug>
  11:     0x7f527585c1f6 - <rustc_errors[4884e65d37508f9c]::HandlerInner>::bug::<&alloc[11de014b21c49521]::string::String>
  12:     0x7f527585bcc0 - <rustc_errors[4884e65d37508f9c]::Handler>::bug::<&alloc[11de014b21c49521]::string::String>
  13:     0x7f5275978b5e - rustc_middle[f5877c76cb27ddbd]::ty::context::tls::with_context_opt::<rustc_middle[f5877c76cb27ddbd]::ty::context::tls::with_opt<rustc_middle[f5877c76cb27ddbd]::util::bug::opt_span_bug_fmt<rustc_span[efe392017a826f2c]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f52759798d9 - rustc_middle[f5877c76cb27ddbd]::util::bug::opt_span_bug_fmt::<rustc_span[efe392017a826f2c]::span_encoding::Span>
  15:     0x7f5272d2c698 - rustc_middle[f5877c76cb27ddbd]::util::bug::bug_fmt
  16:     0x7f5273eda86f - <rustc_borrowck[75b5bf5e0778e576]::universal_regions::UniversalRegionIndices>::to_region_vid
  17:     0x7f5273edc692 - <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::add_implied_bounds
  18:     0x7f5273ec76c4 - <&mut <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0} as core[f125530c738b2a6b]::ops::function::FnOnce<(rustc_middle[f5877c76cb27ddbd]::ty::Ty,)>>::call_once
  19:     0x7f5273faff62 - <core[f125530c738b2a6b]::iter::adapters::flatten::FlatMap<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::cloned::Cloned<core[f125530c738b2a6b]::slice::iter::Iter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::option::IntoIter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}> as core[f125530c738b2a6b]::iter::traits::iterator::Iterator>::next
  20:     0x7f5273faa6f2 - <alloc[11de014b21c49521]::vec::Vec<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints> as alloc[11de014b21c49521]::vec::spec_from_iter::SpecFromIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints, core[f125530c738b2a6b]::iter::adapters::flatten::FlatMap<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::cloned::Cloned<core[f125530c738b2a6b]::slice::iter::Iter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::option::IntoIter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}>>>::from_iter
  21:     0x7f5273ed3d77 - rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::create
  22:     0x7f5273fd5994 - rustc_borrowck[75b5bf5e0778e576]::type_check::type_check
  23:     0x7f5273e77be8 - rustc_borrowck[75b5bf5e0778e576]::nll::compute_regions
  24:     0x7f527403a6b7 - rustc_borrowck[75b5bf5e0778e576]::do_mir_borrowck
  25:     0x7f5273e0829f - <rustc_infer[129bb8f47bdeed8c]::infer::InferCtxtBuilder>::enter::<rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult, rustc_borrowck[75b5bf5e0778e576]::mir_borrowck::{closure#0}>
  26:     0x7f527402bc77 - rustc_borrowck[75b5bf5e0778e576]::mir_borrowck
  27:     0x7f5273ffb26e - <rustc_borrowck[75b5bf5e0778e576]::provide::{closure#0} as core[f125530c738b2a6b]::ops::function::FnOnce<(rustc_middle[f5877c76cb27ddbd]::ty::context::TyCtxt, rustc_span[efe392017a826f2c]::def_id::LocalDefId)>>::call_once
  28:     0x7f52748fd2a4 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::try_execute_query::<rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt, rustc_query_system[9a871e1179eccdb3]::query::caches::DefaultCache<rustc_span[efe392017a826f2c]::def_id::LocalDefId, &rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult>>
  29:     0x7f52749cd3d3 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::get_query::<rustc_query_impl[97fd2279b5932006]::queries::mir_borrowck, rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt>
  30:     0x7f52748467f0 - <rustc_query_impl[97fd2279b5932006]::Queries as rustc_middle[f5877c76cb27ddbd]::ty::query::QueryEngine>::mir_borrowck
  31:     0x7f5273fdfa04 - <rustc_borrowck[75b5bf5e0778e576]::type_check::TypeChecker>::prove_closure_bounds
  32:     0x7f5273fed183 - <rustc_borrowck[75b5bf5e0778e576]::type_check::TypeChecker>::check_rvalue
  33:     0x7f5273ff2941 - <rustc_borrowck[75b5bf5e0778e576]::type_check::TypeChecker>::typeck_mir
  34:     0x7f5273fd6a8f - rustc_borrowck[75b5bf5e0778e576]::type_check::type_check
  35:     0x7f5273e77be8 - rustc_borrowck[75b5bf5e0778e576]::nll::compute_regions
  36:     0x7f527403a6b7 - rustc_borrowck[75b5bf5e0778e576]::do_mir_borrowck
  37:     0x7f5273e0829f - <rustc_infer[129bb8f47bdeed8c]::infer::InferCtxtBuilder>::enter::<rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult, rustc_borrowck[75b5bf5e0778e576]::mir_borrowck::{closure#0}>
  38:     0x7f527402bc77 - rustc_borrowck[75b5bf5e0778e576]::mir_borrowck
  39:     0x7f5273ffb26e - <rustc_borrowck[75b5bf5e0778e576]::provide::{closure#0} as core[f125530c738b2a6b]::ops::function::FnOnce<(rustc_middle[f5877c76cb27ddbd]::ty::context::TyCtxt, rustc_span[efe392017a826f2c]::def_id::LocalDefId)>>::call_once
  40:     0x7f52748fd2a4 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::try_execute_query::<rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt, rustc_query_system[9a871e1179eccdb3]::query::caches::DefaultCache<rustc_span[efe392017a826f2c]::def_id::LocalDefId, &rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult>>
  41:     0x7f52749cd3d3 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::get_query::<rustc_query_impl[97fd2279b5932006]::queries::mir_borrowck, rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt>
  42:     0x7f52748467f0 - <rustc_query_impl[97fd2279b5932006]::Queries as rustc_middle[f5877c76cb27ddbd]::ty::query::QueryEngine>::mir_borrowck
  43:     0x7f5272f2fe76 - <core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[f125530c738b2a6b]::ops::function::FnOnce<()>>::call_once
  44:     0x7f5272ea2a09 - std[6bfb6720db0ddf9e]::panicking::try::<(), core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  45:     0x7f5272e855ad - rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in::<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  46:     0x7f5272f17115 - <rustc_session[bfe4a924967f2530]::session::Session>::time::<(), rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}>
  47:     0x7f5272ede7bb - rustc_interface[fc4d63bdd67dffa1]::passes::analysis
  48:     0x7f527493e504 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::try_execute_query::<rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt, rustc_query_system[9a871e1179eccdb3]::query::caches::DefaultCache<(), core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>>
  49:     0x7f5274a1f514 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::get_query::<rustc_query_impl[97fd2279b5932006]::queries::analysis, rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt>
  50:     0x7f527481c1fa - <rustc_query_impl[97fd2279b5932006]::Queries as rustc_middle[f5877c76cb27ddbd]::ty::query::QueryEngine>::analysis
  51:     0x7f5272de47a6 - <rustc_interface[fc4d63bdd67dffa1]::passes::QueryContext>::enter::<rustc_driver[afe8487019944c47]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  52:     0x7f5272d8db1c - <rustc_interface[fc4d63bdd67dffa1]::interface::Compiler>::enter::<rustc_driver[afe8487019944c47]::run_compiler::{closure#1}::{closure#2}, core[f125530c738b2a6b]::result::Result<core[f125530c738b2a6b]::option::Option<rustc_interface[fc4d63bdd67dffa1]::queries::Linker>, rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  53:     0x7f5272d743e1 - rustc_span[efe392017a826f2c]::with_source_map::<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_interface[fc4d63bdd67dffa1]::interface::create_compiler_and_run<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7f5272da9381 - rustc_interface[fc4d63bdd67dffa1]::interface::create_compiler_and_run::<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>
  55:     0x7f5272d706f2 - <scoped_tls[a00437b9557953e4]::ScopedKey<rustc_span[efe392017a826f2c]::SessionGlobals>>::set::<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  56:     0x7f5272de7bdf - std[6bfb6720db0ddf9e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  57:     0x7f5272d7548e - std[6bfb6720db0ddf9e]::panic::catch_unwind::<core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<<std[6bfb6720db0ddf9e]::thread::Builder>::spawn_unchecked_<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  58:     0x7f5272dea420 - <<std[6bfb6720db0ddf9e]::thread::Builder>::spawn_unchecked_<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#1} as core[f125530c738b2a6b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f52723df645 - std::sys::unix::thread::Thread::new::thread_start::h0584c11e97c1742b
  60:     0x7f527217bb43 - <unknown>
  61:     0x7f527220da00 - <unknown>
  62:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (c2ffbba64 2022-09-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `fail::{closure#0}`
#1 [mir_borrowck] borrow-checking `fail`
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:809:36: cannot convert `RePlaceholder(Placeholder { universe: U1, name: BrNamed(DefId(0:27 ~ closure_malformed_projection_input_issue_99665[9dd7]::fail::'_#1), '_) })` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1465:9
stack backtrace:
stack backtrace:
   0:     0x7f52723cf12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbcfcef02ebd4ef5f
   1:     0x7f5272437c68 - core::fmt::write::heb777e1f8e8d5428
   2:     0x7f52723bf9a1 - std::io::Write::write_fmt::h581f1449f8b0a279
   3:     0x7f52723d211e - std::panicking::default_hook::{{closure}}::h1c499c0cbaac4018
   4:     0x7f52723d1de7 - std::panicking::default_hook::h2fc073321f2cb032
   5:     0x7f5272d85724 - rustc_driver[afe8487019944c47]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f52723d28d1 - std::panicking::rust_panic_with_hook::hb6cede1d780318d4
   7:     0x7f5275862b33 - std[6bfb6720db0ddf9e]::panicking::begin_panic::<rustc_errors[4884e65d37508f9c]::ExplicitBug>::{closure#0}
   8:     0x7f5275861d86 - std[6bfb6720db0ddf9e]::sys_common::backtrace::__rust_end_short_backtrace::<std[6bfb6720db0ddf9e]::panicking::begin_panic<rustc_errors[4884e65d37508f9c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5272d1d656 - std[6bfb6720db0ddf9e]::panicking::begin_panic::<rustc_errors[4884e65d37508f9c]::ExplicitBug>
  10:     0x7f527585ddc6 - std[6bfb6720db0ddf9e]::panic::panic_any::<rustc_errors[4884e65d37508f9c]::ExplicitBug>
  11:     0x7f527585c1f6 - <rustc_errors[4884e65d37508f9c]::HandlerInner>::bug::<&alloc[11de014b21c49521]::string::String>
  12:     0x7f527585bcc0 - <rustc_errors[4884e65d37508f9c]::Handler>::bug::<&alloc[11de014b21c49521]::string::String>
  13:     0x7f5275978b5e - rustc_middle[f5877c76cb27ddbd]::ty::context::tls::with_context_opt::<rustc_middle[f5877c76cb27ddbd]::ty::context::tls::with_opt<rustc_middle[f5877c76cb27ddbd]::util::bug::opt_span_bug_fmt<rustc_span[efe392017a826f2c]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f52759798d9 - rustc_middle[f5877c76cb27ddbd]::util::bug::opt_span_bug_fmt::<rustc_span[efe392017a826f2c]::span_encoding::Span>
  15:     0x7f5272d2c698 - rustc_middle[f5877c76cb27ddbd]::util::bug::bug_fmt
  16:     0x7f5273eda86f - <rustc_borrowck[75b5bf5e0778e576]::universal_regions::UniversalRegionIndices>::to_region_vid
  17:     0x7f5273edc692 - <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::add_implied_bounds
  18:     0x7f5273ec76c4 - <&mut <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0} as core[f125530c738b2a6b]::ops::function::FnOnce<(rustc_middle[f5877c76cb27ddbd]::ty::Ty,)>>::call_once
  19:     0x7f5273faff62 - <core[f125530c738b2a6b]::iter::adapters::flatten::FlatMap<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::cloned::Cloned<core[f125530c738b2a6b]::slice::iter::Iter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::option::IntoIter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}> as core[f125530c738b2a6b]::iter::traits::iterator::Iterator>::next
  20:     0x7f5273faa6f2 - <alloc[11de014b21c49521]::vec::Vec<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints> as alloc[11de014b21c49521]::vec::spec_from_iter::SpecFromIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints, core[f125530c738b2a6b]::iter::adapters::flatten::FlatMap<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::cloned::Cloned<core[f125530c738b2a6b]::slice::iter::Iter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::option::IntoIter<rustc_middle[f5877c76cb27ddbd]::ty::Ty>>, core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::iter::adapters::chain::Chain<core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, core[f125530c738b2a6b]::option::IntoIter<&rustc_middle[f5877c76cb27ddbd]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}>>>::from_iter
  21:     0x7f5273ed3d77 - rustc_borrowck[75b5bf5e0778e576]::type_check::free_region_relations::create
  22:     0x7f5273fd5994 - rustc_borrowck[75b5bf5e0778e576]::type_check::type_check
  23:     0x7f5273e77be8 - rustc_borrowck[75b5bf5e0778e576]::nll::compute_regions
  24:     0x7f527403a6b7 - rustc_borrowck[75b5bf5e0778e576]::do_mir_borrowck
  25:     0x7f5273e0829f - <rustc_infer[129bb8f47bdeed8c]::infer::InferCtxtBuilder>::enter::<rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult, rustc_borrowck[75b5bf5e0778e576]::mir_borrowck::{closure#0}>
  26:     0x7f527402bc77 - rustc_borrowck[75b5bf5e0778e576]::mir_borrowck
  27:     0x7f5273ffb26e - <rustc_borrowck[75b5bf5e0778e576]::provide::{closure#0} as core[f125530c738b2a6b]::ops::function::FnOnce<(rustc_middle[f5877c76cb27ddbd]::ty::context::TyCtxt, rustc_span[efe392017a826f2c]::def_id::LocalDefId)>>::call_once
  28:     0x7f52748fd2a4 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::try_execute_query::<rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt, rustc_query_system[9a871e1179eccdb3]::query::caches::DefaultCache<rustc_span[efe392017a826f2c]::def_id::LocalDefId, &rustc_middle[f5877c76cb27ddbd]::mir::query::BorrowCheckResult>>
  29:     0x7f52749cd3d3 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::get_query::<rustc_query_impl[97fd2279b5932006]::queries::mir_borrowck, rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt>
  30:     0x7f52748467f0 - <rustc_query_impl[97fd2279b5932006]::Queries as rustc_middle[f5877c76cb27ddbd]::ty::query::QueryEngine>::mir_borrowck
  31:     0x7f5272f2fe76 - <core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[f125530c738b2a6b]::ops::function::FnOnce<()>>::call_once
  32:     0x7f5272ea2a09 - std[6bfb6720db0ddf9e]::panicking::try::<(), core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7f5272e855ad - rustc_data_structures[e1dfaf379a0b60fe]::sync::par_for_each_in::<&[rustc_span[efe392017a826f2c]::def_id::LocalDefId], <rustc_middle[f5877c76cb27ddbd]::hir::map::Map>::par_body_owners<rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  34:     0x7f5272f17115 - <rustc_session[bfe4a924967f2530]::session::Session>::time::<(), rustc_interface[fc4d63bdd67dffa1]::passes::analysis::{closure#2}>
  35:     0x7f5272ede7bb - rustc_interface[fc4d63bdd67dffa1]::passes::analysis
  36:     0x7f527493e504 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::try_execute_query::<rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt, rustc_query_system[9a871e1179eccdb3]::query::caches::DefaultCache<(), core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>>
  37:     0x7f5274a1f514 - rustc_query_system[9a871e1179eccdb3]::query::plumbing::get_query::<rustc_query_impl[97fd2279b5932006]::queries::analysis, rustc_query_impl[97fd2279b5932006]::plumbing::QueryCtxt>
  38:     0x7f527481c1fa - <rustc_query_impl[97fd2279b5932006]::Queries as rustc_middle[f5877c76cb27ddbd]::ty::query::QueryEngine>::analysis
  39:     0x7f5272de47a6 - <rustc_interface[fc4d63bdd67dffa1]::passes::QueryContext>::enter::<rustc_driver[afe8487019944c47]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  40:     0x7f5272d8db1c - <rustc_interface[fc4d63bdd67dffa1]::interface::Compiler>::enter::<rustc_driver[afe8487019944c47]::run_compiler::{closure#1}::{closure#2}, core[f125530c738b2a6b]::result::Result<core[f125530c738b2a6b]::option::Option<rustc_interface[fc4d63bdd67dffa1]::queries::Linker>, rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  41:     0x7f5272d743e1 - rustc_span[efe392017a826f2c]::with_source_map::<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_interface[fc4d63bdd67dffa1]::interface::create_compiler_and_run<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7f5272da9381 - rustc_interface[fc4d63bdd67dffa1]::interface::create_compiler_and_run::<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>
  43:     0x7f5272d706f2 - <scoped_tls[a00437b9557953e4]::ScopedKey<rustc_span[efe392017a826f2c]::SessionGlobals>>::set::<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  44:     0x7f5272de7bdf - std[6bfb6720db0ddf9e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  45:     0x7f5272d7548e - std[6bfb6720db0ddf9e]::panic::catch_unwind::<core[f125530c738b2a6b]::panic::unwind_safe::AssertUnwindSafe<<std[6bfb6720db0ddf9e]::thread::Builder>::spawn_unchecked_<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>
  46:     0x7f5272dea420 - <<std[6bfb6720db0ddf9e]::thread::Builder>::spawn_unchecked_<rustc_interface[fc4d63bdd67dffa1]::util::run_in_thread_pool_with_globals<rustc_interface[fc4d63bdd67dffa1]::interface::run_compiler<core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>, rustc_driver[afe8487019944c47]::run_compiler::{closure#1}>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#0}, core[f125530c738b2a6b]::result::Result<(), rustc_errors[4884e65d37508f9c]::ErrorGuaranteed>>::{closure#1} as core[f125530c738b2a6b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f52723df645 - std::sys::unix::thread::Thread::new::thread_start::h0584c11e97c1742b
  48:     0x7f527217bb43 - <unknown>
  49:     0x7f527220da00 - <unknown>
  50:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (c2ffbba64 2022-09-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `fail::{closure#1}`
#1 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors
------------------------------------------


