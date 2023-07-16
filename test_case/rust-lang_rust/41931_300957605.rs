rust
use std::sync::Arc;
use std::any::Any;

fn main() {
    let x: Arc<Any> = Arc::new(());
    x.downcast_ref::<Box<((), Fn() -> ())>>().unwrap().1();
}
