rust
fn run<'a>(cx: Context<'a>) -> impl std::future::Future<Output = ()> + 'a {
    async move {
        do_something_with(cx).await;
    }
}
