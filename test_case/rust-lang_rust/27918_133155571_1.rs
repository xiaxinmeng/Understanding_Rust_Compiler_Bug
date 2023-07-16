 _
thread 'rustc' panicked at 'assertion failed: `(left == right)` (left: `1`, right: `4`)', ../src/librustc\middle\check_match.rs:536

stack backtrace:
   1:         0x699bb42e - sys::backtrace::write::hb12aa40f2f209619ols
   2:         0x699c5115 - rt::unwind::register::h92f30f49d41046e7Qww
   3:         0x6998564f - rt::unwind::begin_unwind_inner::hbffbb4981b0d700aZtw
   4:         0x69985fdc - rt::unwind::begin_unwind_fmt::h393d9d0c019d9fa55sw
   5:         0x6925756e - middle::const_eval::const_expr_to_pat::h09de9c640f8e12dfYuj
   6:         0x692379ac - session::Session::span_note::he19d21226dfbf5ecNYv
   7:         0x69230f81 - middle::check_match::MatchCheckCtxt<'a, 'tcx>.Visitor<'v>::visit_expr::h08c39dd719c14c968uh
   8:         0x692327bb - middle::check_match::MatchCheckCtxt<'a, 'tcx>.Visitor<'v>::visit_fn::ha264f9b30c751779Cvh
   9:         0x69232d11 - middle::check_match::check_crate::hf7369bd73cb3b474kwh
  10:         0x6923292b - middle::check_match::check_crate::hf7369bd73cb3b474kwh
  11:         0x67c0a4a3 - driver::assign_node_ids_and_map::habd9138624442090aEa
  12:         0x67be68c9 - driver::assign_node_ids_and_map::habd9138624442090aEa
  13:         0x67be1bb1 - driver::assign_node_ids_and_map::habd9138624442090aEa
  14:         0x67bc278c - driver::compile_input::hec03b27c99a60d9dTba
  15:         0x67d39bd8 - run_compiler::he293276e81cb97efObc
  16:         0x67d374f0 - run::hd965d7dec4214920ubc
  17:         0x67d36f69 - run::hd965d7dec4214920ubc
  18:         0x699ae9b4 - rt::unwind::try::inner_try::hfa861391a664fbdeSpw
  19:         0x67d37104 - run::hd965d7dec4214920ubc
  20:         0x699c2dee - rt::util::report_overflow::h4a1688dd50f7ee7eyaw
  21:     0x7ffdaf8413d2 - BaseThreadInitThunk
