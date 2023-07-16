rust
use std::{future::Future, pin::Pin};
use futures;
use itertools::Itertools;

fn test() -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        futures::future::join_all(
            std::iter::empty::<&u64>()
                .filter_map(|_v| None)
                .unique()
                .map(abc),
        )
        .await;
    })
}

async fn abc(v: &u64) -> u64 {
    *v
}
