rust
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};

pub struct Robinhood;

impl Robinhood {
    pub fn new() -> Robinhood {
        return Robinhood {};
    }

    async fn foo(&self, t: String, b: Vec<String>) -> Vec<String> {
        // Looks like that the problem is generated from the following code
        let b: Vec<&[String]> = b.chunks(2).collect();
        let futures = b.into_iter().map(|it| {
            return async { vec![] };
        });

        let results: Vec<String> = futures::stream::iter(futures)
            .buffer_unordered(4)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect();
        results
    }
}

async fn foo_ok(t: String, b: Vec<String>) -> Vec<String> {
    let b: Vec<&[String]> = b.chunks(2).collect();
    let futures = b.into_iter().map(|it| {
        return async { vec![] };
    });

    let results: Vec<String> = futures::stream::iter(futures)
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect();
    results
}

async fn robinhood_task() {
    let robinhood = Robinhood::new();
    foo_ok(String::new(), vec![]).await;
    robinhood.foo(String::new(), vec![]).await;
}

fn main() {
    async {
        let handle1 = tokio::task::spawn(robinhood_task());
        let _ = tokio::join!(handle1);
    };
}
