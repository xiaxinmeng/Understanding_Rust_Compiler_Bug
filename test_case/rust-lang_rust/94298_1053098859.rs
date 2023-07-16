plain
[RUSTC-TIMING] core test:false 21.395
[RUSTC-TIMING] addr2line test:false 0.362
[RUSTC-TIMING] gimli test:false 4.019
[RUSTC-TIMING] object test:false 4.250
error: unexpected `cfg` condition name
  --> library/std/src/os/freebsd/fs.rs:79:15
79 |         #[cfg(freebsd12)]
   |               ^^^^^^^^^
   |
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`

error: unexpected `cfg` condition name
  --> library/std/src/os/freebsd/fs.rs:81:19
   |
81 |         #[cfg(not(freebsd12))]


error: unexpected `cfg` condition name
   --> library/std/src/os/freebsd/fs.rs:146:11
146 |     #[cfg(freebsd12)]
    |           ^^^^^^^^^


error: unexpected `cfg` condition name
   --> library/std/src/os/freebsd/fs.rs:150:15
    |
150 |     #[cfg(not(freebsd12))]

[RUSTC-TIMING] std test:false 3.280
error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:05:39
