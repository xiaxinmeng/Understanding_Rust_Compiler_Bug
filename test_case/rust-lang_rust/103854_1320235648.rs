rust
#![feature(async_fn_in_trait)]

trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}

fn spawn_print_all<I: AsyncIterator + Send + 'static>(mut count: I)
where
    I::Item: Display,
    I::next::Return: Send
{
    tokio::spawn(async move {
        while let Some(x) = count.next().await {
            println!("{x}");
        }
    });
}
