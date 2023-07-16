rust
let mut a = 1;
for _ in 0..10 {
    tokio::spawn(async move {
        a += 1;
        if a > 3 {
            // this will never reached
        }
    })
    .await
    .unwrap();
}
