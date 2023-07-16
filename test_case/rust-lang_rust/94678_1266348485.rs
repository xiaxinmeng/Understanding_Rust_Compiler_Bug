plain

error: unused variable: `limit`
   --> library/std/src/sys/sgx/net.rs:456:36
    |
456 |     pub fn set_hop_limit_v6(&self, limit: u32) -> io::Result<()> {
    |                                    ^^^^^ help: if this is intentional, prefix it with an underscore: `_limit`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `limit`
   --> library/std/src/sys/sgx/net.rs:464:46
    |
    |
464 |     pub fn set_multicast_hop_limit_v6(&self, limit: u32) -> io::Result<()> {
    |                                              ^^^^^ help: if this is intentional, prefix it with an underscore: `_limit`
[RUSTC-TIMING] std test:false 3.547
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 2 previous errors; 1 warning emitted
Build completed unsuccessfully in 0:22:44
