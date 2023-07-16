rust
fn secs(&self) -> u64;  // maybe secs_part, as in "the seconds part of the duration"
fn nanos(&self) -> u32; // maybe nanos_part, as in "the nanos part of the duration"
fn total_secs(&self) -> f64;
fn total_nanos(&self) -> f64;
