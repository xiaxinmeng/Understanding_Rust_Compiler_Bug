rust
let s = Duration::from_secs(/* untrusted */);
let ns = Duration::from_nanos(/* untrusted */);

let to_send = s.checked_add(ns).unwrap_or(...);
let to_now = now - time_base;
let rtt4 = to_now.checked_sub(to_send).unwrap_or(...);
