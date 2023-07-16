rust
thread::sleep(100 * MILLIS);
select! {
    recv(r2, v) => assert!(v.is_none()),
    recv(channel::after(1 * SECONDS)) => panic!(),
}
