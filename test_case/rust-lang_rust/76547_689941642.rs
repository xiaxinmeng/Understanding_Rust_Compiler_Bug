rust
> /// Even if we explicitly bound one lifetime by the other, the compiler still
> /// can't handle it.
> pub async fn readv_at3<'a, 'b: 'a>(bufs: &'a mut [&'b mut [u8]]) {
>     ListFut(bufs).await
> }
> 