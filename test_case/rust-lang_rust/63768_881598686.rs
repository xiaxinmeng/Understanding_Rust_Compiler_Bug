rust
fn assert_send() {
    fn is_send(f: impl Future + Send) {}
    is_send(foo()); // ERROR: `Cell<isize>` cannot be shared between threads safely, the trait `Sync` is not implemented for `Cell<isize>`
}

fn spawn_local(x: impl Future) -> impl Future<Output = ()> {
    async { todo!() }
}

async fn foo() {
    let not_send = async {
        let v = RefCell::new(1);
        let mut u = v.borrow_mut();
        async { }.await;
        *u += 1;
    };
    spawn_local(not_send).await;
}

