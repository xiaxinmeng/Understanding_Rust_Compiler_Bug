plain
[RUSTC-TIMING] addr2line test:false 0.449
[RUSTC-TIMING] core test:false 27.179
[RUSTC-TIMING] gimli test:false 5.032
[RUSTC-TIMING] object test:false 5.301
error: associated function is never used: `get_pgroup`
    |
    |
275 |     pub fn get_pgroup(&self) -> Option<pid_t> {
    |
    |
    = note: `-D dead-code` implied by `-D warnings`
[RUSTC-TIMING] std test:false 3.999
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:12:26
