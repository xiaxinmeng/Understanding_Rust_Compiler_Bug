plain
test src\net\udp.rs - net::udp::UdpSocket::take_error (line 599) ... ok
test src\net\udp.rs - net::udp::UdpSocket::try_clone (line 233) ... ok
test src\net\udp.rs - net::udp::UdpSocket::ttl (line 541) ... ok
test src\net\udp.rs - net::udp::UdpSocket::write_timeout (line 358) ... ok
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::as_raw_stat (line 27) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blocks (line 311) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime (line 258) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_dev (line 47) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime_nsec (line 277) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blksize (line 294) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime_nsec (line 205) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime (line 186) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_gid (line 132) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mode (line 81) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_nlink (line 98) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_rdev (line 149) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime_nsec (line 241) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime (line 222) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_size (line 169) ... FAILED
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ino (line 64) ... FAILED
test src\panic.rs - panic::AssertUnwindSafe (line 173) ... ok
test src\panic.rs - panic::AssertUnwindSafe (line 173) ... ok
test src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_uid (line 115) ... FAILED
test src\panicking.rs - panicking::take_hook (line 150) ... ok
test src\path.rs - path (line 23) ... ok
test src\panicking.rs - panicking::set_hook (line 104) ... ok
test src\panic.rs - panic::catch_unwind (line 416) ... ok
---
test src\thread\mod.rs - thread::spawn (line 574) ... ok

failures:

---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::as_raw_stat (line 27) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:30:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `as_raw_stat` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:34:21
  |
9 |     let stat = meta.as_raw_stat();
  |                     ^^^^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blocks (line 311) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:314:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_blocks` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:318:25
  |
9 |     println!("{}", meta.st_blocks());
  |                         ^^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime (line 258) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:261:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_ctime` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:265:25
  |
9 |     println!("{}", meta.st_ctime());
  |                         ^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_dev (line 47) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:50:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_dev` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:54:25
  |
9 |     println!("{}", meta.st_dev());
  |                         ^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime_nsec (line 277) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:280:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_ctime_nsec` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:284:25
  |
9 |     println!("{}", meta.st_ctime_nsec());
  |                         ^^^^^^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blksize (line 294) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:297:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_blksize` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:301:25
9 |     println!("{}", meta.st_blksize());
  |                         ^^^^^^^^^^ method not found in `Metadata`

error: aborting due to 2 previous errors
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime_nsec (line 205) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:208:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_atime_nsec` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:212:25
  |
9 |     println!("{}", meta.st_atime_nsec());
  |                         ^^^^^^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime (line 186) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:189:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_atime` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:193:25
  |
9 |     println!("{}", meta.st_atime());
  |                         ^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_gid (line 132) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:135:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_gid` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:139:25
  |
9 |     println!("{}", meta.st_gid());
  |                         ^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mode (line 81) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:84:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_mode` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:88:25
  |
9 |     println!("{}", meta.st_mode());
  |                         ^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_nlink (line 98) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:101:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_nlink` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:105:25
  |
9 |     println!("{}", meta.st_nlink());
  |                         ^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_rdev (line 149) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:152:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_rdev` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:156:25
  |
9 |     println!("{}", meta.st_rdev());
  |                         ^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime_nsec (line 241) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:244:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_mtime_nsec` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:248:25
  |
9 |     println!("{}", meta.st_mtime_nsec());
  |                         ^^^^^^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime (line 222) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:225:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_mtime` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:229:25
  |
9 |     println!("{}", meta.st_mtime());
  |                         ^^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_size (line 169) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:172:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_size` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:176:25
  |
9 |     println!("{}", meta.st_size());
  |                         ^^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ino (line 64) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:67:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_ino` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:71:25
  |
9 |     println!("{}", meta.st_ino());
  |                         ^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.
---- src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_uid (line 115) stdout ----
error[E0433]: failed to resolve: could not find `linux` in `os`
 --> src\os\linux\fs.rs:118:14
5 | use std::os::linux::fs::MetadataExt;
5 | use std::os::linux::fs::MetadataExt;
  |              ^^^^^ could not find `linux` in `os`

error[E0599]: no method named `st_uid` found for struct `Metadata` in the current scope
 --> src\os\linux\fs.rs:122:25
  |
9 |     println!("{}", meta.st_uid());
  |                         ^^^^^^ method not found in `Metadata`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0433, E0599.
For more information about an error, try `rustc --explain E0433`.
For more information about an error, try `rustc --explain E0433`.
Couldn't compile the test.

failures:
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::as_raw_stat (line 27)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime (line 186)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_atime_nsec (line 205)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blksize (line 294)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_blocks (line 311)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime (line 258)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ctime_nsec (line 277)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_dev (line 47)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_gid (line 132)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_ino (line 64)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mode (line 81)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime (line 222)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_mtime_nsec (line 241)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_nlink (line 98)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_rdev (line 149)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_size (line 169)
    src\os\linux\fs.rs - os::linux::fs::MetadataExt::st_uid (line 115)
test result: FAILED. 1022 passed; 17 failed; 20 ignored; 0 measured; 0 filtered out; finished in 34.97s

error: test failed, to rerun pass '--doc'



command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "--"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui --exclude src/tools/linkchecker
Build completed unsuccessfully in 0:40:01
Build completed unsuccessfully in 0:40:01
make: *** [Makefile:72: ci-subset-1] Error 1
