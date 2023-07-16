
 Compiling playground v0.0.1 (/playground)
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

thread 'rustc' panicked at 'index out of bounds: the len is 0 but the index is 0', src/librustc_mir_build/hair/pattern/_match.rs:2329:13
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:78
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:59
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1069
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1439
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:62
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:198
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:218
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:515
  12: rust_begin_unwind
             at src/libstd/panicking.rs:419
  13: core::panicking::panic_fmt
             at src/libcore/panicking.rs:111
  14: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:69
  15: rustc_mir_build::hair::pattern::_match::PatStack::specialize_constructor
  16: rustc_mir_build::hair::pattern::_match::is_useful_specialized
  17: <core::iter::adapters::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold
  18: rustc_mir_build::hair::pattern::_match::is_useful
  19: rustc_mir_build::hair::pattern::check_match::check_not_useful
  20: rustc_mir_build::hair::pattern::check_match::check_exhaustive
  21: rustc_mir_build::hair::pattern::_match::MatchCheckCtxt::create_and_enter
  22: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  23: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  24: rustc_hir::intravisit::walk_expr
  25: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  26: rustc_hir::intravisit::walk_expr
  27: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  28: rustc_hir::intravisit::walk_block
  29: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  30: rustc_hir::intravisit::walk_block
  31: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  32: rustc_hir::intravisit::walk_expr
  33: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  34: rustc_hir::intravisit::walk_expr
  35: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  36: rustc_hir::intravisit::walk_block
  37: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  38: rustc_hir::intravisit::walk_block
  39: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  40: rustc_hir::intravisit::walk_expr
  41: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  42: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  43: <rustc_mir_build::hair::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_expr
  44: rustc_mir_build::hair::pattern::check_match::check_match
  45: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_match>::compute
  46: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  47: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  48: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  49: rustc_session::utils::<impl rustc_session::session::Session>::time
  50: rustc_interface::passes::analysis
  51: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  52: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  53: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  54: rustc::ty::context::tls::enter_global
  55: rustc_interface::interface::run_compiler_in_existing_thread_pool
  56: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (75208942f 2020-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_match] processing `main`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 36 }, kind: MethodCall(PathSegment { ident: const_chunks_exact#0, hir_id: Some(HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 28 }), res: Some(Err), args: Some(GenericArgs { args: [Const(ConstArg { value: AnonConst { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 24 }, body: BodyId { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 27 } } }, span: src/main.rs:42:65: 42:75 })], bindings: [], parenthesized: false }), infer_args: false }, src/main.rs:42:44: 42:62, [Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 35 }, kind: Array([Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 29 }, kind: Lit(Spanned { node: Int(1, Unsuffixed), span: src/main.rs:42:23: 42:24 }), attrs: ThinVec(None), span: src/main.rs:42:23: 42:24 }, Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 30 }, kind: Lit(Spanned { node: Int(2, Unsuffixed), span: src/main.rs:42:26: 42:27 }), attrs: ThinVec(None), span: src/main.rs:42:26: 42:27 }, Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 31 }, kind: Lit(Spanned { node: Int(3, Unsuffixed), span: src/main.rs:42:29: 42:30 }), attrs: ThinVec(None), span: src/main.rs:42:29: 42:30 }, Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 32 }, kind: Lit(Spanned { node: Int(4, Unsuffixed), span: src/main.rs:42:32: 42:33 }), attrs: ThinVec(None), span: src/main.rs:42:32: 42:33 }, Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 33 }, kind: Lit(Spanned { node: Int(5, Unsuffixed), span: src/main.rs:42:35: 42:36 }), attrs: ThinVec(None), span: src/main.rs:42:35: 42:36 }, Expr { hir_id: HirId { owner: DefId(0:29 ~ playground[4875]::main[0]), local_id: 34 }, kind: Lit(Spanned { node: Int(6, Signed(I32)), span: src/main.rs:42:38: 42:42 }), attrs: ThinVec(None), span: src/main.rs:42:38: 42:42 }]), attrs: ThinVec(None), span: src/main.rs:42:22: 42:43 }]), attrs: ThinVec(None), span: src/main.rs:42:22: 42:78 })

error: internal compiler error: cat_expr Errd
  --> src/main.rs:42:65
   |
42 |     let mut slice = &[1, 2, 3, 4, 5, 6i32].const_chunks_exact::<{ 1usize }>();
   |                                                                 ^^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:41:11
   |
41 |   fn main() {
   |  ___________^
42 | |     let mut slice = &[1, 2, 3, 4, 5, 6i32].const_chunks_exact::<{ 1usize }>();
43 | |
44 | |     for [a, b, c] in slice {
45 | |         dbg!(a, b, c);
46 | |     }
47 | | }
   | |_^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:42:21
   |
42 |     let mut slice = &[1, 2, 3, 4, 5, 6i32].const_chunks_exact::<{ 1usize }>();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:42:22
   |
42 |     let mut slice = &[1, 2, 3, 4, 5, 6i32].const_chunks_exact::<{ 1usize }>();
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:44:22
   |
44 |     for [a, b, c] in slice {
   |                      ^^^^^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:44:9
   |
44 |     for [a, b, c] in slice {
   |         ^^^^^^^^^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:44:28
   |
44 |       for [a, b, c] in slice {
   |  ____________________________^
45 | |         dbg!(a, b, c);
46 | |     }
   | |_____^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:14
   |
45 |         dbg!(a, b, c);
   |              ^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:17
   |
45 |         dbg!(a, b, c);
   |                 ^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:20
   |
45 |         dbg!(a, b, c);
   |                    ^

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: cat_expr Errd
  --> src/main.rs:45:9
   |
45 |         dbg!(a, b, c);
   |         ^^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
stack backtrace:
   0:     0x7fe40417c364 - backtrace::backtrace::libunwind::trace::h03a082ceb81836bc
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7fe40417c364 - backtrace::backtrace::trace_unsynchronized::h34510e7f129d298e
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7fe40417c364 - std::sys_common::backtrace::_print_fmt::h52cf6e6445d665be
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7fe40417c364 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h355ef422c030af04
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7fe4041b9e6c - core::fmt::write::had06c030afbcee40
                               at src/libcore/fmt/mod.rs:1069
   5:     0x7fe40416df83 - std::io::Write::write_fmt::haabb031c309046f2
                               at src/libstd/io/mod.rs:1439
   6:     0x7fe404181185 - std::sys_common::backtrace::_print::h42076b4a9e432704
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7fe404181185 - std::sys_common::backtrace::print::he07d5fc37862a4ff
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7fe404181185 - std::panicking::default_hook::{{closure}}::h28a0f3917540fbd8
                               at src/libstd/panicking.rs:198
   9:     0x7fe404180ec2 - std::panicking::default_hook::h55a07c2c7f473659
                               at src/libstd/panicking.rs:218
  10:     0x7fe4048001b3 - rustc_driver::report_ice::h76afa7ba8a1313b2
  11:     0x7fe404181905 - std::panicking::rust_panic_with_hook::h66d10dbf34a95c6d
                               at src/libstd/panicking.rs:515
  12:     0x7fe406ebee7e - std::panicking::begin_panic::he48f319f5cea64fa
  13:     0x7fe406ecb572 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h5d8fed4f185053ea
  14:     0x7fe404839926 - core::ptr::drop_in_place::h37980ff445b6389d
  15:     0x7fe404848972 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::hb180bf142d35dd8a
  16:     0x7fe40487134d - core::ptr::drop_in_place::h59c5c18d9dba6deb
  17:     0x7fe40485bece - rustc_interface::interface::run_compiler_in_existing_thread_pool::h0ddf0d1de30b048f
  18:     0x7fe404805b6f - rustc_ast::attr::with_globals::hf6f784271402448f
  19:     0x7fe4048143fe - std::sys_common::backtrace::__rust_begin_short_backtrace::he9a56442df093426
  20:     0x7fe40485dcee - core::ops::function::FnOnce::call_once{{vtable.shim}}::haa5d5c73f22394b4
  21:     0x7fe40415e17f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hf5535eba2986b63c
                               at /rustc/75208942f6144daac669e8e382029fc33bdce841/src/liballoc/boxed.rs:1017
  22:     0x7fe404191333 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h855059088bd3c19d
                               at /rustc/75208942f6144daac669e8e382029fc33bdce841/src/liballoc/boxed.rs:1017
  23:     0x7fe404191333 - std::sys_common::thread::start_thread::h6881b5efbe0279f5
                               at src/libstd/sys_common/thread.rs:13
  24:     0x7fe404191333 - std::sys::unix::thread::Thread::new::thread_start::habd9ef16008d1651
                               at src/libstd/sys/unix/thread.rs:80
  25:     0x7fe403ef76db - start_thread
  26:     0x7fe40381488f - __clone
  27:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-nightly (75208942f 2020-03-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `playground`.

