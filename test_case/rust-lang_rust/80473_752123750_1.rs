log
struct 0x137e89b30 dropped
with [] fields
   0: backtrace::backtrace::libunwind::trace
             at /Users/jack/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/backtrace-0.3.55/src/backtrace/libunwind.rs:90:5
      backtrace::backtrace::trace_unsynchronized
             at /Users/jack/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/backtrace-0.3.55/src/backtrace/mod.rs:66:5
   1: backtrace::backtrace::trace
             at /Users/jack/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/backtrace-0.3.55/src/backtrace/mod.rs:53:14
   2: backtrace::capture::Backtrace::create
             at /Users/jack/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/backtrace-0.3.55/src/capture.rs:176:9
   3: backtrace::capture::Backtrace::new
             at /Users/jack/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/backtrace-0.3.55/src/capture.rs:140:22
   4: <fast_json_impl::structs::Struct as core::ops::drop::Drop>::drop
             at fast_json/fast_json_impl/src/structs.rs:60:18
   5: core::ptr::drop_in_place
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:179:1
   6: alloc::sync::Arc<T>::drop_slow
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/sync.rs:951:18
   7: <alloc::sync::Arc<T> as core::ops::drop::Drop>::drop
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/sync.rs:1471:13
   8: core::ptr::drop_in_place
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:179:1
   9: core::ptr::drop_in_place
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:179:1
  10: fast_json_impl::structs::StructRegistry::add_struct
             at fast_json/fast_json_impl/src/structs.rs:158:64
  11: fast_json_impl::fast_json
             at fast_json/fast_json_impl/src/lib.rs:27:5
  12: fast_json::fast_json
             at fast_json/src/lib.rs:5:5
  13: core::ops::function::FnOnce::call_once
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227:5
  14: proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run::{{closure}}
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:410:40
  15: proc_macro::bridge::client::run_client::{{closure}}::{{closure}}
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:377:26
  16: proc_macro::bridge::scoped_cell::ScopedCell<T>::set::{{closure}}
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:80:33
  17: proc_macro::bridge::scoped_cell::ScopedCell<T>::replace
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:75:9
  18: proc_macro::bridge::scoped_cell::ScopedCell<T>::set
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/scoped_cell.rs:80:9
  19: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325:35
  20: std::thread::local::LocalKey<T>::try_with
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:272:16
  21: std::thread::local::LocalKey<T>::with
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/local.rs:248:9
  22: proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:325:9
  23: proc_macro::bridge::client::run_client::{{closure}}
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:370:9
  24: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panic.rs:322:9
  25: std::panicking::try::do_call
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:379:40
  26: ___rust_try
  27: std::panicking::try
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panicking.rs:343:19
  28: std::panic::catch_unwind
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/panic.rs:396:14
  29: proc_macro::bridge::client::run_client
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:369:5
  30: proc_macro::bridge::client::Client<fn(proc_macro::TokenStream) .> proc_macro::TokenStream>::expand1::run
             at /Users/jack/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:410:13
  31: proc_macro::bridge::server::run_server
  32: <rustc_expand::proc_macro::ProcMacroDerive as rustc_expand::base::MultiItemModifier>::expand
  33: rustc_expand::expand::MacroExpander::fully_expand_fragment
  34: rustc_expand::expand::MacroExpander::expand_crate
  35: rustc_session::utils::<impl rustc_session::session::Session>::time
  36: rustc_interface::passes::configure_and_expand_inner
  37: rustc_interface::passes::configure_and_expand::{{closure}}
  38: rustc_data_structures::box_region::PinnedGenerator<I,A,R>::new
  39: rustc_interface::passes::configure_and_expand
  40: rustc_interface::queries::Queries::expansion
  41: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter
  42: rustc_span::with_source_map
  43: rustc_interface::interface::create_compiler_and_run
  44: scoped_tls::ScopedKey<T>::set
  45: std::sys_common::backtrace::__rust_begin_short_backtrace
  46: core::ops::function::FnOnce::call_once{{vtable.shim}}
  47: std::sys::unix::thread::Thread::new::thread_start
  48: _pthread_kill
