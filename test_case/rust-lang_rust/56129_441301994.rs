
Nov 23 04:18:59.734 INFO [stderr] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
Nov 23 04:18:59.734 INFO [stderr]    --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/jobserver-0.1.11/src/lib.rs:566:13
Nov 23 04:18:59.734 INFO [stderr]     |
Nov 23 04:18:59.734 INFO [stderr] 566 | /             cmd.before_exec(move || {
Nov 23 04:18:59.734 INFO [stderr] 567 | |                 set_cloexec(read, false)?;
Nov 23 04:18:59.734 INFO [stderr] 568 | |                 set_cloexec(write, false)?;
Nov 23 04:18:59.734 INFO [stderr] 569 | |                 Ok(())
Nov 23 04:18:59.734 INFO [stderr] 570 | |             });
Nov 23 04:18:59.734 INFO [stderr]     | |______________^ call to unsafe function
Nov 23 04:18:59.734 INFO [stderr]     |
Nov 23 04:18:59.734 INFO [stderr]     = note: consult the function's documentation for information on how to avoid undefined behavior
