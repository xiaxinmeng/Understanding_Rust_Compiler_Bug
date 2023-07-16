
stack backtrace:
   1:         0x711bd332 - sys::backtrace::write::h022859084c97c7d8kss
   2:         0x711c7a03 - rt::unwind::register::hf6cf68ec3d65cb61gZv
   3:         0x71185274 - rt::unwind::begin_unwind_inner::ha0de919eeab7aca9qWv
   4:         0x71185dd7 - rt::unwind::begin_unwind_fmt::hf37080597d178e4bwVv
   5:         0x711c7670 - rust_begin_unwind
   6:         0x711e0732 - panicking::panic_fmt::h705ebd4b98819809PJy
   7:           0x402d6d
   8:           0x4277a9
   9:           0x422aae
  10:           0x414e7a
  11:           0x40af03
  12:           0x44a4c0 - main
  13:         0x6d7a91ad - Bencher::ns_per_iter::he7122ff52105191f77b
  14:         0x6d7a70bd - Bencher::ns_per_iter::he7122ff52105191f77b
  15:         0x71201d0c - rust_try
  16:         0x71201ce9 - rust_try
  17:         0x6d7a737e - Bencher::ns_per_iter::he7122ff52105191f77b
  18:         0x711c5936 - sys::process::Command::cwd::hd77e7bf8cfdcbde1DGu
  19:         0x773f59cd - BaseThreadInitThunk


failures:
    [run-pass] run-pass/process-remove-from-env.rs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', C:/msys64/home/rust/src/compiletest/compiletest.rs:256
stack backtrace:
   1:         0x711bd332 - sys::backtrace::write::h022859084c97c7d8kss
   2:         0x711c7c17 - rt::unwind::register::hf6cf68ec3d65cb61gZv
   3:         0x71185274 - rt::unwind::begin_unwind_inner::ha0de919eeab7aca9qWv
   4:           0x4039f5
   5:           0x4417bb
   6:         0x71201d0c - rust_try
   7:         0x71201ce9 - rust_try
   8:         0x711c899a - rt::lang_start::hf4cd440969ce39bd98v
   9:           0x4013e8
  10:           0x40151b
  11:         0x773f59cd - BaseThreadInitThunk
