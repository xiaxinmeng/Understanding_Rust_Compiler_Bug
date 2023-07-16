rust
loop {
    println!("calling recv_timeout");
    let x = rx.recv_timeout(Duration::from_millis(100));
    println!("{:?}", x);
}
