
thread 'main' panicked at 'assertion failed: sentinel == STR_SENTINEL', /rustc/70b3681bf621bc0de91ffab711b2350068b4c466/compiler/rustc_serialize/src/opaque.rs:656:9
stack backtrace:
   0: rust_begin_unwind
             at /rustc/70b3681bf621bc0de91ffab711b2350068b4c466/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/70b3681bf621bc0de91ffab711b2350068b4c466/library/core/src/panicking.rs:142:14
   2: core::panicking::panic
             at /rustc/70b3681bf621bc0de91ffab711b2350068b4c466/library/core/src/panicking.rs:48:5
   3: <rustc_serialize::opaque::Decoder as rustc_serialize::serialize::Decoder>::read_str
   4: <alloc::string::String as rustc_serialize::serialize::Decodable<D>>::decode
   5: <std::path::PathBuf as rustc_serialize::serialize::Decodable<D>>::decode
   6: rustc_span::_DERIVE_rustc_serialize_Decodable_D_FOR_RealFileName::<impl rustc_serialize::serialize::Decodable<__D> for rustc_span::RealFileName>::decode
   7: rustc_span::_DERIVE_rustc_serialize_Decodable_D_FOR_FileName::<impl rustc_serialize::serialize::Decodable<__D> for rustc_span::FileName>::decode
   8: <rustc_span::SourceFile as rustc_serialize::serialize::Decodable<D>>::decode
   9: rust_out::main
  10: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
