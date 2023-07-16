plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.12.3
    Checking object v0.29.0
    Checking addr2line v0.17.0
error[E0599]: no method named `snooze` found for struct `Backoff` in the current scope
   --> library/std/src/sync/mpmc/list.rs:559:25
    |
559 |                 backoff.snooze();
    |                         ^^^^^^ method not found in `Backoff`
    |
   ::: library/std/src/sync/mpmc/utils.rs:96:1
96  | pub struct Backoff {
96  | pub struct Backoff {
    | ------------------ method `snooze` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:21
