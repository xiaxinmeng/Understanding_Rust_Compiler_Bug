rust
        let mut results: std::pin::Pin<Box<dyn Stream<Item = _> + Send>> = Box::pin(stream::empty().map(move |r: Box<dyn Whelk>| {
            future::ready(())
        })
        .buffered(5));
