rust
        .collect::<futures::stream::FuturesOrdered<_>>()
: FuturesOrdered<impl Future<Output = ()>>
: impl Stream<Item = ()>
