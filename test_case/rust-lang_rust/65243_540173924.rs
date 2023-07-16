plain
2019-10-09T20:16:51.2502348Z [RUSTC-TIMING] hashbrown test:false 0.984
2019-10-09T20:16:54.5948740Z error[E0308]: mismatched types
2019-10-09T20:16:54.5950024Z    --> src/libstd/sys/unix/fs.rs:101:29
2019-10-09T20:16:54.5951478Z     |
2019-10-09T20:16:54.5952105Z 101 |             stat.st_nlink = buf.stx_nlink as u64;
2019-10-09T20:16:54.5952728Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected u32, found u64
2019-10-09T20:16:54.5953459Z help: you can convert an `u64` to `u32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.5953902Z     |
2019-10-09T20:16:54.5954295Z 101 |             stat.st_nlink = (buf.stx_nlink as u64).try_into().unwrap();
2019-10-09T20:16:54.5955020Z 
2019-10-09T20:16:54.5962816Z error[E0308]: mismatched types
2019-10-09T20:16:54.5963716Z    --> src/libstd/sys/unix/fs.rs:107:31
2019-10-09T20:16:54.5964149Z     |
2019-10-09T20:16:54.5964149Z     |
2019-10-09T20:16:54.5964666Z 107 |             stat.st_blksize = buf.stx_blksize as i64;
2019-10-09T20:16:54.5965244Z     |                               ^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.5965757Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.5966766Z     |
2019-10-09T20:16:54.5967392Z 107 |             stat.st_blksize = (buf.stx_blksize as i64).try_into().unwrap();
2019-10-09T20:16:54.5968285Z 
2019-10-09T20:16:54.5977287Z error[E0308]: mismatched types
2019-10-09T20:16:54.5977956Z    --> src/libstd/sys/unix/fs.rs:109:29
2019-10-09T20:16:54.5978477Z     |
2019-10-09T20:16:54.5978477Z     |
2019-10-09T20:16:54.5979353Z 109 |             stat.st_atime = buf.stx_atime.tv_sec;
2019-10-09T20:16:54.5980285Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.5980816Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.5981242Z     |
2019-10-09T20:16:54.5981923Z 109 |             stat.st_atime = buf.stx_atime.tv_sec.try_into().unwrap();
2019-10-09T20:16:54.5982717Z 
2019-10-09T20:16:54.5983372Z error[E0308]: mismatched types
2019-10-09T20:16:54.5983681Z    --> src/libstd/sys/unix/fs.rs:110:34
2019-10-09T20:16:54.5983936Z     |
2019-10-09T20:16:54.5983936Z     |
2019-10-09T20:16:54.5984251Z 110 |             stat.st_atime_nsec = buf.stx_atime.tv_nsec as i64;
2019-10-09T20:16:54.5984874Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.5985408Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.5986236Z     |
2019-10-09T20:16:54.5986961Z 110 |             stat.st_atime_nsec = (buf.stx_atime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:16:54.5987877Z 
2019-10-09T20:16:54.5993736Z error[E0308]: mismatched types
2019-10-09T20:16:54.5994247Z    --> src/libstd/sys/unix/fs.rs:111:29
2019-10-09T20:16:54.5994690Z     |
2019-10-09T20:16:54.5994690Z     |
2019-10-09T20:16:54.5995183Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec;
2019-10-09T20:16:54.5995726Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.5996866Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.5997397Z     |
2019-10-09T20:16:54.5997984Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec.try_into().unwrap();
2019-10-09T20:16:54.5998864Z 
2019-10-09T20:16:54.6002973Z error[E0308]: mismatched types
2019-10-09T20:16:54.6003697Z    --> src/libstd/sys/unix/fs.rs:112:34
2019-10-09T20:16:54.6004126Z     |
2019-10-09T20:16:54.6004126Z     |
2019-10-09T20:16:54.6004652Z 112 |             stat.st_mtime_nsec = buf.stx_mtime.tv_nsec as i64;
2019-10-09T20:16:54.6005269Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.6006737Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.6007557Z     |
2019-10-09T20:16:54.6008196Z 112 |             stat.st_mtime_nsec = (buf.stx_mtime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:16:54.6012743Z 
2019-10-09T20:16:54.6018303Z error[E0308]: mismatched types
2019-10-09T20:16:54.6018937Z    --> src/libstd/sys/unix/fs.rs:113:29
2019-10-09T20:16:54.6019784Z     |
2019-10-09T20:16:54.6019784Z     |
2019-10-09T20:16:54.6020355Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec;
2019-10-09T20:16:54.6020943Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.6021504Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.6021934Z     |
2019-10-09T20:16:54.6022460Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec.try_into().unwrap();
2019-10-09T20:16:54.6023376Z 
2019-10-09T20:16:54.6030429Z error[E0308]: mismatched types
2019-10-09T20:16:54.6030982Z    --> src/libstd/sys/unix/fs.rs:114:34
2019-10-09T20:16:54.6031445Z     |
2019-10-09T20:16:54.6031445Z     |
2019-10-09T20:16:54.6031976Z 114 |             stat.st_ctime_nsec = buf.stx_ctime.tv_nsec as i64;
2019-10-09T20:16:54.6032564Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:16:54.6033109Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:16:54.6033733Z     |
2019-10-09T20:16:54.6034263Z 114 |             stat.st_ctime_nsec = (buf.stx_ctime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:16:54.6035060Z 
2019-10-09T20:16:54.8866852Z error: aborting due to 8 previous errors
2019-10-09T20:16:54.8868039Z 
2019-10-09T20:16:54.8868631Z For more information about this error, try `rustc --explain E0308`.
---
2019-10-09T20:16:54.9470285Z == clock drift check ==
2019-10-09T20:16:54.9484485Z   local time: Wed Oct  9 20:16:54 UTC 2019
2019-10-09T20:16:54.9909569Z   network time: Wed, 09 Oct 2019 20:16:54 GMT
2019-10-09T20:16:54.9912495Z == end clock drift check ==
2019-10-09T20:16:56.4218180Z ##[error]Bash exited with code '1'.
2019-10-09T20:16:56.4252911Z ##[section]Starting: Upload CPU usage statistics
2019-10-09T20:16:56.4269910Z ==============================================================================
2019-10-09T20:16:56.4270027Z Task         : Bash
2019-10-09T20:16:56.4270274Z Description  : Run a Bash script on macOS, Linux, or Windows
