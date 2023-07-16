
> error[E0698]: type inside `async fn` body must be known in this context
>   --> src/lib.rs:15:13
>    |
> 15 |             Ok(hyper::service::service_fn(move |req| {
>    |             ^^ cannot infer type for `ME`
>    |
> note: the type is part of the `async fn` body because of this `await`
>   --> src/lib.rs:24:5
>    |
> 24 |     server.await
>    |     ^^^^^^^^^^^^
> 
> error: aborting due to previous error
> 