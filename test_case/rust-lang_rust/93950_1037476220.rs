plain
   Compiling addr2line v0.16.0
error: argument never used
   --> library/std/src/sys/unix/process/process_unix.rs:685:46
    |
685 |             write!(f, "exit status: {code}", code)
    |                       ---------------------  ^^^^ argument never used
    |                       formatting specifier missing

error: argument never used
   --> library/std/src/sys/unix/process/process_unix.rs:688:61
   --> library/std/src/sys/unix/process/process_unix.rs:688:61
    |
688 |                 write!(f, "signal: {signal} (core dumped)", signal)
    |                           --------------------------------  ^^^^^^ argument never used
    |                           formatting specifier missing

error: argument never used
   --> library/std/src/sys/unix/process/process_unix.rs:690:47
   --> library/std/src/sys/unix/process/process_unix.rs:690:47
    |
690 |                 write!(f, "signal: {signal}", signal)
    |                           ------------------  ^^^^^^ argument never used
    |                           formatting specifier missing

error: argument never used
   --> library/std/src/sys/unix/process/process_unix.rs:693:71
   --> library/std/src/sys/unix/process/process_unix.rs:693:71
    |
693 |             write!(f, "stopped (not terminated) by signal: {signal}", signal)
    |                       ----------------------------------------------  ^^^^^^ argument never used
    |                       formatting specifier missing

error: argument never used
   --> library/std/src/panicking.rs:748:56
   --> library/std/src/panicking.rs:748:56
    |
748 |     rtabort!("failed to initiate panic, error {code}", code)
    |              ----------------------------------------  ^^^^ argument never used
    |              formatting specifier missing

error: could not compile `std` due to 5 previous errors
Build completed unsuccessfully in 0:00:21
