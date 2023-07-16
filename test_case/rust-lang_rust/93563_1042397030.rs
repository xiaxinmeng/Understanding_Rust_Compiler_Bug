rust
loop {
    match rx.try_recv() {
        Ok(msg) => process(msg),
        Err(TryRecvError::Empty) => sleep(std::time::Duration::from_millis(1)),
        Err(TryRecvError::Disconnected) => break,        
    }
}
