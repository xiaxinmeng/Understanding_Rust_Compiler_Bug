rust
use easytime::{Duration, Instant};


let s: u64 = /* untrusted */;
let ns: u32 = /* untrusted */;

let since_base = Duration::new(s, ns);
let rtt4 = now - (time_base + since_base);

// some method to go from easytime::Duration to std::time::Duration:
let rtt4 = rtt4.unwrap_or(...);
