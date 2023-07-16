 javascript
$ rustc --version
rustc 1.5.0 (3d7cd77e4 2015-12-04)

$ rustc -g file_embed.rs 

$ RUST_BACKTRACE=1 ./file_embed
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 9, message: "Bad file descriptor" } }', ../src/libcore/result.rs:738
stack backtrace:
   1:        0x100ff9b88 - sys::backtrace::tracing::imp::write::h06df28c2a8d245f9t9s
   2:        0x100ffb1ef - panicking::log_panic::_<closure>::closure.39451
   3:        0x100ffac92 - panicking::log_panic::h9348b1b0af36ae6aCYw
   4:        0x100ff52c6 - sys_common::unwind::begin_unwind_inner::h005d402245ebd8560cs
   5:        0x100ff53fe - sys_common::unwind::begin_unwind_fmt::h440d2eb18c439fa86bs
   6:        0x100ff9217 - rust_begin_unwind
   7:        0x1010166b0 - panicking::panic_fmt::h4c8d12e3c05f3b8cZEK
   8:        0x100ff3566 - result::_<impl>::unwrap::unwrap::h5984419141872088150
   9:        0x100ff2a3d - write_cpp::hf673fd381a899287Gca
  10:        0x100ff4340 - main::h35317f0a7a25286cpia
  11:        0x100ffaa12 - sys_common::unwind::try::try_fn::h4733637608193038088
  12:        0x100ff9058 - __rust_try
  13:        0x100ffa8b9 - rt::lang_start::h44a8548d0ff91511KVw
  14:        0x100ff4789 - main
