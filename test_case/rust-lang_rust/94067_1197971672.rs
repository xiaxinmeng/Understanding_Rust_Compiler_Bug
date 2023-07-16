rust
struct NotSend(*const ());

async fn foo() {}

pub async fn bar() {
    let var = NotSend(&());
    drop(var);
    foo().await;
}

fn assert_send(_: impl Send) {}

pub fn check() {
    assert_send(bar());
}
