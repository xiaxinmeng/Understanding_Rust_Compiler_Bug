rust
use std::future::Future;

pub trait Service {
    type Response;
    type Future: Future<Output = Self::Response>;
}

pub trait ThriftService: Service<Future = Box<dyn Future<Output = i32>>, Response = i32> {
    fn foo(&self) {}
}

pub trait ThriftService2: Service<Response = i32, Future = Box<dyn Future<Output = i32>>> {
    fn foo(&self) {}
}

fn main() {
    let x: &dyn ThriftService = todo!();
    let y: &dyn ThriftService2 = todo!();
}
