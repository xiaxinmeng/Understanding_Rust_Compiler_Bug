rs
use futures::{stream, StreamExt};
use std::sync::Arc;
use tokio::task::JoinHandle;
async fn execution_process<A>(active_authority: &ActiveAuthority<A>)
where
    A: Send + Sync + 'static,
{
    stream::empty::<Result<(), ()>>()
        .zip(stream::iter(Vec::<u64>::new().iter().map(|seq| *seq)))
        .filter_map(|(result, seq)| async move { result.ok().map(|_| seq) })
        .collect::<Vec<_>>()
        .await;
}
struct ActiveAuthority<A> {
    node_sync_state: A,
}
impl<A> ActiveAuthority<A>
where
    A: Send + Sync + 'static,
{
    async fn spawn_execute_process(self: Arc<Self>) -> JoinHandle<()> {
        tokio::task::spawn(async move {
            execution_process(&self).await;
        })
    }
}
