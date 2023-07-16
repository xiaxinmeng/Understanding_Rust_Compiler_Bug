
fn my_err(s: str) -> ! { log_err s; fail; }
fn main() { 3u == my_err("bye"); }
