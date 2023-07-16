plain
---- [mir-opt] src/test/mir-opt/const_goto.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/const_goto.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_goto" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_goto" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_goto/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7ff57217abbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   0:     0x7ff57217abbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7ff5721e0588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7ff57216bf51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7ff57217db8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7ff57217d857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7ff573194774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff57217e331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7ff57217e119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7ff57217b164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7ff57217de32 - rust_begin_unwind
  10:     0x7ff572130da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7ff572130c6d - core::panicking::panic::h15c971491c034559
  12:     0x7ff579b0a87c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7ff57951b389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7ff57951bac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7ff57951cf8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7ff576bd5539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7ff576bd85c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7ff576bc5f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7ff5784d187e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7ff578620c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7ff5780c040a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7ff576bc6b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7ff5785c7b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7ff5780c2aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7ff577aacb91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7ff577a71644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7ff5784e5cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7ff5785c720a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7ff5780d91e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7ff57330c666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  31:     0x7ff5733227db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7ff57335b94d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  33:     0x7ff573359cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  34:     0x7ff578528585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  35:     0x7ff57861e827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  36:     0x7ff5780ba78e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  37:     0x7ff573233132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  38:     0x7ff5731965ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  39:     0x7ff57317fc00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7ff573199583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  41:     0x7ff573175a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7ff5731e3c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7ff57323b649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7ff572189e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  45:     0x7ff571eaa609 - start_thread
  46:     0x7ff571fec133 - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_goto -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `issue_77355_opt`
#1 [mir_promoted] processing `issue_77355_opt`
#2 [mir_borrowck] borrow-checking `issue_77355_opt`
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/combine_array_len.rs stdout ----
---- [mir-opt] src/test/mir-opt/combine_array_len.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/combine_array_len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
stack backtrace:
   0:     0x7f8160952bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7f81609b8588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7f8160943f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7f8160955b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7f8160955857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7f816196c774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8160956331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7f8160956119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7f8160953164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7f8160955e32 - rust_begin_unwind
  10:     0x7f8160908da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7f8160908c6d - core::panicking::panic::h15c971491c034559
  12:     0x7f81682e287c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7f8167cf3389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7f8167cf3ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7f8167cf4f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7f81653ad539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7f81653b05c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7f816539df99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7f8166ca987e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7f8166df8c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7f816689840a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7f816539eb24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7f8166d9fb16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7f816689aaca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7f8166284b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7f8166249644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7f8166cbdcc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7f8166d9f20a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7f81668b11e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7f8161ae4666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  31:     0x7f8161afa7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f8161b3394d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  33:     0x7f8161b31cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  34:     0x7f8166d00585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  35:     0x7f8166df6827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  36:     0x7f816689278e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  37:     0x7f8161a0b132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  38:     0x7f816196e5ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  39:     0x7f8161957c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f8161971583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  41:     0x7f816194da62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7f81619bbc0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7f8161a13649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f8160961e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  45:     0x7f8160682609 - start_thread
  46:     0x7f81607c4133 - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `main`
#1 [mir_promoted] processing `main`
#2 [mir_borrowck] borrow-checking `main`
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/const_allocation.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_allocation.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/const_allocation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7f05444b7bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7f054451d588 - core::fmt::write::hb8853908ec2f1387
   1:     0x7f054451d588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7f05444a8f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7f05444bab8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7f05444ba857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7f05454d1774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f05444bb331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7f05444bb119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7f05444b8164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7f05444bae32 - rust_begin_unwind
  10:     0x7f054446dda3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7f054446dc6d - core::panicking::panic::h15c971491c034559
  12:     0x7f054be4787c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7f054b858389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7f054b858ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7f054b859f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7f0548f12539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7f0548f155c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7f0548f02f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7f054a80e87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7f054a95dc95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7f054a3fd40a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7f0548f0230e - rustc_mir_transform[7e8ebed866a1f72]::mir_const_qualif
  23:     0x7f0548ef786b - <rustc_mir_transform[7e8ebed866a1f72]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::DefId)>>::call_once
  24:     0x7f054a846300 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::DefId, rustc_middle[3db99ce9e3be536b]::mir::query::ConstQualifs>>
  25:     0x7f054a913580 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const_qualif, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  26:     0x7f054a3fc389 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const_qualif
  27:     0x7f0548f03969 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  28:     0x7f054a904b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7f054a3ffaca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  30:     0x7f0549de9b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  31:     0x7f0549dae644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  32:     0x7f054a822cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  33:     0x7f054a90420a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  34:     0x7f054a4161e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  35:     0x7f0545649666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  36:     0x7f054565f7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  37:     0x7f054569894d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  38:     0x7f0545696cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  39:     0x7f054a865585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  40:     0x7f054a95b827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  41:     0x7f054a3f778e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  42:     0x7f0545570132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7f05454d35ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  44:     0x7f05454bcc00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f05454d6583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  46:     0x7f05454b2a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  47:     0x7f0545520c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  48:     0x7f0545578649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f05444c6e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  50:     0x7f05441e7609 - start_thread
  51:     0x7f0544329133 - clone
  52:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `FOO`
