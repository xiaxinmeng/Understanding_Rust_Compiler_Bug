rust
use std::future::Future;

async fn foo() {
    let some_vec = Vec::<()>::new();
    
    let flat = some_vec.iter()
        .map(|_| std::iter::empty::<()>())
        .flatten();
    std::mem::drop(flat);
    async{}.await;
}

fn assert_send<'u, R>(fut: impl 'u + Send + Future<Output = R>)
  -> impl 'u + Send + Future<Output = R>
{
    fut
}

fn main() {
    let fut = assert_send(foo());
}
