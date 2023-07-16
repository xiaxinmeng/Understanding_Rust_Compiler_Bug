
$ RUST_BACKTRACE=1 rustc die.rs
die.rs:11:9: 11:18 error: attempted access of field `data` on type `&Bad<T>`, but no field with that name was found
die.rs:11         self.data.slice_from(0).slice_to(1);
                  ^~~~~~~~~
die.rs:18:20: 18:27 error: cannot use call notation; the first type parameter for the function trait is neither a tuple nor unit
die.rs:18     println!("{}", v(0, 0));
                             ^~~~~~~
note: in expansion of format_args!
<std macros>:2:23: 2:77 note: expansion site
<std macros>:1:1: 3:2 note: in expansion of println!
die.rs:18:5: 18:29 note: expansion site
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 1 but the index is 1', /home/nathan/devel/rust/rust/src/librustc/lib.rs:1

stack backtrace:
   1:     0x7fd888990d90 - rt::backtrace::imp::write::h436707714ecadf3elIp::v0.11.0.pre
   2:     0x7fd8889984c0 - failure::on_fail::hf6cceb4413eaef6dA3p::v0.11.0.pre
   3:     0x7fd88913fe30 - unwind::begin_unwind_inner::h769b42ddf5cb3d4bQRd::v0.11.0.pre
   4:     0x7fd88913f8c0 - unwind::begin_unwind_fmt::h7e40d6e72bc51347jPd::v0.11.0.pre
   5:     0x7fd88913f880 - rust_begin_unwind
   6:     0x7fd889188920 - failure::begin_unwind::h182011a43b7144bf23v::v0.11.0.pre
   7:     0x7fd88918c710 - failure::fail_bounds_check::hfa3dd6729a018c22e2v::v0.11.0.pre
   8:     0x7fd889b239f0 - middle::typeck::check::check_argument_types::h4f89c5a01c41142b2Zd::v0.11.0.pre
   9:     0x7fd889b23470 - middle::typeck::check::check_method_argument_types::hbaff6a741ca4aa435Xd::v0.11.0.pre
  10:     0x7fd889b22690 - middle::typeck::check::try_overloaded_call::h1aef66e95de5d9f6vSd::v0.11.0.pre
  11:     0x7fd889b26080 - middle::typeck::check::check_expr_with_unifier::h859fa852455df2bfDye::v0.11.0.pre
  12:     0x7fd889b26080 - middle::typeck::check::check_expr_with_unifier::h859fa852455df2bfDye::v0.11.0.pre
  13:     0x7fd889b26080 - middle::typeck::check::check_expr_with_unifier::h859fa852455df2bfDye::v0.11.0.pre
  14:     0x7fd889a6bfc0 - middle::typeck::check::_match::check_match::h7d3d137e0628a024lW3::v0.11.0.pre
  15:     0x7fd889b26080 - middle::typeck::check::check_expr_with_unifier::h859fa852455df2bfDye::v0.11.0.pre
  16:     0x7fd889b42d90 - middle::typeck::check::check_stmt::h16245f3a95fccff6YBg::v0.11.0.pre
  17:     0x7fd889afe190 - middle::typeck::check::check_block_with_expected::ha510f76b9f1231639Fg::v0.11.0.pre
  18:     0x7fd889af9b70 - middle::typeck::check::check_fn::h69104696ae3dbd2bdjc::v0.11.0.pre
  19:     0x7fd889af9930 - middle::typeck::check::check_bare_fn::h9009b665b2cddbf2L8b::v0.11.0.pre
  20:     0x7fd889af2580 - middle::typeck::check::check_item::h0a03a1af941c197eZFc::v0.11.0.pre
  21:     0x7fd889af9730 - middle::typeck::check::check_item_types::hd7ce737b3d0be28a37b::v0.11.0.pre
  22:     0x7fd889c787d0 - middle::typeck::check_crate::h45e9c8a03d43b18abJA::v0.11.0.pre
  23:     0x7fd88a1212f0 - driver::driver::phase_3_run_analysis_passes::h9e13cfc7310c64f0Agv::v0.11.0.pre
  24:     0x7fd88a119850 - driver::driver::compile_input::h2b811b1075169df3P4u::v0.11.0.pre
  25:     0x7fd88a1e4440 - driver::run_compiler::h10a82712b6039942rOx::v0.11.0.pre
  26:     0x7fd88a1e4350 - driver::main_args::closure.98482
  27:     0x7fd88a1ffa70 - driver::monitor::closure.99572
  28:     0x7fd88a1fa9a0 - task::TaskBuilder::try::closure.99335
  29:     0x7fd88c138de0 - task::spawn_opts::closure.7192
  30:     0x7fd88913ca00 - task::Task::run::closure.5321
  31:     0x7fd8891a21a0 - rust_try
  32:     0x7fd88913f450 - unwind::try::h45292833fd63fac7fGd::v0.11.0.pre
  33:     0x7fd88913c880 - task::Task::run::ha683dd2de6346afbVVc::v0.11.0.pre
  34:     0x7fd88c138b80 - task::spawn_opts::closure.7165
  35:     0x7fd88913ea10 - thread::thread_start::hac5cc2b4940a2301kdd::v0.11.0.pre
  36:     0x7fd8883fe000 - start_thread
  37:     0x7fd888e07fc9 - __clone
  38:                0x0 - <unknown>
