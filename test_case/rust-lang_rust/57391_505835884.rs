rust
use std::time::{prelude::*, Instant};
use std::thread;

let now = Instant::now();
thread::sleep(2.secs());
dbg!(now.elapsed());
