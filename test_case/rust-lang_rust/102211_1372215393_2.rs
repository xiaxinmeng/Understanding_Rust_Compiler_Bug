rust
assert_stream_send(
    stream::iter(&Vec::<()>::new())
        .map(|_| stream::empty::<()>())
)
    .flatten()
    .collect::<Vec<_>>()
    .await;
