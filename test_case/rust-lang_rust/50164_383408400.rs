plain
[00:33:56]    Compiling pulldown-cmark v0.1.2
[00:33:56]    Compiling libc v0.2.40
[00:33:56]    Compiling bitflags v0.9.1
[00:33:56]    Compiling remove_dir_all v0.5.1
m        major |= (dev & 0xfffff00000000000) >> 32;
[00:33:58]      |                                             ^^ expected u64, found i32
[00:33:58] 
[00:33:58] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:33:58]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1437:45
[00:33:58]      |
[00:33:58] 1437 |         minor |= (dev & 0x00000000000000ff) >> 0;
[00:33:58]      |                                             ^^ expected u64, found i32
[00:33:58] 
[00:33:58] error[E0271]: type mismatch resolving `<u64 as std::ops::Shr<i32>>::Output == i32`
[00:33:58]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1438:45
[00:33:58]      |
[00:33:58] 1438 |         minor |= (dev & 0x00000ffffff00000) >> 12;
[00:33:58]      |                                             ^^ expected u64, found i32
[00:33:58] 
[00:33:58] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:33:58]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1446:37
[00:33:58]      |
[00:33:58] 1446 |         dev |= (major & 0x00000fff) << 8;
[00:33:58]      |                                     ^^ expected u64, found i32
[00:33:58] 
[00:33:58] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:33:58]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1447:37
[00:33:58]      |
[00:33:58] 1447 |         dev |= (major & 0xfffff000) << 32;
[00:33:58]      |                                     ^^ expected u64, found i32
[00:33:58] 
[00:33:58] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:33:58]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1448:37
[00:33:58]      |
[00:33:58] 1448 |         dev |= (minor & 0x000000ff) << 0;
[00:33:58]      or 1
[00:33:58] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11bf7d43
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
