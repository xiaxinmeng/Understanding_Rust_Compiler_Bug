
$ RUST_BACKTRACE=1 rustc test.rs 
test.rs:5:9: 5:15 error: the type of this value must be known in this context
test.rs:5         arr[j] = 0.0f64;
                  ^~~~~~
error: internal compiler error: no type for node 46: expr j (id=46) in fcx 0x7fb2403ec488
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:     0x7fb2481e72d0 - sys::backtrace::write::h8532e701ef86014f4it
   2:     0x7fb24820cb00 - failure::on_fail::h7532e1f79d134d5dzvz
   3:     0x7fb2481721c0 - rt::unwind::begin_unwind_inner::h97b151606151d62deaz
   4:     0x7fb243046c60 - rt::unwind::begin_unwind::h15809447133099964284
   5:     0x7fb243047510 - diagnostic::Handler::bug::h8818b567cf47e6a0DLF
   6:     0x7fb246381cf0 - session::Session::bug::h251da16737c526c0lrq
   7:     0x7fb2477a7ec0 - check::FnCtxt<'a, 'tcx>::node_ty::h8b4513e3177b34b9FAl
   8:     0x7fb2477bd390 - check::writeback::WritebackCx<'cx, 'tcx>::visit_node_id::hadb4e437b164af77oDb
   9:     0x7fb2477b9620 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h773756089457ec03otb
  10:     0x7fb2477b9620 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h773756089457ec03otb
  11:     0x7fb2477bd290 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_block::h73bb9a8ed95e2890Dub
  12:     0x7fb24788db40 - check::check_bare_fn::h592fda8aa741d710Qtj
  13:     0x7fb247885170 - check::check_item::h9827005a9cde6421CMj
  14:     0x7fb247a25fe0 - check_crate::unboxed_closure.40162
  15:     0x7fb247a20c30 - check_crate::h19fb6dea5733566ajsx
  16:     0x7fb248738640 - driver::phase_3_run_analysis_passes::h46b1604d9f9f5633Tva
  17:     0x7fb248726ae0 - driver::compile_input::h68b8602933aad8d7wba
  18:     0x7fb2487f1eb0 - thunk::F.Invoke<A, R>::invoke::h18029802347644288836
  19:     0x7fb2487f0c60 - rt::unwind::try::try_fn::h6518866316425934196
  20:     0x7fb248273400 - rust_try_inner
  21:     0x7fb2482733f0 - rust_try
  22:     0x7fb2487f0fb0 - thunk::F.Invoke<A, R>::invoke::h15513809553472565307
  23:     0x7fb2481f8e40 - sys::thread::thread_start::h5ea7ba97235331d5a9v
  24:     0x7fb24286b250 - start_thread
  25:     0x7fb247e29219 - clone
  26:                0x0 - <unknown>
