
$ RUST_BACKTRACE=1 rustc compiler_error.rs 
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'path not fully resolved: PathResolution { base_def: DefTyParam(TypeSpace, 0, DefId { krate: 0, node: 19 }, "D"(63)), last_private: LastMod(AllPublic), depth: 1 }', /build/rust/src/rustc-1.1.0/src/librustc/middle/def.rs:81

stack backtrace:
   1:     0x7f0cec9c92d9 - sys::backtrace::write::h534114ba1b06b93294r
   2:     0x7f0cec9d11f9 - panicking::on_panic::h4e97afe0febd2c7biJw
   3:     0x7f0cec991ff2 - rt::unwind::begin_unwind_inner::hfe7dd1cef83ccc0fsow
   4:     0x7f0cec992d87 - rt::unwind::begin_unwind_fmt::h97b246841383a432ynw
   5:     0x7f0cea8364a6 - middle::def::PathResolution::full_def::h6662dd4d69d7f710mjm
   6:     0x7f0ceb4ffb8e - PrivacyVisitor<'a, 'tcx>::check_path::he5062fdd97c5ddddwbb
   7:     0x7f0ceb500c87 - PrivacyVisitor<'a, 'tcx>.Visitor<'v>::visit_path::h63eb4ef2b9ae8266Pzb
   8:     0x7f0ceb500b2f - PrivacyVisitor<'a, 'tcx>.Visitor<'v>::visit_item::hef87e41e5698d047akb
   9:     0x7f0ceb50751b - check_crate::h029a1fe227fad1e5v2b
  10:     0x7f0cecf0f2d8 - driver::phase_3_run_analysis_passes::h7109deb2f0a65b9dtGa
  11:     0x7f0cecef2dec - driver::compile_input::hd8975b759aec0d60Qba
  12:     0x7f0cecfa90e1 - run_compiler::h3b0c3beaef2163aa75b
  13:     0x7f0cecfa6932 - boxed::F.FnBox<A>::call_box::h5202619178778601284
  14:     0x7f0cecfa5ef9 - rt::unwind::try::try_fn::h4053862560305393821
  15:     0x7f0ceca403d8 - rust_try_inner
  16:     0x7f0ceca403c5 - rust_try
  17:     0x7f0cecfa6194 - boxed::F.FnBox<A>::call_box::h4599430653034051152
  18:     0x7f0cec9cff91 - sys::thread::Thread::new::thread_start::h089e987aa4c0e52dzvv
  19:     0x7f0ce7521353 - start_thread
  20:     0x7f0cec621bfc - __clone
  21:                0x0 - <unknown>