#1 [mir_const_qualif] const checking `FOO`
#2 [mir_promoted] processing `FOO`
#3 [mir_borrowck] borrow-checking `FOO`
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/box_expr.rs stdout ----
---- [mir-opt] src/test/mir-opt/box_expr.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/box_expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
stack backtrace:
   0:     0x7feeef568bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7feeef5ce588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7feeef559f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7feeef56bb8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7feeef56b857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7feef0582774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7feeef56c331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7feeef56c119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7feeef569164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7feeef56be32 - rust_begin_unwind
  10:     0x7feeef51eda3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7feeef51ec6d - core::panicking::panic::h15c971491c034559
  12:     0x7feef6ef887c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7feef6909389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7feef6909ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7feef690af8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7feef3fc3539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7feef3fc65c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7feef3fb3f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7feef58bf87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7feef5a0ec95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7feef54ae40a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7feef3fb4b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7feef59b5b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7feef54b0aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7feef4e9ab91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7feef4e5f644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7feef58d3cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7feef59b520a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7feef54c71e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7feef06fa666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  31:     0x7feef07107db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7feef074994d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  33:     0x7feef0747cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  34:     0x7feef5916585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  35:     0x7feef5a0c827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  36:     0x7feef54a878e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  37:     0x7feef0621132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  38:     0x7feef05845ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  39:     0x7feef056dc00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7feef0587583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  41:     0x7feef0563a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7feef05d1c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7feef0629649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7feeef577e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  45:     0x7feeef298609 - start_thread
  46:     0x7feeef3da133 - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `<impl at /checkout/src/test/mir-opt/box_expr.rs:17:1: 17:16>::drop`
#1 [mir_promoted] processing `<impl at /checkout/src/test/mir-opt/box_expr.rs:17:1: 17:16>::drop`
#2 [mir_borrowck] borrow-checking `<impl at /checkout/src/test/mir-opt/box_expr.rs:17:1: 17:16>::drop`
#3 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_allocation2.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/const_allocation2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
stack backtrace:
   0:     0x7fae03cfdbbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7fae03d63588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7fae03ceef51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7fae03d00b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7fae03d00857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7fae04d17774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fae03d01331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7fae03d01119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7fae03cfe164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7fae03d00e32 - rust_begin_unwind
  10:     0x7fae03cb3da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7fae03cb3c6d - core::panicking::panic::h15c971491c034559
  12:     0x7fae0b68d87c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7fae0b09e389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7fae0b09eac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7fae0b09ff8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7fae08758539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7fae0875b5c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7fae08748f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7fae0a05487e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7fae0a1a3c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7fae09c4340a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7fae0874830e - rustc_mir_transform[7e8ebed866a1f72]::mir_const_qualif
  23:     0x7fae0873d86b - <rustc_mir_transform[7e8ebed866a1f72]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::DefId)>>::call_once
  24:     0x7fae0a08c300 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::DefId, rustc_middle[3db99ce9e3be536b]::mir::query::ConstQualifs>>
  25:     0x7fae0a159580 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const_qualif, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  26:     0x7fae09c42389 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const_qualif
  27:     0x7fae08749969 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  28:     0x7fae0a14ab16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7fae09c45aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  30:     0x7fae0962fb91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  31:     0x7fae095f4644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  32:     0x7fae0a068cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  33:     0x7fae0a14a20a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  34:     0x7fae09c5c1e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  35:     0x7fae0b195ac1 - <rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt>::mir_borrowck_opt_const_arg
  36:     0x7fae0874acf6 - rustc_mir_transform[7e8ebed866a1f72]::mir_drops_elaborated_and_const_checked
  37:     0x7fae0a05487e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  38:     0x7fae0a19bf95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  39:     0x7fae09c44a5a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  40:     0x7fae0874a8ef - rustc_mir_transform[7e8ebed866a1f72]::inner_mir_for_ctfe
  41:     0x7fae0874a366 - rustc_mir_transform[7e8ebed866a1f72]::mir_for_ctfe
  42:     0x7fae0a09044e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::DefId, &rustc_middle[3db99ce9e3be536b]::mir::Body>>
  43:     0x7fae0a14a374 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_for_ctfe, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  44:     0x7fae09c44fc9 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_for_ctfe
  45:     0x7fae09a60319 - <rustc_const_eval[3d4e9bcedd370475]::const_eval::machine::CompileTimeInterpreter as rustc_const_eval[3d4e9bcedd370475]::interpret::machine::Machine>::load_mir
  46:     0x7fae099af8f6 - <rustc_const_eval[3d4e9bcedd370475]::interpret::eval_context::InterpCx<rustc_const_eval[3d4e9bcedd370475]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  47:     0x7fae09aee976 - rustc_const_eval[3d4e9bcedd370475]::const_eval::eval_queries::eval_to_allocation_raw_provider
  48:     0x7fae0a173afb - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::eval_to_allocation_raw, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  49:     0x7fae09c5e81c - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::eval_to_allocation_raw
  50:     0x7fae09aee3b1 - rustc_const_eval[3d4e9bcedd370475]::const_eval::eval_queries::eval_to_allocation_raw_provider
  51:     0x7fae0a173afb - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::eval_to_allocation_raw, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  52:     0x7fae09c5e81c - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::eval_to_allocation_raw
  53:     0x7fae0b1db40d - <rustc_middle[3db99ce9e3be536b]::ty::query::TyCtxtAt>::eval_static_initializer
  54:     0x7fae0b0a3a05 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_allocations
  55:     0x7fae08759442 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  56:     0x7fae0875b5c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  57:     0x7fae08748f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  58:     0x7fae0a05487e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  59:     0x7fae0a1a3c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  60:     0x7fae09c4340a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  61:     0x7fae08749b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  62:     0x7fae0a14ab16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  63:     0x7fae09c45aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  64:     0x7fae0962fb91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  65:     0x7fae095f4644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  66:     0x7fae0a068cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  67:     0x7fae0a14a20a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  68:     0x7fae09c5c1e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  69:     0x7fae04e8f666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  70:     0x7fae04ea57db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  71:     0x7fae04ede94d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  72:     0x7fae04edccb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  73:     0x7fae0a0ab585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  74:     0x7fae0a1a1827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  75:     0x7fae09c3d78e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  76:     0x7fae04db6132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  77:     0x7fae04d195ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  78:     0x7fae04d02c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  79:     0x7fae04d1c583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  80:     0x7fae04cf8a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  81:     0x7fae04d66c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  82:     0x7fae04dbe649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  83:     0x7fae03d0ce65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  84:     0x7fae03a2d609 - start_thread
  85:     0x7fae03b6f133 - clone
  86:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `FOO`
