plain
[00:34:46]    Compiling pulldown-cmark v0.1.2
[00:34:46]    Compiling libc v0.2.40
[00:34:46]    Compiling remove_dir_all v0.5.1
[00:34:46]    Compiling bitflags v0.9.1
    const OPTION_ENABLE_FOOTNOTES = 1 << 2;
[00:34:47]     |                                           ^^ expected u32, found i32
[00:34:47] error: aborting due to 3 previous errors
[00:34:47] 
[00:34:47] For more information about this error, try `rustc --explain E0271`.
[00:34:47] For more information about this error, try `rustc --explain E0271`.
[00:34:47] error: Could not compile `pulldown-cmark`.
[00:34:47] warning: build failed, waiting for other jobs to finish...
[00:34:48] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:34:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1430:45
[00:34:48]      |
[00:34:48] 1430 |         major |= (dev & 0x00000000000fff00) >> 8;
[00:34:48]      |                                             ^^ expected u64, found i32
[00:34:48] 
[00:34:48] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:34:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1431:45
[00:34:48]      |
[00:34:48] 1431 |         major |= (dev & 0xfffff00000000000) >> 32;
[00:34:48]      |                                             ^^ expected u64, found i32
[00:34:48] 
[00:34:48] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:34:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1437:45
[00:34:48]      |
[00:34:48] 1437 |         minor |= (dev & 0x00000000000000ff) >> 0;
[00:34:48]      |                                             ^^ expected u64, found i32
[00:34:48] 
[00:34:48] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:34:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1438:45
[00:34:48]      |
[00:34:48] 1438 |         minor |= (dev & 0x00000ffffff00000) >> 12;
[00:34:48]      |                                             ^^ expected u64, found i32
[00:34:48] 
[00:34:48] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:34:48]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1446:37
[00:34:48]      |
[00:34:48] 1446 |         dev |= (major & 0x00000fff) << 8;
[00:34:48]      |                                     ^^ expected u64, found i32
[00:34:48] 
[00:34:48] error[E0271]: type m grep ^Date: | sed 's/Date: //g' || true)
travis_fold:start:after_failure.1
travis_time:start:15532440
