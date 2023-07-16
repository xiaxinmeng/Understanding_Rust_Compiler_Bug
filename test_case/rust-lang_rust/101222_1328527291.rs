plain
   Compiling addr2line v0.17.0
error: unused variable: `sched_param`
   --> library/std/src/sys/unix/thread.rs:107:21
    |
107 |                 let sched_param = libc::sched_param { sched_priority: priority.0 };
    |                     ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_sched_param`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: associated function `new` is never used
 --> library/std/src/os/linux/thread.rs:8:8
  |
  |
8 |     fn new(priority: i32) -> Self {
  |
  |
  = note: `-D dead-code` implied by `-D warnings`
error: missing documentation for a struct
 --> library/std/src/thread/schedule.rs:4:1
  |
4 | pub struct Priority(imp::Priority);
4 | pub struct Priority(imp::Priority);
  | ^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D missing-docs` implied by `-D warnings`
error: missing documentation for a struct
  --> library/std/src/thread/schedule.rs:19:1
   |
   |
19 | pub struct Affinity(pub(crate) imp::Affinity);

error: missing documentation for an associated function
   --> library/std/src/thread/mod.rs:384:5
    |
    |
384 |     pub fn priority(mut self, priority: Priority) -> Builder {

error: missing documentation for an associated function
   --> library/std/src/thread/mod.rs:390:5
    |
    |
390 |     pub fn affinity(mut self, affinity: Affinity) -> Builder {

error: could not compile `std` due to 6 previous errors
Build completed unsuccessfully in 0:00:18
