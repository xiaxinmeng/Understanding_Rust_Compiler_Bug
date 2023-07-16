plain
2019-10-18T00:03:37.1647126Z [RUSTC-TIMING] hashbrown test:false 0.885
2019-10-18T00:03:40.1143946Z error[E0308]: mismatched types
2019-10-18T00:03:40.1145467Z    --> src/libstd/sys/unix/fs.rs:313:34
2019-10-18T00:03:40.1146207Z     |
2019-10-18T00:03:40.1146980Z 313 |                         tv_nsec: ext.stx_btime.tv_nsec as libc::c_long,
2019-10-18T00:03:40.1148117Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i64, found i32
2019-10-18T00:03:40.5030154Z error: aborting due to previous error
2019-10-18T00:03:40.5030369Z 
2019-10-18T00:03:40.5030953Z For more information about this error, try `rustc --explain E0308`.
2019-10-18T00:03:40.5766619Z [RUSTC-TIMING] std test:false 3.408
---
2019-10-18T00:03:40.5917443Z == clock drift check ==
2019-10-18T00:03:40.5943356Z   local time: Fri Oct 18 00:03:40 UTC 2019
2019-10-18T00:03:40.8761373Z   network time: Fri, 18 Oct 2019 00:03:40 GMT
2019-10-18T00:03:40.8765347Z == end clock drift check ==
2019-10-18T00:03:43.3162891Z ##[error]Bash exited with code '1'.
2019-10-18T00:03:43.3224023Z ##[section]Starting: Upload CPU usage statistics
2019-10-18T00:03:43.3232327Z ==============================================================================
2019-10-18T00:03:43.3232448Z Task         : Bash
2019-10-18T00:03:43.3232537Z Description  : Run a Bash script on macOS, Linux, or Windows
