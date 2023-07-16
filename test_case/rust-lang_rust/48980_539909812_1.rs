rust
let d = Duration::from_secs(3600);
let mut now = Instant::now();
now -= d;
let t = now;
