
   0:         0x67dfeb08 - backtrace::backtrace::dbghelp::trace::hd396a5624bee7ab4
                               at C:\msys64\home\alex\code\backtrace-rs/C:\msys64\home\alex\code\backtrace-rs\src\backtrace/dbghelp.rs:99
                           backtrace::backtrace::trace_unsynchronized::hf2dd82ace568fe9f
                               at C:\msys64\home\alex\code\backtrace-rs/C:\msys64\home\alex\code\backtrace-rs\src\backtrace/mod.rs:66
   1:         0x67dfe9c9 - backtrace::backtrace::trace::hde1671460dc308a4
                               at C:\msys64\home\alex\code\backtrace-rs/C:\msys64\home\alex\code\backtrace-rs\src\backtrace/mod.rs:53
   2:         0x67d8d5d8 - backtrace::capture::Backtrace::create::h1a8aa354e3fb0be1
                               at C:\msys64\home\alex\code\backtrace-rs/C:\msys64\home\alex\code\backtrace-rs\src/capture.rs:164
   3:         0x67d8d4c5 - backtrace::capture::Backtrace::new::ha19592734a1d7e0e
                               at C:\msys64\home\alex\code\backtrace-rs/C:\msys64\home\alex\code\backtrace-rs\src/capture.rs:128
   4:         0x67d8527c - pm::foo::h1e509f19291595a2
                               at C:\msys64\home\alex\code\wut/pm\src/lib.rs:3
   5:         0x67d83148 - core::ops::function::FnOnce::call_once::hc034a2591886bbd3
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libcore\ops/function.rs:232
   6:         0x67d82766 - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run::{{closure}}::h412d0ed1ab20a412
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:394
   7:         0x67d81db4 - proc_macro::bridge::client::run_client::{{closure}}::{{closure}}::h240b46d3e60e1d66
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:362
   8:         0x67d85db8 - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}}::he8b02e8e643901e2
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/scoped_cell.rs:79
   9:         0x67d865f2 - proc_macro::bridge::scoped_cell::ScopedCell<T>::replace::h506f21e840707659
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/scoped_cell.rs:74
  10:         0x67d85d95 - proc_macro::bridge::scoped_cell::ScopedCell<T>::set::hc549006b80555ddd
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/scoped_cell.rs:79
  11:         0x67d8256c - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::habdf288ae2ac6b64
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:310
  12:         0x67d82ee4 - std::thread::local::LocalKey<T>::try_with::hb7dcab13b0a81215
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd\thread/local.rs:263
  13:         0x67d82a0e - std::thread::local::LocalKey<T>::with::h6e17ed637ca840bf
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd\thread/local.rs:239
  14:         0x67d85636 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::h5b16ebd94254838d
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:310
  15:         0x67d81c95 - proc_macro::bridge::client::run_client::{{closure}}::h5bb0f9a769e901a6
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:355
  16:         0x67d828a6 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h575dcb772ab8c60e
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd/panic.rs:318
  17:         0x67d86d90 - std::panicking::try::do_call::h6beb5bf073c74cf8
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd/panicking.rs:331
  18:         0x67d86e5d - __rust_try
  19:         0x67d86cba - std::panicking::try::hbb9d78f95702cc2f
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd/panicking.rs:274
  20:         0x67d828e6 - std::panic::catch_unwind::hcfc29a4c0e696d4f
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libstd/panic.rs:394
  21:         0x67d81a7f - proc_macro::bridge::client::run_client::hb6fa1f767ba54e5b
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:354
  22:         0x67d8271d - proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run::h0c73b131837ea539
                               at /rustc/49cae55760da0a43428eba73abcb659bb70cf2e4\src\libproc_macro\bridge/client.rs:394
