rust
loop {
    if let Ok(x) = timeout(Duration::from_secs(1), ...some TryFuture...).await {
        let _ = x?;
        break;
    }
}
