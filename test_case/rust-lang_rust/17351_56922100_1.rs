
$ RUST_BACKTRACE=1 rustc ref-ice.rs
error: internal compiler error: trying to take the sizing type of str, an unsized type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/nodakai/src/rust-HEAD/src/libsyntax/diagnostic.rs:169

stack backtrace:
   1:     0x7f7ce2a896d0 - rt::backtrace::imp::write::hbe598dccc42c44bcvUq
   2:     0x7f7ce2a8c890 - failure::on_fail::hf35317f400fdba2fXfr
   3:     0x7f7ce704ac80 - unwind::begin_unwind_inner::h6f8b2a1fb914a9f1MTd
   4:     0x7f7ce59fcb20 - unwind::begin_unwind::h4350470456619517975
   5:     0x7f7ce59fd290 - diagnostic::Handler::bug::h5d9ce6fca7ccfb68OnF
   6:     0x7f7ce769b3c0 - driver::session::Session::bug::h9fb96e7547adf908Rix
   7:     0x7f7ce7a53510 - middle::trans::type_of::sizing_type_of::hed2e082010dc4455yI9
   8:     0x7f7ce7a831a0 - middle::trans::meth::get_vtable::hf8260e145d4c1737GIk
   9:     0x7f7ce7a82ae0 - middle::trans::expr::apply_adjustments::unsized_info::hdd775491ae0bafb9yb3
  10:     0x7f7ce7a85a80 - middle::trans::expr::apply_adjustments::unsize_expr::closure.123436
  11:     0x7f7ce7a85bc0 - middle::trans::expr::apply_adjustments::into_fat_ptr::h413a152fb7576cbbRi3
  12:     0x7f7ce7a818c0 - middle::trans::expr::apply_adjustments::apply_autoref::h3fa9af8370a776a9vY2
  13:     0x7f7ce7a818c0 - middle::trans::expr::apply_adjustments::apply_autoref::h3fa9af8370a776a9vY2
  14:     0x7f7ce7a41ff0 - middle::trans::expr::trans::h7de3de0790dc0c2bJM2
  15:     0x7f7ce7ae50d0 - middle::trans::_match::store_local::h770d8827c10345ba4hi
  16:     0x7f7ce7a400a0 - middle::trans::base::init_local::hc4ffd72108fbdbe9G2d
  17:     0x7f7ce7a3f5d0 - middle::trans::controlflow::trans_stmt::hf5c2949442fd16f3HMY
  18:     0x7f7ce7a40f80 - middle::trans::controlflow::trans_block::h58e1b3f21ebc3e6eDRY
  19:     0x7f7ce7aec250 - middle::trans::base::trans_closure::h649c196adecff5986Te
  20:     0x7f7ce7a33440 - middle::trans::base::trans_fn::h604a3d28a1b59852j5e
  21:     0x7f7ce7a30640 - middle::trans::base::trans_item::h73aa5253eb5d70eesof
  22:     0x7f7ce7af7450 - middle::trans::base::trans_crate::h9bc711dea61e7bf4Iog
  23:     0x7f7ce7f168e0 - driver::driver::phase_4_translate_to_llvm::h647b738894006b4fPJw
  24:     0x7f7ce7f0e0b0 - driver::driver::compile_input::hacecb9271bb5449eJgw
  25:     0x7f7ce7f90580 - driver::run_compiler::h5197019ba5ec6a85N6z
  26:     0x7f7ce7f90460 - driver::main_args::closure.146094
  27:     0x7f7ce76c9c40 - task::TaskBuilder<S>::try_future::closure.101461
  28:     0x7f7ce76c9a30 - task::TaskBuilder<S>::spawn_internal::closure.101432
  29:     0x7f7ce7399c40 - task::spawn_opts::closure.8485
  30:     0x7f7ce70a0e30 - rust_try_inner
  31:     0x7f7ce70a0e20 - rust_try
  32:     0x7f7ce70482c0 - unwind::try::h981c354b14bfedbduId
  33:     0x7f7ce7048120 - task::Task::run::hf1703086af82fbbacYc
  34:     0x7f7ce73999b0 - task::spawn_opts::closure.8425
  35:     0x7f7ce7049d10 - thread::thread_start::h1064d7802b5351b6rid
  36:     0x7f7ce1e3adc0 - start_thread
  37:                0x0 - <unknown>

$ rustc -v
rustc 0.12.0-pre (375fe1721 2014-09-25 12:02:52 +0000)
