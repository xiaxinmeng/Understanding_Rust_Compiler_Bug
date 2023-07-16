
async fn bar() -> Result<(), i32> {
    std::future::pending().await
}

pub async fn async_with_warnings() {
    bar().await;
}
