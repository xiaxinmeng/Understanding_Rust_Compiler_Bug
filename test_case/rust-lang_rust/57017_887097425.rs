
pub fn wat() -> impl Future + Send {
    let client = Client(Box::new(true));
    async move {
        if let 200 = client.status() {
            get().await;
        }
    }
}
