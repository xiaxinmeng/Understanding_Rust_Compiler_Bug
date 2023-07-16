plain
2019-10-15T20:48:14.0158246Z [RUSTC-TIMING] backtrace test:false 0.527
2019-10-15T20:48:16.8733742Z error[E0308]: mismatched types
2019-10-15T20:48:16.8735408Z    --> src/libstd/sys/unix/fs.rs:151:38
2019-10-15T20:48:16.8736293Z     |
2019-10-15T20:48:16.8736963Z 151 |                 stat.st_atime_nsec = buf.stx_atime.tv_nsec as libc::c_long;
2019-10-15T20:48:16.8738226Z     |                                      |
2019-10-15T20:48:16.8738835Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8738835Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8740335Z     |                                      help: you can convert an `i32` to `i64`: `(buf.stx_atime.tv_nsec as libc::c_long).into()`
2019-10-15T20:48:16.8759519Z error[E0308]: mismatched types
2019-10-15T20:48:16.8760367Z    --> src/libstd/sys/unix/fs.rs:153:38
2019-10-15T20:48:16.8764606Z     |
2019-10-15T20:48:16.8764606Z     |
2019-10-15T20:48:16.8765401Z 153 |                 stat.st_mtime_nsec = buf.stx_mtime.tv_nsec as libc::c_long;
2019-10-15T20:48:16.8766626Z     |                                      |
2019-10-15T20:48:16.8767260Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8767260Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8767929Z     |                                      help: you can convert an `i32` to `i64`: `(buf.stx_mtime.tv_nsec as libc::c_long).into()`
2019-10-15T20:48:16.8777950Z error[E0308]: mismatched types
2019-10-15T20:48:16.8779791Z    --> src/libstd/sys/unix/fs.rs:155:38
2019-10-15T20:48:16.8781153Z     |
2019-10-15T20:48:16.8781153Z     |
2019-10-15T20:48:16.8782927Z 155 |                 stat.st_ctime_nsec = buf.stx_ctime.tv_nsec as libc::c_long;
2019-10-15T20:48:16.8786301Z     |                                      |
2019-10-15T20:48:16.8787918Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8787918Z     |                                      expected i64, found i32
2019-10-15T20:48:16.8789726Z     |                                      help: you can convert an `i32` to `i64`: `(buf.stx_ctime.tv_nsec as libc::c_long).into()`
2019-10-15T20:48:16.9252462Z error[E0308]: mismatched types
2019-10-15T20:48:16.9254714Z    --> src/libstd/sys/unix/fs.rs:311:34
2019-10-15T20:48:16.9256413Z     |
2019-10-15T20:48:16.9256413Z     |
2019-10-15T20:48:16.9258092Z 311 |                         tv_nsec: ext.stx_btime.tv_nsec as libc::c_long,
2019-10-15T20:48:16.9260382Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i64, found i32
2019-10-15T20:48:17.3127402Z error: aborting due to 4 previous errors
2019-10-15T20:48:17.3127537Z 
2019-10-15T20:48:17.3127911Z For more information about this error, try `rustc --explain E0308`.
2019-10-15T20:48:17.3815704Z [RUSTC-TIMING] std test:false 3.361
---
2019-10-15T20:48:17.3951184Z == clock drift check ==
2019-10-15T20:48:17.4001534Z   local time: Tue Oct 15 20:48:17 UTC 2019
2019-10-15T20:48:17.5444193Z   network time: Tue, 15 Oct 2019 20:48:17 GMT
2019-10-15T20:48:17.5445628Z == end clock drift check ==
2019-10-15T20:48:19.7576259Z ##[error]Bash exited with code '1'.
2019-10-15T20:48:19.7616879Z ##[section]Starting: Upload CPU usage statistics
2019-10-15T20:48:19.7625552Z ==============================================================================
2019-10-15T20:48:19.7625898Z Task         : Bash
2019-10-15T20:48:19.7625999Z Description  : Run a Bash script on macOS, Linux, or Windows
