rust
// Using futures-preview v0.3.0-alpha.19
use futures::StreamExt;

fn main() {
    enter(run())
}

async fn run() {
    let entries: Vec<_> = vec![()]
        .into_iter()
        .map(|x| async { vec![()].into_iter().map(|_| x) })
        .collect::<futures::stream::FuturesUnordered<_>>()
        .map(futures::stream::iter)
        .flatten()
        .collect()
        .await;
}

pub fn enter(x: impl Send) {}
