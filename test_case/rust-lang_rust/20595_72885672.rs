
Running `rustc src/lib.rs --crate-name nalgebra --crate-type lib -g -C metadata=a9022f5b54c1ddb2 -C extra-filename=-a9022f5b54c1ddb2 --out-dir /home/arturo/Code/rin/libs/nalgebra/target --emit=dep-info,link -L dependency=/home/arturo/Code/rin/libs/nalgebra/target -L dependency=/home/arturo/Code/rin/libs/nalgebra/target/deps --extern rustc-serialize=/home/arturo/Code/rin/libs/nalgebra/target/deps/librustc-serialize-285c9bed5802d3eb.rlib`
src/lib.rs:612:7: 612:19 error: internal compiler error: coherence failed to report ambiguity: cannot locate the impl of the trait `traits::structure::Mat<N, LV, LV>` for the type `<R as traits::geometry::RotationMatrix<N, LV, AV>>::Output`
src/lib.rs:612     r.to_rot_mat()
                     ^~~~~~~~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:     0x7fd616a5ae80 - sys::backtrace::write::h84a1347ec8a3d425Aru
   2:     0x7fd616a7e4a0 - failure::on_fail::h1cb2299eb5e255ae9GB
   3:     0x7fd6169e92d0 - rt::unwind::begin_unwind_inner::ha34a1c7f0630f466IlB
   4:     0x7fd613de2bf0 - rt::unwind::begin_unwind::h12804436560652988965
   5:     0x7fd613de2b80 - diagnostic::SpanHandler::span_bug::h7782d96edbc369daAME
   6:     0x7fd614b29890 - middle::traits::error_reporting::report_fulfillment_errors::h9d98ce1dd7e3fcdeGnN
   7:     0x7fd616126a20 - check::vtable::select_all_fcx_obligations_or_error::hc5e5e8e652f479efRvb
   8:     0x7fd6161d6680 - check::check_bare_fn::h306f8ecc226858bctAm
   9:     0x7fd6161cde70 - check::check_item::h7d555ba822408851KTm
  10:     0x7fd61629cd10 - check_crate::closure.34569
  11:     0x7fd616297850 - check_crate::h2469153404b647bauEA
  12:     0x7fd616feb710 - driver::phase_3_run_analysis_passes::h198ed79d6101094dtGa
  13:     0x7fd616fd22b0 - driver::compile_input::h08caf95be513bd6bBba
  14:     0x7fd61709cdd0 - run_compiler::hf1a36c26381910e09ac
  15:     0x7fd61709b460 - thunk::F.Invoke<A, R>::invoke::h6996223324138754953
  16:     0x7fd61709a390 - rt::unwind::try::try_fn::h16059966258548481805
  17:     0x7fd616ae9910 - rust_try_inner
  18:     0x7fd616ae9900 - rust_try
  19:     0x7fd61709a640 - thunk::F.Invoke<A, R>::invoke::h12998514134271589737
  20:     0x7fd616a6aca0 - sys::thread::thread_start::h756cedb2df1b4200Pnx
  21:     0x7fd610c66fe0 - start_thread
  22:     0x7fd61666b859 - __clone
  23:                0x0 - <unknown>

Could not compile `nalgebra`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name nalgebra --crate-type lib -g -C metadata=a9022f5b54c1ddb2 -C extra-filename=-a9022f5b54c1ddb2 --out-dir /home/arturo/Code/rin/libs/nalgebra/target --emit=dep-info,link -L dependency=/home/arturo/Code/rin/libs/nalgebra/target -L dependency=/home/arturo/Code/rin/libs/nalgebra/target/deps --extern rustc-serialize=/home/arturo/Code/rin/libs/nalgebra/target/deps/librustc-serialize-285c9bed5802d3eb.rlib` (status=101)
