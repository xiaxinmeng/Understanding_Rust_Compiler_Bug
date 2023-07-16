
   Compiling playground v0.0.1 (/playground)
thread 'rustc' panicked at 'internal error: entered unreachable code: we captured two identical projections: capture1 = CapturedPlace { place: Place { base_ty: std::result::Result<std::result::Result<(), _>, Error>, base: Upvar(UpvarId(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 22 };`a`;DefId(0:25 ~ playground[d8bd]::main::{closure#0}::{closure#0}))), projections: [Projection { ty: Error, kind: Field(0, 1) }] }, info: CaptureInfo { capture_kind_expr_id: Some(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 25 }), path_expr_id: Some(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 25 }), capture_kind: ByValue }, mutability: Not, region: None }, capture2 = CapturedPlace { place: Place { base_ty: std::result::Result<std::result::Result<(), _>, Error>, base: Upvar(UpvarId(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 22 };`a`;DefId(0:25 ~ playground[d8bd]::main::{closure#0}::{closure#0}))), projections: [Projection { ty: std::result::Result<(), _>, kind: Field(0, 0) }, Projection { ty: (), kind: Field(0, 0) }] }, info: CaptureInfo { capture_kind_expr_id: Some(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 25 }), path_expr_id: Some(HirId { owner: DefId(0:23 ~ playground[d8bd]::main), local_id: 25 }), capture_kind: ByRef(ImmBorrow) }, mutability: Not, region: Some('_#0r) }', compiler/rustc_typeck/src/check/upvar.rs:712:17
stack backtrace:
   0:     0x7f35fd402d40 - std::backtrace_rs::backtrace::libunwind::trace::h32eb3e08e874dd27
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f35fd402d40 - std::backtrace_rs::backtrace::trace_unsynchronized::haa3f451d27bc11a5
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f35fd402d40 - std::sys_common::backtrace::_print_fmt::h5b94a01bb4289bb5
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f35fd402d40 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb070b7fa7e3175df
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f35fd45dbfe - core::fmt::write::hd5207aebbb9a86e9
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/fmt/mod.rs:1202:17
   5:     0x7f35fd3f3935 - std::io::Write::write_fmt::h3bd699bbd129ab8a
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/io/mod.rs:1679:15
   6:     0x7f35fd4059f3 - std::sys_common::backtrace::_print::h7a21be552fdf58da
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f35fd4059f3 - std::sys_common::backtrace::print::ha85c41fe4dd80b13
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f35fd4059f3 - std::panicking::default_hook::{{closure}}::h04cca40023d0eeca
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:295:22
   9:     0x7f35fd4056df - std::panicking::default_hook::haa3ca8c310ed5402
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:314:9
  10:     0x7f35ffc332e1 - rustc_driver[cfb34b1539811fe8]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f35fd40622d - std::panicking::rust_panic_with_hook::h7b190ce1a948faac
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:702:17
  12:     0x7f35fd406087 - std::panicking::begin_panic_handler::{{closure}}::hbafbfdc3e1b97f68
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:588:13
  13:     0x7f35fd4031ec - std::sys_common::backtrace::__rust_end_short_backtrace::hda93e5fef243b4c0
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f35fd405da2 - rust_begin_unwind
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
  15:     0x7f35fd45a7d3 - core::panicking::panic_fmt::h8d17ca1073d9a733
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
  16:     0x7f35ff6aa606 - <[rustc_middle[a9ca1c3f9fd197cd]::ty::closure::CapturedPlace]>::sort_by::<<rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::compute_min_captures::{closure#1}>::{closure#0}
  17:     0x7f35ff6a91f1 - alloc[188ed69dc0d14b4b]::slice::merge_sort::<rustc_middle[a9ca1c3f9fd197cd]::ty::closure::CapturedPlace, <[rustc_middle[a9ca1c3f9fd197cd]::ty::closure::CapturedPlace]>::sort_by<<rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::compute_min_captures::{closure#1}>::{closure#0}>
  18:     0x7f35ff6a7a3e - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::compute_min_captures
  19:     0x7f35ff6a1ad2 - <rustc_typeck[4a3b2f91c49b3c3e]::check::fn_ctxt::FnCtxt>::analyze_closure
  20:     0x7f35fe6717a5 - <rustc_typeck[4a3b2f91c49b3c3e]::check::upvar::InferBorrowKindVisitor as rustc_hir[f29014e7b3f8eb6c]::intravisit::Visitor>::visit_expr
  21:     0x7f35fe671a07 - <rustc_typeck[4a3b2f91c49b3c3e]::check::upvar::InferBorrowKindVisitor as rustc_hir[f29014e7b3f8eb6c]::intravisit::Visitor>::visit_expr
  22:     0x7f35fe671781 - <rustc_typeck[4a3b2f91c49b3c3e]::check::upvar::InferBorrowKindVisitor as rustc_hir[f29014e7b3f8eb6c]::intravisit::Visitor>::visit_expr
  23:     0x7f35fe671a7c - <rustc_typeck[4a3b2f91c49b3c3e]::check::upvar::InferBorrowKindVisitor as rustc_hir[f29014e7b3f8eb6c]::intravisit::Visitor>::visit_expr
  24:     0x7f35fe6719dd - <rustc_typeck[4a3b2f91c49b3c3e]::check::upvar::InferBorrowKindVisitor as rustc_hir[f29014e7b3f8eb6c]::intravisit::Visitor>::visit_expr
  25:     0x7f35ff258bbb - <rustc_infer[ed26b14e1208c12f]::infer::InferCtxtBuilder>::enter::<&rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults, <rustc_typeck[4a3b2f91c49b3c3e]::check::inherited::InheritedBuilder>::enter<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_with_fallback<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>::{closure#0}>
  26:     0x7f35ff256c29 - rustc_typeck[4a3b2f91c49b3c3e]::check::typeck
  27:     0x7f35ff4f5103 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<rustc_span[8f00505efff112e2]::def_id::LocalDefId, &rustc_middle[a9ca1c3f9fd197cd]::ty::context::TypeckResults>>
  28:     0x7f35ffb6fcae - <rustc_query_impl[d6191eea714bd250]::Queries as rustc_middle[a9ca1c3f9fd197cd]::ty::query::QueryEngine>::typeck
  29:     0x7f35ff9b0057 - rustc_data_structures[62ce3989ce8140ee]::sync::par_for_each_in::<&[rustc_span[8f00505efff112e2]::def_id::LocalDefId], <rustc_middle[a9ca1c3f9fd197cd]::hir::map::Map>::par_body_owners<rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  30:     0x7f35ff9afe13 - rustc_typeck[4a3b2f91c49b3c3e]::check::typeck_item_bodies
  31:     0x7f35ff8b8f99 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<(), ()>>
  32:     0x7f35ff8b8cc7 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::get_query::<rustc_query_impl[d6191eea714bd250]::queries::typeck_item_bodies, rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt>
  33:     0x7f35fefa6a9f - <rustc_session[c5946fac61f8bc34]::session::Session>::time::<(), rustc_typeck[4a3b2f91c49b3c3e]::check_crate::{closure#7}>
  34:     0x7f35fefa665f - rustc_typeck[4a3b2f91c49b3c3e]::check_crate
  35:     0x7f35fefa5c27 - rustc_interface[3182dd864eff9d7d]::passes::analysis
  36:     0x7f35ffa037ed - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::try_execute_query::<rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt, rustc_query_system[860ed7b39cdfb46b]::query::caches::DefaultCache<(), core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>>
  37:     0x7f35ffa03517 - rustc_query_system[860ed7b39cdfb46b]::query::plumbing::get_query::<rustc_query_impl[d6191eea714bd250]::queries::analysis, rustc_query_impl[d6191eea714bd250]::plumbing::QueryCtxt>
  38:     0x7f35feaef897 - <rustc_interface[3182dd864eff9d7d]::passes::QueryContext>::enter::<rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  39:     0x7f35feae328c - rustc_interface[3182dd864eff9d7d]::interface::create_compiler_and_run::<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>
  40:     0x7f35feae1ac1 - <scoped_tls[e395fa6e23b19669]::ScopedKey<rustc_span[8f00505efff112e2]::SessionGlobals>>::set::<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  41:     0x7f35feae17af - std[71cb4861428b0c25]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3182dd864eff9d7d]::util::run_in_thread_pool_with_globals<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>
  42:     0x7f35ffa96d70 - <<std[71cb4861428b0c25]::thread::Builder>::spawn_unchecked_<rustc_interface[3182dd864eff9d7d]::util::run_in_thread_pool_with_globals<rustc_interface[3182dd864eff9d7d]::interface::run_compiler<core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>, rustc_driver[cfb34b1539811fe8]::run_compiler::{closure#1}>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#0}, core[8c92e53db3fc2eaa]::result::Result<(), rustc_errors[f77a66b68db622d5]::ErrorGuaranteed>>::{closure#1} as core[8c92e53db3fc2eaa]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f35fd410003 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h49f797984e2121bf
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
  44:     0x7f35fd410003 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfa4f3d0ee6440e0b
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/alloc/src/boxed.rs:1940:9
  45:     0x7f35fd410003 - std::sys::unix::thread::Thread::new::thread_start::h62ca48b42d48a8fc
                               at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys/unix/thread.rs:108:17
  46:     0x7f35fd2e3609 - start_thread
  47:     0x7f35fd206133 - clone
  48:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0 (897e37553 2022-11-02) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `playground`
