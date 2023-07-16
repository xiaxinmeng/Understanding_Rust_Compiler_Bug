
> error[E0277]: `impl std::marker::Send` is not a future
>   --> linkerd/app/outbound/src/http/logical.rs:97:30
>    |
> 97 |                     Future = impl Future<Output = Result<http::Response<http::BoxBody>, Error>> + Send,
>    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `impl std::marker::Send` is not a future
>    |
>    = help: the trait `futures::Future` is not implemented for `impl std::marker::Send`
>    = note: impl std::marker::Send must be a future or must implement `IntoFuture` to be awaited
>    = note: required for `stack::map_err::ResponseFuture<(), impl std::marker::Send>` to implement `futures::Future`
> 
> For more information about this error, try `rustc --explain E0277`.
> 