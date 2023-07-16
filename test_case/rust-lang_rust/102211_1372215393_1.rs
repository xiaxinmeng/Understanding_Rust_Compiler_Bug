rust
fn f(_: &()) -> stream::Empty::<()> {
    stream::empty()
}

stream::iter(&Vec::<()>::new())
    .map(f)
    .flatten()
    .collect::<Vec<_>>()
    .await;
