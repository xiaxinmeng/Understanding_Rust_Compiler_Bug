 rust
let res: Result<u32, Void> = Ok(0); // same memory layout as plain u32, since Err is unreachable
let Ok(num) = res; // Works because Err is unreachable