#1 [mir_const_qualif] const checking `FOO`
#2 [mir_promoted] processing `FOO`
#3 [mir_borrowck] borrow-checking `FOO`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `FOO`
#5 [mir_for_ctfe] caching mir of `FOO` for CTFE
#6 [eval_to_allocation_raw] const-evaluating + checking `FOO`
#7 [eval_to_allocation_raw] const-evaluating + checking `FOO`
#8 [mir_const] processing MIR for `main`
#9 [mir_promoted] processing `main`
#10 [mir_borrowck] borrow-checking `main`
#11 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
stack backtrace:
   0:     0x7fae03cfdbbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7fae03d63588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7fae03ceef51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7fae03d00b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7fae03d00857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7fae04d17774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fae03d01331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7fae03d01119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7fae03cfe164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7fae03d00e32 - rust_begin_unwind
  10:     0x7fae03cb3da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7fae03cb3c6d - core::panicking::panic::h15c971491c034559
  12:     0x7fae0b68d87c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7fae0b09e389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7fae087588cb - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  15:     0x7fae0875b5c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  16:     0x7fae08748f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  17:     0x7fae0a05487e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  18:     0x7fae0a1a3c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  19:     0x7fae09c4340a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  20:     0x7fae0874830e - rustc_mir_transform[7e8ebed866a1f72]::mir_const_qualif
  21:     0x7fae0873d86b - <rustc_mir_transform[7e8ebed866a1f72]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::DefId)>>::call_once
  22:     0x7fae0a08c300 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::DefId, rustc_middle[3db99ce9e3be536b]::mir::query::ConstQualifs>>
  23:     0x7fae0a159580 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const_qualif, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7fae09c42389 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const_qualif
  25:     0x7fae08749969 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  26:     0x7fae0a14ab16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  27:     0x7fae09c45aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  28:     0x7fae0962fb91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  29:     0x7fae095f4644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  30:     0x7fae0a068cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  31:     0x7fae0a14a20a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  32:     0x7fae09c5c1e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  33:     0x7fae04e8f666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  34:     0x7fae04ea57db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  35:     0x7fae04ede94d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  36:     0x7fae04edccb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  37:     0x7fae0a0ab585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  38:     0x7fae0a1a1827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  39:     0x7fae09c3d78e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  40:     0x7fae04db6132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  41:     0x7fae04d195ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7fae04d02c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7fae04d1c583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  44:     0x7fae04cf8a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  45:     0x7fae04d66c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  46:     0x7fae04dbe649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7fae03d0ce65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  48:     0x7fae03a2d609 - start_thread
  49:     0x7fae03b6f133 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
---
   |
