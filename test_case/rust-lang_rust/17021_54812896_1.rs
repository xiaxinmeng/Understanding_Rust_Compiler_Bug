
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 12u32}, ReScope(6u32))
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/ast_util.rs:776

stack backtrace:
   1:        0x110c9f115 - rt::backtrace::imp::write::h38c3950ca30819fdNhr
   2:        0x110ca2451 - failure::on_fail::h61277a1f47996a75Hyr
   3:        0x110f53875 - unwind::begin_unwind_inner::h486b778488114bbbb0d
   4:        0x10ec5067a - unwind::begin_unwind::h3175822293847778141
   5:        0x10ec50e93 - diagnostic::Handler::bug::hd6b702e7e7b3a49eohF
   6:        0x10d6624f8 - driver::session::Session::bug::h815f6a1f5ba33169uqE
   7:        0x10db41e1c - middle::trans::debuginfo::TypeMap::get_unique_type_id_of_type::h1e440d0de59b8198a6q
   8:        0x10db4b627 - middle::trans::debuginfo::type_metadata::h6274e0cc3b47fb5cBvt
   9:        0x10dac6888 - middle::trans::debuginfo::create_function_debug_context::h7f8454964fde3412ZSr
  10:        0x10da3ef61 - middle::trans::base::new_fn_ctxt::hc9142e1507966736Xpf
  11:        0x10daca652 - middle::trans::base::trans_closure::h0c688b38a6dcca51gSf
  12:        0x10da75266 - middle::trans::closure::trans_unboxed_closure::hb0507895820464a1Onk
  13:        0x10da5fc26 - middle::trans::expr::trans_rvalue_dps_unadjusted::h34800865705f8eeeVG4
  14:        0x10da1bf9a - middle::trans::expr::trans_into::ha853a39b9a7f24dajk3
  15:        0x10da6a218 - middle::trans::expr::trans_uniq_expr::h40fb1130cbca349eit5
  16:        0x10da6aea3 - middle::trans::expr::trans_unary::hfc200761698d9538dp5
  17:        0x10da5ed3c - middle::trans::expr::trans_unadjusted::h82416de87d2e89c4t73
  18:        0x10da1bffb - middle::trans::expr::trans_into::ha853a39b9a7f24dajk3
  19:        0x10db1ea8e - middle::trans::_match::mk_binding_alloca::h346642657361229732
  20:        0x10dac1a26 - middle::trans::_match::store_local::h8c0c54cf00e6ee77cuj
  21:        0x10da1b5c2 - middle::trans::base::init_local::h1d29be9b103cc7b5HZe
  22:        0x10da1aafd - middle::trans::controlflow::trans_stmt::hc6927708a476c2f1B3Y
  23:        0x10da1c19c - middle::trans::controlflow::trans_block::hd755847507381b83M8Y
  24:        0x10dacbe66 - middle::trans::base::trans_closure::h0c688b38a6dcca51gSf
  25:        0x10da0c322 - middle::trans::base::trans_fn::h097de052d534817a33f
  26:        0x10da07bf3 - middle::trans::base::trans_item::h7c651a6b253c43b1Sng
  27:        0x10dad677c - middle::trans::base::trans_crate::h8a752ffbd6768478rnh
  28:        0x10def5241 - driver::driver::phase_4_translate_to_llvm::hff28f92720c498desQD
  29:        0x10deee1e8 - driver::driver::compile_input::h6fd531f77f6169fctsD
  30:        0x10df78faa - driver::run_compiler::hb663589ebabe2c20IjH
  31:        0x10df773b6 - driver::main_args::closure.147451
  32:        0x10d68fd6b - task::TaskBuilder<S>::try_future::closure.100018
  33:        0x10d68fc73 - task::TaskBuilder<S>::spawn_internal::closure.99989
  34:        0x10f3302cd - task::spawn_opts::closure.8473
  35:        0x110fb5d2c - rust_try_inner
  36:        0x110fb5d16 - rust_try
  37:        0x110f50d17 - unwind::try::h513c5138bad9ea23lOd
  38:        0x110f50b8c - task::Task::run::h87d8d159864e6bc2L0c
  39:        0x10f330122 - task::spawn_opts::closure.8412
  40:        0x110f527ba - thread::thread_start::hb27c396f8741958eSld
  41:     0x7fff8cd30899 - _pthread_body
  42:     0x7fff8cd3072a - _pthread_struct_init
