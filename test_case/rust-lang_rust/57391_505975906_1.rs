rust
use std::time::{Instant, TimeUnits as _};

sleep(100.milliseconds());

let deadline = Instant::now() + 5.seconds();
