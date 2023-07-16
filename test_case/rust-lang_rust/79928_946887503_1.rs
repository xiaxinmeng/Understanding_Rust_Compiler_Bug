
thread 'reqwest-internal-sync-core' panicked at 'called `Option::unwrap()` on a `None` value', /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:602:65
stack backtrace:
   0: rust_begin_unwind
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/panicking.rs:515:5
   1: core::panicking::panic_fmt
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/panicking.rs:50:5
   3: core::option::Option<T>::unwrap
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/option.rs:722:21
   4: security_framework::secure_transport::SslContext::enabled_ciphers::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:602:39
   5: core::iter::adapters::map::map_fold::{{closure}}
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/iter/adapters/map.rs:82:28
   6: core::iter::traits::iterator::Iterator::fold
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/iter/traits/iterator.rs:2174:21
   7: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/iter/adapters/map.rs:122:9
   8: core::iter::traits::iterator::Iterator::for_each
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/iter/traits/iterator.rs:737:9
   9: <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<T,I>>::spec_extend
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/alloc/src/vec/spec_extend.rs:40:17
  10: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<T,I>>::from_iter
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/alloc/src/vec/spec_from_iter_nested.rs:56:9
  11: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/alloc/src/vec/spec_from_iter.rs:33:9
  12: <alloc::vec::Vec<T> as core::iter::traits::collect::FromIterator<T>>::from_iter
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/alloc/src/vec/mod.rs:2453:9
  13: core::iter::traits::iterator::Iterator::collect
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/iter/traits/iterator.rs:1749:9
  14: security_framework::secure_transport::SslContext::enabled_ciphers
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:602:16
  15: security_framework::secure_transport::ClientBuilder::configure_ciphers
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:1277:18
  16: security_framework::secure_transport::ClientBuilder::ctx_into_stream
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:1231:14
  17: security_framework::secure_transport::ClientBuilder::handshake_inner
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:1245:26
  18: security_framework::secure_transport::ClientBuilder::handshake2
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/security-framework-0.1.16/src/secure_transport.rs:1198:9
  19: native_tls::imp::TlsConnector::connect_inner
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/native-tls-0.1.5/src/imp/security_framework.rs:346:29
  20: native_tls::imp::TlsConnector::connect
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/native-tls-0.1.5/src/imp/security_framework.rs:318:9
  21: native_tls::TlsConnector::connect
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/native-tls-0.1.5/src/lib.rs:411:22
  22: <native_tls::TlsConnector as tokio_tls::TlsConnectorExt>::connect_async
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-tls-0.1.4/src/lib.rs:188:29
  23: <hyper_tls::client::HttpsConnector<T> as tokio_service::Service>::call::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/hyper-tls-0.1.4/src/client.rs:99:21
  24: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll::{{closure}}::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:34:21
  25: core::result::Result<T,E>::map
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/result.rs:748:25
  26: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:33:13
  27: futures::future::chain::Chain<A,B,C>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/chain.rs:39:15
  28: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:32:9
  29: <alloc::boxed::Box<F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/mod.rs:113:13
  30: <hyper_tls::client::HttpsConnecting<T> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/hyper-tls-0.1.4/src/client.rs:127:9
  31: <futures::future::map::Map<A,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/map.rs:30:23
  32: <alloc::boxed::Box<F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/mod.rs:113:13
  33: <futures::future::from_err::FromErr<A,E> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/from_err.rs:29:23
  34: futures::future::chain::Chain<A,B,C>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/chain.rs:26:23
  35: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:32:9
  36: futures::future::chain::Chain<A,B,C>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/chain.rs:26:23
  37: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:32:9
  38: <futures::future::lazy::Lazy<F,R> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/lazy.rs:82:9
  39: <futures::future::select::Select<A,B> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/select.rs:52:31
  40: <futures::future::map::Map<A,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/map.rs:30:23
  41: <futures::future::map_err::MapErr<A,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/map_err.rs:30:23
  42: futures::future::chain::Chain<A,B,C>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/chain.rs:26:23
  43: <futures::future::and_then::AndThen<A,B,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/and_then.rs:32:9
  44: <alloc::boxed::Box<F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/mod.rs:113:13
  45: <hyper::client::RetryableSendRequest<C,B> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/hyper-0.11.27/src/client/mod.rs:372:19
  46: <alloc::boxed::Box<F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/mod.rs:113:13
  47: <hyper::client::FutureResponse as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/hyper-0.11.27/src/client/mod.rs:349:9
  48: <reqwest::async_impl::client::PendingRequest as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/async_impl/client.rs:507:35
  49: <reqwest::async_impl::client::Pending as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/async_impl/client.rs:495:51
  50: futures::future::chain::Chain<A,B,C>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/chain.rs:26:23
  51: <futures::future::then::Then<A,B,F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/then.rs:32:9
  52: <alloc::boxed::Box<F> as futures::future::Future>::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/future/mod.rs:113:13
  53: futures::task_impl::Spawn<T>::poll_future_notify::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/mod.rs:329:45
  54: futures::task_impl::Spawn<T>::enter::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/mod.rs:399:27
  55: futures::task_impl::std::set
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/std/mod.rs:86:13
  56: futures::task_impl::Spawn<T>::enter
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/mod.rs:399:9
  57: futures::task_impl::Spawn<T>::poll_fn_notify
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/mod.rs:291:9
  58: futures::task_impl::Spawn<T>::poll_future_notify
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/futures-0.1.31/src/task_impl/mod.rs:329:9
  59: tokio_current_thread::scheduler::Scheduled<U>::tick
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/scheduler.rs:351:25
  60: tokio_current_thread::scheduler::Scheduler<U>::tick::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/scheduler.rs:330:47
  61: tokio_current_thread::Borrow<U>::enter::{{closure}}::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:800:40
  62: tokio_current_thread::CurrentRunner::set_spawn
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:841:9
  63: tokio_current_thread::Borrow<U>::enter::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:800:13
  64: std::thread::local::LocalKey<T>::try_with
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/thread/local.rs:399:16
  65: std::thread::local::LocalKey<T>::with
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/thread/local.rs:375:9
  66: tokio_current_thread::Borrow<U>::enter
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:798:9
  67: tokio_current_thread::scheduler::Scheduler<U>::tick
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/scheduler.rs:330:24
  68: tokio_current_thread::Entered<P>::tick
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:624:9
  69: tokio_current_thread::Entered<P>::turn
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-current-thread-0.1.7/src/lib.rs:551:22
  70: tokio_core::reactor::Core::poll::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:299:25
  71: scoped_tls::ScopedKey<T>::set
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/scoped-tls-0.1.2/src/lib.rs:155:9
  72: tokio_core::reactor::Core::poll::{{closure}}::{{closure}}::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:298:21
  73: tokio_timer::timer::handle::with_default
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-timer-0.2.13/src/timer/handle.rs:74:5
  74: tokio_core::reactor::Core::poll::{{closure}}::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:276:17
  75: tokio_executor::global::with_default::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-executor-0.1.10/src/global.rs:221:9
  76: std::thread::local::LocalKey<T>::try_with
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/thread/local.rs:399:16
  77: std::thread::local::LocalKey<T>::with
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/thread/local.rs:375:9
  78: tokio_executor::global::with_default
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-executor-0.1.10/src/global.rs:190:5
  79: tokio_core::reactor::Core::poll::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:275:13
  80: tokio_reactor::with_default
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-reactor-0.1.12/src/lib.rs:220:5
  81: tokio_core::reactor::Core::poll
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:274:9
  82: tokio_core::reactor::Core::run
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/tokio-core-0.1.18/src/reactor/mod.rs:249:13
  83: reqwest::client::ClientHandle::new::{{closure}}
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/client.rs:447:21
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'core thread panicked', /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/client.rs:487:17
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/std/src/panicking.rs:541:12
   1: reqwest::client::ClientHandle::execute_request
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/client.rs:487:17
   2: reqwest::client::Client::execute
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/client.rs:367:9
   3: reqwest::request::RequestBuilder::send
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/request.rs:393:9
   4: reqwest::get
             at /Users/zhouliuyi/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/reqwest-0.8.8/src/lib.rs:243:5
   5: http::main
             at ./src/main.rs:4:25
   6: core::ops::function::FnOnce::call_once
             at /rustc/c8dfcfe046a7680554bf4eb612bad840e7631c4b/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
