rs
fn assert_send<'u, R>(fut: impl 'u + Send + Future<Output = R>)
  -> impl 'u + Send + Future<Output = R>
{
    fut
}

let f = assert_send(futures::future::try_join_all(futs));
f.await
