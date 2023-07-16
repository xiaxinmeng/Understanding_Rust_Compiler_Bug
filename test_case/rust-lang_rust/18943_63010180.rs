
task 'rustc' panicked at 'assertion failed: bounds.is_none()', /build/rust-git/src/rust/src/librustc/metadata/encoder.rs:1231

stack backtrace:
   1:     0x7fdb42752c20 - rt::backtrace::imp::write::hcc244d480521f5e9Pdt
   2:     0x7fdb42755ca0 - <unknown>
   3:     0x7fdb42f13c80 - unwind::begin_unwind_inner::h5eadd82ce47d0284E9c
   4:     0x7fdb432c0680 - <unknown>
   5:     0x7fdb43a535d0 - <unknown>
   6:     0x7fdb43a58d70 - <unknown>
   7:     0x7fdb435066c0 - <unknown>
   8:     0x7fdb435066c0 - <unknown>
   9:     0x7fdb43a5ae20 - <unknown>
  10:     0x7fdb43a5f320 - <unknown>
  11:     0x7fdb43684470 - metadata::encoder::encode_metadata::h633a4478d051f03eSbw
  12:     0x7fdb43683c70 - middle::trans::base::write_metadata::h30a48bed88474a06zQi
  13:     0x7fdb43685950 - middle::trans::base::trans_crate::haaa13fd4751fa866GYi
  14:     0x7fdb43a9fe70 - driver::driver::phase_4_translate_to_llvm::h85e6560af237b5f1kkC
  15:     0x7fdb43a97470 - driver::driver::compile_input::h02996843b5f2a565eRB
  16:     0x7fdb43b1a800 - <unknown>
  17:     0x7fdb43b1a6f0 - <unknown>
  18:     0x7fdb432f3bc0 - <unknown>
  19:     0x7fdb432f39b0 - <unknown>
  20:     0x7fdb44344a30 - <unknown>
  21:     0x7fdb42f60e00 - <unknown>
  22:     0x7fdb42f60df0 - rust_try
  23:     0x7fdb42f11600 - unwind::try::h333ab25385d8e012mYc
  24:     0x7fdb42f11490 - task::Task::run::h2157d702c714dd0du4b
  25:     0x7fdb44344770 - <unknown>
  26:     0x7fdb42f12ca0 - <unknown>
  27:     0x7fdb3def3060 - start_thread
  28:     0x7fdb42be4489 - __clone
  29:                0x0 - <unknown>