50 |     upper: bool,
   |     ^^^^^ help: if this is intentional, prefix it with an underscore: `_upper`

thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7f2e25147bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7f2e251ad588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7f2e25138f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   2:     0x7f2e25138f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7f2e2514ab8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7f2e2514a857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7f2e26161774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2e2514b331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7f2e2514b119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7f2e25148164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7f2e2514ae32 - rust_begin_unwind
  10:     0x7f2e250fdda3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7f2e250fdc6d - core::panicking::panic::h15c971491c034559
  12:     0x7f2e2cad787c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7f2e2c4e8389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7f2e2c4e8ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7f2e2c4e9f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7f2e29ba2539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7f2e29ba55c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7f2e29b92f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7f2e2b49e87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7f2e2b5edc95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7f2e2b08d40a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7f2e29b93b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7f2e2b594b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7f2e2b08faca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7f2e2aa79b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7f2e2aa3e644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7f2e2b4b2cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7f2e2b59420a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7f2e2b0a61e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7f2e262d9666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  31:     0x7f2e262ef7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f2e2632894d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  33:     0x7f2e26326cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  34:     0x7f2e2b4f5585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  35:     0x7f2e2b5eb827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  36:     0x7f2e2b08778e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  37:     0x7f2e26200132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  38:     0x7f2e261635ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  39:     0x7f2e2614cc00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f2e26166583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  41:     0x7f2e26142a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7f2e261b0c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7f2e26208649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f2e25156e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  45:     0x7f2e24e77609 - start_thread
  46:     0x7f2e24fb9133 - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/funky_arms -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -C debug-assertions=no
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `float_to_exponential_common_exact`
#1 [mir_promoted] processing `float_to_exponential_common_exact`
#2 [mir_borrowck] borrow-checking `float_to_exponential_common_exact`
#3 [analysis] running analysis passes on this crate
end of query stack
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7f2e25147bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7f2e251ad588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7f2e25138f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   2:     0x7f2e25138f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7f2e2514ab8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7f2e2514a857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7f2e26161774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2e2514b331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7f2e2514b119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7f2e25148164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7f2e2514ae32 - rust_begin_unwind
  10:     0x7f2e250fdda3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7f2e250fdc6d - core::panicking::panic::h15c971491c034559
  12:     0x7f2e2cad787c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7f2e2c4e8389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7f2e2c4e8ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7f2e2c4e9f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7f2e29ba2539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7f2e29ba55c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7f2e29b92f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7f2e2b49e87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7f2e2b5edc95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7f2e2b08d40a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7f2e29b93b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7f2e2b594b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7f2e2b08faca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7f2e2aa79b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7f2e2aa3e644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7f2e2b4b2cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7f2e2b59420a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7f2e2b0a61e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7f2e262d9666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  31:     0x7f2e262ef7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  32:     0x7f2e2632894d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  33:     0x7f2e26326cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  34:     0x7f2e2b4f5585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  35:     0x7f2e2b5eb827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  36:     0x7f2e2b08778e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  37:     0x7f2e26200132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  38:     0x7f2e261635ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  39:     0x7f2e2614cc00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f2e26166583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  41:     0x7f2e26142a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  42:     0x7f2e261b0c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  43:     0x7f2e26208649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f2e25156e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  45:     0x7f2e24e77609 - start_thread
  46:     0x7f2e24fb9133 - clone
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/funky_arms -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib -C debug-assertions=no
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `float_to_exponential_common_shortest`
#1 [mir_promoted] processing `float_to_exponential_common_shortest`
#2 [mir_borrowck] borrow-checking `float_to_exponential_common_shortest`
#3 [analysis] running analysis passes on this crate
warning: 9 warnings emitted
------------------------------------------



---- [mir-opt] src/test/mir-opt/generator-drop-cleanup.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/generator-drop-cleanup.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `gen`
   |
