rust
async move {
    let fut = g.get();
    assert!(is_send(&fut));
    fut.await
}
