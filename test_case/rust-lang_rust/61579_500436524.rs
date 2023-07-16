rust
pub async fn foo() {
    let _ = std::thread::spawn(move || ());
    bar().await;
}
