console
/tmp ❯ rustc -g -C opt-level=1 main.rs
/tmp ❯ rust-gdb ./main
(gdb) b main.rs:10
Breakpoint 1 at 0x883a: file main.rs, line 12.
(gdb) r
Starting program: /tmp/main
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/nix/store/4nlgxhb09sdr51nc9hdm8az5b08vzkgx-glibc-2.35-163/lib/libthread_db.so.1".


Breakpoint 1, main::main () at main.rs:12
12	                assert_eq!(counter, 2);
(gdb) p counter
$1 = <optimized out>
