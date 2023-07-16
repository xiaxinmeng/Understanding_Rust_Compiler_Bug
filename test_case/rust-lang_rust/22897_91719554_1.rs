
$ RUST_BACKTRACE=1 rustc ./test.rs
./test.rs:2:5: 2:8 error: internal compiler error: cat_expr Errd
./test.rs:2     &[];
                ^~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:130

stack backtrace:
   1:        0x1106d3f77 - sys::backtrace::write::h799ab135045ca9ca1LC
   2:        0x110701e4d - panicking::on_panic::h9502e488a92fc8bcHAI
   3:        0x1106262de - rt::unwind::begin_unwind_inner::hf0537788ee16eadbiiI
   4:        0x10fdf85ae - rt::unwind::begin_unwind::h447731439410619827
   5:        0x10fdf855b - diagnostic::SpanHandler::span_bug::h3f51c1d819f22bcfjnB
   6:        0x10d5631ac - session::Session::span_bug::h460cdb4f2d9be26b5Xq
   7:        0x10cc93991 - check::regionck::visit_expr::h5f402212d9d0fff7PJd
   8:        0x10cc8e2e3 - check::regionck::Rcx<'a, 'tcx>::visit_fn_body::h9e50b5f998fa9b18amd
   9:        0x10cd2c76c - check::check_bare_fn::h05c77be534ada2cfUQn
  10:        0x10cd25ee2 - check::check_item::h9d89bedf29e4fe82F9n
  11:        0x10ce02676 - check_crate::closure.36302
  12:        0x10cdfcae7 - check_crate::h09aa7856a322fd44bwC
  13:        0x10cb1b355 - driver::phase_3_run_analysis_passes::h1af60e7c926ca39dgGa
  14:        0x10caffa85 - driver::compile_input::h27cf4320fd56e5cbQba
  15:        0x10cbbda15 - run_compiler::h93c68a2a3c26052bV4b
  16:        0x10cbbb1e2 - boxed::F.FnBox<A>::call_box::h4498816167135712709
  17:        0x10cbba6d7 - rt::unwind::try::try_fn::h2387014819904265058
  18:        0x11078bea8 - rust_try_inner
  19:        0x11078be95 - rust_try
  20:        0x10cbba9ca - boxed::F.FnBox<A>::call_box::h4850138824497903476
  21:        0x1106eb37d - sys::thread::create::thread_start::h42d6d904fce5365aihH
  22:     0x7fff97b50267 - _pthread_body
  23:     0x7fff97b501e4 - _pthread_start
