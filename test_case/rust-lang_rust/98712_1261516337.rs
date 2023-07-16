
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Yield {
    init: bool,
}

impl std::future::Future for Yield {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if !self.init {
            self.init = true;
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        } else {
            return std::task::Poll::Ready(());
        }
    }
}

pub async fn do_things() {
    println!("Hello world");
    Yield::default().await;        // Causes a yield then completes on next `poll`
}

#[tokio::main]
pub async fn main() {
    do_things()
        .await;                    // This is the line/span we care about coverage for
}
