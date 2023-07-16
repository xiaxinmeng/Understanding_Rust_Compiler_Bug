
fn test_poll_next<F,T>(first: Pin<&mut F>) -> Poll<Option<T>> 
where
    F: Stream<Item = T>
{
    first.poll_next()
}
