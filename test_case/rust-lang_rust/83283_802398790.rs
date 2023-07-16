plain
[RUSTC-TIMING] addr2line test:false 0.461
[RUSTC-TIMING] core test:false 37.006
[RUSTC-TIMING] gimli test:false 5.552
[RUSTC-TIMING] object test:false 10.638
error: field is never read: `create_pidfd`
  --> library/std/src/sys/unix/process/process_common.rs:82:5
   |
82 |     pub(crate) create_pidfd: bool,
   |
   |
   = note: `-D dead-code` implied by `-D warnings`

error: associated function is never used: `create_pidfd`
   --> library/std/src/sys/unix/process/process_common.rs:182:12
    |
182 |     pub fn create_pidfd(&mut self, val: bool) {

error: aborting due to 2 previous errors

[RUSTC-TIMING] std test:false 3.883