10 |     let gen = || {
   |         ^^^ help: if this is intentional, prefix it with an underscore: `_gen`
   |
   |
   = note: `#[warn(unused_variables)]` on by default

thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7fac8e3ebbbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   0:     0x7fac8e3ebbbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7fac8e451588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7fac8e3dcf51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7fac8e3eeb8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7fac8e3ee857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7fac8f405774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fac8e3ef331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7fac8e3ef119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7fac8e3ec164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7fac8e3eee32 - rust_begin_unwind
  10:     0x7fac8e3a1da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7fac8e3a1c6d - core::panicking::panic::h15c971491c034559
  12:     0x7fac95d7b87c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7fac9578c389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7fac9578cac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7fac9578df8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7fac92e46539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7fac92e495c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7fac92e36f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7fac9474287e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7fac94891c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7fac9433140a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7fac92e37b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7fac94838b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7fac94333aca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7fac93d1db91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7fac93ce2644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7fac94756cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7fac9483820a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7fac9434a1e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7fac93affd51 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::prove_closure_bounds
  31:     0x7fac93b15656 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::check_rvalue
  32:     0x7fac93b1b613 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::typeck_mir
  33:     0x7fac93b0abc1 - rustc_borrowck[e4e359a81edfbe98]::type_check::type_check_internal::<rustc_data_structures[d00a1437fc0a9f76]::vec_map::VecMap<rustc_middle[3db99ce9e3be536b]::ty::OpaqueTypeKey, (rustc_middle[3db99ce9e3be536b]::ty::OpaqueHiddenType, rustc_hir[62b9684b39d0a887]::hir::OpaqueTyOrigin)>, rustc_borrowck[e4e359a81edfbe98]::type_check::type_check::{closure#0}>
  34:     0x7fac93af46ae - rustc_borrowck[e4e359a81edfbe98]::nll::compute_regions
  35:     0x7fac93d27e98 - rustc_borrowck[e4e359a81edfbe98]::do_mir_borrowck
  36:     0x7fac93c7cf06 - <rustc_infer[67a33d8afc45227d]::infer::InferCtxtBuilder>::enter::<rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult, rustc_borrowck[e4e359a81edfbe98]::mir_borrowck::{closure#0}>
  37:     0x7fac93d1dc02 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  38:     0x7fac93ce2644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  39:     0x7fac94756cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  40:     0x7fac9483820a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  41:     0x7fac9434a1e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  42:     0x7fac8f57d666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  43:     0x7fac8f5937db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  44:     0x7fac8f5cc94d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  45:     0x7fac8f5cacb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  46:     0x7fac94799585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  47:     0x7fac9488f827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  48:     0x7fac9432b78e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  49:     0x7fac8f4a4132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  50:     0x7fac8f4075ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  51:     0x7fac8f3f0c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7fac8f40a583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  53:     0x7fac8f3e6a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  54:     0x7fac8f454c0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  55:     0x7fac8f4ac649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7fac8e3fae65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  57:     0x7fac8e11b609 - start_thread
  58:     0x7fac8e25d133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `main::{closure#0}`
#1 [mir_promoted] processing `main::{closure#0}`
#2 [mir_borrowck] borrow-checking `main::{closure#0}`
#3 [mir_borrowck] borrow-checking `main`
#4 [analysis] running analysis passes on this crate
warning: 1 warning emitted
------------------------------------------



---- [mir-opt] src/test/mir-opt/generator-tiny.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/generator-tiny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7fa2d2aa3bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   0:     0x7fa2d2aa3bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7fa2d2b09588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7fa2d2a94f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7fa2d2aa6b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7fa2d2aa6857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7fa2d3abd774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa2d2aa7331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7fa2d2aa7119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7fa2d2aa4164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7fa2d2aa6e32 - rust_begin_unwind
  10:     0x7fa2d2a59da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7fa2d2a59c6d - core::panicking::panic::h15c971491c034559
  12:     0x7fa2da43387c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7fa2d9e44389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7fa2d9e44ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7fa2d9e45f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7fa2d74fe539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7fa2d75015c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7fa2d74eef99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7fa2d8dfa87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7fa2d8f49c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7fa2d89e940a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7fa2d74efb24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7fa2d8ef0b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7fa2d89ebaca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7fa2d83d5b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7fa2d839a644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7fa2d8e0ecc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7fa2d8ef020a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7fa2d8a021e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7fa2d81b7d51 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::prove_closure_bounds
  31:     0x7fa2d81cd656 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::check_rvalue
  32:     0x7fa2d81d3613 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::typeck_mir
  33:     0x7fa2d81c2bc1 - rustc_borrowck[e4e359a81edfbe98]::type_check::type_check_internal::<rustc_data_structures[d00a1437fc0a9f76]::vec_map::VecMap<rustc_middle[3db99ce9e3be536b]::ty::OpaqueTypeKey, (rustc_middle[3db99ce9e3be536b]::ty::OpaqueHiddenType, rustc_hir[62b9684b39d0a887]::hir::OpaqueTyOrigin)>, rustc_borrowck[e4e359a81edfbe98]::type_check::type_check::{closure#0}>
  34:     0x7fa2d81ac6ae - rustc_borrowck[e4e359a81edfbe98]::nll::compute_regions
  35:     0x7fa2d83dfe98 - rustc_borrowck[e4e359a81edfbe98]::do_mir_borrowck
  36:     0x7fa2d8334f06 - <rustc_infer[67a33d8afc45227d]::infer::InferCtxtBuilder>::enter::<rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult, rustc_borrowck[e4e359a81edfbe98]::mir_borrowck::{closure#0}>
  37:     0x7fa2d83d5c02 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  38:     0x7fa2d839a644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  39:     0x7fa2d8e0ecc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  40:     0x7fa2d8ef020a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  41:     0x7fa2d8a021e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  42:     0x7fa2d3c35666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  43:     0x7fa2d3c4b7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  44:     0x7fa2d3c8494d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  45:     0x7fa2d3c82cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  46:     0x7fa2d8e51585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  47:     0x7fa2d8f47827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  48:     0x7fa2d89e378e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  49:     0x7fa2d3b5c132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  50:     0x7fa2d3abf5ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  51:     0x7fa2d3aa8c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7fa2d3ac2583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  53:     0x7fa2d3a9ea62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  54:     0x7fa2d3b0cc0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  55:     0x7fa2d3b64649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7fa2d2ab2e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  57:     0x7fa2d27d3609 - start_thread
  58:     0x7fa2d2915133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny -C rpath -C debuginfo=0 -C panic=abort
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `main::{closure#0}`
#1 [mir_promoted] processing `main::{closure#0}`
#2 [mir_borrowck] borrow-checking `main::{closure#0}`
#3 [mir_borrowck] borrow-checking `main`
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs stdout ----
---- [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/generator-storage-dead-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7f4bd8755bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   0:     0x7f4bd8755bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7f4bd87bb588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7f4bd8746f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7f4bd8758b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7f4bd8758857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7f4bd976f774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4bd8759331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7f4bd8759119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7f4bd8756164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7f4bd8758e32 - rust_begin_unwind
  10:     0x7f4bd870bda3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7f4bd870bc6d - core::panicking::panic::h15c971491c034559
  12:     0x7f4be00e587c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7f4bdfaf6389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7f4bdfaf6ac2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7f4bdfaf7f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7f4bdd1b0539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7f4bdd1b35c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7f4bdd1a0f99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7f4bdeaac87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7f4bdebfbc95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7f4bde69b40a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7f4bdd1a1b24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7f4bdeba2b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7f4bde69daca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7f4bde087b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7f4bde04c644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7f4bdeac0cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7f4bdeba220a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7f4bde6b41e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7f4bdde69d51 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::prove_closure_bounds
  31:     0x7f4bdde7f656 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::check_rvalue
  32:     0x7f4bdde85613 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::typeck_mir
  33:     0x7f4bdde74bc1 - rustc_borrowck[e4e359a81edfbe98]::type_check::type_check_internal::<rustc_data_structures[d00a1437fc0a9f76]::vec_map::VecMap<rustc_middle[3db99ce9e3be536b]::ty::OpaqueTypeKey, (rustc_middle[3db99ce9e3be536b]::ty::OpaqueHiddenType, rustc_hir[62b9684b39d0a887]::hir::OpaqueTyOrigin)>, rustc_borrowck[e4e359a81edfbe98]::type_check::type_check::{closure#0}>
  34:     0x7f4bdde5e6ae - rustc_borrowck[e4e359a81edfbe98]::nll::compute_regions
  35:     0x7f4bde091e98 - rustc_borrowck[e4e359a81edfbe98]::do_mir_borrowck
  36:     0x7f4bddfe6f06 - <rustc_infer[67a33d8afc45227d]::infer::InferCtxtBuilder>::enter::<rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult, rustc_borrowck[e4e359a81edfbe98]::mir_borrowck::{closure#0}>
  37:     0x7f4bde087c02 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  38:     0x7f4bde04c644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  39:     0x7f4bdeac0cc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  40:     0x7f4bdeba220a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  41:     0x7f4bde6b41e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  42:     0x7f4bd98e7666 - <core[d84a42c780dfa8d5]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once
  43:     0x7f4bd98fd7db - rustc_data_structures[d00a1437fc0a9f76]::sync::par_for_each_in::<&[rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId], <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>::{closure#0}>
  44:     0x7f4bd993694d - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::par_body_owners::<rustc_interface[1b372c8eff60934b]::passes::analysis::{closure#2}::{closure#0}>
  45:     0x7f4bd9934cb6 - rustc_interface[1b372c8eff60934b]::passes::analysis
  46:     0x7f4bdeb03585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  47:     0x7f4bdebf9827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  48:     0x7f4bde69578e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  49:     0x7f4bd980e132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  50:     0x7f4bd97715ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  51:     0x7f4bd975ac00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  52:     0x7f4bd9774583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  53:     0x7f4bd9750a62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  54:     0x7f4bd97bec0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  55:     0x7f4bd9816649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f4bd8764e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  57:     0x7f4bd8485609 - start_thread
  58:     0x7f4bd85c7133 - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (2cba43904 2022-07-28) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=4 -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_const] processing MIR for `main::{closure#0}`
#1 [mir_promoted] processing `main::{closure#0}`
#2 [mir_borrowck] borrow-checking `main::{closure#0}`
#3 [mir_borrowck] borrow-checking `main`
#4 [analysis] running analysis passes on this crate
------------------------------------------


---- [mir-opt] src/test/mir-opt/inline/inline-async.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-async.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/inline/inline-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=4" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-async" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-async" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-async/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: relative_to.contains(sp)', compiler/rustc_span/src/source_map.rs:472:9
   0:     0x7ff8dab23bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   0:     0x7ff8dab23bbd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8c29c1eddb901fb7
   1:     0x7ff8dab89588 - core::fmt::write::hb8853908ec2f1387
   2:     0x7ff8dab14f51 - std::io::Write::write_fmt::h4ecec8493ec1d9ff
   3:     0x7ff8dab26b8e - std::panicking::default_hook::{{closure}}::habb3655b87133066
   4:     0x7ff8dab26857 - std::panicking::default_hook::hd2a7053eba1cf01b
   5:     0x7ff8dbb3d774 - rustc_driver[f4604cd0fc6fcc44]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff8dab27331 - std::panicking::rust_panic_with_hook::ha6f507cd591f6d7d
   7:     0x7ff8dab27119 - std::panicking::begin_panic_handler::{{closure}}::hb57531832bd0af9a
   8:     0x7ff8dab24164 - std::sys_common::backtrace::__rust_end_short_backtrace::h7bd39425c8ebd7e2
   9:     0x7ff8dab26e32 - rust_begin_unwind
  10:     0x7ff8daad9da3 - core::panicking::panic_fmt::h72a29c74cf9eec6d
  11:     0x7ff8daad9c6d - core::panicking::panic::h15c971491c034559
  12:     0x7ff8e24b387c - <rustc_span[5dce3d22dbbc8b66]::source_map::SourceMap>::span_to_relative_line_string
  13:     0x7ff8e1ec4389 - rustc_middle[3db99ce9e3be536b]::mir::pretty::comment
  14:     0x7ff8e1ec464c - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_scope_tree
  15:     0x7ff8e1ec5f8a - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_intro
  16:     0x7ff8df57e539 - rustc_middle[3db99ce9e3be536b]::mir::pretty::write_mir_fn::<rustc_mir_transform[7e8ebed866a1f72]::mir_const::{closure#0}>
  17:     0x7ff8df5815c2 - rustc_middle[3db99ce9e3be536b]::mir::pretty::dump_matched_mir_node::<<rustc_mir_transform[7e8ebed866a1f72]::generator::StateTransform as rustc_middle[3db99ce9e3be536b]::mir::MirPass>::run_pass::{closure#1}>
  18:     0x7ff8df56ef99 - rustc_mir_transform[7e8ebed866a1f72]::mir_const
  19:     0x7ff8e0e7a87e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_middle[3db99ce9e3be536b]::ty::WithOptConstParam<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId>, &rustc_data_structures[d00a1437fc0a9f76]::steal::Steal<rustc_middle[3db99ce9e3be536b]::mir::Body>>>
  20:     0x7ff8e0fc9c95 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_const, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  21:     0x7ff8e0a6940a - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_const
  22:     0x7ff8df56fb24 - rustc_mir_transform[7e8ebed866a1f72]::mir_promoted
  23:     0x7ff8e0f70b16 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_promoted, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  24:     0x7ff8e0a6baca - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_promoted
  25:     0x7ff8e0455b91 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  26:     0x7ff8e041a644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  27:     0x7ff8e0e8ecc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  28:     0x7ff8e0f7020a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  29:     0x7ff8e0a821e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  30:     0x7ff8e0237d51 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::prove_closure_bounds
  31:     0x7ff8e024d656 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::check_rvalue
  32:     0x7ff8e0253613 - <rustc_borrowck[e4e359a81edfbe98]::type_check::TypeChecker>::typeck_mir
  33:     0x7ff8e0242bc1 - rustc_borrowck[e4e359a81edfbe98]::type_check::type_check_internal::<rustc_data_structures[d00a1437fc0a9f76]::vec_map::VecMap<rustc_middle[3db99ce9e3be536b]::ty::OpaqueTypeKey, (rustc_middle[3db99ce9e3be536b]::ty::OpaqueHiddenType, rustc_hir[62b9684b39d0a887]::hir::OpaqueTyOrigin)>, rustc_borrowck[e4e359a81edfbe98]::type_check::type_check::{closure#0}>
  34:     0x7ff8e022c6ae - rustc_borrowck[e4e359a81edfbe98]::nll::compute_regions
  35:     0x7ff8e045fe98 - rustc_borrowck[e4e359a81edfbe98]::do_mir_borrowck
  36:     0x7ff8e03b4f06 - <rustc_infer[67a33d8afc45227d]::infer::InferCtxtBuilder>::enter::<rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult, rustc_borrowck[e4e359a81edfbe98]::mir_borrowck::{closure#0}>
  37:     0x7ff8e0455c02 - rustc_borrowck[e4e359a81edfbe98]::mir_borrowck
  38:     0x7ff8e041a644 - <rustc_borrowck[e4e359a81edfbe98]::provide::{closure#0} as core[d84a42c780dfa8d5]::ops::function::FnOnce<(rustc_middle[3db99ce9e3be536b]::ty::context::TyCtxt, rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId)>>::call_once
  39:     0x7ff8e0e8ecc5 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, &rustc_middle[3db99ce9e3be536b]::mir::query::BorrowCheckResult>>
  40:     0x7ff8e0f7020a - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::mir_borrowck, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  41:     0x7ff8e0a821e4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::mir_borrowck
  42:     0x7ff8dfaa8eb7 - rustc_typeck[48f2af67685c318f]::collect::type_of::find_opaque_ty_constraints_for_rpit
  43:     0x7ff8dfaa7ce1 - rustc_typeck[48f2af67685c318f]::collect::type_of::type_of
  44:     0x7ff8e0ea818e - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::DefId, rustc_middle[3db99ce9e3be536b]::ty::Ty>>
  45:     0x7ff8e0fc7713 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::type_of, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  46:     0x7ff8e0a63229 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::type_of
  47:     0x7ff8dfbb7533 - rustc_typeck[48f2af67685c318f]::check::check::check_opaque
  48:     0x7ff8dfbbb4e0 - rustc_typeck[48f2af67685c318f]::check::check::check_item_type
  49:     0x7ff8dfbc5cde - rustc_typeck[48f2af67685c318f]::check::check::check_mod_item_types
  50:     0x7ff8e0e917f0 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<rustc_span[5dce3d22dbbc8b66]::def_id::LocalDefId, ()>>
  51:     0x7ff8e0f8f2f6 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::check_mod_item_types, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  52:     0x7ff8e0a7cba4 - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::check_mod_item_types
  53:     0x7ff8dfbd093a - <rustc_middle[3db99ce9e3be536b]::hir::map::Map>::for_each_module::<rustc_typeck[48f2af67685c318f]::check_crate::{closure#6}::{closure#0}>
  54:     0x7ff8dfcc04da - rustc_typeck[48f2af67685c318f]::check_crate
  55:     0x7ff8dbd02c16 - rustc_interface[1b372c8eff60934b]::passes::analysis
  56:     0x7ff8e0ed1585 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::try_execute_query::<rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt, rustc_query_system[997565aa72d6bfb3]::query::caches::DefaultCache<(), core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>>
  57:     0x7ff8e0fc7827 - rustc_query_system[997565aa72d6bfb3]::query::plumbing::get_query::<rustc_query_impl[3a76233a457f4b54]::queries::analysis, rustc_query_impl[3a76233a457f4b54]::plumbing::QueryCtxt>
  58:     0x7ff8e0a6378e - <rustc_query_impl[3a76233a457f4b54]::Queries as rustc_middle[3db99ce9e3be536b]::ty::query::QueryEngine>::analysis
  59:     0x7ff8dbbdc132 - <rustc_interface[1b372c8eff60934b]::passes::QueryContext>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  60:     0x7ff8dbb3f5ff - <rustc_interface[1b372c8eff60934b]::interface::Compiler>::enter::<rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}::{closure#2}, core[d84a42c780dfa8d5]::result::Result<core[d84a42c780dfa8d5]::option::Option<rustc_interface[1b372c8eff60934b]::queries::Linker>, rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  61:     0x7ff8dbb28c00 - rustc_span[5dce3d22dbbc8b66]::with_source_map::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#1}>
  62:     0x7ff8dbb42583 - rustc_interface[1b372c8eff60934b]::interface::create_compiler_and_run::<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>
  63:     0x7ff8dbb1ea62 - <scoped_tls[f27d19fb53a99375]::ScopedKey<rustc_span[5dce3d22dbbc8b66]::SessionGlobals>>::set::<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  64:     0x7ff8dbb8cc0f - std[c78b2397758a6c9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>
  65:     0x7ff8dbbe4649 - <<std[c78b2397758a6c9a]::thread::Builder>::spawn_unchecked_<rustc_interface[1b372c8eff60934b]::util::run_in_thread_pool_with_globals<rustc_interface[1b372c8eff60934b]::interface::run_compiler<core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>, rustc_driver[f4604cd0fc6fcc44]::run_compiler::{closure#1}>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#0}, core[d84a42c780dfa8d5]::result::Result<(), rustc_errors[ba781cc5ef901d6f]::ErrorGuaranteed>>::{closure#1} as core[d84a42c780dfa8d5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  66:     0x7ff8dab32e65 - std::sys::unix::thread::Thread::new::thread_start::h81139db33d5fdedd
  67:     0x7ff8da853609 - start_thread
  68:     0x7ff8da995133 - clone
  69:                0x0 - <unknown>
