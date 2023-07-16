rust
async fn f(_: String) {}

fn g() -> impl Send {
    async move {
        let x = 123;
        f(format!("{}", x)).await;
    }
}
