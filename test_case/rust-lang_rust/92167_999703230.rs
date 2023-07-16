plain
    Checking tracing-subscriber v0.2.16
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0277]: the trait bound `HierarchicalLayer<fn() -> Stderr {stderr}>: __tracing_subscriber_Layer<Layered<EnvFilter, tracing_subscriber::Registry>>` is not satisfied
    |
    |
250 |     let subscriber = tracing_subscriber::Registry::default().with(filter).with(layer);
    |                                                                           ---- ^^^^^ the trait `__tracing_subscriber_Layer<Layered<EnvFilter, tracing_subscriber::Registry>>` is not implemented for `HierarchicalLayer<fn() -> Stderr {stderr}>`
    |                                                                           required by a bound introduced by this call
    |
    |
note: required by a bound in `tracing_subscriber::layer::SubscriberExt::with`
    |
    |
524 |         L: Layer<Self>,
    |            ^^^^^^^^^^^ required by this bound in `tracing_subscriber::layer::SubscriberExt::with`

error[E0277]: the trait bound `HierarchicalLayer<fn() -> Stderr {stderr}>: __tracing_subscriber_Layer<Layered<EnvFilter, tracing_subscriber::Registry>>` is not satisfied
    |
    |
251 |     tracing::subscriber::set_global_default(subscriber).unwrap();
    |     --------------------------------------- ^^^^^^^^^^ the trait `__tracing_subscriber_Layer<Layered<EnvFilter, tracing_subscriber::Registry>>` is not implemented for `HierarchicalLayer<fn() -> Stderr {stderr}>`
    |     required by a bound introduced by this call
    |
    |
    = note: required because of the requirements on the impl of `tracing::Subscriber` for `Layered<HierarchicalLayer<fn() -> Stderr {stderr}>, Layered<EnvFilter, tracing_subscriber::Registry>>`
note: required by a bound in `tracing::subscriber::set_global_default`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-0.1.28/src/subscriber.rs:41:8
41  |     S: Subscriber + Send + Sync + 'static,
41  |     S: Subscriber + Send + Sync + 'static,
    |        ^^^^^^^^^^ required by this bound in `tracing::subscriber::set_global_default`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustdoc` due to 2 previous errors
Build completed unsuccessfully in 0:02:55
