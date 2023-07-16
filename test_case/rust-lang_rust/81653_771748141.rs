
use futures::{Stream, StreamExt};

#[derive(Debug, Clone)]
struct Config {}

async fn bad(stream: impl Stream<Item = usize>, config: Config) {
    stream.for_each(|item| async {
        dbg!{item, &config};
    }).await;
}
