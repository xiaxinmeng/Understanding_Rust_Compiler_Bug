
error[E0308]: mismatched types
  --> notarius_server_http/src/main.rs:21:5
   |
20 | ) -> Result<impl Reply, Rejection> {
   |      ----------------------------- expected `Result<_, Rejection>` because of return type
21 |     handler.handle_sign(bytes)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found opaque type
   |
note: while checking the return type of the `async fn`
  --> notarius_server_http/src/main.rs:27:55
   |
27 |     pub async fn handle_sign(&self, _bytes: Bytes) -> Result<impl Reply, Rejection> {
   |                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, found opaque type
   = note:     expected enum `Result<_, Rejection>`
           found opaque type `impl Future<Output = Result<impl Reply, Rejection>>`
help: consider `await`ing on the `Future`
   |
21 |     handler.handle_sign(bytes).await
   |                               ++++++
help: try wrapping the expression in `Ok`
   |
21 |     Ok(handler.handle_sign(bytes))
   |     +++                          +

error[E0282]: type annotations needed
  --> notarius_server_http/src/main.rs:27:85
   |
27 |       pub async fn handle_sign(&self, _bytes: Bytes) -> Result<impl Reply, Rejection> {
   |  _____________________________________________________________________________________^
28 | |         loop {}
29 | |     }
   | |_____^ cannot infer type

error[E0277]: `Result<impl Reply, Rejection>` is not a future
  --> notarius_server_http/src/main.rs:13:14
   |
13 |             .and_then(lambda);
   |              ^^^^^^^^ `Result<impl Reply, Rejection>` is not a future
   |
   = help: the trait `Future` is not implemented for `Result<impl Reply, Rejection>`
   = note: Result<impl Reply, Rejection> must be a future or must implement `IntoFuture` to be awaited
   = note: required for `Result<impl Reply, Rejection>` to implement `futures_core::future::TryFuture`

error[E0599]: the method `run` exists for struct `warp::Server<warp::filter::and_then::AndThen<warp::filter::and::And<impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (), Error = Rejection>, impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (bytes::Bytes,), Error = Rejection>>, [closure@notarius_server_http/src/main.rs:9:18: 9:30]>>`, but its trait bounds were not satisfied
   --> notarius_server_http/src/main.rs:14:23
    |
14  |     warp::serve(sign).run(([0, 0, 0, 0], 3030)).await;
    |                       ^^^ method cannot be called on `warp::Server<warp::filter::and_then::AndThen<warp::filter::and::And<impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (), Error = Rejection>, impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (bytes::Bytes,), Error = Rejection>>, [closure@notarius_server_http/src/main.rs:9:18: 9:30]>>` due to unsatisfied trait bounds
    |
   ::: /home/aavasiljev/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:504:1
    |
504 | pub enum Result<T, E> {
    | ---------------------
    | |
    | doesn't satisfy `<_ as Future>::Output = Result<_, _>`
    | doesn't satisfy `Result<impl Reply, Rejection>: Future`
    | doesn't satisfy `_: futures_core::future::TryFuture`
    |
   ::: /home/aavasiljev/.cargo/registry/src/github.com-1ecc6299db9ec823/warp-0.3.3/src/filter/and_then.rs:12:1
    |
12  | pub struct AndThen<T, F> {
    | ------------------------ doesn't satisfy `_: warp::Filter`
...
37  | pub struct AndThenFuture<T, F>
    | ------------------------------
    | |
    | doesn't satisfy `<_ as Future>::Output = Result<_, _>`
    | doesn't satisfy `_: Future`
    |
    = note: the following trait bounds were not satisfied:
            `warp::filter::and_then::AndThen<warp::filter::and::And<impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (), Error = Rejection>, impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (bytes::Bytes,), Error = Rejection>>, [closure@notarius_server_http/src/main.rs:9:18: 9:30]>: warp::Filter`
            `Result<impl Reply, Rejection>: futures_core::future::TryFuture`
            `<Result<impl Reply, Rejection> as Future>::Output = Result<_, _>`
            `Result<impl Reply, Rejection>: Future`
            `<warp::filter::and_then::AndThenFuture<warp::filter::and::And<impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (), Error = Rejection>, impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (bytes::Bytes,), Error = Rejection>>, [closure@notarius_server_http/src/main.rs:9:18: 9:30]> as Future>::Output = Result<_, _>`
            `warp::filter::and_then::AndThenFuture<warp::filter::and::And<impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (), Error = Rejection>, impl warp::Filter + Copy + warp::filter::FilterBase<Extract = (bytes::Bytes,), Error = Rejection>>, [closure@notarius_server_http/src/main.rs:9:18: 9:30]>: Future`

Some errors have detailed explanations: E0277, E0282, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `notarius_server_http` due to 4 previous errors
