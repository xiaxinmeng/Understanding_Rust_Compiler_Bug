rust
use std::rc::Rc;

async fn non_trivial_drop() {
    let x = Rc::new(vec![0]);
    drop(x);
    async {}.await;
}


fn assert_send<T: Send>(_: T) {}

fn main() {
    assert_send(non_trivial_drop());
}
