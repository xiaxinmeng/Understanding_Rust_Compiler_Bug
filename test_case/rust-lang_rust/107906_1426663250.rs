plain
[RUSTC-TIMING] core test:false 23.257
[RUSTC-TIMING] addr2line test:false 0.435
[RUSTC-TIMING] gimli test:false 4.691
[RUSTC-TIMING] object test:false 5.314
error[E0599]: no method named `as_mut_ptr` found for struct `AtomicI8` in the current scope
  --> library/std/src/sys_common/thread_parking/id.rs:63:33
   |
63 |                 park(self.state.as_mut_ptr().addr());


error[E0599]: no method named `as_mut_ptr` found for struct `AtomicI8` in the current scope
  --> library/std/src/sys_common/thread_parking/id.rs:79:42
   |
79 |             park_timeout(dur, self.state.as_mut_ptr().addr());


error[E0599]: no method named `as_mut_ptr` found for struct `AtomicI8` in the current scope
   --> library/std/src/sys_common/thread_parking/id.rs:102:36
    |
102 |             unpark(tid, self.state.as_mut_ptr().addr());

For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] std test:false 2.030
error: could not compile `std` due to 3 previous errors
