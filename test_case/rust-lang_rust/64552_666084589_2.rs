rust
fn f1<'r>(request: &'r Foo) -> impl Future<Output = Bar<'r>> {
    async move { f(request) }
}

async fn f1<'r>(request: &'r Foo) -> Bar<'r> {
    f(request)
}
