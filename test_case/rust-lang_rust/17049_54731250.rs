
     Running `rustc /home/mike/string-cache/macros/src/lib.rs --crate-name string_cache_macros --crate-type dylib -g -C metadata=bf39164d0dc7352f -C extra-filename=-bf39164d0dc7352f --out-dir /home/mike/string-cache/target/deps --dep-info /home/mike/string-cache/target/.fingerprint/string_cache_macros-bf39164d0dc7352f/dep-lib-string_cache_macros -L /home/mike/string-cache/target/deps -L /home/mike/string-cache/target/deps --extern lazy_static=/home/mike/string-cache/target/deps/liblazy_static-518089d616e0fe56.rlib`
     Running `rustc src/lazy_static.rs --crate-name lazy_static --crate-type lib -g -C metadata=518089d616e0fe56 -C extra-filename=-518089d616e0fe56 --out-dir /home/mike/string-cache/target/deps --dep-info /home/mike/string-cache/target/.fingerprint/lazy_static-518089d616e0fe56/dep-lib-lazy_static -L /home/mike/string-cache/target/deps -L /home/mike/string-cache/target/deps`
     Running `rustc src/lib.rs --crate-name phf --crate-type lib -g -C metadata=4fc01267f85abb4e -C extra-filename=-4fc01267f85abb4e --out-dir /home/mike/string-cache/target/deps --dep-info /home/mike/string-cache/target/.fingerprint/phf-4fc01267f85abb4e/dep-lib-phf -L /home/mike/string-cache/target/deps -L /home/mike/string-cache/target/deps`
     Running `rustc src/lib.rs --crate-name phf_mac --crate-type dylib -g -C metadata=78b8694be35c3487 -C extra-filename=-78b8694be35c3487 --out-dir /home/mike/string-cache/target/deps --dep-info /home/mike/string-cache/target/.fingerprint/phf_mac-78b8694be35c3487/dep-lib-phf_mac -L /home/mike/string-cache/target/deps -L /home/mike/string-cache/target/deps`
     Running `rustc /home/mike/string-cache/src/lib.rs --crate-name string_cache --crate-type lib -g -C metadata=3c09159f66283f48 -C extra-filename=-3c09159f66283f48 --out-dir /home/mike/string-cache/target --dep-info /home/mike/string-cache/target/.fingerprint/string_cache-3c09159f66283f48/dep-lib-string_cache -L /home/mike/string-cache/target -L /home/mike/string-cache/target/deps --extern string_cache_macros=/home/mike/string-cache/target/deps/libstring_cache_macros-bf39164d0dc7352f.so --extern phf=/home/mike/string-cache/target/deps/libphf-4fc01267f85abb4e.rlib --extern phf_mac=/home/mike/string-cache/target/deps/libphf_mac-78b8694be35c3487.so`
       Fresh lazy_static v0.1.0 (https://github.com/Kimundi/lazy-static.rs#e62a6537)
       Fresh phf_mac v0.0.0 (https://github.com/sfackler/rust-phf#d7ae8800)
       Fresh phf v0.0.0 (https://github.com/sfackler/rust-phf#d7ae8800)
   Compiling string_cache_macros v0.0.0 (file:///home/mike/string-cache)
Could not compile `string_cache_macros`.

--- stdout
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:776

stack backtrace:
   1:     0x7f88e7621ed0 - rt::backtrace::imp::write::h53108151a1c919cdOLq
   2:     0x7f88e7625070 - failure::on_fail::hce96b96ffc877504w7q
   3:     0x7f88ebc02930 - unwind::begin_unwind_inner::hab46511b32d4adddZ2d
   4:     0x7f88e8e5f4b0 - unwind::begin_unwind::h8363082939923295011
   5:     0x7f88e8e5fc20 - diagnostic::Handler::bug::h3586631f64308fe6ufF
   6:     0x7f88ec25fbf0 - driver::session::Session::bug::h7c4efc9e6e69e371MGC
   7:     0x7f88ec67c160 - middle::trans::consts::const_deref::h994d1bab8c0569bccg8
   8:     0x7f88ec67aab0 - middle::trans::consts::const_expr::hde53f8b5ba3ea703wl8
   9:     0x7f88ec5efb00 - middle::trans::base::get_item_val::hf51c1a11475b5c10ABf
  10:     0x7f88ec64b1c0 - middle::trans::expr::trans_def::hfb492b44752d31f3mN3
  11:     0x7f88ec641d10 - middle::trans::expr::trans_unadjusted::h5f343dfce53924b0wn3
  12:     0x7f88ec601540 - middle::trans::expr::trans::h264fae1f71a954e6iE2
  13:     0x7f88ec63a6c0 - middle::trans::callee::trans_args::h7ad814cf9be11da6O61
  14:     0x7f88ec609100 - middle::trans::callee::trans_call_inner::h1e8ce065ac7a042eOL1
  15:     0x7f88ec635140 - middle::trans::callee::trans_method_call::h84dca6926ed068a02G1
  16:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  17:     0x7f88ec641d10 - middle::trans::expr::trans_unadjusted::h5f343dfce53924b0wn3
  18:     0x7f88ec601540 - middle::trans::expr::trans::h264fae1f71a954e6iE2
  19:     0x7f88ec605fa0 - middle::trans::controlflow::trans_for::h27cc5ffa9e390491KFY
  20:     0x7f88ec6451a0 - middle::trans::expr::trans_rvalue_stmt_unadjusted::h1cdb65f7a13d5bbasR3
  21:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  22:     0x7f88ec6003a0 - middle::trans::controlflow::trans_block::hc6e990dd6ea246cacsY
  23:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  24:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  25:     0x7f88ec6d01f0 - middle::trans::_match::trans_match_inner::h042c86355422f43em3h
  26:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  27:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  28:     0x7f88ec5ff460 - middle::trans::controlflow::trans_stmt_semi::hdaac58c7fef10d0firY
  29:     0x7f88ec5feb40 - middle::trans::controlflow::trans_stmt::h87b829ce2fd7b6fc1mY
  30:     0x7f88ec6003a0 - middle::trans::controlflow::trans_block::hc6e990dd6ea246cacsY
  31:     0x7f88ec6ae170 - middle::trans::base::trans_closure::hca82336ddae206bcvEe
  32:     0x7f88ec5f1620 - middle::trans::base::trans_fn::h80c61a61ef5e8177iQe
  33:     0x7f88ec5ecff0 - middle::trans::base::trans_item::h61aebb3e8aff1b7ei8e
  34:     0x7f88ec5ecff0 - middle::trans::base::trans_item::h61aebb3e8aff1b7ei8e
  35:     0x7f88ec6b8920 - middle::trans::base::trans_crate::h1ecf6b249bfcb51cJ2f
  36:     0x7f88ecacbb50 - driver::driver::phase_4_translate_to_llvm::hfecc798fa173c5bam7B
  37:     0x7f88ecac3780 - driver::driver::compile_input::h8608525c5b471524uJB
  38:     0x7f88ecb574d0 - driver::run_compiler::h69b3fcd2a164d565UyF
  39:     0x7f88ecb573e0 - driver::main_args::closure.141489
  40:     0x7f88ecb69fe0 - task::TaskBuilder<S>::try_future::closure.142658
  41:     0x7f88ecb69de0 - task::TaskBuilder<S>::spawn_internal::closure.142635
  42:     0x7f88ebf5a530 - task::spawn_opts::closure.8397
  43:     0x7f88ebc58660 - rust_try_inner
  44:     0x7f88ebc58650 - rust_try
  45:     0x7f88ebc000f0 - unwind::try::hd6387be7e7aa758afRd
  46:     0x7f88ebbfff50 - task::Task::run::hb83ab5c92810e88692c
  47:     0x7f88ebf5a290 - task::spawn_opts::closure.8343
  48:     0x7f88ebc01a60 - thread::thread_start::hd97e8e956809086bgod
  49:     0x7f88e69bf060 - start_thread
  50:     0x7f88eb8d1489 - __clone
  51:                0x0 - <unknown>


--- stderr
error: internal compiler error: can't dereference const of type [(&'static str,&'static str), .. 6]
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace


Caused by:
  Process didn't exit successfully: `rustc /home/mike/string-cache/macros/src/lib.rs --crate-name string_cache_macros --crate-type dylib -g -C metadata=bf39164d0dc7352f -C extra-filename=-bf39164d0dc7352f --out-dir /home/mike/string-cache/target/deps --dep-info /home/mike/string-cache/target/.fingerprint/string_cache_macros-bf39164d0dc7352f/dep-lib-string_cache_macros -L /home/mike/string-cache/target/deps -L /home/mike/string-cache/target/deps --extern lazy_static=/home/mike/string-cache/target/deps/liblazy_static-518089d616e0fe56.rlib` (status=101)
--- stdout
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:776

stack backtrace:
   1:     0x7f88e7621ed0 - rt::backtrace::imp::write::h53108151a1c919cdOLq
   2:     0x7f88e7625070 - failure::on_fail::hce96b96ffc877504w7q
   3:     0x7f88ebc02930 - unwind::begin_unwind_inner::hab46511b32d4adddZ2d
   4:     0x7f88e8e5f4b0 - unwind::begin_unwind::h8363082939923295011
   5:     0x7f88e8e5fc20 - diagnostic::Handler::bug::h3586631f64308fe6ufF
   6:     0x7f88ec25fbf0 - driver::session::Session::bug::h7c4efc9e6e69e371MGC
   7:     0x7f88ec67c160 - middle::trans::consts::const_deref::h994d1bab8c0569bccg8
   8:     0x7f88ec67aab0 - middle::trans::consts::const_expr::hde53f8b5ba3ea703wl8
   9:     0x7f88ec5efb00 - middle::trans::base::get_item_val::hf51c1a11475b5c10ABf
  10:     0x7f88ec64b1c0 - middle::trans::expr::trans_def::hfb492b44752d31f3mN3
  11:     0x7f88ec641d10 - middle::trans::expr::trans_unadjusted::h5f343dfce53924b0wn3
  12:     0x7f88ec601540 - middle::trans::expr::trans::h264fae1f71a954e6iE2
  13:     0x7f88ec63a6c0 - middle::trans::callee::trans_args::h7ad814cf9be11da6O61
  14:     0x7f88ec609100 - middle::trans::callee::trans_call_inner::h1e8ce065ac7a042eOL1
  15:     0x7f88ec635140 - middle::trans::callee::trans_method_call::h84dca6926ed068a02G1
  16:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  17:     0x7f88ec641d10 - middle::trans::expr::trans_unadjusted::h5f343dfce53924b0wn3
  18:     0x7f88ec601540 - middle::trans::expr::trans::h264fae1f71a954e6iE2
  19:     0x7f88ec605fa0 - middle::trans::controlflow::trans_for::h27cc5ffa9e390491KFY
  20:     0x7f88ec6451a0 - middle::trans::expr::trans_rvalue_stmt_unadjusted::h1cdb65f7a13d5bbasR3
  21:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  22:     0x7f88ec6003a0 - middle::trans::controlflow::trans_block::hc6e990dd6ea246cacsY
  23:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  24:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  25:     0x7f88ec6d01f0 - middle::trans::_match::trans_match_inner::h042c86355422f43em3h
  26:     0x7f88ec642c40 - middle::trans::expr::trans_rvalue_dps_unadjusted::hb435d260fcbf01f8YW3
  27:     0x7f88ec5fffa0 - middle::trans::expr::trans_into::he76810e0b15663femA2
  28:     0x7f88ec5ff460 - middle::trans::controlflow::trans_stmt_semi::hdaac58c7fef10d0firY
  29:     0x7f88ec5feb40 - middle::trans::controlflow::trans_stmt::h87b829ce2fd7b6fc1mY
  30:     0x7f88ec6003a0 - middle::trans::controlflow::trans_block::hc6e990dd6ea246cacsY
  31:     0x7f88ec6ae170 - middle::trans::base::trans_closure::hca82336ddae206bcvEe
  32:     0x7f88ec5f1620 - middle::trans::base::trans_fn::h80c61a61ef5e8177iQe
  33:     0x7f88ec5ecff0 - middle::trans::base::trans_item::h61aebb3e8aff1b7ei8e
  34:     0x7f88ec5ecff0 - middle::trans::base::trans_item::h61aebb3e8aff1b7ei8e
  35:     0x7f88ec6b8920 - middle::trans::base::trans_crate::h1ecf6b249bfcb51cJ2f
  36:     0x7f88ecacbb50 - driver::driver::phase_4_translate_to_llvm::hfecc798fa173c5bam7B
  37:     0x7f88ecac3780 - driver::driver::compile_input::h8608525c5b471524uJB
  38:     0x7f88ecb574d0 - driver::run_compiler::h69b3fcd2a164d565UyF
  39:     0x7f88ecb573e0 - driver::main_args::closure.141489
  40:     0x7f88ecb69fe0 - task::TaskBuilder<S>::try_future::closure.142658
  41:     0x7f88ecb69de0 - task::TaskBuilder<S>::spawn_internal::closure.142635
  42:     0x7f88ebf5a530 - task::spawn_opts::closure.8397
  43:     0x7f88ebc58660 - rust_try_inner
  44:     0x7f88ebc58650 - rust_try
  45:     0x7f88ebc000f0 - unwind::try::hd6387be7e7aa758afRd
  46:     0x7f88ebbfff50 - task::Task::run::hb83ab5c92810e88692c
  47:     0x7f88ebf5a290 - task::spawn_opts::closure.8343
  48:     0x7f88ebc01a60 - thread::thread_start::hd97e8e956809086bgod
  49:     0x7f88e69bf060 - start_thread
  50:     0x7f88eb8d1489 - __clone
  51:                0x0 - <unknown>


--- stderr
error: internal compiler error: can't dereference const of type [(&'static str,&'static str), .. 6]
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
