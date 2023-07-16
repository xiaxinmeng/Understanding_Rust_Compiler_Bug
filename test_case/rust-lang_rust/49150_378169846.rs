rust
pub trait Read {
    type Error;

    fn poll_read(
        self: Pin<Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> Poll<usize, Self::Error>;
}

pub fn read_exact<'a, 'b: 'a, R: Read + 'a>(
    mut this: Pin<'a, R>,
    buf: &'b mut [u8],
) -> impl StableFuture<Item = (), Error = Error<R::Error>>
         + Captures<'a>
         + Captures<'b> {
    async_block_pinned! {
        let mut position = 0;
        while position < buf.len() {
            let amount = await!(poll_fn(|cx| {
                Pin::borrow(&mut this).poll_read(cx, &mut buf[position..])
            }))?;
            position += amount;
            if amount == 0 {
                Err(Error::UnexpectedEof)?;
            }
        }
        Ok(())
    }
}
