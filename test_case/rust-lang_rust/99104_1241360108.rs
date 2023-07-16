rust
async fn foo(x: *const usize) {
    let _ = self.hash_map.get(&x);  // Won't work
    bar().await;
}
