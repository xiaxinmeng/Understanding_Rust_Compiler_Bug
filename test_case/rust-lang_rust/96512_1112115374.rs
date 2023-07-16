rust
struct Variant;

enum Enum {
    Variant(Variant),
}

fn spawn<T>(_: T)
where
    T: std::future::Future + Send + 'static,
    T::Output: Send + 'static,
{
}

async fn dispatch(_: Variant) {}

fn main() {
    let eh = Enum::Variant(Variant);

    spawn(async move {
        let Enum::Variant(value) = eh;
        dispatch(value).await;
    });
}
