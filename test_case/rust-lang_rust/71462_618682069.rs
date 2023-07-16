rust
use std::future::Future;

trait Service<R> {
    type Future: Future<Output = i32>;
    
    fn call(&mut self, req: R) -> Self::Future;
}

struct BorrowerService<S> {
    service: Option<S>,
}

impl<S> Service<Box<dyn Sync + Send>> for BorrowerService<S>
where
    for<'b> S: Service<&'b (dyn 'static + Sync + Send)> + 'static + Send,
    for<'b> <S as Service<&'b (dyn 'static + Sync + Send)>>::Future: Send,
{
    type Future = Box<dyn Future<Output = i32> + Send>;
    
    fn call(&mut self, req: Box<dyn Sync + Send>) -> Self::Future {
        let mut service = self.service.take().unwrap();
        Box::new(async move {
            service.call(&req).await
        })
    }
}
