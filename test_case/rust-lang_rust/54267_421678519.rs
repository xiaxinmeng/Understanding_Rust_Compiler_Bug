rust
loop {
    let x = rx.recv().unwrap();
    println!("{:?}", x);
}
