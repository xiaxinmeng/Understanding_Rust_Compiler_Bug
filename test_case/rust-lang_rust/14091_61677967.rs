
$ RUST_BACKTRACE=1 cargo test
   Compiling repoman v0.0.1 (file:///home/tim/projects/perso/rust/repoman)
/home/tim/projects/perso/rust/repoman/src/package.rs:47:9: 50:6 error: mismatched types: expected `core::option::Option<package::Package>`, found `()` (expected enum core::option::Option, found ())
/home/tim/projects/perso/rust/repoman/src/package.rs:47         for i in s.graphemes(true).rev() {
/home/tim/projects/perso/rust/repoman/src/package.rs:48             // if 
/home/tim/projects/perso/rust/repoman/src/package.rs:49         }
/home/tim/projects/perso/rust/repoman/src/package.rs:50     }
/home/tim/projects/perso/rust/repoman/src/package.rs:67:29: 67:71 error: mismatched types: expected `package::Package`, found `core::option::Option<_>` (expected struct package::Package, found enum core::option::Option)
/home/tim/projects/perso/rust/repoman/src/package.rs:67     let package1: Package = from_str("lnav-0.5.1-1-x86_64.pkg.tar.xz");
                                                                                    ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:8:12: 69:21 error: cannot apply unary operator `!` to type `package::Package`
<std macros>:8         if !$cond {
<std macros>:9             panic!($($arg),+)
<std macros>:10         }
<std macros>:11     );
<std macros>:12 )
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'index out of bounds: the len is 12 but the index is 12', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/codemap.rs:296

stack backtrace:
   1:     0x7f8d8c6a0db0 - rt::backtrace::imp::write::hfcc56047c8677681dcq
   2:     0x7f8d8c6a3eb0 - failure::on_fail::h56735b7b61c170bezxq
   3:     0x7f8d8ce35b00 - unwind::begin_unwind_inner::h7023b6652e7e5a93ZSd
   4:     0x7f8d8ce35750 - unwind::begin_unwind_fmt::h0e9ac2335b083bc1rQd
   5:     0x7f8d8ce35710 - rust_begin_unwind
   6:     0x7f8d8ce80f60 - panicking::panic_fmt::hdece59d38ae06313V6j
   7:     0x7f8d8ce80db0 - panicking::panic_bounds_check::h8e39cee29604d35eu5j
   8:     0x7f8d8b138ce0 - codemap::FileMap::get_line::hd58d1686dbf46629ffF
   9:     0x7f8d8b166a40 - diagnostic::emit::h12ba7a29b479ec9aaEG
  10:     0x7f8d8b163ca0 - diagnostic::EmitterWriter.Emitter::emit::h4f79bd6e9eeee7ecZzG
  11:     0x7f8d8b162500 - diagnostic::Handler::emit::hdd43971400185998qiG
  12:     0x7f8d8b135570 - diagnostic::SpanHandler::span_err::h65e13d2c8ffec314q6F
  13:     0x7f8d8d831610 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message_str_with_expected::he148908c47756db1JJh
  14:     0x7f8d8d7667a0 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message::hc03252dd89bc3f86VOh
  15:     0x7f8d8d7952e0 - middle::typeck::check::check_expr_with_unifier::check_user_unop::closure.133104
  16:     0x7f8d8d7947b0 - middle::typeck::check::check_expr_with_unifier::lookup_op_method::h925b57359fc7c14cKeZ
  17:     0x7f8d8d776490 - middle::typeck::check::check_expr_with_unifier::check_user_unop::h2153e474e2d365fbUsZ
  18:     0x7f8d8d76acd0 - middle::typeck::check::check_expr_with_unifier::hc017bd90e7abbc35l3Y
  19:     0x7f8d8d76acd0 - middle::typeck::check::check_expr_with_unifier::hc017bd90e7abbc35l3Y
  20:     0x7f8d8d7c1750 - middle::typeck::check::check_stmt::h76df15d5e1964844450
  21:     0x7f8d8d735140 - middle::typeck::check::check_block_with_expected::h7d0af6847d3f929bda1
  22:     0x7f8d8d7306f0 - middle::typeck::check::check_fn::h0cd0ebef9d88ad0bxPV
  23:     0x7f8d8d730430 - middle::typeck::check::check_bare_fn::h43a8375bf2965f78MEV
  24:     0x7f8d8d72c180 - middle::typeck::check::check_item::h2141c51e729a2e29VYV
  25:     0x7f8d8d72fbb0 - visit::Visitor::visit_mod::h11718002965167792138
  26:     0x7f8d8d72e6e0 - visit::walk_item::h1923608722162431246
  27:     0x7f8d8d72ff00 - middle::typeck::check::check_item_types::h21c4d2e562b90154WDV
  28:     0x7f8d8d232570 - util::common::time::h4350920800167001695
  29:     0x7f8d8da0bf50 - middle::typeck::check_crate::h29145ae1f68971acnmo
  30:     0x7f8d8da75b90 - driver::driver::phase_3_run_analysis_passes::h46c788750669329bn7A
  31:     0x7f8d8da70330 - driver::driver::compile_input::hb8c2da13155ca3868NA
  32:     0x7f8d8daf2a60 - driver::run_compiler::h9cc567da53bf425bMAE
  33:     0x7f8d8daf2950 - driver::run::closure.145705
  34:     0x7f8d8d24b800 - task::TaskBuilder<S>::try_future::closure.104009
  35:     0x7f8d8d24b5f0 - task::TaskBuilder<S>::spawn_internal::closure.103980
  36:     0x7f8d8e36f4e0 - task::NativeSpawner.Spawner::spawn::closure.8441
  37:     0x7f8d8ce8ec70 - rust_try_inner
  38:     0x7f8d8ce8ec60 - rust_try
  39:     0x7f8d8ce33430 - unwind::try::hfb30d5c8546abbdfjHd
  40:     0x7f8d8ce332c0 - task::Task::run::h04bab45a966850d49Mc
  41:     0x7f8d8e36f220 - task::NativeSpawner.Spawner::spawn::closure.8379
  42:     0x7f8d8ce34ad0 - thread::thread_start::h17b6830d32612a2do8c
  43:     0x7f8d8c16f250 - start_thread
  44:     0x7f8d8cb0f3b9 - clone
  45:                0x0 - <unknown>

Could not compile `repoman`.

To learn more, run the command again with --verbose.
