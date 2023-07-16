rust
pub async fn f<Fut: std::future::Future<Output = ()> + std::marker::Unpin>(data: &mut [Fut]) {
    futures::stream::from_iter(data.iter_mut().map(async move |d| { d.await; }))
        .buffered(16)
        .for_each(async move |()| ())
        .await;
}
