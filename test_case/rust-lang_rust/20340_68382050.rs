
thread '<unnamed>' panicked at 'No option 'verbose' defined', /home/brian/dev/rust2/src/libgetopts/lib.rs:286
stack backtrace:
   1:     0x7f520b35fda0 - sys::backtrace::write::h7573bc5da646212ezat
   2:     0x7f520b37fba0 - failure::on_fail::h820f495fd7cf557asrz
   3:     0x7f520b2f5b00 - rt::unwind::begin_unwind_inner::h235e1612586c5a19w5y
   4:     0x7f520b2f6370 - rt::unwind::begin_unwind_fmt::h530121e87b555cdfb3y
   5:     0x7f52074c5270 - Matches::opt_vals::h4dd9249f598f859adPa
   6:     0x7f52074c5770 - Matches::opt_present::h62475234898191315Qa
   7:     0x7f520a9cec40 - version::hd4a1641352dfcccfHcc
   8:     0x7f520b9ad830 - main_args::h74df2535148fded4Obu
   9:     0x7f520b9ad700 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h15964369035831039695
  10:     0x7f520b9ac500 - rt::unwind::try::try_fn::h1827587574546456317
  11:     0x7f520b3e48c0 - rust_try_inner
  12:     0x7f520b3e48b0 - rust_try
  13:     0x7f520b9ac870 - thunk::F.Invoke<A,$u{20}R$GT$::invoke::h4649510423361328005
  14:     0x7f520b36f520 - sys::thread::thread_start::h9d0b1d2b3381ba14u3v
  15:     0x7f52064500c0 - start_thread
  16:     0x7f520af9aec9 - __clone
  17:                0x0 - <unknown>
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: ()', /home/brian/dev/rust2/src/libcore/result.rs:746
stack backtrace:
   1:     0x7f520b35fda0 - sys::backtrace::write::h7573bc5da646212ezat
   2:     0x7f520b37fba0 - failure::on_fail::h820f495fd7cf557asrz
   3:     0x7f520b2f5b00 - rt::unwind::begin_unwind_inner::h235e1612586c5a19w5y
   4:     0x7f520b2f6370 - rt::unwind::begin_unwind_fmt::h530121e87b555cdfb3y
   5:     0x7f520b37f870 - rust_begin_unwind
   6:     0x7f520b3c7be0 - panicking::panic_fmt::h8dfb1beb747ffed8lxl
   7:     0x7f520b9aaed0 - main::he83326c4d465a87bs7t
   8:     0x7f520b3e48c0 - rust_try_inner
   9:     0x7f520b3e48b0 - rust_try
  10:     0x7f520b3813b0 - rt::lang_start::hd6228bdcc3cebbe2flz
  11:     0x7f520aec1dd0 - __libc_start_main
  12:     0x7f520c0a3750 - <unknown>
  13:                0x0 - <unknown>
