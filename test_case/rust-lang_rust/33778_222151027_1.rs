
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 0 }', ../src/libcore/result.rs:746
stack backtrace:
   1:     0x7fada89d8570 - std::sys::backtrace::tracing::imp::write::h4c73fcd3363076f5
   2:     0x7fada89e5c6b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::h0422dbb3077e6747
   3:     0x7fada89e580c - std::panicking::default_hook::haac48fa641db8fa2
   4:     0x7fada89aa79f - std::sys_common::unwind::begin_unwind_inner::h39d40f52add53ef7
   5:     0x7fada89ac888 - std::sys_common::unwind::begin_unwind_fmt::h64c0ff793199cc1b
   6:     0x7fada89d5d81 - rust_begin_unwind
   7:     0x7fada8a2f85f - core::panicking::panic_fmt::h73bf9d7e8e891a73
   8:     0x7fad9ff89a0b - core::result::unwrap_failed::h82c81d60621f70bd
   9:     0x7fad9ff8997e - rbml::Doc::as_str_slice::hd88d17b04c3f2cca
  10:     0x7fada67e0a62 - rustc_metadata::decoder::maybe_get_crate_hash::h1c0939ce4d9ba236
  11:     0x7fada681401b - rustc_metadata::loader::Context::extract_one::h56453781fb65bffa
  12:     0x7fada680aada - rustc_metadata::loader::Context::find_library_crate::h30ea71d32623d713
  13:     0x7fada67f1f96 - rustc_metadata::loader::Context::load_library_crate::h7787c1fa9559d604
  14:     0x7fada67f0e1f - rustc_metadata::creader::CrateReader::resolve_crate::h98decfaedbcfe12a
  15:     0x7fada67eab41 - _<creader..LocalCrateReader<'a, 'b> as rustc..hir..intravisit..Visitor<'hir>>::visit_item::h2f4b84ef1536be92
  16:     0x7fada67ff4be - rustc_metadata::creader::LocalCrateReader::read_crates::h7c2989bf6ef8974b
  17:     0x7fada8f21c86 - rustc_driver::driver::phase_3_run_analysis_passes::h83da042ec4b8ea10
  18:     0x7fada8ef6f9f - rustc_driver::driver::compile_input::h6491aaddd9e61258
  19:     0x7fada8edd4e4 - rustc_driver::run_compiler::h80b2ba5e4d787c5f
  20:     0x7fada8eda941 - std::sys_common::unwind::try::try_fn::h34e27823ddd1d5e9
  21:     0x7fada89d5d0b - __rust_try
  22:     0x7fada89d5c9d - std::sys_common::unwind::inner_try::h9eebd8dc83f388a6
  23:     0x7fada8edb18a - _<F as std..boxed..FnBox<A>>::call_box::h3d5d78986dfac5b2
  24:     0x7fada89e3e04 - std::sys::thread::Thread::new::thread_start::h471ad90789353b5b
  25:     0x7fada13ca473 - start_thread
  26:     0x7fada864269c - clone
  27:                0x0 - <unknown>
