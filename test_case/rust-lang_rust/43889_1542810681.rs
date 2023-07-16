rust
use std::future::Future;
use std::pin::Pin;

struct Unsendable(*mut ());

fn get() -> impl Future<Output=()>
{
    let unsendable = Unsendable(&mut ());
    async move {
        //make sure the unsendable thing is used in the future
        std::hint::black_box(unsendable);
    }
}


fn main() {
    let fut: Pin<Box<dyn Future<Output=()> + Send>> = Box::pin(get());
}
