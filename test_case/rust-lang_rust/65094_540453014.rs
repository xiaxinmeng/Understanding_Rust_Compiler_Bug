plain
2019-10-10T08:09:19.8019111Z [RUSTC-TIMING] hashbrown test:false 0.984
2019-10-10T08:09:23.2127937Z error[E0308]: mismatched types
2019-10-10T08:09:23.2128761Z    --> src/libstd/sys/unix/fs.rs:101:29
2019-10-10T08:09:23.2129046Z     |
2019-10-10T08:09:23.2129519Z 101 |             stat.st_nlink = buf.stx_nlink as u64;
2019-10-10T08:09:23.2129976Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected u32, found u64
2019-10-10T08:09:23.2130366Z help: you can convert an `u64` to `u32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2130675Z     |
2019-10-10T08:09:23.2131060Z 101 |             stat.st_nlink = (buf.stx_nlink as u64).try_into().unwrap();
2019-10-10T08:09:23.2131755Z 
2019-10-10T08:09:23.2148960Z error[E0308]: mismatched types
2019-10-10T08:09:23.2149374Z    --> src/libstd/sys/unix/fs.rs:107:31
2019-10-10T08:09:23.2149664Z     |
2019-10-10T08:09:23.2149664Z     |
2019-10-10T08:09:23.2150053Z 107 |             stat.st_blksize = buf.stx_blksize as i64;
2019-10-10T08:09:23.2150504Z     |                               ^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2151282Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2151651Z     |
2019-10-10T08:09:23.2152032Z 107 |             stat.st_blksize = (buf.stx_blksize as i64).try_into().unwrap();
2019-10-10T08:09:23.2152779Z 
2019-10-10T08:09:23.2160791Z error[E0308]: mismatched types
2019-10-10T08:09:23.2161174Z    --> src/libstd/sys/unix/fs.rs:109:29
2019-10-10T08:09:23.2161450Z     |
2019-10-10T08:09:23.2161450Z     |
2019-10-10T08:09:23.2161821Z 109 |             stat.st_atime = buf.stx_atime.tv_sec;
2019-10-10T08:09:23.2162607Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2162988Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2163312Z     |
2019-10-10T08:09:23.2163710Z 109 |             stat.st_atime = buf.stx_atime.tv_sec.try_into().unwrap();
2019-10-10T08:09:23.2165985Z 
2019-10-10T08:09:23.2173259Z error[E0308]: mismatched types
2019-10-10T08:09:23.2173627Z    --> src/libstd/sys/unix/fs.rs:110:34
2019-10-10T08:09:23.2173900Z     |
2019-10-10T08:09:23.2173900Z     |
2019-10-10T08:09:23.2174297Z 110 |             stat.st_atime_nsec = buf.stx_atime.tv_nsec as i64;
2019-10-10T08:09:23.2174756Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2175863Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2176195Z     |
2019-10-10T08:09:23.2176602Z 110 |             stat.st_atime_nsec = (buf.stx_atime.tv_nsec as i64).try_into().unwrap();
2019-10-10T08:09:23.2177143Z 
2019-10-10T08:09:23.2188286Z error[E0308]: mismatched types
2019-10-10T08:09:23.2189014Z    --> src/libstd/sys/unix/fs.rs:111:29
2019-10-10T08:09:23.2189294Z     |
2019-10-10T08:09:23.2189294Z     |
2019-10-10T08:09:23.2189667Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec;
2019-10-10T08:09:23.2190105Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2190768Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2191074Z     |
2019-10-10T08:09:23.2191450Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec.try_into().unwrap();
2019-10-10T08:09:23.2192064Z 
2019-10-10T08:09:23.2199887Z error[E0308]: mismatched types
2019-10-10T08:09:23.2204638Z    --> src/libstd/sys/unix/fs.rs:112:34
2019-10-10T08:09:23.2206795Z     |
2019-10-10T08:09:23.2206795Z     |
2019-10-10T08:09:23.2207510Z 112 |             stat.st_mtime_nsec = buf.stx_mtime.tv_nsec as i64;
2019-10-10T08:09:23.2208255Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2209030Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2209544Z     |
2019-10-10T08:09:23.2210188Z 112 |             stat.st_mtime_nsec = (buf.stx_mtime.tv_nsec as i64).try_into().unwrap();
2019-10-10T08:09:23.2211077Z 
2019-10-10T08:09:23.2211745Z error[E0308]: mismatched types
2019-10-10T08:09:23.2212286Z    --> src/libstd/sys/unix/fs.rs:113:29
2019-10-10T08:09:23.2212772Z     |
2019-10-10T08:09:23.2212772Z     |
2019-10-10T08:09:23.2213476Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec;
2019-10-10T08:09:23.2214063Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2214626Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2216148Z     |
2019-10-10T08:09:23.2216774Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec.try_into().unwrap();
2019-10-10T08:09:23.2217633Z 
2019-10-10T08:09:23.2218103Z error[E0308]: mismatched types
2019-10-10T08:09:23.2218775Z    --> src/libstd/sys/unix/fs.rs:114:34
2019-10-10T08:09:23.2219449Z     |
2019-10-10T08:09:23.2219449Z     |
2019-10-10T08:09:23.2220017Z 114 |             stat.st_ctime_nsec = buf.stx_ctime.tv_nsec as i64;
2019-10-10T08:09:23.2220688Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-10T08:09:23.2221282Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-10T08:09:23.2221784Z     |
2019-10-10T08:09:23.2222378Z 114 |             stat.st_ctime_nsec = (buf.stx_ctime.tv_nsec as i64).try_into().unwrap();
2019-10-10T08:09:23.2223835Z 
2019-10-10T08:09:23.5100574Z error: aborting due to 8 previous errors
2019-10-10T08:09:23.5142941Z 
2019-10-10T08:09:23.5143943Z For more information about this error, try `rustc --explain E0308`.
---
2019-10-10T08:09:23.5700155Z == clock drift check ==
2019-10-10T08:09:23.5715859Z   local time: Thu Oct 10 08:09:23 UTC 2019
2019-10-10T08:09:23.8416114Z   network time: Thu, 10 Oct 2019 08:09:23 GMT
2019-10-10T08:09:23.8417045Z == end clock drift check ==
2019-10-10T08:09:25.4838511Z ##[error]Bash exited with code '1'.
2019-10-10T08:09:25.4875507Z ##[section]Starting: Upload CPU usage statistics
2019-10-10T08:09:25.4878983Z ==============================================================================
2019-10-10T08:09:25.4879078Z Task         : Bash
2019-10-10T08:09:25.4879190Z Description  : Run a Bash script on macOS, Linux, or Windows
