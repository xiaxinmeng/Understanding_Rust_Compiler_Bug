
$ RUST_BACKTRACE=1 cargo test --features all
   Compiling rusoto v0.14.1 (file:///home/adimarco/code/rusoto)
ERROR:rbml::reader: failed to find block with tag 272
ERROR:rbml::reader: failed to find block with tag 272
ERROR:rbml::reader: failed to find block with tag 272
ERROR:rbml::reader: failed to find block with tag 272
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', ../src/librbml/lib.rs:436
stack backtrace:
   1:     0x7f165f6a4570 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:     0x7f165f6b1c6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:     0x7f165f6b180c - std::panicking::default_hook::haac48fa641db8fa2
   4:     0x7f165f67679f - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:     0x7f1656c2ac3f - std::sys_common::unwind::begin_unwind::hd78480d5ec5c51b2
   6:     0x7f1656c28906 - rbml::reader::get_doc::hdb38d8b870a9741e
   7:     0x7f165d49282e - rustc_metadata::creader::CrateReader::register_crate::hdfd966766b932c68
   8:     0x7f165d497562 - rustc_metadata::creader::CrateReader::read_extension_crate::hb0997de4fb0918fb
   9:     0x7f165d49985e - rustc_metadata::creader::CrateReader::read_exported_macros::hf3b61431145aafb6
  10:     0x7f165d4bd0e8 - _<macro_import..MacroLoader<'a> as syntax..visit..Visitor<'v>>::visit_item::h668e7bfc1acab9b3
  11:     0x7f165d4bbeea - rustc_metadata::macro_import::read_macro_defs::h240a20dad60749cc
  12:     0x7f165fbda9ec - rustc_driver::driver::phase_2_configure_and_expand::h505e157de342d54f
  13:     0x7f165fbc109a - rustc_driver::driver::compile_input::h6491aaddd9e61258
  14:     0x7f165fba94e4 - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  15:     0x7f165fba6941 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  16:     0x7f165f6a1d0b - __rust_try
  17:     0x7f165f6a1c9d - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  18:     0x7f165fba718a - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  19:     0x7f165f6afe04 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  20:     0x7f165806e6f9 - start_thread
  21:     0x7f165f305b5c - clone
  22:                0x0 - <unknown>

Build failed, waiting for other jobs to finish...
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', ../src/librbml/lib.rs:436
stack backtrace:
   1:     0x7f5eab7bf570 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:     0x7f5eab7ccc6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:     0x7f5eab7cc80c - std::panicking::default_hook::haac48fa641db8fa2
   4:     0x7f5eab79179f - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:     0x7f5ea2d45c3f - std::sys_common::unwind::begin_unwind::hd78480d5ec5c51b2
   6:     0x7f5ea2d43906 - rbml::reader::get_doc::hdb38d8b870a9741e
   7:     0x7f5ea95ad82e - rustc_metadata::creader::CrateReader::register_crate::hdfd966766b932c68
   8:     0x7f5ea95b079f - rustc_metadata::creader::CrateReader::resolve_crate::h98decfaedbcfe12a
   9:     0x7f5ea95a9b41 - _<creader..LocalCrateReader<'a, 'b> as rustc..hir..intravisit..Visitor<'hir>>::visit_item::h2f4b84ef1536be92
  10:     0x7f5ea95be4be - rustc_metadata::creader::LocalCrateReader::read_crates::h7c2989bf6ef8974b
  11:     0x7f5eabd08c86 - rustc_driver::driver::phase_3_run_analysis_passes::h83da042ec4b8ea10
  12:     0x7f5eabcddf9f - rustc_driver::driver::compile_input::h6491aaddd9e61258
  13:     0x7f5eabcc44e4 - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  14:     0x7f5eabcc1941 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  15:     0x7f5eab7bcd0b - __rust_try
  16:     0x7f5eab7bcc9d - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  17:     0x7f5eabcc218a - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  18:     0x7f5eab7cae04 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  19:     0x7f5ea41896f9 - start_thread
  20:     0x7f5eab420b5c - clone
  21:                0x0 - <unknown>

error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', ../src/librbml/lib.rs:436
stack backtrace:
   1:     0x7f1f53753570 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:     0x7f1f53760c6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:     0x7f1f5376080c - std::panicking::default_hook::haac48fa641db8fa2
   4:     0x7f1f5372579f - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:     0x7f1f4acd9c3f - std::sys_common::unwind::begin_unwind::hd78480d5ec5c51b2
   6:     0x7f1f4acd7906 - rbml::reader::get_doc::hdb38d8b870a9741e
   7:     0x7f1f5154182e - rustc_metadata::creader::CrateReader::register_crate::hdfd966766b932c68
   8:     0x7f1f5154479f - rustc_metadata::creader::CrateReader::resolve_crate::h98decfaedbcfe12a
   9:     0x7f1f5153db41 - _<creader..LocalCrateReader<'a, 'b> as rustc..hir..intravisit..Visitor<'hir>>::visit_item::h2f4b84ef1536be92
  10:     0x7f1f515524be - rustc_metadata::creader::LocalCrateReader::read_crates::h7c2989bf6ef8974b
  11:     0x7f1f53c9cc86 - rustc_driver::driver::phase_3_run_analysis_passes::h83da042ec4b8ea10
  12:     0x7f1f53c71f9f - rustc_driver::driver::compile_input::h6491aaddd9e61258
  13:     0x7f1f53c584e4 - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  14:     0x7f1f53c55941 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  15:     0x7f1f53750d0b - __rust_try
  16:     0x7f1f53750c9d - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  17:     0x7f1f53c5618a - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  18:     0x7f1f5375ee04 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  19:     0x7f1f4c11d6f9 - start_thread
  20:     0x7f1f533b4b5c - clone
  21:                0x0 - <unknown>

error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', ../src/librbml/lib.rs:436
stack backtrace:
   1:     0x7f2df6386570 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:     0x7f2df6393c6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:     0x7f2df639380c - std::panicking::default_hook::haac48fa641db8fa2
   4:     0x7f2df635879f - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:     0x7f2ded90cc3f - std::sys_common::unwind::begin_unwind::hd78480d5ec5c51b2
   6:     0x7f2ded90a906 - rbml::reader::get_doc::hdb38d8b870a9741e
   7:     0x7f2df417482e - rustc_metadata::creader::CrateReader::register_crate::hdfd966766b932c68
   8:     0x7f2df417779f - rustc_metadata::creader::CrateReader::resolve_crate::h98decfaedbcfe12a
   9:     0x7f2df4170b41 - _<creader..LocalCrateReader<'a, 'b> as rustc..hir..intravisit..Visitor<'hir>>::visit_item::h2f4b84ef1536be92
  10:     0x7f2df41854be - rustc_metadata::creader::LocalCrateReader::read_crates::h7c2989bf6ef8974b
  11:     0x7f2df68cfc86 - rustc_driver::driver::phase_3_run_analysis_passes::h83da042ec4b8ea10
  12:     0x7f2df68a4f9f - rustc_driver::driver::compile_input::h6491aaddd9e61258
  13:     0x7f2df688b4e4 - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  14:     0x7f2df6888941 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  15:     0x7f2df6383d0b - __rust_try
  16:     0x7f2df6383c9d - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  17:     0x7f2df688918a - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  18:     0x7f2df6391e04 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  19:     0x7f2deed506f9 - start_thread
  20:     0x7f2df5fe7b5c - clone
  21:                0x0 - <unknown>
