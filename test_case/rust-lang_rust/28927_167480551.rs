
mindTree:dsp-chain Mitch$ RUST_BACKTRACE=1 cargo doc
 Documenting dsp-chain v0.10.0 (file:///Users/Mitch/Programming/Rust/dsp-chain)
thread '<unnamed>' panicked at 'Unexpected empty destination: ["dsp", "daggy"]', ../src/librustdoc/html/render.rs:1184
stack backtrace:
   1:        0x111601058 - sys::backtrace::tracing::imp::write::h06df28c2a8d245f9t9s
   2:        0x11160725f - panicking::log_panic::_<closure>::closure.39451
   3:        0x111606d02 - panicking::log_panic::h9348b1b0af36ae6aCYw
   4:        0x1115c8c86 - sys_common::unwind::begin_unwind_inner::h005d402245ebd8560cs
   5:        0x1115c962e - sys_common::unwind::begin_unwind_fmt::h440d2eb18c439fa86bs
   6:        0x10d85c166 - html::render::run::h42919733a08147f6xsn
   7:        0x10d8df047 - main_args::h9f2afc62a54ae9fbE2t
   8:        0x10d8da9f8 - sys_common::unwind::try::try_fn::try_fn::h7212718370687295260
   9:        0x1115fec08 - __rust_try
  10:        0x1115f2f8e - sys_common::unwind::try::inner_try::hcc3a84c80e36120fy9r
  11:        0x10d8daebe - boxed::_<impl>::call_box::call_box::h14966036215268722962
  12:        0x1116065ed - sys::thread::_<impl>::new::thread_start::h5776bf5f72f331a7Rvw
  13:     0x7fff889bf059 - _pthread_body
  14:     0x7fff889befd6 - _pthread_start
Could not document `dsp-chain`.

