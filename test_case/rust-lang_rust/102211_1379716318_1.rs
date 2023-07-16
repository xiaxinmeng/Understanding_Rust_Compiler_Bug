rust
use std::future::Future;

fn foo() -> impl Future<Output = ()> + Send {
    let some_vec = Vec::<()>::new();
    
    async move {
        let flat = some_vec.iter()
            .map(|_| std::iter::empty::<()>())
            .flatten();
        std::mem::drop(flat);
        async{}.await;
    }
}
