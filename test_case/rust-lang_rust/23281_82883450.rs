
examples/beta/main.rs:1:1: 1:1 error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(gfx_p
hase::phase::QueuePhase<<gfx_device_gl::GlDevice as gfx::device::Device>::Resources, gfx_scene::Entity<gfx_device_gl::Gl
Resources, Material, World, cgmath::aabb::Aabb3<f32>>, ViewInfo>)` during trans
examples/beta/main.rs:1 #![feature(plugin, custom_attribute)]
                        ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', C:/bot/slave/nightly-dist-rustc-win-64/build/src/libsyntax\diagnostic.rs:130

stack backtrace:
   1:         0x7121db87 - sys::backtrace::write::h5718530388604c081TC
   2:         0x7123392f - rt::unwind::register::he8b3e6064992df829sI
   3:         0x711833b7 - rt::unwind::begin_unwind_inner::h1fa294b799e7e18eyqI
   4:           0x6c4689 - diagnostic::SpanHandler::span_bug::hb3e52aef4f9745fdecB
   5:           0x6c4653 - diagnostic::SpanHandler::span_bug::hb3e52aef4f9745fdecB
   6:           0xd12402 - session::Session::span_bug::h61937faaf13ccf56Zxn
   7:         0x63699fb3 - trans::context::CrateContext<'b, 'tcx>::const_cstr_cache::hd0e81dec443c873fC8l
   8:         0x63724e32 - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::hae59a47711e5d3206Aw
   9:         0x63724be0 - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::hae59a47711e5d3206Aw
  10:         0x6372462d - trans::_match::ReassignmentChecker.euv..Delegate<'tcx>::mutate::hae59a47711e5d3206Aw
  11:         0x63668608 - trans::common::erase_regions::RegionEraser<'a, 'tcx>.TypeFolder<'tcx>::fold_substs::h914fc30a
1a889d8cttk
  12:         0x6366a177 - trans::common::erase_regions::RegionEraser<'a, 'tcx>.TypeFolder<'tcx>::fold_substs::h914fc30a
1a889d8cttk
  13:         0x63662d21 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h68ace62b1a8043d7jAl
  14:         0x6360e5e6 - trans::expr::Dest...std..cmp..PartialEq::eq::h55966b95830454f5Ljh
  15:         0x63648f40 - trans::type_::Type...std..cmp..PartialEq::ne::h481fe24219cbffdf1OI
  16:         0x6364f849 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h68ace62b1a8043d7jAl
  17:         0x63658920 - trans::common::ExprOrMethodCall...std..cmp..PartialEq::ne::h68ace62b1a8043d7jAl
  18:         0x6360c575 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cle
anup_scope::h12fe383583d881b38GJ
  19:         0x6360b8d6 - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::push_ast_cleanup_scop
e::hd07f5b73e1c7c5b4uxJ
  20:         0x6360d2af - trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cle
anup_scope::h12fe383583d881b38GJ
  21:         0x636dd91a - trans::base::FindNestedReturn.Visitor<'v>::visit_expr::h8740d18820feb8d14Qs
  22:         0x635f658d - trans::context::CrateContext<'b, 'tcx>::sess::h5bc8955d2b0a00abe1l
  23:         0x635f22ef - trans::context::CrateContext<'b, 'tcx>::stats::h92d4e7510b4a3e69Ocm
  24:         0x636e68ac - trans::base::trans_crate::h52e3124b1088931bGPu
  25:         0x652e815b - driver::phase_4_translate_to_llvm::hc934fb9157ef9964kOa
  26:         0x652c274d - driver::compile_input::h066fd244e155433eRba
  27:         0x6537690a - run_compiler::h7aeb1051e7e97e93x2b
  28:         0x65374b49 - run::hd99b745df46cea3cd2b
  29:         0x65373c1e - run::hd99b745df46cea3cd2b
  30:         0x71264ecc - rust_try
  31:         0x71264ea9 - rust_try
  32:         0x653740cf - run::hd99b745df46cea3cd2b
  33:         0x71226f92 - sys::tcp::TcpListener::bind::h7c8290347217363aWPG
  34:     0x7ffd9b4116ad - BaseThreadInitThunk
``
