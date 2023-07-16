rust
    ctrlc::set_handler(|| println!("Caught ctrl-c")).unwrap();
    let bytes = std::io::stdin().lock().read_line(&mut line)?;
