txt


error[E0698]: type inside `async` object must be known in this context
  --> src/lib.rs:23:25
   |
23 |     let server = server.serve(hyper::service::make_service_fn(move |_| {
   |                         ^^^^^ cannot infer type for `ME`
   |
note: the type is part of the `async` object because of this `await`
  --> src/lib.rs:34:5
   |
34 |     server.await
   |     ^^^^^^^^^^^^
