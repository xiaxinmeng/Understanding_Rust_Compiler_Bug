rust
use std::time::SystemTime;
SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
