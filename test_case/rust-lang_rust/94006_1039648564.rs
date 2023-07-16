plain
......................................................................i............................. 5600/12634
.................................................................................................... 5700/12634
.................................................................................................... 5800/12634
.................................................................................................... 5900/12634
.......................................................F.......F.................................... 6000/12634
.................i...................F.............................................................. 6200/12634
..................................................................................................i. 6300/12634
.................................................................................................... 6400/12634
...............................................................i.................................... 6500/12634
---
failures:

---- [ui] ui/async-await/issue-69446-fnmut-capture.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-69446-fnmut-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f1b496fce9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f1b496fce9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f1b4976e62f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f1b496ea411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f1b4970139e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f1b49700f69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f1b4a11e8d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1b49701bc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f1b497019f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f1b496fd394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f1b497016d9 - rust_begin_unwind
  10:     0x7f1b496b5803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f1b4976a9e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f1b4976a98b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f1b496b5676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f1b4b005dc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f1b4b2b4eaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f1b4b17dde0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f1b4b2a88fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f1b4b277e7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f1b4b6bf8b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f1b4b77b266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f1b4a2aaa35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f1b4a2c1832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f1b4a35915b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f1b4b6f0a70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f1b4b7de5b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f1b4a19fa8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f1b4a162909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f1b4a138a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f1b4a135b69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f1b4a19deee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f1b4a1387aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f1b497117a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f1b43a80609 - start_thread
  34:     0x7f1b49577293 - clone
  35:                0x0 - <unknown>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/borrowck/borrowck-describe-lvalue.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-describe-lvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-describe-lvalue/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |             let y = &mut x;
   |                     ------ first mutable borrow occurs here
LL |             &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |             ^^^^^^ second mutable borrow occurs here
LL |             *y = 1;


error[E0499]: cannot borrow `x` as mutable more than once at a time
   |
   |
LL |                    let y = &mut x;
   |                            ------ first mutable borrow occurs here
LL |                    &mut x; //~ ERROR cannot borrow `x` as mutable more than once at a time
   |                    ^^^^^^ second mutable borrow occurs here
LL |                    *y = 1;


thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f9ceee3ce9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f9ceee3ce9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f9ceeeae62f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f9ceee2a411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f9ceee4139e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f9ceee40f69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f9cef85e8d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9ceee41bc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f9ceee419f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f9ceee3d394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f9ceee416d9 - rust_begin_unwind
  10:     0x7f9ceedf5803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f9ceeeaa9e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f9ceeeaa98b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f9ceedf5676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f9cf0745dc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f9cf09f4eaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f9cf08bdde0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f9cf09e88fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f9cf09b7e7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f9cf0dff8b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f9cf0ebb266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f9cef9eaa35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f9cefa01832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f9cefa9915b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f9cf0e30a70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f9cf0f1e5b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f9cef8dfa8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f9cef8a2909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f9cef878a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f9cef875b69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f9cef8ddeee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f9cef8787aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f9ceee517a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f9ce91c0609 - start_thread
  34:     0x7f9ceecb7293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#1}`
#1 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0499`.


------------------------------------------


