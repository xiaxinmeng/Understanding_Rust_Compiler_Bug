plain
.......................................................................i............................ 5600/12638
.................................................................................................... 5700/12638
.................................................................................................... 5800/12638
.................................................................................................... 5900/12638
...........................................................F..F..................................... 6000/12638
..................i...................F............................................................. 6200/12638
...................................................................................................i 6300/12638
.................................................................................................... 6400/12638
................................................................i................................... 6500/12638
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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7feffbd6ee3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7feffbde05cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7feffbd5c371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7feffbd7333e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7feffbd72f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7feffc7924a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7feffbd73b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7feffbd73997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7feffbd6f334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7feffbd73679 - rust_begin_unwind
  10:     0x7feffbd27803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7feffbddc981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7feffbddc92b - core::panicking::panic_str::h8a3c23d2bf3bf629
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  13:     0x7feffbd27676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7feffd67fef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7feffd927436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7feffd7ec390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7feffd91ae4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7feffd8ea4ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7feffdd2fef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7feffddeb896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7feffc91e825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7feffc9351c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7feffc9cce8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7feffdd610b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7feffde4ebb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7feffc81387c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7feffc7d65c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7feffc7ac652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7feffc7a9739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7feffc811cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7feffc7ac37a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7feffbd83743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7feff60f2609 - start_thread
  34:     0x7feffbbe9293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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


thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7fcd745d3e3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7fcd746455cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7fcd745c1371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7fcd745d833e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7fcd745d7f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7fcd74ff74a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fcd745d8b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7fcd745d8997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7fcd745d4334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7fcd745d8679 - rust_begin_unwind
  10:     0x7fcd7458c803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7fcd74641981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7fcd7464192b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7fcd7458c676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7fcd75ee4ef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7fcd7618c436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7fcd76051390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7fcd7617fe4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7fcd7614f4ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7fcd76594ef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7fcd76650896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7fcd75183825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7fcd7519a1c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7fcd75231e8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7fcd765c60b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7fcd766b3bb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7fcd7507887c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7fcd7503b5c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7fcd75011652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7fcd7500e739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7fcd75076cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7fcd7501137a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7fcd745e8743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7fcd6e957609 - start_thread
  34:     0x7fcd7444e293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7f9b8b692e3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7f9b8b7045cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7f9b8b680371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7f9b8b69733e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7f9b8b696f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7f9b8c0b64a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9b8b697b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7f9b8b697997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7f9b8b693334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7f9b8b697679 - rust_begin_unwind
  10:     0x7f9b8b64b803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7f9b8b700981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7f9b8b70092b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7f9b8b64b676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7f9b8cfa3ef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7f9b8d24b436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7f9b8d110390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7f9b8d23ee4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7f9b8d20e4ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7f9b8d653ef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7f9b8d70f896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7f9b8c242825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f9b8c2591c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7f9b8c2f0e8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7f9b8d6850b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7f9b8d772bb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7f9b8c13787c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7f9b8c0fa5c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7f9b8c0d0652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7f9b8c0cd739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7f9b8c135cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7f9b8c0d037a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f9b8b6a7743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7f9b85a16609 - start_thread
  34:     0x7f9b8b50d293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7f337a0b6e3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7f337a1285cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7f337a0a4371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7f337a0bb33e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7f337a0baf09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7f337aada4a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f337a0bbb68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7f337a0bb997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7f337a0b7334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7f337a0bb679 - rust_begin_unwind
  10:     0x7f337a06f803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7f337a124981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7f337a12492b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7f337a06f676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7f337b9c7ef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7f337bc6f436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7f337bb34390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7f337bc62e4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7f337bc324ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7f337c077ef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7f337c133896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7f337ac66825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f337ac7d1c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7f337ad14e8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7f337c0a90b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7f337c196bb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7f337ab5b87c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7f337ab1e5c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7f337aaf4652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7f337aaf1739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7f337ab59cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7f337aaf437a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f337a0cb743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7f337443a609 - start_thread
  34:     0x7f3379f31293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7f930c9ece3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7f930ca5e5cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7f930c9da371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7f930c9f133e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7f930c9f0f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7f930d4104a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f930c9f1b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7f930c9f1997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7f930c9ed334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7f930c9f1679 - rust_begin_unwind
  10:     0x7f930c9a5803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7f930ca5a981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7f930ca5a92b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7f930c9a5676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7f930e2fdef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7f930e5a5436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7f930e46a390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7f930e598e4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7f930e5684ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7f930e9adef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7f930ea69896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7f930d59c825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f930d5b31c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7f930d64ae8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7f930e9df0b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7f930eaccbb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7f930d49187c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7f930d4545c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7f930d42a652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7f930d427739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7f930d48fcfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7f930d42a37a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f930ca01743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7f9306d70609 - start_thread
  34:     0x7f930c867293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7f6c01742e3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7f6c017b45cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7f6c01730371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7f6c0174733e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7f6c01746f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7f6c021664a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6c01747b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7f6c01747997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7f6c01743334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7f6c01747679 - rust_begin_unwind
  10:     0x7f6c016fb803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7f6c017b0981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7f6c017b092b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7f6c016fb676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7f6c03053ef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7f6c032fb436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7f6c031c0390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7f6c032eee4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7f6c032be4ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7f6c03703ef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7f6c037bf896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7f6c022f2825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f6c023091c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7f6c023a0e8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7f6c037350b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7f6c03822bb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7f6c021e787c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7f6c021aa5c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7f6c02180652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7f6c0217d739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7f6c021e5cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7f6c0218037a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f6c01757743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7f6bfbac6609 - start_thread
  34:     0x7f6c015bd293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

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
thread 'rustc' panicked at 'IndexMap: key not found', compiler/rustc_borrowck/src/diagnostics/region_errors.rs:442:30
stack backtrace:
   0:     0x7f1476ecfe3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c17f1cc6053c525
   1:     0x7f1476f415cf - core::fmt::write::hd3ab39fb91479af2
   2:     0x7f1476ebd371 - std::io::Write::write_fmt::hcc57d992e00ce546
   3:     0x7f1476ed433e - std::panicking::default_hook::{{closure}}::hf981df5f5577bc11
   4:     0x7f1476ed3f09 - std::panicking::default_hook::h5972e2c0a0ec90f9
   5:     0x7f14778f34a1 - rustc_driver[5e16ef1c10d3bac9]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1476ed4b68 - std::panicking::rust_panic_with_hook::h54e5c415acaceacc
   7:     0x7f1476ed4997 - std::panicking::begin_panic_handler::{{closure}}::hf89676e04a82cdfd
   8:     0x7f1476ed0334 - std::sys_common::backtrace::__rust_end_short_backtrace::h16a4096dd9aa561b
   9:     0x7f1476ed4679 - rust_begin_unwind
  10:     0x7f1476e88803 - core::panicking::panic_fmt::ha604282588be48fa
  11:     0x7f1476f3d981 - core::panicking::panic_display::h862c43c561c19246
  12:     0x7f1476f3d92b - core::panicking::panic_str::h8a3c23d2bf3bf629
  13:     0x7f1476e88676 - core::option::expect_failed::hce94c13bcef642bd
  14:     0x7f14787e0ef0 - <indexmap[1014df49d952025b]::map::IndexMap<rustc_hir[a05b78fbb1563a8d]::hir_id::HirId, rustc_hir[a05b78fbb1563a8d]::hir::Upvar, core[b310549c7c376a3e]::hash::BuildHasherDefault<rustc_hash[4769e52227c8f1d1]::FxHasher>> as core[b310549c7c376a3e]::ops::index::Index<&rustc_hir[a05b78fbb1563a8d]::hir_id::HirId>>::index
  15:     0x7f1478a88436 - rustc_borrowck[ac866137a783a174]::do_mir_borrowck
  16:     0x7f147894d390 - <rustc_infer[11b9c62c3f6f34e8]::infer::InferCtxtBuilder>::enter::<rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult, rustc_borrowck[ac866137a783a174]::mir_borrowck::{closure#0}>
  17:     0x7f1478a7be4e - rustc_borrowck[ac866137a783a174]::mir_borrowck
  18:     0x7f1478a4b4ab - <rustc_borrowck[ac866137a783a174]::provide::{closure#0} as core[b310549c7c376a3e]::ops::function::FnOnce<(rustc_middle[99e17d9443b26ab6]::ty::context::TyCtxt, rustc_span[8c71255dc5a16d86]::def_id::LocalDefId)>>::call_once
  19:     0x7f1478e90ef0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<rustc_span[8c71255dc5a16d86]::def_id::LocalDefId, &rustc_middle[99e17d9443b26ab6]::mir::query::BorrowCheckResult>>
  20:     0x7f1478f4c896 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::mir_borrowck, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  21:     0x7f1477a7f825 - <rustc_middle[99e17d9443b26ab6]::hir::map::Map>::par_body_owners::<rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}::{closure#0}>
  22:     0x7f1477a961c2 - <rustc_session[467d65793c29b8b8]::session::Session>::time::<(), rustc_interface[3b243cab395f776e]::passes::analysis::{closure#2}>
  23:     0x7f1477b2de8b - rustc_interface[3b243cab395f776e]::passes::analysis
  24:     0x7f1478ec20b0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::try_execute_query::<rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt, rustc_query_system[7fa01d563cd9220c]::query::caches::DefaultCache<(), core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>>
  25:     0x7f1478fafbb0 - rustc_query_system[7fa01d563cd9220c]::query::plumbing::get_query::<rustc_query_impl[5489e0384d8be5b0]::queries::analysis, rustc_query_impl[5489e0384d8be5b0]::plumbing::QueryCtxt>
  26:     0x7f147797487c - <rustc_interface[3b243cab395f776e]::passes::QueryContext>::enter::<rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  27:     0x7f14779375c9 - rustc_interface[3b243cab395f776e]::interface::create_compiler_and_run::<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>
  28:     0x7f147790d652 - <scoped_tls[4d539cb60d44dc0c]::ScopedKey<rustc_span[8c71255dc5a16d86]::SessionGlobals>>::set::<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  29:     0x7f147790a739 - std[e9f17b1c980415f4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  30:     0x7f1477972cfe - std[e9f17b1c980415f4]::panic::catch_unwind::<core[b310549c7c376a3e]::panic::unwind_safe::AssertUnwindSafe<<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1}::{closure#0}>, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>
  31:     0x7f147790d37a - <<std[e9f17b1c980415f4]::thread::Builder>::spawn_unchecked_<rustc_interface[3b243cab395f776e]::util::run_in_thread_pool_with_globals<rustc_interface[3b243cab395f776e]::interface::run_compiler<core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>, rustc_driver[5e16ef1c10d3bac9]::run_compiler::{closure#1}>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#0}, core[b310549c7c376a3e]::result::Result<(), rustc_errors[822cdcd7dfa46057]::ErrorReported>>::{closure#1} as core[b310549c7c376a3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f1476ee4743 - std::sys::unix::thread::Thread::new::thread_start::hb4d8c46462e6cabc
  33:     0x7f1471253609 - start_thread
  34:     0x7f1476d4a293 - clone
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (74e45fcfe 2022-02-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{closure#0}`
#1 [analysis] running analysis passes on this crate

------------------------------------------

