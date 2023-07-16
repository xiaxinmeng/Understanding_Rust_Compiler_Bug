rust
select! {
    recv(r2, v) => assert!(v.is_none()),
    recv(channel::after(ms(1000))) => panic!(),
}
