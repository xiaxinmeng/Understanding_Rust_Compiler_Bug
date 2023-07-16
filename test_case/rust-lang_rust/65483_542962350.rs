plain
2019-10-17T01:55:39.8202146Z 
2019-10-17T01:55:42.5319437Z error[E0308]: mismatched types
2019-10-17T01:55:42.5320092Z    --> src/libstd/sys/unix/fs.rs:790:52
2019-10-17T01:55:42.5320751Z     |
2019-10-17T01:55:42.5322271Z 790 |         let n = cvt(unsafe { lseek64(self.0.raw(), pos, whence) })?;
2019-10-17T01:55:42.5323454Z     |                                                    |
2019-10-17T01:55:42.5324056Z     |                                                    expected i64, found i32
2019-10-17T01:55:42.5324056Z     |                                                    expected i64, found i32
2019-10-17T01:55:42.5324689Z     |                                                    help: you can convert an `i32` to `i64`: `pos.into()`
2019-10-17T01:55:42.7080448Z error: aborting due to previous error
2019-10-17T01:55:42.7080661Z 
2019-10-17T01:55:42.7081710Z For more information about this error, try `rustc --explain E0308`.
2019-10-17T01:55:42.7561945Z [RUSTC-TIMING] std test:false 3.155
---
2019-10-17T01:55:42.7683064Z == clock drift check ==
2019-10-17T01:55:42.7701865Z   local time: Thu Oct 17 01:55:42 UTC 2019
2019-10-17T01:55:42.8760618Z   network time: Thu, 17 Oct 2019 01:55:42 GMT
2019-10-17T01:55:42.8761469Z == end clock drift check ==
2019-10-17T01:55:47.2666837Z ##[error]Bash exited with code '1'.
2019-10-17T01:55:47.2705667Z ##[section]Starting: Upload CPU usage statistics
2019-10-17T01:55:47.2719620Z ==============================================================================
2019-10-17T01:55:47.2719754Z Task         : Bash
2019-10-17T01:55:47.2719825Z Description  : Run a Bash script on macOS, Linux, or Windows
