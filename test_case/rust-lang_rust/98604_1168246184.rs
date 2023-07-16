Rust 
type AsyncFnPtr = Box<
    dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
>;

async fn test() {}

fn main() {
    let _: AsyncFnPtr = Box::new(test);
}
