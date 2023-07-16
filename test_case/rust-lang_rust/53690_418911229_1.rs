rust
async fn foo(x: bool) -> u32 {
    if x {
        let f: std::future::FutureObj<u32> = std::future::FutureObj::new(std::pin::PinBox::new(foo(false)));
        await!(f) + 1
    }
    else {
        4
    }
}
