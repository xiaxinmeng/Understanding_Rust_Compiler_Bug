
stack backtrace:
   1:     0x7f9575258029 - sys::backtrace::write::hf53a0a0304290fee0Yr
   2:     0x7f957525fcf6 - panicking::on_panic::h60c28f18db36901f8lw
   3:     0x7f9575223132 - rt::unwind::begin_unwind_inner::h5d1eccd17bdb1225i1v
   4:     0x7f9575223ddc - rt::unwind::begin_unwind_fmt::hb401ae9bbc90d7afWZv
   5:     0x7f9573281fb0 - middle::ty::resolve_expr::hf61e430f2bd8af7cpA6
   6:     0x7f95733dfb14 - middle::ty::expr_kind::h6cb69ef7f4fda43evC6
   7:     0x7f95733df8da - middle::ty::expr_is_lval::h138e0c77224333e0sB6
   8:     0x7f95740f58ee - check::check_expr_with_unifier::h4343063250928622814
   9:     0x7f95740fb12d - check::check_expr_with_unifier::h6766827305629123850
  10:     0x7f957411f37b - check::check_decl_local::h3d3e5e5cd67bc079wJr
  11:     0x7f95740ca071 - check::check_block_with_expected::heaa9263342617e41CPr
  12:     0x7f95740acdc6 - check::check_fn::h663282a6a4f370d69Bn
  13:     0x7f95740c5ad4 - check::check_bare_fn::h7d7ab3fe69c299f6Irn
  14:     0x7f95740ce58c - check::check_method_body::hcb2705c1c7a9d05dd3n
  15:     0x7f95740c3a98 - check::CheckItemBodiesVisitor<'a, 'tcx>.Visitor<'tcx>::visit_item::ha3ca0ac0d25dbd13Lon
  16:     0x7f9574186f5a - check_crate::closure.37936
  17:     0x7f95741822f0 - check_crate::h7ab098187e399857c8B
  18:     0x7f9575798738 - driver::phase_3_run_analysis_passes::hbeaaad99c28fdb0bnGa
  19:     0x7f9575779b95 - driver::compile_input::hf2d9c43b8b07f43bQba
  20:     0x7f957583a241 - run_compiler::h977e0e80487442d8z4b
  21:     0x7f9575837e92 - boxed::F.FnBox<A>::call_box::h11406239215262416381
  22:     0x7f95758373c9 - rt::unwind::try::try_fn::h16514645961410991188
  23:     0x7f95752d2518 - rust_try_inner
  24:     0x7f95752d2505 - rust_try
  25:     0x7f9575837678 - boxed::F.FnBox<A>::call_box::h6172280421237676693
  26:     0x7f957525ebe1 - sys::thread::create::thread_start::ha7b08762db72e408Zkv
  27:     0x7f956fb0e0a3 - start_thread
  28:     0x7f9574eb106c - clone
  29:                0x0 - <unknown>
