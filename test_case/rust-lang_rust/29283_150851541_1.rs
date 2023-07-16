
bug.rs:2:1: 4:2 warning: enum is never used: `A`, #[warn(dead_code)] on by default
bug.rs:2 enum A{
bug.rs:3         B(C),
bug.rs:4 }
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'Unexpected type returned from struct_tail: A for ty=A', ../src/librustc_trans/trans/type_of.rs:385
stack backtrace:
   1:     0x7fd5e45263e0 - sys::backtrace::tracing::imp::write::h88824fa7cde382fcamt
   2:     0x7fd5e452ccb5 - panicking::log_panic::_<closure>::closure.39651
   3:     0x7fd5e452c725 - panicking::log_panic::h6e56f24ee89301beimx
   4:     0x7fd5e44f0063 - sys_common::unwind::begin_unwind_inner::h4469900167bcc3a4efs
   5:     0x7fd5e44f09c8 - sys_common::unwind::begin_unwind_fmt::h3d525738d0e295e8kes
   6:     0x7fd5e3a24252 - trans::type_of::in_memory_type_of::ha83e24ebd5c0460fNNN
   7:     0x7fd5e3a36c97 - trans::type_of::arg_type_of::h56c2a4c873c0a26fbMN
   8:     0x7fd5e3aff6ae - trans::type_of::type_of_rust_fn::h29e768c9f0be56885xN
   9:     0x7fd5e3a310fe - trans::declare::declare_rust_fn::hd7b899e52fe7c39aAEA
  10:     0x7fd5e3a5f345 - trans::base::register_fn::hfa00ae8d65b318b270i
  11:     0x7fd5e3a60e51 - trans::base::register_method::hf91d920ea3378723grj
  12:     0x7fd5e3a594fb - trans::base::get_item_val::hbd738babbb4d7e93ydj
  13:     0x7fd5e3a56595 - trans::base::trans_item::h4980eeed9cb665cdMRi
  14:     0x7fd5e3a63ba3 - trans::base::trans_crate::h9f7ce6cf717c6143JDj
  15:     0x7fd5e4a04dbc - driver::phase_4_translate_to_llvm::hce73529426bb2908EPa
  16:     0x7fd5e49fed41 - driver::phase_3_run_analysis_passes::_<closure>::closure.21915
  17:     0x7fd5e49df463 - middle::ty::context::_<impl>::create_and_enter::create_and_enter::h17188378283146851130
  18:     0x7fd5e49da45e - driver::phase_3_run_analysis_passes::h13798991786934394891
  19:     0x7fd5e49baca2 - driver::compile_input::h21f7caa6eac185d17ba
  20:     0x7fd5e4b11b8b - run_compiler::h0d0562bdd0cad39chqc
  21:     0x7fd5e4b0ec06 - sys_common::unwind::try::try_fn::try_fn::h8047687583817760752
  22:     0x7fd5e4524108 - __rust_try
  23:     0x7fd5e451815b - sys_common::unwind::try::inner_try::hcc921df9d6a17f8fMbs
  24:     0x7fd5e4b0ef54 - boxed::_<impl>::call_box::call_box::h1436226236130625411
  25:     0x7fd5e452b783 - sys::thread::_<impl>::new::thread_start::hea8939a5a657d2e7wEw
  26:     0x7fd5ddc07a50 - start_thread
  27:     0x7fd5e41c593c - clone
  28:                0x0 - <unknown>
