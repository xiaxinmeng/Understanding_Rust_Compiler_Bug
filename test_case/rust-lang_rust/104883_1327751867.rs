rust
fn main() {
    spawn(async {
        let non_send: Option<*mut ()> = None;
        drop(non_send);
        std::future::ready(1).await;
    });
}

fn spawn(_: impl Send) {}
