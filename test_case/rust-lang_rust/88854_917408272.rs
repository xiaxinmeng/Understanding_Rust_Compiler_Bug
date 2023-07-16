plain
[RUSTC-TIMING] object test:false 6.569
[RUSTC-TIMING] gimli test:false 6.689
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-emscripten`

error[E0433]: failed to resolve: could not find `unix` in `os`
  |
  |
9 | use crate::os::unix::io::OwnedFd;
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
    |
206 |             use crate::os::unix::prelude::*;
206 |             use crate::os::unix::prelude::*;
    |                            ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
   |
76 |     use crate::os::unix::prelude::*;
76 |     use crate::os::unix::prelude::*;
   |                    ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/unix/fd.rs:8:16
  |
8 | use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
  |
1 | use crate::os::unix::prelude::*;
1 | use crate::os::unix::prelude::*;
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
  |
  |
7 | use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd};
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
     |
     |
1257 |     use crate::os::unix::fs::{OpenOptionsExt, PermissionsExt};
     |                    ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
  |
  |
6 | use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/unix/os.rs:8:16
8 | use crate::os::unix::prelude::*;
8 | use crate::os::unix::prelude::*;
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/unix/pipe.rs:3:16
  |
3 | use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/unix/process/process_common.rs:4:16
4 | use crate::os::unix::prelude::*;
4 | use crate::os::unix::prelude::*;
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: could not find `unix` in `os`
 --> library/std/src/sys/unix/stdio.rs:3:16
  |
3 | use crate::os::unix::io::{AsFd, BorrowedFd, FromRawFd};
  |                ^^^^ could not find `unix` in `os`

error[E0433]: failed to resolve: use of undeclared type `OsStringExt`
    |
    |
146 |                     OsStringExt::from_vec(cstr.to_bytes().to_vec())
    |                     ^^^^^^^^^^^ use of undeclared type `OsStringExt`

error[E0433]: failed to resolve: use of undeclared type `OsStrExt`
   --> library/std/src/sys/unix/os.rs:184:33
    |
184 |         PathBuf::from(<OsStr as OsStrExt>::from_bytes(b))
    |                                 ^^^^^^^^ use of undeclared type `OsStrExt`

error[E0433]: failed to resolve: use of undeclared type `OsStringExt`
   --> library/std/src/sys/unix/os.rs:227:8
    |
227 |     Ok(OsStringExt::from_vec(joined))
    |        ^^^^^^^^^^^ use of undeclared type `OsStringExt`

error[E0433]: failed to resolve: use of undeclared type `OsStringExt`
   --> library/std/src/sys/unix/os.rs:520:17
    |
520 |                 OsStringExt::from_vec(input[..p].to_vec()),
    |                 ^^^^^^^^^^^ use of undeclared type `OsStringExt`

error[E0433]: failed to resolve: use of undeclared type `OsStringExt`
   --> library/std/src/sys/unix/os.rs:521:17
    |
521 |                 OsStringExt::from_vec(input[p + 1..].to_vec()),
    |                 ^^^^^^^^^^^ use of undeclared type `OsStringExt`

error[E0433]: failed to resolve: use of undeclared type `OsStringExt`
   --> library/std/src/sys/unix/os.rs:537:18
    |
537 |             Some(OsStringExt::from_vec(CStr::from_ptr(s).to_bytes().to_vec()))
    |                  ^^^^^^^^^^^ use of undeclared type `OsStringExt`

error[E0433]: failed to resolve: use of undeclared type `OwnedFd`
    |
    |
153 |         unsafe { fs::File::from(OwnedFd::from_raw_fd(fd)) }
    |
help: consider importing this struct
    |
5   | use crate::os::fd::owned::OwnedFd;
5   | use crate::os::fd::owned::OwnedFd;
    |

error[E0412]: cannot find type `OwnedFd` in this scope
  --> library/std/src/sys/unix/fd.rs:15:21
   |
15 | pub struct FileDesc(OwnedFd);
   |
help: consider importing this struct
   |
   |
6  | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:299:14
    |
299 | impl AsInner<OwnedFd> for FileDesc {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:300:28
    |
300 |     fn as_inner(&self) -> &OwnedFd {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:305:16
    |
305 | impl IntoInner<OwnedFd> for FileDesc {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:306:28
    |
306 |     fn into_inner(self) -> OwnedFd {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:311:16
    |
311 | impl FromInner<OwnedFd> for FileDesc {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0412]: cannot find type `OwnedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:312:29
    |
