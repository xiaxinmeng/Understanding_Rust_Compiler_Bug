rust
async fn foo() {
    spawn_local(async {
        let v = RefCell::new(1);
        let mut u = v.borrow_mut();
        async { }.await;
        *u += 1;
    }).await;
}
