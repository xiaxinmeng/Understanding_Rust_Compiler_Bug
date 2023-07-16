rust
async fn foo(_: String) {
}

fn bar() -> impl Send {
    async move {
        foo(format!("{}:{}", 1, 2)).await;
    }
}
