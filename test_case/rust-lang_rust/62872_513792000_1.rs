rust
        .map(async move |d| {
            d.await;
        })
: std::iter::Map<
    impl Iterator<Item = &mut impl Future<Output = ()> + Unpin>>,
    impl Fn(&mut impl Future<Output = ()> + Unpin) -> impl Future<Output = ()>,
>
: impl Iterator<Item = impl Future<Output = ()>>
