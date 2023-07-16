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
Successfully built 71f34bd3d764
Successfully tagged rust-ci:latest
Built container sha256:71f34bd3d764bcc71d1f96a8c71da092c65a67235cbaa765e5da414f4d98944d
Uploading finished image to https://ci-caches.rust-lang.org/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78
upload failed: - to s3://rust-lang-ci-sccache2/docker/c5b89d39ad489cccff774ea7e5f2ba4bac05c17cec4e3fc9bc723a692a9f276f24875116b84f5c924781dbf77a3fb5eb823154191e0d213687b94a5b21600b78 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
.........................................................iii............................ 13376/13452
............................................................................
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
   0:     0x7ff0452ef12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb05fc2047b3ffa8a
   1:     0x7ff045357c68 - core::fmt::write::hcdf30c593fbea752
   2:     0x7ff0452df9a1 - std::io::Write::write_fmt::hc39087f327c6e63a
   3:     0x7ff0452f211e - std::panicking::default_hook::{{closure}}::h94f5f412f85a2c48
   4:     0x7ff0452f1de7 - std::panicking::default_hook::hbaf32b4cbb8583fc
   5:     0x7ff045ca5714 - rustc_driver[d1fafcebf668dbf6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff0452f28d1 - std::panicking::rust_panic_with_hook::h9971f09bf8e52cc1
   7:     0x7ff04877bf03 - std[a4a65442c62ed73e]::panicking::begin_panic::<rustc_errors[648b08d855d0f249]::ExplicitBug>::{closure#0}
   8:     0x7ff04877b196 - std[a4a65442c62ed73e]::sys_common::backtrace::__rust_end_short_backtrace::<std[a4a65442c62ed73e]::panicking::begin_panic<rustc_errors[648b08d855d0f249]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff045c3d616 - std[a4a65442c62ed73e]::panicking::begin_panic::<rustc_errors[648b08d855d0f249]::ExplicitBug>
  10:     0x7ff04877afb6 - std[a4a65442c62ed73e]::panic::panic_any::<rustc_errors[648b08d855d0f249]::ExplicitBug>
  11:     0x7ff0487755c6 - <rustc_errors[648b08d855d0f249]::HandlerInner>::bug::<&alloc[f548f3539343f2a7]::string::String>
  12:     0x7ff048775090 - <rustc_errors[648b08d855d0f249]::Handler>::bug::<&alloc[f548f3539343f2a7]::string::String>
  13:     0x7ff048891f2e - rustc_middle[8c890ad0eeabd44a]::ty::context::tls::with_context_opt::<rustc_middle[8c890ad0eeabd44a]::ty::context::tls::with_opt<rustc_middle[8c890ad0eeabd44a]::util::bug::opt_span_bug_fmt<rustc_span[7d523c8e4ebbaaec]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  14:     0x7ff048892ca9 - rustc_middle[8c890ad0eeabd44a]::util::bug::opt_span_bug_fmt::<rustc_span[7d523c8e4ebbaaec]::span_encoding::Span>
  15:     0x7ff045c4c658 - rustc_middle[8c890ad0eeabd44a]::util::bug::bug_fmt
  16:     0x7ff046df6e1f - <rustc_borrowck[8b66c16e7dacd2aa]::universal_regions::UniversalRegionIndices>::to_region_vid
  17:     0x7ff046df8c42 - <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::add_implied_bounds
  18:     0x7ff046de3c74 - <&mut <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0} as core[77539812655752c7]::ops::function::FnOnce<(rustc_middle[8c890ad0eeabd44a]::ty::Ty,)>>::call_once
  19:     0x7ff046ecc492 - <core[77539812655752c7]::iter::adapters::flatten::FlatMap<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::cloned::Cloned<core[77539812655752c7]::slice::iter::Iter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::option::IntoIter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}> as core[77539812655752c7]::iter::traits::iterator::Iterator>::next
  20:     0x7ff046ec6c22 - <alloc[f548f3539343f2a7]::vec::Vec<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints> as alloc[f548f3539343f2a7]::vec::spec_from_iter::SpecFromIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints, core[77539812655752c7]::iter::adapters::flatten::FlatMap<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::cloned::Cloned<core[77539812655752c7]::slice::iter::Iter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::option::IntoIter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}>>>::from_iter
  21:     0x7ff046df0327 - rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::create
  22:     0x7ff046ef1ec4 - rustc_borrowck[8b66c16e7dacd2aa]::type_check::type_check
  23:     0x7ff046d94312 - rustc_borrowck[8b66c16e7dacd2aa]::nll::compute_regions
  24:     0x7ff046f56be7 - rustc_borrowck[8b66c16e7dacd2aa]::do_mir_borrowck
  25:     0x7ff046d23b8f - <rustc_infer[6cbbca7b43ed0c83]::infer::InferCtxtBuilder>::enter::<rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult, rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck::{closure#0}>
  26:     0x7ff046f481a7 - rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck
  27:     0x7ff046f1779e - <rustc_borrowck[8b66c16e7dacd2aa]::provide::{closure#0} as core[77539812655752c7]::ops::function::FnOnce<(rustc_middle[8c890ad0eeabd44a]::ty::context::TyCtxt, rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId)>>::call_once
  28:     0x7ff047819414 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::try_execute_query::<rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt, rustc_query_system[e65e4f5159fbdeee]::query::caches::DefaultCache<rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId, &rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult>>
  29:     0x7ff0478e9543 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::get_query::<rustc_query_impl[9c54576b682f786c]::queries::mir_borrowck, rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt>
  30:     0x7ff047762960 - <rustc_query_impl[9c54576b682f786c]::Queries as rustc_middle[8c890ad0eeabd44a]::ty::query::QueryEngine>::mir_borrowck
  31:     0x7ff046efbf34 - <rustc_borrowck[8b66c16e7dacd2aa]::type_check::TypeChecker>::prove_closure_bounds
  32:     0x7ff046f096b3 - <rustc_borrowck[8b66c16e7dacd2aa]::type_check::TypeChecker>::check_rvalue
  33:     0x7ff046f0ee71 - <rustc_borrowck[8b66c16e7dacd2aa]::type_check::TypeChecker>::typeck_mir
  34:     0x7ff046ef2fbf - rustc_borrowck[8b66c16e7dacd2aa]::type_check::type_check
  35:     0x7ff046d94312 - rustc_borrowck[8b66c16e7dacd2aa]::nll::compute_regions
  36:     0x7ff046f56be7 - rustc_borrowck[8b66c16e7dacd2aa]::do_mir_borrowck
  37:     0x7ff046d23b8f - <rustc_infer[6cbbca7b43ed0c83]::infer::InferCtxtBuilder>::enter::<rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult, rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck::{closure#0}>
  38:     0x7ff046f481a7 - rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck
  39:     0x7ff046f1779e - <rustc_borrowck[8b66c16e7dacd2aa]::provide::{closure#0} as core[77539812655752c7]::ops::function::FnOnce<(rustc_middle[8c890ad0eeabd44a]::ty::context::TyCtxt, rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId)>>::call_once
  40:     0x7ff047819414 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::try_execute_query::<rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt, rustc_query_system[e65e4f5159fbdeee]::query::caches::DefaultCache<rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId, &rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult>>
  41:     0x7ff0478e9543 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::get_query::<rustc_query_impl[9c54576b682f786c]::queries::mir_borrowck, rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt>
  42:     0x7ff047762960 - <rustc_query_impl[9c54576b682f786c]::Queries as rustc_middle[8c890ad0eeabd44a]::ty::query::QueryEngine>::mir_borrowck
  43:     0x7ff045e4fbf6 - <core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[77539812655752c7]::ops::function::FnOnce<()>>::call_once
  44:     0x7ff045dc59a9 - std[a4a65442c62ed73e]::panicking::try::<(), core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  45:     0x7ff045da50dd - rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in::<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  46:     0x7ff045e36ed5 - <rustc_session[e203300fce7ddc6d]::session::Session>::time::<(), rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}>
  47:     0x7ff045dfe58b - rustc_interface[c6c1738ec755d852]::passes::analysis
  48:     0x7ff04785a674 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::try_execute_query::<rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt, rustc_query_system[e65e4f5159fbdeee]::query::caches::DefaultCache<(), core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>>
  49:     0x7ff04793b684 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::get_query::<rustc_query_impl[9c54576b682f786c]::queries::analysis, rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt>
  50:     0x7ff04773836a - <rustc_query_impl[9c54576b682f786c]::Queries as rustc_middle[8c890ad0eeabd44a]::ty::query::QueryEngine>::analysis
  51:     0x7ff045d04006 - <rustc_interface[c6c1738ec755d852]::passes::QueryContext>::enter::<rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  52:     0x7ff045cadb7c - <rustc_interface[c6c1738ec755d852]::interface::Compiler>::enter::<rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}::{closure#2}, core[77539812655752c7]::result::Result<core[77539812655752c7]::option::Option<rustc_interface[c6c1738ec755d852]::queries::Linker>, rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  53:     0x7ff045c943d1 - rustc_span[7d523c8e4ebbaaec]::with_source_map::<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_interface[c6c1738ec755d852]::interface::create_compiler_and_run<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7ff045cc9191 - rustc_interface[c6c1738ec755d852]::interface::create_compiler_and_run::<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>
  55:     0x7ff045c906c2 - <scoped_tls[9537188a63803edd]::ScopedKey<rustc_span[7d523c8e4ebbaaec]::SessionGlobals>>::set::<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  56:     0x7ff045d07acf - std[a4a65442c62ed73e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  57:     0x7ff045c9641e - std[a4a65442c62ed73e]::panic::catch_unwind::<core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<<std[a4a65442c62ed73e]::thread::Builder>::spawn_unchecked_<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  58:     0x7ff045d0a310 - <<std[a4a65442c62ed73e]::thread::Builder>::spawn_unchecked_<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#1} as core[77539812655752c7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7ff0452ff645 - std::sys::unix::thread::Thread::new::thread_start::h2c8966d42c63d15d
  60:     0x7ff04509bb43 - <unknown>
  61:     0x7ff04512da00 - <unknown>
  62:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (70b7d2861 2022-09-02) running on x86_64-unknown-linux-gnu

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
   0:     0x7ff0452ef12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb05fc2047b3ffa8a
   1:     0x7ff045357c68 - core::fmt::write::hcdf30c593fbea752
   2:     0x7ff0452df9a1 - std::io::Write::write_fmt::hc39087f327c6e63a
   3:     0x7ff0452f211e - std::panicking::default_hook::{{closure}}::h94f5f412f85a2c48
   4:     0x7ff0452f1de7 - std::panicking::default_hook::hbaf32b4cbb8583fc
   5:     0x7ff045ca5714 - rustc_driver[d1fafcebf668dbf6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff0452f28d1 - std::panicking::rust_panic_with_hook::h9971f09bf8e52cc1
   7:     0x7ff04877bf03 - std[a4a65442c62ed73e]::panicking::begin_panic::<rustc_errors[648b08d855d0f249]::ExplicitBug>::{closure#0}
   8:     0x7ff04877b196 - std[a4a65442c62ed73e]::sys_common::backtrace::__rust_end_short_backtrace::<std[a4a65442c62ed73e]::panicking::begin_panic<rustc_errors[648b08d855d0f249]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff045c3d616 - std[a4a65442c62ed73e]::panicking::begin_panic::<rustc_errors[648b08d855d0f249]::ExplicitBug>
  10:     0x7ff04877afb6 - std[a4a65442c62ed73e]::panic::panic_any::<rustc_errors[648b08d855d0f249]::ExplicitBug>
  11:     0x7ff0487755c6 - <rustc_errors[648b08d855d0f249]::HandlerInner>::bug::<&alloc[f548f3539343f2a7]::string::String>
  12:     0x7ff048775090 - <rustc_errors[648b08d855d0f249]::Handler>::bug::<&alloc[f548f3539343f2a7]::string::String>
  13:     0x7ff048891f2e - rustc_middle[8c890ad0eeabd44a]::ty::context::tls::with_context_opt::<rustc_middle[8c890ad0eeabd44a]::ty::context::tls::with_opt<rustc_middle[8c890ad0eeabd44a]::util::bug::opt_span_bug_fmt<rustc_span[7d523c8e4ebbaaec]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7ff048892ca9 - rustc_middle[8c890ad0eeabd44a]::util::bug::opt_span_bug_fmt::<rustc_span[7d523c8e4ebbaaec]::span_encoding::Span>
  15:     0x7ff045c4c658 - rustc_middle[8c890ad0eeabd44a]::util::bug::bug_fmt
  16:     0x7ff046df6e1f - <rustc_borrowck[8b66c16e7dacd2aa]::universal_regions::UniversalRegionIndices>::to_region_vid
  17:     0x7ff046df8c42 - <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::add_implied_bounds
  18:     0x7ff046de3c74 - <&mut <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0} as core[77539812655752c7]::ops::function::FnOnce<(rustc_middle[8c890ad0eeabd44a]::ty::Ty,)>>::call_once
  19:     0x7ff046ecc492 - <core[77539812655752c7]::iter::adapters::flatten::FlatMap<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::cloned::Cloned<core[77539812655752c7]::slice::iter::Iter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::option::IntoIter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}> as core[77539812655752c7]::iter::traits::iterator::Iterator>::next
  20:     0x7ff046ec6c22 - <alloc[f548f3539343f2a7]::vec::Vec<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints> as alloc[f548f3539343f2a7]::vec::spec_from_iter::SpecFromIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints, core[77539812655752c7]::iter::adapters::flatten::FlatMap<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::cloned::Cloned<core[77539812655752c7]::slice::iter::Iter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::option::IntoIter<rustc_middle[8c890ad0eeabd44a]::ty::Ty>>, core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::iter::adapters::chain::Chain<core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, core[77539812655752c7]::option::IntoIter<&rustc_middle[8c890ad0eeabd44a]::infer::canonical::QueryRegionConstraints>>, <rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::UniversalRegionRelationsBuilder>::create::{closure#0}>>>::from_iter
  21:     0x7ff046df0327 - rustc_borrowck[8b66c16e7dacd2aa]::type_check::free_region_relations::create
  22:     0x7ff046ef1ec4 - rustc_borrowck[8b66c16e7dacd2aa]::type_check::type_check
  23:     0x7ff046d94312 - rustc_borrowck[8b66c16e7dacd2aa]::nll::compute_regions
  24:     0x7ff046f56be7 - rustc_borrowck[8b66c16e7dacd2aa]::do_mir_borrowck
  25:     0x7ff046d23b8f - <rustc_infer[6cbbca7b43ed0c83]::infer::InferCtxtBuilder>::enter::<rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult, rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck::{closure#0}>
  26:     0x7ff046f481a7 - rustc_borrowck[8b66c16e7dacd2aa]::mir_borrowck
  27:     0x7ff046f1779e - <rustc_borrowck[8b66c16e7dacd2aa]::provide::{closure#0} as core[77539812655752c7]::ops::function::FnOnce<(rustc_middle[8c890ad0eeabd44a]::ty::context::TyCtxt, rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId)>>::call_once
  28:     0x7ff047819414 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::try_execute_query::<rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt, rustc_query_system[e65e4f5159fbdeee]::query::caches::DefaultCache<rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId, &rustc_middle[8c890ad0eeabd44a]::mir::query::BorrowCheckResult>>
  29:     0x7ff0478e9543 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::get_query::<rustc_query_impl[9c54576b682f786c]::queries::mir_borrowck, rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt>
  30:     0x7ff047762960 - <rustc_query_impl[9c54576b682f786c]::Queries as rustc_middle[8c890ad0eeabd44a]::ty::query::QueryEngine>::mir_borrowck
  31:     0x7ff045e4fbf6 - <core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[77539812655752c7]::ops::function::FnOnce<()>>::call_once
  32:     0x7ff045dc59a9 - std[a4a65442c62ed73e]::panicking::try::<(), core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  33:     0x7ff045da50dd - rustc_data_structures[1882dc3cdec47aa]::sync::par_for_each_in::<&[rustc_span[7d523c8e4ebbaaec]::def_id::LocalDefId], <rustc_middle[8c890ad0eeabd44a]::hir::map::Map>::par_body_owners<rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  34:     0x7ff045e36ed5 - <rustc_session[e203300fce7ddc6d]::session::Session>::time::<(), rustc_interface[c6c1738ec755d852]::passes::analysis::{closure#2}>
  35:     0x7ff045dfe58b - rustc_interface[c6c1738ec755d852]::passes::analysis
  36:     0x7ff04785a674 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::try_execute_query::<rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt, rustc_query_system[e65e4f5159fbdeee]::query::caches::DefaultCache<(), core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>>
  37:     0x7ff04793b684 - rustc_query_system[e65e4f5159fbdeee]::query::plumbing::get_query::<rustc_query_impl[9c54576b682f786c]::queries::analysis, rustc_query_impl[9c54576b682f786c]::plumbing::QueryCtxt>
  38:     0x7ff04773836a - <rustc_query_impl[9c54576b682f786c]::Queries as rustc_middle[8c890ad0eeabd44a]::ty::query::QueryEngine>::analysis
  39:     0x7ff045d04006 - <rustc_interface[c6c1738ec755d852]::passes::QueryContext>::enter::<rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  40:     0x7ff045cadb7c - <rustc_interface[c6c1738ec755d852]::interface::Compiler>::enter::<rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}::{closure#2}, core[77539812655752c7]::result::Result<core[77539812655752c7]::option::Option<rustc_interface[c6c1738ec755d852]::queries::Linker>, rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  41:     0x7ff045c943d1 - rustc_span[7d523c8e4ebbaaec]::with_source_map::<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_interface[c6c1738ec755d852]::interface::create_compiler_and_run<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#1}>
  42:     0x7ff045cc9191 - rustc_interface[c6c1738ec755d852]::interface::create_compiler_and_run::<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>
  43:     0x7ff045c906c2 - <scoped_tls[9537188a63803edd]::ScopedKey<rustc_span[7d523c8e4ebbaaec]::SessionGlobals>>::set::<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  44:     0x7ff045d07acf - std[a4a65442c62ed73e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  45:     0x7ff045c9641e - std[a4a65442c62ed73e]::panic::catch_unwind::<core[77539812655752c7]::panic::unwind_safe::AssertUnwindSafe<<std[a4a65442c62ed73e]::thread::Builder>::spawn_unchecked_<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>
  46:     0x7ff045d0a310 - <<std[a4a65442c62ed73e]::thread::Builder>::spawn_unchecked_<rustc_interface[c6c1738ec755d852]::util::run_in_thread_pool_with_globals<rustc_interface[c6c1738ec755d852]::interface::run_compiler<core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>, rustc_driver[d1fafcebf668dbf6]::run_compiler::{closure#1}>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#0}, core[77539812655752c7]::result::Result<(), rustc_errors[648b08d855d0f249]::ErrorGuaranteed>>::{closure#1} as core[77539812655752c7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7ff0452ff645 - std::sys::unix::thread::Thread::new::thread_start::h2c8966d42c63d15d
  48:     0x7ff04509bb43 - <unknown>
  49:     0x7ff04512da00 - <unknown>
  50:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (70b7d2861 2022-09-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `fail::{closure#1}`
#1 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors
------------------------------------------


