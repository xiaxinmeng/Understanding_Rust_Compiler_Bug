rust
> let rtt4 = if now >= send_time { now - send_time } else { Duration::from_secs(999)};
> 