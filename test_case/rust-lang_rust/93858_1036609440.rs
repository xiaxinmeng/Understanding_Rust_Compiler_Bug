`
error: associated function is never used: `get_pgroup`
   --> library/std/src/sys/unix/process/process_common.rs:275:12
    |
275 |     pub fn get_pgroup(&self) -> Option<pid_t> {
    |            ^^^^^^^^^^
    |
    = note: `-D dead-code` implied by `-D warnings`

[RUSTC-TIMING] std test:false 3.999
error: could not compile `std` due to previous error
