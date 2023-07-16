
$ rustc src/main.rs 
src/main.rs:3:12: 3:17 error: type `collections::string::String` does not implement any method in scope named `foo`
src/main.rs:3     string.foo()[0];
                         ^~~~~
error: internal compiler error: no type for node 17: expr 0 (id=17) in fcx 0x7f369a7ec488
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:     0x7f36a22cdd10 - sys::backtrace::write::hde9d0c7005312c09CQs
   2:     0x7f36a22f3a60 - failure::on_fail::h6fe6edb8db0347b4e3y
   3:     0x7f36a2259890 - rt::unwind::begin_unwind_inner::h59b1686c02dda4e3VHy
   4:     0x7f369d1e9e30 - rt::unwind::begin_unwind::h12162784926453586192
   5:     0x7f369d1ea6e0 - diagnostic::Handler::bug::haaef3c02788d02f80qF
   6:     0x7f36a05500a0 - session::Session::bug::hb63dd294b5253b83BWp
   7:     0x7f36a19c2b60 - check::FnCtxt<'a, 'tcx>::node_ty::hb6cac7314f203334Rzl
   8:     0x7f36a19d8090 - check::writeback::WritebackCx<'cx, 'tcx>::visit_node_id::h1d3427de52b65854pDb
   9:     0x7f36a19d4320 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_expr::h080cb55b8711012fptb
  10:     0x7f36a19d7f90 - check::writeback::WritebackCx<'cx, 'tcx>.Visitor<'v>::visit_block::h31a05e2e23ab63c1Eub
  11:     0x7f36a1a740d0 - check::check_bare_fn::h722855d499a8f8782sj
  12:     0x7f36a1a6b820 - check::check_item::h24b59d262c9736e5OLj
  13:     0x7f36a1b60310 - check_crate::unboxed_closure.39887
  14:     0x7f36a1b5af50 - check_crate::h47b99cd0e2ad3eb1amx
  15:     0x7f36a280f3e0 - driver::phase_3_run_analysis_passes::h19c1c734b891aff0Sva
  16:     0x7f36a27fd910 - driver::compile_input::h5b791fe2c3c1b976vba
  17:     0x7f36a28c49c0 - thunk::F.Invoke<A, R>::invoke::h13644216210312096233
  18:     0x7f36a28c3770 - rt::unwind::try::try_fn::h9904251112696932147
  19:     0x7f36a2357fa0 - rust_try_inner
  20:     0x7f36a2357f90 - rust_try
  21:     0x7f36a28c3ac0 - thunk::F.Invoke<A, R>::invoke::h8809310375033820670
  22:     0x7f36a22df890 - sys::thread::thread_start::h0ee022c3d5527b83IGv
  23:     0x7f369ca0e250 - start_thread
  24:     0x7f36a1f14219 - clone
  25:                0x0 - <unknown>
