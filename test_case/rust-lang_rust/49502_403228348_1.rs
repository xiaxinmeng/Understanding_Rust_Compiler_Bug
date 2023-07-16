rust
use std::time::{SystemTime, UNIX_EPOCH};
SystemTime::now().duration_since(UNIX_EPOCH);
