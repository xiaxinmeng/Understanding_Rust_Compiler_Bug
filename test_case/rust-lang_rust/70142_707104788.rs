rust
use async_std::{fs, future};
use std::time::Duration;

let dur = Duration::from_millis(100);
let s = fs::read_to_string("./my.db").timeout(dur).await??;
