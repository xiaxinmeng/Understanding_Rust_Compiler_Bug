rust
{
    let mut pinned = future;
    loop {
        match ::std::future::poll_with_tls_context(unsafe {
            <::std::pin::Pin>::new_unchecked(&mut pinned)
        }) {
            ::std::task::Poll::Ready(result) => break result,
            ::std::task::Poll::Pending => {}
        }
        yield ();
    }
}
