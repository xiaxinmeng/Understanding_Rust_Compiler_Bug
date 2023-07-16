plain
2019-10-10T19:39:14.8311030Z [RUSTC-TIMING] panic_unwind test:false 0.442
2019-10-10T19:39:15.6477540Z [RUSTC-TIMING] alloc test:false 6.897
2019-10-10T19:39:16.0165130Z [RUSTC-TIMING] hashbrown test:false 1.559
2019-10-10T19:39:16.4833620Z [RUSTC-TIMING] core test:false 40.167
2019-10-10T19:39:17.6837950Z error[E0412]: cannot find type `statx_timestamp` in crate `libc`
2019-10-10T19:39:17.6839040Z   --> src/libstd/sys/unix/fs.rs:55:22
2019-10-10T19:39:17.6839840Z    |
2019-10-10T19:39:17.6840600Z 55 |     stx_btime: libc::statx_timestamp,
2019-10-10T19:39:17.6841450Z    |                      ^^^^^^^^^^^^^^^ not found in `libc`
2019-10-10T19:39:21.4701030Z error: aborting due to previous error
2019-10-10T19:39:21.4701330Z 
2019-10-10T19:39:21.4702090Z For more information about this error, try `rustc --explain E0412`.
2019-10-10T19:39:21.5660190Z [RUSTC-TIMING] std test:false 5.059
---
2019-10-10T19:39:21.5890870Z == clock drift check ==
2019-10-10T19:39:21.5951760Z   local time: Thu Oct 10 19:39:21 UTC 2019
2019-10-10T19:39:21.6333750Z   network time: Thu, 10 Oct 2019 19:39:21 GMT
2019-10-10T19:39:21.6336280Z == end clock drift check ==
2019-10-10T19:39:21.6520630Z ##[error]Bash exited with code '1'.
2019-10-10T19:39:21.6575060Z ##[section]Starting: Upload CPU usage statistics
2019-10-10T19:39:21.6580530Z ==============================================================================
2019-10-10T19:39:21.6580660Z Task         : Bash
2019-10-10T19:39:21.6580740Z Description  : Run a Bash script on macOS, Linux, or Windows
