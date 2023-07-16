rust
> use tracing::debug;
> client
>     .protocol("https")
>     .method("GET")
>     .send()
>     .await // returns `Result<_, _>
>     .inspect(|ok_val| debug!(result = ?Ok::<_, ()>(ok_val)))
>     .inspect_err(|err_val| debug!(result = ?Err::<(), _>(err_val)))
>     .map_err(Error::from)
> 