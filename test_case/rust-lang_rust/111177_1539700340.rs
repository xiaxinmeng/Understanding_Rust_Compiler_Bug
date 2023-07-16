
> /*
> 
> error[E0720]: cannot resolve opaque type
>   --> src/main.rs:12:31
>    |
> 12 | fn do_something() -> Pin<Box<impl Future<Output = ()>>> {
>    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^ recursive opaque type
> 13 |     Box::pin(async { do_something().await })
>    |     -----------------------------------------
>    |     |                |
>    |     |                async closure captures itself here
>    |     returning here with type `Pin<Box<[async block@src/main.rs:13:14: 13:45]>>`
> 
> */
> 