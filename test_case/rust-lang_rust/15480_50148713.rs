
export RUST_BACKTRACE=1; rustc r15480.rs 
r15480.rs:2:9: 2:15 error: internal compiler error: Explicit deref of non-derefable type: &[uint]
r15480.rs:2     let &ref a = [0u].as_slice();
                    ^~~~~~
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/ast_util.rs:784

stack backtrace:
   1:        0x112430835 - rt::backtrace::imp::write::h5d63a9f72ef354013fq
   2:        0x112433d9b - failure::on_fail::h89ada4236fabc57cmwq
   3:        0x1126d3829 - unwind::begin_unwind_inner::h4d1846566643d36fYXd
   4:        0x111ba14e4 - unwind::begin_unwind::h1316945459152882216
   5:        0x111ba1465 - diagnostic::SpanHandler::span_bug::h050393a5020e8271W3C
   6:        0x10f40d955 - driver::session::Session::span_bug::hc10d21d2b457e141DJx
   7:        0x10f7e5145 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_deref::h8125247981898923686
   8:        0x10f7e31a5 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern::h13645017680664184286
   9:        0x10f7cb20d - middle::typeck::check::regionck::visit_local::h42acb69149b80471OaM
  10:        0x10f7cb3ca - visit::walk_block::h17231782441881723029
  11:        0x10f80c181 - middle::typeck::check::check_bare_fn::h5be886e12a74689ciNQ
  12:        0x10f805232 - middle::typeck::check::check_item::hfe41449e63dbede7VlR
  13:        0x10f80bead - middle::typeck::check::check_item_types::h08bd559eea95a15cAMQ
  14:        0x10f2519c4 - util::common::time::h7914738111600996838
  15:        0x10f9f682c - middle::typeck::check_crate::hfd4595a3c987ab71i5h
  16:        0x10fac7ce8 - driver::driver::phase_3_run_analysis_passes::h79777ef02491f761QBw
  17:        0x10fac2b20 - driver::driver::compile_input::h26a8582bf38a3d4avnw
  18:        0x10fb6bfbf - driver::run_compiler::h70fa80d88f179db6Ixz
  19:        0x10fb6a5a6 - driver::main_args::closure.$x22closure$x22$LP$138335$RP$
  20:        0x10fb7ce7b - task::TaskBuilder<S>::try_future::closure.$x22closure$x22$LP$139479$RP$
  21:        0x10fb7cd70 - task::TaskBuilder<S>::spawn_internal::closure.$x22closure$x22$LP$139456$RP$
  22:        0x11062c44c - task::spawn_opts::closure.$x22closure$x22$LP$8327$RP$
  23:        0x1127378ec - rust_try
  24:        0x1126d07ab - unwind::try::hd934ccd3bea02a143Ld
  25:        0x1126d0513 - task::Task::run::hfec74f59cf869534VXc
  26:        0x11062c2e1 - task::spawn_opts::closure.$x22closure$x22$LP$8272$RP$
  27:        0x1126d25d6 - thread::thread_start::h830410741c29e779tkd
  28:     0x7fff901fa899 - _pthread_body
  29:     0x7fff901fa72a - _pthread_struct_init
