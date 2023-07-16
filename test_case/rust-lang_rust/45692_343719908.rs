
[01:20:28] [m[m[32m[1m   Compiling[m cargo v0.24.0 (file:///checkout/src/tools/cargo)
[01:20:30] [1m[31merror[E0308](B[m[1m: mismatched types(B[m
[01:20:30]    (B[m[1m[34m--> (B[m/checkout/src/tools/cargo/src/cargo/core/shell.rs:299:49(B[m
[01:20:30]     (B[m[1m[34m|(B[m
[01:20:30] [1m[34m299(B[m (B[m[1m[34m| (B[m            if libc::ioctl(libc::STDERR_FILENO, libc::TIOCGWINSZ, &mut winsize) < 0 {(B[m
[01:20:30]     (B[m[1m[34m| (B[m                                                (B[m[1m[31m^^^^^^^^^^^^^^^^(B[m (B[m[1m[31mexpected u64, found u32(B[m
[01:20:30] 
[01:20:34] [1m[31merror(B[m[1m: aborting due to previous error(B[m
[01:20:34] 
[01:20:34] [m[m[31m[1merror:[m Could not compile `cargo`.
[01:20:34] 
[01:20:34] To learn more, run the command again with --verbose.
[01:20:34] 
