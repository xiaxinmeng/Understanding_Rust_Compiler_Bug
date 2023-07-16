
error[E0271]: type mismatch resolving `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item == std::result::Result<(), futures_channel::mpsc::SendError>`
   --> src/main.rs:14:13
    |
5   | fn create_stream() -> impl futures::TryStream<Ok=(), Error=mpsc::SendError> { // ERROR
    |                       ----------------------------------------------------- the expected opaque type
...
14  |     let _ = futures::executor::block_on(foo);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected associated type, found enum `std::result::Result`
    |
   ::: /Users/ekuber/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-executor-0.3.4/src/local_pool.rs:315:20
    |
315 | pub fn block_on<F: Future>(f: F) -> F::Output {
    |                    ------ required by this bound in `futures_executor::local_pool::block_on`
    |
    = note: expected associated type `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item`
                          found enum `std::result::Result<(), futures_channel::mpsc::SendError>`
    = note: required because of the requirements on the impl of `core::future::future::Future` for `futures_util::stream::stream::forward::Forward<impl futures_core::stream::TryStream, futures_channel::mpsc::UnboundedSender<()>>`
help: consider constraining the associated type `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item` to `std::result::Result<(), futures_channel::mpsc::SendError>`
    |
5   | fn create_stream() -> impl futures::TryStream<Ok=(), Error=mpsc::SendError, Item = std::result::Result<(), futures_channel::mpsc::SendError>> { // ERROR
    |                                                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