---- [ui] ui/issues/issue-40510-1.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f94c9e97e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f94c9e97e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f94c9f0962f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f94c9e85411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f94c9e9c39e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f94c9e9bf69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f94ca8b98d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f94c9e9cbc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f94c9e9c9f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f94c9e98394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f94c9e9c6d9 - rust_begin_unwind
  10:     0x7f94c9e50803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f94c9f059e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f94c9f0598b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f94c9e50676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f94cb7a0dc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f94cba4feaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f94cb918de0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f94cba438fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f94cba12e7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f94cbe5a8b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f94cbf16266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f94caa45a35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f94caa5c832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f94caaf415b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f94cbe8ba70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f94cbf795b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f94ca93aa8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f94ca8fd909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f94ca8d3a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f94ca8d0b69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f94ca938eee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f94ca8d37aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f94c9eac7a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f94c421b609 - start_thread
  34:     0x7f94c9d12293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `f::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/issues/issue-40510-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-40510-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-40510-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f92c9a85e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f92c9a85e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f92c9af762f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f92c9a73411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f92c9a8a39e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f92c9a89f69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f92ca4a78d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f92c9a8abc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f92c9a8a9f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f92c9a86394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f92c9a8a6d9 - rust_begin_unwind
  10:     0x7f92c9a3e803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f92c9af39e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f92c9af398b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f92c9a3e676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f92cb38edc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f92cb63deaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f92cb506de0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f92cb6318fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f92cb600e7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f92cba488b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f92cbb04266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f92ca633a35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f92ca64a832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f92ca6e215b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f92cba79a70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f92cbb675b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f92ca528a8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f92ca4eb909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f92ca4c1a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f92ca4beb69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f92ca526eee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f92ca4c17aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f92c9a9a7a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f92c3e09609 - start_thread
  34:     0x7f92c9900293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `f::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/issues/issue-49824.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49824.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49824/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7fd07638fe9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7fd07638fe9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7fd07640162f - core::fmt::write::h07c15968a6ea330b
   2:     0x7fd07637d411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7fd07639439e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7fd076393f69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7fd076db18d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd076394bc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7fd0763949f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7fd076390394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7fd0763946d9 - rust_begin_unwind
  10:     0x7fd076348803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7fd0763fd9e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7fd0763fd98b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7fd076348676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7fd077c98dc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7fd077f47eaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7fd077e10de0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7fd077f3b8fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7fd077f0ae7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7fd0783528b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7fd07840e266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7fd076f3da35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7fd076f54832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7fd076fec15b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7fd078383a70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7fd0784715b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7fd076e32a8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7fd076df5909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7fd076dcba82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7fd076dc8b69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7fd076e30eee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7fd076dcb7aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7fd0763a47a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7fd070713609 - start_thread
  34:     0x7fd07620a293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/nll/issue-53040.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-53040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-53040/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f4cd8c04e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f4cd8c04e9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f4cd8c7662f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f4cd8bf2411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f4cd8c0939e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f4cd8c08f69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f4cd96268d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4cd8c09bc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f4cd8c099f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f4cd8c05394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f4cd8c096d9 - rust_begin_unwind
  10:     0x7f4cd8bbd803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f4cd8c729e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f4cd8c7298b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f4cd8bbd676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f4cda50ddc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f4cda7bceaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f4cda685de0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f4cda7b08fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f4cda77fe7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f4cdabc78b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f4cdac83266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f4cd97b2a35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f4cd97c9832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f4cd986115b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f4cdabf8a70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f4cdace65b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f4cd96a7a8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f4cd966a909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f4cd9640a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f4cd963db69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f4cd96a5eee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f4cd96407aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f4cd8c197a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f4cd2f88609 - start_thread
  34:     0x7f4cd8a7f293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------



