rust
let now = Instant::now();
if now > deadline {
    break;
}

let s: u64 = /* untrusted */;
let ns: u32 = /* untrusted */;

let since_base = Duration::new(s, ns);
let send_time = time_base + since_base;
let rtt4 = if now >= send_time { now - send_time } else { Duration::from_secs(999) };
