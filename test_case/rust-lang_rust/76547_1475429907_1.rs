
> error[E0623]: lifetime mismatch
>   --> src/main.rs:24:13
>    |
> 23 | pub async fn readv_at(bufs: &mut [&mut [u8]]) {
>    |                                   ---------   -
>    |                                   |           |
>    |                                   |           this `async fn` implicitly returns an `impl Future<Output = ()>`
>    |                                   this parameter and the returned future are declared with different lifetimes...
> 24 |     ListFut(bufs).await
>    |             ^^^^ ...but data from `bufs` is held across an await point here
> 