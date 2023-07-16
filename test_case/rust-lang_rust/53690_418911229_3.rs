rust
fn foo(x: bool) -> impl std::future::Future<Output = u32> {
    async move {
        if x {
            let f: std::future::FutureObj<u32> = std::future::FutureObj::new(std::pin::PinBox::new(foo(false)));
            await!(f) + 1
        }
        else {
            4
        }
    }
}
