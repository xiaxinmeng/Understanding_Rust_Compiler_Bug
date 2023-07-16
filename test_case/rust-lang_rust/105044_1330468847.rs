
warning: Error finalizing incremental compilation session directory `/data00/home/liujie.roger/rust/git/volo/target/debug/incremental/hello_grpc_server-2r9arx04lojyo/s-gfv6wd3lvx-9bb9z2-working`: No such file or directory (os error 2)

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DropGlue(DefId(2:2801 ~ core[bc14]::ptr::drop_in_place), Some(volo_grpc::codegen::hyper::proto::h1::dispatch::Server<volo::service::Tower<volo_grpc::server::meta::MetaService<volo_grpc::server::Router>, [closure@volo_grpc::server::Server<volo::layer::Identity>::run<volo::net::Address>::{closure#0}::{closure#0}], volo_grpc::context::ServerContext, http::request::Request<volo_grpc::codegen::hyper::body::body::Body>>, volo_grpc::codegen::hyper::body::body::Body>)) (after phase change to runtime-optimized) at bb0[0]:
                                Field projection `(*_1).field[0]` specified type `std::pin::Pin<std::boxed::Box<std::option::Option<[async block@<volo::service::Tower<volo_grpc::server::meta::MetaService<volo_grpc::server::Router>, [closure@volo_grpc::server::Server<volo::layer::Identity>::run<volo::net::Address>::{closure#0}::{closure#0}], volo_grpc::context::ServerContext, http::request::Request<volo_grpc::codegen::hyper::body::body::Body>> as tower_service::Service<http::request::Request<volo_grpc::codegen::hyper::body::body::Body>>>::call::{closure#0}]>>>`, but actual type is `std::pin::Pin<std::boxed::Box<std::option::Option<<volo::service::Tower<volo_grpc::server::meta::MetaService<volo_grpc::server::Router>, [closure@volo_grpc::server::Server<volo::layer::Identity>::run<volo::net::Address>::{closure#0}::{closure#0}], volo_grpc::context::ServerContext, http::request::Request<volo_grpc::codegen::hyper::body::body::Body>> as volo_grpc::codegen::hyper::service::http::HttpService<volo_grpc::codegen::hyper::body::body::Body>>::Future>>>`
   --> /data00/home/liujie.roger/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:490:1
    |
490 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:229:30

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1609:13
stack backtrace:
   0:     0x7fd67761d20a - std::backtrace_rs::backtrace::libunwind::trace::h5d63982369f255be
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd67761d20a - std::backtrace_rs::backtrace::trace_unsynchronized::h3f0ca420a5fce959
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd67761d20a - std::sys_common::backtrace::_print_fmt::hb06d2f24224d5008
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fd67761d20a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3ab5a4a02080cf95
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fd677b2212e - core::fmt::write::h7d275b25684d2f4b
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fd6776114d5 - std::io::Write::write_fmt::habd0270971df3efd
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/io/mod.rs:1682:15
   6:     0x7fd67761cfd5 - std::sys_common::backtrace::_print::h61bb07d45642a6b0
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fd67761cfd5 - std::sys_common::backtrace::print::h5d41efe8a7680c9e
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fd67761f33f - std::panicking::default_hook::{{closure}}::he45a8add8cd6c157
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/panicking.rs:267:22
   9:     0x7fd67761f07b - std::panicking::default_hook::h5bdfb218ae8ccf1a
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/panicking.rs:286:9
  10:     0x7fd676864e54 - rustc_driver[9b4589fbd0ca3010]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd67761fb3d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc626e6f5815a4649
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/alloc/src/boxed.rs:2032:9
  12:     0x7fd67761fb3d - std::panicking::rust_panic_with_hook::haa2a8406479dd255
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/panicking.rs:692:13
  13:     0x7fd67689d8d1 - std[85b9d86509865b2b]::panicking::begin_panic::<rustc_errors[7d36137f8dd089c7]::ExplicitBug>::{closure#0}
  14:     0x7fd67689c196 - std[85b9d86509865b2b]::sys_common::backtrace::__rust_end_short_backtrace::<std[85b9d86509865b2b]::panicking::begin_panic<rustc_errors[7d36137f8dd089c7]::ExplicitBug>::{closure#0}, !>
  15:     0x7fd676894816 - std[85b9d86509865b2b]::panicking::begin_panic::<rustc_errors[7d36137f8dd089c7]::ExplicitBug>
  16:     0x7fd676898026 - std[85b9d86509865b2b]::panic::panic_any::<rustc_errors[7d36137f8dd089c7]::ExplicitBug>
  17:     0x7fd675d7fe86 - <rustc_errors[7d36137f8dd089c7]::HandlerInner>::flush_delayed::<alloc[edfeea1ed307d55e]::vec::Vec<rustc_errors[7d36137f8dd089c7]::diagnostic::Diagnostic>, &str>
  18:     0x7fd675d7f18b - <rustc_errors[7d36137f8dd089c7]::HandlerInner as core[bc14861035b9f032]::ops::drop::Drop>::drop
  19:     0x7fd675ad2a7e - core[bc14861035b9f032]::ptr::drop_in_place::<rustc_session[74924a319b9741d3]::parse::ParseSess>
  20:     0x7fd675ac373f - core[bc14861035b9f032]::ptr::drop_in_place::<rustc_session[74924a319b9741d3]::session::Session>
  21:     0x7fd675ac3470 - core[bc14861035b9f032]::ptr::drop_in_place::<rustc_interface[4869d893f0e6a446]::interface::Compiler>
  22:     0x7fd675ac257e - rustc_span[fe585761646e9245]::with_source_map::<core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>, rustc_interface[4869d893f0e6a446]::interface::run_compiler<core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>, rustc_driver[9b4589fbd0ca3010]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  23:     0x7fd675ac1eb5 - <scoped_tls[88353d2f7a38210b]::ScopedKey<rustc_span[fe585761646e9245]::SessionGlobals>>::set::<rustc_interface[4869d893f0e6a446]::interface::run_compiler<core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>, rustc_driver[9b4589fbd0ca3010]::run_compiler::{closure#1}>::{closure#0}, core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>>
  24:     0x7fd675ac14a2 - std[85b9d86509865b2b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4869d893f0e6a446]::util::run_in_thread_pool_with_globals<rustc_interface[4869d893f0e6a446]::interface::run_compiler<core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>, rustc_driver[9b4589fbd0ca3010]::run_compiler::{closure#1}>::{closure#0}, core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>>
  25:     0x7fd675ac11b8 - <<std[85b9d86509865b2b]::thread::Builder>::spawn_unchecked_<rustc_interface[4869d893f0e6a446]::util::run_in_thread_pool_with_globals<rustc_interface[4869d893f0e6a446]::interface::run_compiler<core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>, rustc_driver[9b4589fbd0ca3010]::run_compiler::{closure#1}>::{closure#0}, core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[bc14861035b9f032]::result::Result<(), rustc_errors[7d36137f8dd089c7]::ErrorGuaranteed>>::{closure#1} as core[bc14861035b9f032]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7fd677626d73 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h87f315d78631c89a
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/alloc/src/boxed.rs:2000:9
  27:     0x7fd677626d73 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h44e64edef3723eae
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/alloc/src/boxed.rs:2000:9
  28:     0x7fd677626d73 - std::sys::unix::thread::Thread::new::thread_start::h6b1af1d92f601469
                               at /rustc/2585bcea0bc2a9c42a4be2c1eba5c61137f2b167/library/std/src/sys/unix/thread.rs:108:17
  29:     0x7fd672fcd4a4 - start_thread
                               at /build/glibc-77giwP/glibc-2.24/nptl/pthread_create.c:456
  30:     0x7fd672d0fd0f - __GI___clone
                               at /build/glibc-77giwP/glibc-2.24/misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:97
  31:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (2585bcea0 2022-11-28) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