---- [ui] ui/regions/regions-return-ref-to-upvar-issue-17403.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-return-ref-to-upvar-issue-17403.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-return-ref-to-upvar-issue-17403" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-return-ref-to-upvar-issue-17403/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:433:30
   0:     0x7f02b3aabe9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   0:     0x7f02b3aabe9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he8b2125ae7bf60d0
   1:     0x7f02b3b1d62f - core::fmt::write::h07c15968a6ea330b
   2:     0x7f02b3a99411 - std::io::Write::write_fmt::ha330453cc03de01b
   3:     0x7f02b3ab039e - std::panicking::default_hook::{{closure}}::h1a3f0d5a22d7a512
   4:     0x7f02b3aaff69 - std::panicking::default_hook::h599b04a528b08f1f
   5:     0x7f02b44cd8d1 - rustc_driver[742a8fe1d20aca0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f02b3ab0bc8 - std::panicking::rust_panic_with_hook::h0811277ab9a07341
   7:     0x7f02b3ab09f7 - std::panicking::begin_panic_handler::{{closure}}::h9b85d3b3685e95e3
   8:     0x7f02b3aac394 - std::sys_common::backtrace::__rust_end_short_backtrace::hbb881faa8a257a8b
   9:     0x7f02b3ab06d9 - rust_begin_unwind
  10:     0x7f02b3a64803 - core::panicking::panic_fmt::h407e0de12d0e4032
  11:     0x7f02b3b199e1 - core::panicking::panic_display::h0d436a93cdbb80dd
  12:     0x7f02b3b1998b - core::panicking::panic_str::hbc6ee64467dd8988
  13:     0x7f02b3a64676 - core::option::expect_failed::he5dda5766feb0182
  14:     0x7f02b53b4dc0 - <indexmap[fdc7bf74b9137337]::map::IndexMap<rustc_hir[42b7fec0ec8e4923]::hir_id::HirId, rustc_hir[42b7fec0ec8e4923]::hir::Upvar, core[2cf3e031220c1576]::hash::BuildHasherDefault<rustc_hash[37347b30f26493bf]::FxHasher>> as core[2cf3e031220c1576]::ops::index::Index<&rustc_hir[42b7fec0ec8e4923]::hir_id::HirId>>::index
  15:     0x7f02b5663eaf - rustc_borrowck[39cc14cc8cce7c85]::do_mir_borrowck
  16:     0x7f02b552cde0 - <rustc_infer[d3517570c72fc721]::infer::InferCtxtBuilder>::enter::<rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult, rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck::{closure#0}>
  17:     0x7f02b56578fe - rustc_borrowck[39cc14cc8cce7c85]::mir_borrowck
  18:     0x7f02b5626e7b - <rustc_borrowck[39cc14cc8cce7c85]::provide::{closure#0} as core[2cf3e031220c1576]::ops::function::FnOnce<(rustc_middle[d91afcfab9fb202]::ty::context::TyCtxt, rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId)>>::call_once
  19:     0x7f02b5a6e8b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<rustc_span[bf30d311f88b7c2c]::def_id::LocalDefId, &rustc_middle[d91afcfab9fb202]::mir::query::BorrowCheckResult>>
  20:     0x7f02b5b2a266 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::mir_borrowck, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  21:     0x7f02b4659a35 - <rustc_middle[d91afcfab9fb202]::hir::map::Map>::par_body_owners::<rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f02b4670832 - <rustc_session[3fd3e06f6bc47e32]::session::Session>::time::<(), rustc_interface[de7c11bd04a1d656]::passes::analysis::{closure#2}>
  23:     0x7f02b470815b - rustc_interface[de7c11bd04a1d656]::passes::analysis
  24:     0x7f02b5a9fa70 - rustc_query_system[78e2d94aafa69350]::query::plumbing::try_execute_query::<rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt, rustc_query_system[78e2d94aafa69350]::query::caches::DefaultCache<(), core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>>
  25:     0x7f02b5b8d5b0 - rustc_query_system[78e2d94aafa69350]::query::plumbing::get_query::<rustc_query_impl[540fbd8cfc915b62]::queries::analysis, rustc_query_impl[540fbd8cfc915b62]::plumbing::QueryCtxt>
  26:     0x7f02b454ea8c - <rustc_interface[de7c11bd04a1d656]::passes::QueryContext>::enter::<rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  27:     0x7f02b4511909 - rustc_interface[de7c11bd04a1d656]::interface::create_compiler_and_run::<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>
  28:     0x7f02b44e7a82 - <scoped_tls[5eeab4efab77b3bd]::ScopedKey<rustc_span[bf30d311f88b7c2c]::SessionGlobals>>::set::<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  29:     0x7f02b44e4b69 - std[ae8c75c6161abcf0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  30:     0x7f02b454ceee - std[ae8c75c6161abcf0]::panic::catch_unwind::<core[2cf3e031220c1576]::panic::unwind_safe::AssertUnwindSafe<<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1}::{closure#0}>, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>
  31:     0x7f02b44e77aa - <<std[ae8c75c6161abcf0]::thread::Builder>::spawn_unchecked_<rustc_interface[de7c11bd04a1d656]::util::run_in_thread_pool_with_globals<rustc_interface[de7c11bd04a1d656]::interface::run_compiler<core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>, rustc_driver[742a8fe1d20aca0]::run_compiler::{closure#1}>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#0}, core[2cf3e031220c1576]::result::Result<(), rustc_errors[25abde8b37595d65]::ErrorReported>>::{closure#1} as core[2cf3e031220c1576]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f02b3ac07a3 - std::sys::unix::thread::Thread::new::thread_start::h440239320b8f9c7e
  33:     0x7f02ade2f609 - start_thread
  34:     0x7f02b3926293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (8b7fe4553 2022-02-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------

