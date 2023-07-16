rs
trait Funnel {
    type This;
}
impl<T: 'static> Funnel for T {
    type This = T;
}

async fn bar<T: Funnel>() {
    let _x: T::This = todo!();
    std::future::ready(()).await;
}

pub fn test() -> impl Send {
    async { bar::<Box<dyn Send>>().await }
}
