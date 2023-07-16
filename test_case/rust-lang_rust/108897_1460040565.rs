rust
trait Handler {}
impl<F, Fut> Handler for F
where
    Fut: Send,
    F: FnOnce() -> Fut,
{}

fn require_handler<H: Handler>(h: H) {}

async fn handler() {
    let a = &1 as *const i32;
    async {}.await;
}

fn main() {
    require_handler(handler)
}
