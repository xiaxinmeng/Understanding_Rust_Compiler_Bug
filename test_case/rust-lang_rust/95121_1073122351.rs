plain
[RUSTC-TIMING] addr2line test:false 0.508
[RUSTC-TIMING] core test:false 27.684
[RUSTC-TIMING] gimli test:false 5.299
[RUSTC-TIMING] object test:false 5.874
error[E0615]: attempted to take value of method `si_addr` on type `siginfo_t`
  --> library/std/src/sys/unix/stack_overflow.rs:70:17
   |
70 |         (*info).si_addr as usize
   |
help: use parentheses to call the method
   |
   |
70 |         (*info).si_addr() as usize

For more information about this error, try `rustc --explain E0615`.
[RUSTC-TIMING] std test:false 2.975
error: could not compile `std` due to previous error
