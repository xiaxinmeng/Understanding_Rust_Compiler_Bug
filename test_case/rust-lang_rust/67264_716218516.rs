rust
match timeout {
    timeval { tv_sec: sec @ 0.., tv_usec: usec @ 0..1_000_000, .. } => Ok((sec, usec)),
    timeval { tv_sec: ..0, .. } => Err("is negative"),
    timeval { tv_usec: 1_000_000.., .. } | timeval { tv_usec: ..0, .. } => Err("has invalid microsecond part"),
}
