
Dec 05 10:49:36.575 INFO [stderr] error[E0599]: no associated item named `O_CREAT` found for type `unix::file::OpenFlags` in the current scope
Dec 05 10:49:36.575 INFO [stderr]   --> /cargo-home/registry/src/github.com-1ecc6299db9ec823/unix-0.5.5/src/file.rs:44:35
Dec 05 10:49:36.575 INFO [stderr]    |
Dec 05 10:49:36.575 INFO [stderr] 44 | pub const O_CREAT   : OpenFlags = OpenFlags::O_CREAT;
Dec 05 10:49:36.575 INFO [stderr]    |                                   ^^^^^^^^^^^^^^^^^^ associated item not found in `unix::file::OpenFlags`