312 |     fn from_inner(owned_fd: OwnedFd) -> Self {
    |
help: consider importing this struct
    |
    |
6   | use crate::os::fd::net::OwnedFd;


error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/fd.rs:317:6
    |
317 | impl AsFd for FileDesc {
    |
help: consider importing this trait
    |
6   | use crate::os::fd::owned::AsFd;
6   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/fd.rs:318:24
    |
318 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
6   | use crate::os::fd::owned::BorrowedFd;
6   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsRawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:323:6
    |
323 | impl AsRawFd for FileDesc {
    |
help: consider importing this trait
    |
6   | use crate::os::fd::net::AsRawFd;
6   | use crate::os::fd::net::AsRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:324:28
    |
324 |     fn as_raw_fd(&self) -> RawFd {
    |
help: consider importing this type alias
    |
6   | use crate::os::fd::net::RawFd;
6   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `IntoRawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:329:6
329 | impl IntoRawFd for FileDesc {
    |      ^^^^^^^^^ not found in this scope
    |
help: consider importing this trait
help: consider importing this trait
    |
6   | use crate::os::fd::net::IntoRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:330:29
    |
330 |     fn into_raw_fd(self) -> RawFd {
    |
help: consider importing this type alias
    |
6   | use crate::os::fd::net::RawFd;
6   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `FromRawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:335:6
    |
335 | impl FromRawFd for FileDesc {
    |
help: consider importing this trait
    |
6   | use crate::os::fd::net::FromRawFd;
6   | use crate::os::fd::net::FromRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fd.rs:336:35
    |
336 |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
    |
help: consider importing this type alias
    |
6   | use crate::os::fd::net::RawFd;
6   | use crate::os::fd::net::RawFd;
    |

error[E0433]: failed to resolve: use of undeclared type `FromRawFd`
   --> library/std/src/sys/unix/fd.rs:337:14
    |
337 |         Self(FromRawFd::from_raw_fd(raw_fd))
    |
help: consider importing this trait
    |
6   | use crate::os::fd::net::FromRawFd;
6   | use crate::os::fd::net::FromRawFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
    |
    |
954 | impl AsFd for File {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
    |
    |
955 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsRawFd` in this scope
    |
    |
960 | impl AsRawFd for File {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::AsRawFd;
1   | use crate::os::fd::net::AsRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fs.rs:961:28
    |
961 |     fn as_raw_fd(&self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `IntoRawFd` in this scope
    |
966 | impl IntoRawFd for File {
    |      ^^^^^^^^^ not found in this scope
    |
---

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fs.rs:967:29
    |
967 |     fn into_raw_fd(self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `FromRawFd` in this scope
   --> library/std/src/sys/unix/fs.rs:972:6
    |
972 | impl FromRawFd for File {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/fs.rs:973:35
    |
973 |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0433]: failed to resolve: use of undeclared type `FromRawFd`
   --> library/std/src/sys/unix/fs.rs:974:14
    |
974 |         Self(FromRawFd::from_raw_fd(raw_fd))
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/net.rs:441:29
    |
441 |     pub fn as_raw(&self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
    |
    |
464 | impl AsFd for Socket {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
    |
    |
465 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsRawFd` in this scope
    |
    |
470 | impl AsRawFd for Socket {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::AsRawFd;
1   | use crate::os::fd::net::AsRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/net.rs:471:28
    |
471 |     fn as_raw_fd(&self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `IntoRawFd` in this scope
    |
476 | impl IntoRawFd for Socket {
    |      ^^^^^^^^^ not found in this scope
    |
---

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/net.rs:477:29
    |
477 |     fn into_raw_fd(self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `FromRawFd` in this scope
   --> library/std/src/sys/unix/net.rs:482:6
    |
482 | impl FromRawFd for Socket {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/net.rs:483:35
    |
483 |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0433]: failed to resolve: use of undeclared type `FromRawFd`
   --> library/std/src/sys/unix/net.rs:484:14
    |
484 |         Self(FromRawFd::from_raw_fd(raw_fd))
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0405]: cannot find trait `AsRawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:129:6
    |
129 | impl AsRawFd for AnonPipe {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::AsRawFd;
1   | use crate::os::fd::net::AsRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:130:28
    |
130 |     fn as_raw_fd(&self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:135:6
    |
135 | impl AsFd for AnonPipe {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:136:24
    |
136 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `IntoRawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:141:6
    |
141 | impl IntoRawFd for AnonPipe {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::IntoRawFd;
1   | use crate::os::fd::net::IntoRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:142:29
    |
142 |     fn into_raw_fd(self) -> RawFd {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0405]: cannot find trait `FromRawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:147:6
    |
147 | impl FromRawFd for AnonPipe {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0412]: cannot find type `RawFd` in this scope
   --> library/std/src/sys/unix/pipe.rs:148:35
    |
148 |     unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
    |
help: consider importing this type alias
    |
1   | use crate::os::fd::net::RawFd;
1   | use crate::os::fd::net::RawFd;
    |

error[E0433]: failed to resolve: use of undeclared type `FromRawFd`
   --> library/std/src/sys/unix/pipe.rs:149:14
    |
149 |         Self(FromRawFd::from_raw_fd(raw_fd))
    |
help: consider importing this trait
    |
1   | use crate::os::fd::net::FromRawFd;
1   | use crate::os::fd::net::FromRawFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
  --> library/std/src/sys/unix/stdio.rs:96:6
   |
96 | impl AsFd for io::Stdin {
   |
help: consider importing this trait
   |
1  | use crate::os::fd::owned::AsFd;
1  | use crate::os::fd::owned::AsFd;
   |

error[E0412]: cannot find type `BorrowedFd` in this scope
  --> library/std/src/sys/unix/stdio.rs:98:24
   |
98 |     fn as_fd(&self) -> BorrowedFd<'_> {
   |
help: consider importing this struct
   |
1  | use crate::os::fd::owned::BorrowedFd;
1  | use crate::os::fd::owned::BorrowedFd;
   |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
  --> library/std/src/sys/unix/stdio.rs:99:18
   |
99 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDIN_FILENO) }
   |
help: consider importing this struct
   |
1  | use crate::os::fd::owned::BorrowedFd;
1  | use crate::os::fd::owned::BorrowedFd;
   |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:104:10
    |
104 | impl<'a> AsFd for io::StdinLock<'a> {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:106:24
    |
106 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
   --> library/std/src/sys/unix/stdio.rs:107:18
    |
107 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDIN_FILENO) }
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:112:6
    |
112 | impl AsFd for io::Stdout {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:114:24
    |
114 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
   --> library/std/src/sys/unix/stdio.rs:115:18
    |
115 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDOUT_FILENO) }
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:120:10
    |
120 | impl<'a> AsFd for io::StdoutLock<'a> {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:122:24
    |
122 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
   --> library/std/src/sys/unix/stdio.rs:123:18
    |
123 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDOUT_FILENO) }
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:128:6
    |
128 | impl AsFd for io::Stderr {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:130:24
    |
130 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
   --> library/std/src/sys/unix/stdio.rs:131:18
    |
131 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDERR_FILENO) }
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0405]: cannot find trait `AsFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:136:10
    |
136 | impl<'a> AsFd for io::StderrLock<'a> {
    |
help: consider importing this trait
    |
1   | use crate::os::fd::owned::AsFd;
1   | use crate::os::fd::owned::AsFd;
    |

error[E0412]: cannot find type `BorrowedFd` in this scope
   --> library/std/src/sys/unix/stdio.rs:138:24
    |
138 |     fn as_fd(&self) -> BorrowedFd<'_> {
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
1   | use crate::os::fd::owned::BorrowedFd;
    |

error[E0433]: failed to resolve: use of undeclared type `BorrowedFd`
   --> library/std/src/sys/unix/stdio.rs:139:18
    |
139 |         unsafe { BorrowedFd::borrow_raw_fd(libc::STDERR_FILENO) }
    |
help: consider importing this struct
    |
1   | use crate::os::fd::owned::BorrowedFd;
