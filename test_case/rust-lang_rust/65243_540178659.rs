plain
2019-10-09T20:27:22.5409497Z [RUSTC-TIMING] hashbrown test:false 0.874
2019-10-09T20:27:25.6628959Z error[E0308]: mismatched types
2019-10-09T20:27:25.6629696Z    --> src/libstd/sys/unix/fs.rs:101:29
2019-10-09T20:27:25.6629919Z     |
2019-10-09T20:27:25.6630218Z 101 |             stat.st_nlink = buf.stx_nlink as u64;
2019-10-09T20:27:25.6630571Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected u32, found u64
2019-10-09T20:27:25.6630913Z help: you can convert an `u64` to `u32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6631156Z     |
2019-10-09T20:27:25.6631490Z 101 |             stat.st_nlink = (buf.stx_nlink as u64).try_into().unwrap();
2019-10-09T20:27:25.6632267Z 
2019-10-09T20:27:25.6672744Z error[E0308]: mismatched types
2019-10-09T20:27:25.6674435Z    --> src/libstd/sys/unix/fs.rs:107:31
2019-10-09T20:27:25.6675158Z     |
2019-10-09T20:27:25.6675158Z     |
2019-10-09T20:27:25.6679136Z 107 |             stat.st_blksize = buf.stx_blksize as i64;
2019-10-09T20:27:25.6680544Z     |                               ^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6682032Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6682742Z     |
2019-10-09T20:27:25.6683347Z 107 |             stat.st_blksize = (buf.stx_blksize as i64).try_into().unwrap();
2019-10-09T20:27:25.6684175Z 
2019-10-09T20:27:25.6684613Z error[E0308]: mismatched types
2019-10-09T20:27:25.6685101Z    --> src/libstd/sys/unix/fs.rs:109:29
2019-10-09T20:27:25.6685862Z     |
2019-10-09T20:27:25.6685862Z     |
2019-10-09T20:27:25.6686506Z 109 |             stat.st_atime = buf.stx_atime.tv_sec;
2019-10-09T20:27:25.6687264Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6687891Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6688904Z     |
2019-10-09T20:27:25.6689595Z 109 |             stat.st_atime = buf.stx_atime.tv_sec.try_into().unwrap();
2019-10-09T20:27:25.6690323Z 
2019-10-09T20:27:25.6690715Z error[E0308]: mismatched types
2019-10-09T20:27:25.6691148Z    --> src/libstd/sys/unix/fs.rs:110:34
2019-10-09T20:27:25.6691565Z     |
2019-10-09T20:27:25.6691565Z     |
2019-10-09T20:27:25.6692928Z 110 |             stat.st_atime_nsec = buf.stx_atime.tv_nsec as i64;
2019-10-09T20:27:25.6693916Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6694520Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6695116Z     |
2019-10-09T20:27:25.6696171Z 110 |             stat.st_atime_nsec = (buf.stx_atime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:27:25.6696900Z 
2019-10-09T20:27:25.6697254Z error[E0308]: mismatched types
2019-10-09T20:27:25.6697710Z    --> src/libstd/sys/unix/fs.rs:111:29
2019-10-09T20:27:25.6698120Z     |
2019-10-09T20:27:25.6698120Z     |
2019-10-09T20:27:25.6698983Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec;
2019-10-09T20:27:25.6699723Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6700209Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6700606Z     |
2019-10-09T20:27:25.6701254Z 111 |             stat.st_mtime = buf.stx_mtime.tv_sec.try_into().unwrap();
2019-10-09T20:27:25.6702633Z 
2019-10-09T20:27:25.6703237Z error[E0308]: mismatched types
2019-10-09T20:27:25.6703760Z    --> src/libstd/sys/unix/fs.rs:112:34
2019-10-09T20:27:25.6704398Z     |
2019-10-09T20:27:25.6704398Z     |
2019-10-09T20:27:25.6705014Z 112 |             stat.st_mtime_nsec = buf.stx_mtime.tv_nsec as i64;
2019-10-09T20:27:25.6706137Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6706638Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6707432Z     |
2019-10-09T20:27:25.6708132Z 112 |             stat.st_mtime_nsec = (buf.stx_mtime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:27:25.6709789Z 
2019-10-09T20:27:25.6718013Z error[E0308]: mismatched types
2019-10-09T20:27:25.6718678Z    --> src/libstd/sys/unix/fs.rs:113:29
2019-10-09T20:27:25.6718914Z     |
2019-10-09T20:27:25.6718914Z     |
2019-10-09T20:27:25.6719209Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec;
2019-10-09T20:27:25.6719540Z     |                             ^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6719861Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6720096Z     |
2019-10-09T20:27:25.6720392Z 113 |             stat.st_ctime = buf.stx_ctime.tv_sec.try_into().unwrap();
2019-10-09T20:27:25.6720994Z 
2019-10-09T20:27:25.6721221Z error[E0308]: mismatched types
2019-10-09T20:27:25.6721473Z    --> src/libstd/sys/unix/fs.rs:114:34
2019-10-09T20:27:25.6721678Z     |
2019-10-09T20:27:25.6721678Z     |
2019-10-09T20:27:25.6722832Z 114 |             stat.st_ctime_nsec = buf.stx_ctime.tv_nsec as i64;
2019-10-09T20:27:25.6723304Z     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected i32, found i64
2019-10-09T20:27:25.6723733Z help: you can convert an `i64` to `i32` and panic if the converted value wouldn't fit
2019-10-09T20:27:25.6724047Z     |
2019-10-09T20:27:25.6724441Z 114 |             stat.st_ctime_nsec = (buf.stx_ctime.tv_nsec as i64).try_into().unwrap();
2019-10-09T20:27:25.6724970Z 
2019-10-09T20:27:25.9601028Z error: aborting due to 8 previous errors
2019-10-09T20:27:25.9601212Z 
2019-10-09T20:27:25.9601792Z For more information about this error, try `rustc --explain E0308`.
---
2019-10-09T20:27:26.0205005Z == clock drift check ==
2019-10-09T20:27:26.0223286Z   local time: Wed Oct  9 20:27:26 UTC 2019
2019-10-09T20:27:26.1602897Z   network time: Wed, 09 Oct 2019 20:27:26 GMT
2019-10-09T20:27:26.1603113Z == end clock drift check ==
2019-10-09T20:27:27.8706095Z ##[error]Bash exited with code '1'.
2019-10-09T20:27:27.8749164Z ##[section]Starting: Upload CPU usage statistics
2019-10-09T20:27:27.8751748Z ==============================================================================
2019-10-09T20:27:27.8751821Z Task         : Bash
2019-10-09T20:27:27.8752344Z Description  : Run a Bash script on macOS, Linux, or Windows
