plain
[00:37:14]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:37:14]    Compiling cc v1.0.10
[00:37:14]    Compiling num_cpus v1.8.0
[00:37:16]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
ismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:38:08]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1447:37
[00:38:08]      |
[00:38:08] 1447 |         dev |= (major & 0xfffff000) << 32;
[00:38:08]      |                                     ^^ expected u64, found i32
[00:38:08] 
[00:38:08] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:38:08]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1448:37
[00:38:08]      |
[00:38:08] 1448 |         dev |= (minor & 0x000000ff) << 0;
[00:38:08]      |                                     ^^ expected u64, found i32
[00:38:08] 
[00:38:08] error[E0271]: type mismatch resolving `<u64 as std::ops::Shl<i32>>::Output == i32`
[00:38:08]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.40/src/unix/notbsd/linux/mod.rs:1449:37
[00:38:08]      |
[00:38:08] 1449 |         dev |= (minor & 0xffffff00Sun Apr 22 23:36:24 UTC 2018
travis_time:end:03d2e3fd:start=1524440184085647645,finish=1524440184141966552,duration=56318907

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
