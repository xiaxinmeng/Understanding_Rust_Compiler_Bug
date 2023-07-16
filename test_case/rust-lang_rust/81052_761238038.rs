plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.9.0
   Compiling object v0.22.0
   Compiling addr2line v0.14.0
error: expected one of `!`, `(`, `+`, `::`, `<`, `where`, or `{`, found `;`
     |
     |
2495 | impl<T> SizeHint for T;
     |                       ^ expected one of 7 possible tokens

error[E0432]: unresolved imports `crate::io::Initializer`, `crate::io::IoSlice`, `crate::io::IoSliceMut`, `crate::io::Read`, `crate::io::Seek`, `crate::io::SeekFrom`, `crate::io::Write`
   |
   |
16 | use crate::io::{self, Initializer, IoSlice, IoSliceMut, Read, Seek, SeekFrom, Write};
   |                       ^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^  ^^^^  ^^^^  ^^^^^^^^  ^^^^^ no `Write` in `io`
   |                       |            |        |           |     |     |
   |                       |            |        |           |     |     no `SeekFrom` in `io`
   |                       |            |        |           |     no `Seek` in `io`
   |                       |            |        |           no `Read` in `io`
   |                       |            |        no `IoSliceMut` in `io`
   |                       |            no `IoSlice` in `io`
   |                       no `Initializer` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`
  --> library/std/src/net/mod.rs:20:23
   |
20 | use crate::io::{self, Error, ErrorKind};
   |                       ^^^^^  ^^^^^^^^^ no `ErrorKind` in `io`
   |                       |
   |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::Write`
 --> library/std/src/net/addr.rs:8:23
  |
  |
8 | use crate::io::{self, Write};
  |                       ^^^^^ no `Write` in `io`
error[E0432]: unresolved import `crate::io::Write`
  --> library/std/src/net/ip.rs:16:5
   |
   |
16 | use crate::io::Write as IoWrite;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `Write` in `io`
error[E0432]: unresolved import `crate::io::prelude`
 --> library/std/src/net/tcp.rs:6:16
  |
6 | use crate::io::prelude::*;
6 | use crate::io::prelude::*;
  |                ^^^^^^^ could not find `prelude` in `io`

error[E0432]: unresolved imports `crate::io::Initializer`, `crate::io::IoSlice`, `crate::io::IoSliceMut`
  |
  |
9 | use crate::io::{self, Initializer, IoSlice, IoSliceMut};
  |                       ^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
  |                       |            |
  |                       |            no `IoSlice` in `io`
  |                       no `Initializer` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`
  |
  |
5 | use crate::io::{self, Error, ErrorKind};
  |                       ^^^^^  ^^^^^^^^^ no `ErrorKind` in `io`
  |                       |
  |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::prelude`
   --> library/std/src/process.rs:103:16
    |
103 | use crate::io::prelude::*;
103 | use crate::io::prelude::*;
    |                ^^^^^^^ could not find `prelude` in `io`

error[E0432]: unresolved imports `crate::io::Initializer`, `crate::io::IoSlice`, `crate::io::IoSliceMut`
    |
    |
108 | use crate::io::{self, Initializer, IoSlice, IoSliceMut};
    |                       ^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
    |                       |            |
    |                       |            no `IoSlice` in `io`
    |                       no `Initializer` in `io`
error[E0432]: unresolved import `crate::io::prelude`
 --> library/std/src/sys_common/backtrace.rs:8:16
  |
8 | use crate::io::prelude::*;
8 | use crate::io::prelude::*;
  |                ^^^^^^^ could not find `prelude` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`
 --> library/std/src/sys_common/fs.rs:4:23
  |
4 | use crate::io::{self, Error, ErrorKind};
  |                       ^^^^^  ^^^^^^^^^ no `ErrorKind` in `io`
  |                       |
  |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::prelude`
 --> library/std/src/sys_common/util.rs:2:16
  |
2 | use crate::io::prelude::*;
2 | use crate::io::prelude::*;
  |                ^^^^^^^ could not find `prelude` in `io`
error[E0432]: unresolved import `crate::io::set_output_capture`
  --> library/std/src/panicking.rs:27:5
   |
27 | use crate::io::set_output_capture;
27 | use crate::io::set_output_capture;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `set_output_capture` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`, `crate::io::IoSlice`, `crate::io::IoSliceMut`
 --> library/std/src/sys_common/net.rs:8:23
  |
8 | use crate::io::{self, Error, ErrorKind, IoSlice, IoSliceMut};
  |                       ^^^^^  ^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
  |                       |      |          |
  |                       |      |          no `IoSlice` in `io`
  |                       |      no `ErrorKind` in `io`
  |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::ErrorKind`
 --> library/std/src/sys/unix/mod.rs:3:5
  |
3 | use crate::io::ErrorKind;
3 | use crate::io::ErrorKind;
  |     ^^^^^^^^^^^^^^^^^^^^ no `ErrorKind` in `io`

error[E0432]: unresolved imports `io::Read`, `io::Write`
   |
   |
13 | use io::{Read, Write};
   |          ^^^^  ^^^^^ no `Write` in `io`
   |          |
   |          no `Read` in `io`
error[E0432]: unresolved import `crate::io::IoSliceMut`
 --> library/std/src/sys/unix/ext/net/ancillary.rs:3:23
  |
  |
3 | use crate::io::{self, IoSliceMut};
  |                       ^^^^^^^^^^ no `IoSliceMut` in `io`
error[E0432]: unresolved import `crate::io::IoSliceMut`
  --> library/std/src/sys/unix/ext/net/datagram.rs:22:5
   |
22 | use crate::io::IoSliceMut;
22 | use crate::io::IoSliceMut;
   |     ^^^^^^^^^^^^^^^^^^^^^ no `IoSliceMut` in `io`

error[E0432]: unresolved imports `crate::io::Initializer`, `crate::io::IoSlice`, `crate::io::IoSliceMut`
   |
   |
14 | use crate::io::{self, Initializer, IoSlice, IoSliceMut};
   |                       ^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
   |                       |            |
   |                       |            no `IoSlice` in `io`
   |                       no `Initializer` in `io`

error[E0432]: unresolved imports `crate::io::Initializer`, `crate::io::IoSlice`, `crate::io::IoSliceMut`, `crate::io::Read`
 --> library/std/src/sys/unix/fd.rs:7:23
  |
7 | use crate::io::{self, Initializer, IoSlice, IoSliceMut, Read};
  |                       ^^^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^  ^^^^ no `Read` in `io`
  |                       |            |        |
  |                       |            |        no `IoSliceMut` in `io`
  |                       |            no `IoSlice` in `io`
  |                       no `Initializer` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`, `crate::io::IoSlice`, `crate::io::IoSliceMut`, `crate::io::SeekFrom`
  |
  |
5 | use crate::io::{self, Error, ErrorKind, IoSlice, IoSliceMut, SeekFrom};
  |                       ^^^^^  ^^^^^^^^^  ^^^^^^^  ^^^^^^^^^^  ^^^^^^^^ no `SeekFrom` in `io`
  |                       |      |          |        |
  |                       |      |          |        no `IoSliceMut` in `io`
  |                       |      |          no `IoSlice` in `io`
  |                       |      no `ErrorKind` in `io`
  |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::copy`
  --> library/std/src/sys/unix/kernel_copy.rs:50:16
   |
50 | use crate::io::copy::generic_copy;
50 | use crate::io::copy::generic_copy;
   |                ^^^^ could not find `copy` in `io`

error[E0432]: unresolved imports `crate::io::BufRead`, `crate::io::BufReader`, `crate::io::BufWriter`, `crate::io::Error`, `crate::io::Read`, `crate::io::Result`, `crate::io::StderrLock`, `crate::io::StdinLock`, `crate::io::StdoutLock`, `crate::io::Take`, `crate::io::Write`
  --> library/std/src/sys/unix/kernel_copy.rs:52:5
   |
52 |     BufRead, BufReader, BufWriter, Error, Read, Result, StderrLock, StdinLock, StdoutLock, Take,
   |     ^^^^^^^  ^^^^^^^^^  ^^^^^^^^^  ^^^^^  ^^^^  ^^^^^^  ^^^^^^^^^^  ^^^^^^^^^  ^^^^^^^^^^  ^^^^ no `Take` in `io`
   |     |        |          |          |      |     |       |           |          |
   |     |        |          |          |      |     |       |           |          no `StdoutLock` in `io`
   |     |        |          |          |      |     |       |           no `StdinLock` in `io`
   |     |        |          |          |      |     |       no `StderrLock` in `io`
   |     |        |          |          |      |     no `Result` in `io`
   |     |        |          |          |      no `Read` in `io`
   |     |        |          |          no `Error` in `io`
   |     |        |          no `BufWriter` in `io`
   |     |        no `BufReader` in `io`
   |     no `BufRead` in `io`
   |     ^^^^^


error[E0432]: unresolved imports `crate::io::IoSlice`, `crate::io::IoSliceMut`
 --> library/std/src/sys/unix/net.rs:3:23
  |
3 | use crate::io::{self, IoSlice, IoSliceMut};
  |                       ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
  |                       |
  |                       no `IoSlice` in `io`

error[E0432]: unresolved imports `crate::io::IoSlice`, `crate::io::IoSliceMut`
 --> library/std/src/sys/unix/pipe.rs:1:23
  |
1 | use crate::io::{self, IoSlice, IoSliceMut};
  |                       ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
  |                       |
  |                       no `IoSlice` in `io`

error[E0432]: unresolved imports `crate::io::Error`, `crate::io::ErrorKind`
 --> library/std/src/sys/unix/process/process_unix.rs:3:23
  |
3 | use crate::io::{self, Error, ErrorKind};
  |                       ^^^^^  ^^^^^^^^^ no `ErrorKind` in `io`
  |                       |
  |                       no `Error` in `io`
error[E0432]: unresolved import `crate::io::Read`
  --> library/std/src/sys/unix/rand.rs:25:9
   |
25 |     use crate::io::Read;
25 |     use crate::io::Read;
   |         ^^^^^^^^^^^^^^^ no `Read` in `io`

error[E0432]: unresolved imports `crate::io::IoSlice`, `crate::io::IoSliceMut`
 --> library/std/src/sys/unix/stdio.rs:1:23
  |
1 | use crate::io::{self, IoSlice, IoSliceMut};
  |                       ^^^^^^^  ^^^^^^^^^^ no `IoSliceMut` in `io`
  |                       |
  |                       no `IoSlice` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   |
   |
84 |                 0 => Err(io::Error::new(io::ErrorKind::NotFound, "The number of hardware threads is not known for the target platform")),
   |                                             ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
     |
     |
1041 |         io::Error::new(io::ErrorKind::InvalidInput, "data provided contains a nul byte")
     |                            ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
     |
     |
2181 |             Err(ref e) if e.kind() == io::ErrorKind::NotFound => {}
     |                                           ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
     |
     |
2188 |                 return Err(io::Error::new(io::ErrorKind::Other, "failed to create whole tree"));
     |                                               ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys_common/net.rs:174:59
170 | /         macro_rules! try_opt {
170 | /         macro_rules! try_opt {
171 | |             ($e:expr, $msg:expr) => {
172 | |                 match $e {
173 | |                     Some(r) => r,
174 | |                     None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
    | |                                                           ^^^^^^^^^ could not find `ErrorKind` in `io`
176 | |             };
177 | |         }
177 | |         }
    | |_________- in this expansion of `try_opt!`
...
180 |           let (host, port_str) = try_opt!(s.rsplit_once(':'), "invalid socket address");


error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys_common/net.rs:174:59
170 | /         macro_rules! try_opt {
170 | /         macro_rules! try_opt {
171 | |             ($e:expr, $msg:expr) => {
172 | |                 match $e {
173 | |                     Some(r) => r,
174 | |                     None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
    | |                                                           ^^^^^^^^^ could not find `ErrorKind` in `io`
176 | |             };
177 | |         }
177 | |         }
    | |_________- in this expansion of `try_opt!`
...
181 |           let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");


error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
    |
107 |                 Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
    |                                               ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
    |
112 |             Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
    |                                    ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
    |
195 |                         io::ErrorKind::WriteZero,
    |                             ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
    |
203 |                 Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
    |                                               ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   |
33 |             io::ErrorKind::InvalidInput,
33 |             io::ErrorKind::InvalidInput,
   |                 ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   |
40 |             io::ErrorKind::InvalidInput,
40 |             io::ErrorKind::InvalidInput,
   |                 ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
122 |                 io::ErrorKind::InvalidInput,
122 |                 io::ErrorKind::InvalidInput,
    |                     ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `Read` in `io`
    |
    |
575 |         io::Read::read(&mut &*self, buf)
    |             ^^^^ could not find `Read` in `io`

error[E0433]: failed to resolve: could not find `Read` in `io`
    |
    |
579 |         io::Read::read_vectored(&mut &*self, bufs)
    |             ^^^^ could not find `Read` in `io`

error[E0433]: failed to resolve: could not find `Read` in `io`
    |
    |
584 |         io::Read::is_read_vectored(&&*self)
    |             ^^^^ could not find `Read` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
361 |                         io::ErrorKind::Other,
361 |                         io::ErrorKind::Other,
    |                             ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
369 |             io::ErrorKind::Other,
369 |             io::ErrorKind::Other,
    |                 ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
    |
    |
822 |                 size.try_into().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    |                                                                ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `copy` in `io`
     |
     |
1216 |         CopyResult::Fallback(written) => match io::copy::generic_copy(&mut reader, &mut writer) {
     |                                                    ^^^^ could not find `copy` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
  --> library/std/src/sys/unix/net.rs:41:13
41 |         io::ErrorKind::Other,
41 |         io::ErrorKind::Other,
   |             ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/net.rs:143:21
143 |                 io::ErrorKind::InvalidInput,
143 |                 io::ErrorKind::InvalidInput,
    |                     ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/net.rs:153:47
    |
153 |                 return Err(io::Error::new(io::ErrorKind::TimedOut, "connection timed out"));
    |                                               ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/net.rs:170:42
    |
170 |                     if err.kind() != io::ErrorKind::Interrupted {
    |                                          ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/net.rs:180:48
    |
180 | ...                   io::Error::new(io::ErrorKind::Other, "no error set after POLLHUP")
    |                                          ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/net.rs:328:29
328 |                         io::ErrorKind::InvalidInput,
328 |                         io::ErrorKind::InvalidInput,
    |                             ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/os.rs:345:39
    |
345 |         Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(io::Error::new(
    |                                       ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0433]: failed to resolve: could not find `ErrorKind` in `io`
   --> library/std/src/sys/unix/os.rs:346:17
346 |             io::ErrorKind::Other,
346 |             io::ErrorKind::Other,
    |                 ^^^^^^^^^ could not find `ErrorKind` in `io`

error[E0412]: cannot find type `Result` in module `io`
   |
   |
36 | pub fn available_concurrency() -> io::Result<NonZeroUsize> {
   |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
81 |         fn available_concurrency_internal() -> io::Result<NonZeroUsize> {
   |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   |
   |
83 |                 -1 => Err(io::Error::last_os_error()),
   |                               ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   |
   |
84 |                 0 => Err(io::Error::new(io::ErrorKind::NotFound, "The number of hardware threads is not known for the target platform")),
   |                              ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
375 |     pub fn spawn<F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
151 | use alloc::fmt::Result;
    |
---
151 | use crate::fmt::Result;
    |
      and 2 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
443 |     pub unsafe fn spawn_unchecked<'a, F, T>(self, f: F) -> io::Result<JoinHandle<T>>
    |                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
151 | use alloc::fmt::Result;
    |
---
151 | use crate::fmt::Result;
    |
      and 2 other candidates

error[E0425]: cannot find function `set_output_capture` in module `crate::io`
    |
    |
459 |         let output_capture = crate::io::set_output_capture(None);
    |                                         ^^^^^^^^^^^^^^^^^^ not found in `crate::io`

error[E0425]: cannot find function `set_output_capture` in module `crate::io`
    |
    |
460 |         crate::io::set_output_capture(output_capture.clone());
    |                    ^^^^^^^^^^^^^^^^^^ not found in `crate::io`

error[E0425]: cannot find function `set_output_capture` in module `crate::io`
    |
    |
467 |             crate::io::set_output_capture(output_capture);
    |                        ^^^^^^^^^^^^^^^^^^ not found in `crate::io`

error[E0412]: cannot find type `Result` in module `io`
   |
   |
46 | pub fn current_dir() -> io::Result<PathBuf> {
   |                             ^^^^^^ not found in `io`
help: consider importing one of these items
   |
16 | use alloc::fmt::Result;
   |
---
16 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
65 | pub fn set_current_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
   |                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
   |
16 | use alloc::fmt::Result;
   |
---
16 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
662 | pub fn current_exe() -> io::Result<PathBuf> {
    |                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
16  | use alloc::fmt::Result;
    |
---
16  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
     |
     |
1038 | impl From<NulError> for io::Error {
     |                             ^^^^^ not found in `io`
help: consider importing one of these items
     |
6    | use alloc::fmt::Error;
     |
---
6    | use crate::fmt::Error;
     |
       and 1 other candidate

error[E0412]: cannot find type `Error` in module `io`
     |
     |
1040 |     fn from(_: NulError) -> io::Error {
     |                                 ^^^^^ not found in `io`
help: consider importing one of these items
     |
6    | use alloc::fmt::Error;
     |
---
6    | use crate::fmt::Error;
     |
       and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
     |
     |
1041 |         io::Error::new(io::ErrorKind::InvalidInput, "data provided contains a nul byte")
     |             ^^^^^ not found in `io`
help: consider importing one of these items
     |
6    | use alloc::fmt::Error;
     |
---
6    | use crate::fmt::Error;
     |
       and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
235 | pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    |                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
236 |     fn inner(path: &Path) -> io::Result<Vec<u8>> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
275 | pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
276 |     fn inner(path: &Path) -> io::Result<String> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
307 | pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    |                                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
308 |     fn inner(path: &Path, contents: &[u8]) -> io::Result<()> {
    |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
335 |     pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
    |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
357 |     pub fn create<P: AsRef<Path>>(path: P) -> io::Result<File> {
    |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
414 |     pub fn sync_all(&self) -> io::Result<()> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
445 |     pub fn sync_data(&self) -> io::Result<()> {
    |                                    ^^^^^^ not found in `io`
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
482 |     pub fn set_len(&self, size: u64) -> io::Result<()> {
    |                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
500 |     pub fn metadata(&self) -> io::Result<Metadata> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
544 |     pub fn try_clone(&self) -> io::Result<File> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
581 |     pub fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
611 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
615 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
632 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
636 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
645 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
651 |     fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
657 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
661 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
678 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
682 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
691 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
697 |     fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
918 |     pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
    |                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
922 |     fn _open(&self, path: &Path) -> io::Result<File> {
    |                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
14  | use alloc::fmt::Result;
    |
---
14  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1074 |     pub fn modified(&self) -> io::Result<SystemTime> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1109 |     pub fn accessed(&self) -> io::Result<SystemTime> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1141 |     pub fn created(&self) -> io::Result<SystemTime> {
     |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
1344 |     type Item = io::Result<DirEntry>;
     |                     ^^^^^^ not found in `io`
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1346 |     fn next(&mut self) -> Option<io::Result<DirEntry>> {
     |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1419 |     pub fn metadata(&self) -> io::Result<Metadata> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1454 |     pub fn file_type(&self) -> io::Result<FileType> {
     |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1528 | pub fn remove_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
     |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1566 | pub fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
     |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1600 | pub fn symlink_metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
     |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1643 | pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
     |                                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1695 | pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
     |                                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1736 | pub fn hard_link<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
     |                                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1768 | pub fn soft_link<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
     |                                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1802 | pub fn read_link<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
     |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1843 | pub fn canonicalize<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
     |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1883 | pub fn create_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
     |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1927 | pub fn create_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
     |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1962 | pub fn remove_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
     |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1999 | pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
     |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
2072 | pub fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<ReadDir> {
     |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
2107 | pub fn set_permissions<P: AsRef<Path>>(path: P, perm: Permissions) -> io::Result<()> {
     |                                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
2166 |     pub fn create<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
     |                                                          ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
2170 |     fn _create(&self, path: &Path) -> io::Result<()> {
     |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
2174 |     fn create_dir_all(&self, path: &Path) -> io::Result<()> {
     |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Result;
     |
---
14   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
     |
     |
2188 |                 return Err(io::Error::new(io::ErrorKind::Other, "failed to create whole tree"));
     |                                ^^^^^ not found in `io`
help: consider importing one of these items
     |
14   | use alloc::fmt::Error;
     |
---
14   | use crate::fmt::Error;
     |
       and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
867 |     fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
873 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
881 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
889 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
897 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
909 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
918 |     fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
924 | fn resolve_socket_addr(lh: LookupHost) -> io::Result<vec::IntoIter<SocketAddr>> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
938 |     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
958 |     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
967 |     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
981 |     fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
989 |     fn to_socket_addrs(&self) -> io::Result<T::Iter> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
997 |     fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
142 |     pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
    |                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
158 |     pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
175 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
192 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
219 |     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
    |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
240 |     pub fn try_clone(&self) -> io::Result<TcpStream> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
284 |     pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
328 |     pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
353 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
378 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
400 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
422 |     pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
441 |     pub fn nodelay(&self) -> io::Result<bool> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
460 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
479 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
499 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
499 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Error;
    |
---
6   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
544 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in this scope
    |
    |
550 | impl Read for TcpStream {


error[E0412]: cannot find type `Result` in module `io`
    |
    |
551 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
555 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Write` in this scope
    |
571 | impl Write for TcpStream {
    |      ^^^^^ not found in this scope
    |
---
    |
6   | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
572 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
576 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
585 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in this scope
    |
    |
590 | impl Read for &TcpStream {


error[E0412]: cannot find type `Result` in module `io`
    |
    |
591 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
595 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Write` in this scope
    |
    |
611 | impl Write for &TcpStream {
    |
help: consider importing one of these items
    |
6   | use alloc::fmt::Write;
6   | use alloc::fmt::Write;
    |
6   | use core::fmt::Write;
    |
6   | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
612 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
616 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
625 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
696 |     pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
    |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
712 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
731 |     pub fn try_clone(&self) -> io::Result<TcpListener> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
753 |     pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
802 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
820 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
830 |     pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
840 |     pub fn only_v6(&self) -> io::Result<bool> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
859 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
859 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Error;
    |
---
6   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
906 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
913 |     type Item = io::Result<TcpStream>;
    |                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
914 |     fn next(&mut self) -> Option<io::Result<TcpStream>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
94 |     pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
   |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
117 |     pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
146 |     pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
173 |     pub fn send_to<A: ToSocketAddrs>(&self, buf: &[u8], addr: A) -> io::Result<usize> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
205 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
221 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
240 |     pub fn try_clone(&self) -> io::Result<UdpSocket> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
283 |     pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
326 |     pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
346 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
366 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
384 |     pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
    |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
402 |     pub fn broadcast(&self) -> io::Result<bool> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
420 |     pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
438 |     pub fn multicast_loop_v4(&self) -> io::Result<bool> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
459 |     pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
    |                                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
477 |     pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
495 |     pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
513 |     pub fn multicast_loop_v6(&self) -> io::Result<bool> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
531 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
549 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
561 |     pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
    |                                                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
571 |     pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
    |                                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
579 |     pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
    |                                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
587 |     pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
    |                                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
610 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
610 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
    |
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
643 |     pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
662 |     pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
690 |     pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
730 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
775 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/net/mod.rs:77:18
   |
77 |     F: FnMut(io::Result<&SocketAddr>) -> io::Result<T>,
   |                  ^^^^^^ not found in `io`
help: consider importing one of these items
   |
20 | use alloc::fmt::Result;
   |
---
20 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/net/mod.rs:77:46
   |
77 |     F: FnMut(io::Result<&SocketAddr>) -> io::Result<T>,
   |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
   |
20 | use alloc::fmt::Result;
   |
---
20 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/net/mod.rs:75:64
   |
75 | fn each_addr<A: ToSocketAddrs, F, T>(addr: A, mut f: F) -> io::Result<T>
   |                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
   |
20 | use alloc::fmt::Result;
   |
---
20 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    --> library/std/src/path.rs:2308:35
     |
2308 |     pub fn metadata(&self) -> io::Result<fs::Metadata> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
66   | use alloc::fmt::Result;
     |
---
66   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    --> library/std/src/path.rs:2326:43
     |
2326 |     pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
     |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
     |
66   | use alloc::fmt::Result;
     |
---
66   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    --> library/std/src/path.rs:2344:39
     |
2344 |     pub fn canonicalize(&self) -> io::Result<PathBuf> {
     |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
     |
66   | use alloc::fmt::Result;
     |
---
66   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    --> library/std/src/path.rs:2361:36
     |
2361 |     pub fn read_link(&self) -> io::Result<PathBuf> {
     |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
     |
66   | use alloc::fmt::Result;
     |
---
66   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    --> library/std/src/path.rs:2385:35
     |
2385 |     pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
66   | use alloc::fmt::Result;
     |
---
66   | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0405]: cannot find trait `Write` in this scope
    |
253 | impl Write for ChildStdin {
    |      ^^^^^ not found in this scope
    |
---
    |
103 | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
254 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
258 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Write` in `io`
    |
    |
263 |         io::Write::is_write_vectored(&&*self)
    |             ^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Write;
    |
    |
103 | use core::fmt::Write;
    |
103 | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
266 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Write` in this scope
    |
    |
272 | impl Write for &ChildStdin {
    |
help: consider importing one of these items
    |
103 | use alloc::fmt::Write;
103 | use alloc::fmt::Write;
    |
103 | use core::fmt::Write;
    |
103 | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
273 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
277 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
285 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in this scope
    |
    |
330 | impl Read for ChildStdout {


error[E0412]: cannot find type `Result` in module `io`
    |
    |
331 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
335 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in this scope
    |
    |
391 | impl Read for ChildStderr {


error[E0412]: cannot find type `Result` in module `io`
    |
    |
392 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
396 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
847 |     pub fn spawn(&mut self) -> io::Result<Child> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
876 |     pub fn output(&mut self) -> io::Result<Output> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
903 |     pub fn status(&mut self) -> io::Result<ExitStatus> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
103 | use alloc::fmt::Result;
    |
---
103 | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1517 |     pub fn kill(&mut self) -> io::Result<()> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
103  | use alloc::fmt::Result;
     |
---
103  | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1567 |     pub fn wait(&mut self) -> io::Result<ExitStatus> {
     |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
     |
103  | use alloc::fmt::Result;
     |
---
103  | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1607 |     pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
     |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
     |
103  | use alloc::fmt::Result;
     |
---
103  | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1644 |     pub fn wait_with_output(mut self) -> io::Result<Output> {
     |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
     |
103  | use alloc::fmt::Result;
     |
---
103  | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0425]: cannot find function `_eprint` in module `$crate::io`
     |
152  | / macro_rules! eprintln {
152  | / macro_rules! eprintln {
153  | |     () => ($crate::eprint!("\n"));
154  | |     ($($arg:tt)*) => ({
155  | |         $crate::io::_eprint($crate::format_args_nl!($($arg)*));
     | |                     ^^^^^^^ not found in `$crate::io`
157  | | }
157  | | }
     | |_- in this expansion of `eprintln!`
    ::: library/std/src/process.rs:1854:9
     |
     |
1854 |           eprintln!("Error: {:?}", err);


error[E0405]: cannot find trait `Write` in this scope
   |
   |
23 | pub fn print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
   |
help: consider importing one of these items
   |
1  | use alloc::fmt::Write;
1  | use alloc::fmt::Write;
   |
1  | use core::fmt::Write;
   |
1  | use crate::fmt::Write;
   |

error[E0412]: cannot find type `Result` in module `io`
   |
   |
23 | pub fn print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
   |                                                          ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0405]: cannot find trait `Write` in this scope
   |
   |
40 | unsafe fn _print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
   |
help: consider importing one of these items
   |
1  | use alloc::fmt::Write;
1  | use alloc::fmt::Write;
   |
1  | use core::fmt::Write;
   |
1  | use crate::fmt::Write;
   |

error[E0412]: cannot find type `Result` in module `io`
   |
   |
40 | unsafe fn _print(w: &mut dyn Write, format: PrintFmt) -> io::Result<()> {
   |                                                              ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
 --> library/std/src/sys_common/fs.rs:7:44
  |
7 | pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
  |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
  |
3 | use alloc::fmt::Result;
  |
---
3 | use crate::fmt::Result;
  |
    and 3 other candidates

error[E0425]: cannot find function `copy` in module `io`
  --> library/std/src/sys_common/fs.rs:21:19
   |
21 |     let ret = io::copy(&mut reader, &mut writer)?;
   |                   ^^^^ not found in `io`
help: consider importing one of these items
   |
3  | use core::ptr::copy;
   |
   |
3  | use crate::fs::copy;
   |
3  | use crate::ptr::copy;
   |
3  | use crate::sys::fs::copy;
   |

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/fs.rs:26:43
   |
26 | pub fn remove_dir_all(path: &Path) -> io::Result<()> {
   |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
   |
3  | use alloc::fmt::Result;
   |
---
3  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/fs.rs:31:49
   |
31 | fn remove_dir_all_recursive(path: &Path) -> io::Result<()> {
   |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
   |
3  | use alloc::fmt::Result;
   |
---
3  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/net.rs:61:80
   |
61 | pub fn setsockopt<T>(sock: &Socket, opt: c_int, val: c_int, payload: T) -> io::Result<()> {
   |                                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/net.rs:75:74
   |
75 | pub fn getsockopt<T: Copy>(sock: &Socket, opt: c_int, val: c_int) -> io::Result<T> {
   |                                                                          ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/net.rs:85:29
   |
85 | fn sockname<F>(f: F) -> io::Result<SocketAddr>
   |                             ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys_common/net.rs:97:75
   |
97 | pub fn sockaddr_to_addr(storage: &c::sockaddr_storage, len: usize) -> io::Result<SocketAddr> {
   |                                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys_common/net.rs:167:22
167 |     type Error = io::Error;
    |                      ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:169:33
    |
169 |     fn try_from(s: &str) -> io::Result<LookupHost> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys_common/net.rs:174:44
170 | /         macro_rules! try_opt {
170 | /         macro_rules! try_opt {
171 | |             ($e:expr, $msg:expr) => {
172 | |                 match $e {
173 | |                     Some(r) => r,
174 | |                     None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
    | |                                            ^^^^^ not found in `io`
176 | |             };
177 | |         }
177 | |         }
    | |_________- in this expansion of `try_opt!`
...
180 |           let (host, port_str) = try_opt!(s.rsplit_once(':'), "invalid socket address");
    |
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys_common/net.rs:174:44
170 | /         macro_rules! try_opt {
170 | /         macro_rules! try_opt {
171 | |             ($e:expr, $msg:expr) => {
172 | |                 match $e {
173 | |                     Some(r) => r,
174 | |                     None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
    | |                                            ^^^^^ not found in `io`
176 | |             };
177 | |         }
177 | |         }
    | |_________- in this expansion of `try_opt!`
...
181 |           let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");
    |
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys_common/net.rs:187:22
187 |     type Error = io::Error;
    |                      ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:189:54
    |
189 |     fn try_from((host, port): (&'a str, u16)) -> io::Result<LookupHost> {
    |                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:212:30
    |
212 |     pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:212:58
    |
212 |     pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
    |                                                          ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:224:73
    |
224 |     pub fn connect_timeout(addr: &SocketAddr, timeout: Duration) -> io::Result<TcpStream> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:240:66
    |
240 |     pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:244:67
    |
244 |     pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:248:39
    |
248 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:252:40
    |
252 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:256:47
    |
256 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:260:47
    |
260 |     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:264:69
    |
264 |     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:273:44
    |
273 |     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:281:63
    |
281 |     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:290:36
    |
290 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:294:38
    |
294 |     pub fn socket_addr(&self) -> io::Result<SocketAddr> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:298:50
    |
298 |     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
    |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:302:36
    |
302 |     pub fn duplicate(&self) -> io::Result<TcpStream> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:306:53
    |
306 |     pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:310:34
    |
310 |     pub fn nodelay(&self) -> io::Result<bool> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:314:44
    |
314 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:318:30
    |
318 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:323:37
    |
323 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys_common/net.rs:323:55
    |
323 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
    |
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:327:61
    |
327 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:364:27
    |
364 |     pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
    |                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:364:55
    |
364 |     pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:398:38
    |
398 |     pub fn socket_addr(&self) -> io::Result<SocketAddr> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:402:33
    |
402 |     pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:410:36
    |
410 |     pub fn duplicate(&self) -> io::Result<TcpListener> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:414:44
    |
414 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:418:30
    |
418 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:423:53
    |
423 |     pub fn set_only_v6(&self, only_v6: bool) -> io::Result<()> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:427:34
    |
427 |     pub fn only_v6(&self) -> io::Result<bool> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:432:37
    |
432 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys_common/net.rs:432:55
    |
432 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
    |
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:436:61
    |
436 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:469:27
    |
469 |     pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
    |                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:469:55
    |
469 |     pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:488:36
    |
488 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:492:38
    |
492 |     pub fn socket_addr(&self) -> io::Result<SocketAddr> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:496:52
    |
496 |     pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:500:52
    |
500 |     pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:504:64
    |
504 |     pub fn send_to(&self, buf: &[u8], dst: &SocketAddr) -> io::Result<usize> {
    |                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:520:36
    |
520 |     pub fn duplicate(&self) -> io::Result<UdpSocket> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:524:66
    |
524 |     pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:528:67
    |
528 |     pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
    |                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:532:39
    |
532 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:536:40
    |
536 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:540:57
    |
540 |     pub fn set_broadcast(&self, broadcast: bool) -> io::Result<()> {
    |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:544:36
    |
544 |     pub fn broadcast(&self) -> io::Result<bool> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:549:73
    |
549 |     pub fn set_multicast_loop_v4(&self, multicast_loop_v4: bool) -> io::Result<()> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:558:44
    |
558 |     pub fn multicast_loop_v4(&self) -> io::Result<bool> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:563:70
    |
563 |     pub fn set_multicast_ttl_v4(&self, multicast_ttl_v4: u32) -> io::Result<()> {
    |                                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:572:43
    |
572 |     pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:577:73
    |
577 |     pub fn set_multicast_loop_v6(&self, multicast_loop_v6: bool) -> io::Result<()> {
    |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:581:44
    |
581 |     pub fn multicast_loop_v6(&self) -> io::Result<bool> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:586:88
    |
586 |     pub fn join_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
    |                                                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:594:82
    |
594 |     pub fn join_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
    |                                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:602:89
    |
602 |     pub fn leave_multicast_v4(&self, multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> io::Result<()> {
    |                                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:610:83
    |
610 |     pub fn leave_multicast_v6(&self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()> {
    |                                                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:618:44
    |
618 |     pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:622:30
    |
622 |     pub fn ttl(&self) -> io::Result<u32> {
    |                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:627:37
    |
627 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys_common/net.rs:627:55
    |
627 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Error;
    |
---
4   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:631:61
    |
631 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:635:47
    |
635 |     pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:639:47
    |
639 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:643:43
    |
643 |     pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:651:37
    |
651 |     pub fn connect(&self, addr: io::Result<&SocketAddr>) -> io::Result<()> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys_common/net.rs:651:65
    |
651 |     pub fn connect(&self, addr: io::Result<&SocketAddr>) -> io::Result<()> {
    |                                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
50 |     fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;
   |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
   |
5  | use alloc::fmt::Result;
   |
---
5  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
98 |     fn read_exact_at(&self, mut buf: &mut [u8], mut offset: u64) -> io::Result<()> {
   |                                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
   |
5  | use alloc::fmt::Result;
   |
---
5  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
    |
112 |             Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"))
    |                     ^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Error;
    |
---
5   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
151 |     fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize>;
    |                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Result;
    |
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
190 |     fn write_all_at(&self, mut buf: &[u8], mut offset: u64) -> io::Result<()> {
    |                                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Result;
    |
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
194 |                     return Err(io::Error::new(
    |                                    ^^^^^ not found in `io`
    |
---
5   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
213 |     fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Result;
    |
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
216 |     fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
    |                                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Result;
    |
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
857 | pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    |                                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Result;
    |
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Stdin` in module `io`
    |
    |
144 | impl AsRawFd for io::Stdin {
    |                      ^^^^^ not found in `io`
help: consider importing this struct
    |
5   | use crate::sys::stdio::Stdin;
    |
    |

error[E0412]: cannot find type `Stdout` in module `io`
    |
    |
151 | impl AsRawFd for io::Stdout {
    |                      ^^^^^^ not found in `io`
help: consider importing this struct
    |
5   | use crate::sys::stdio::Stdout;
    |
    |

error[E0412]: cannot find type `Stderr` in module `io`
    |
    |
158 | impl AsRawFd for io::Stderr {
    |                      ^^^^^^ not found in `io`
help: consider importing this struct
    |
5   | use crate::sys::stdio::Stderr;
    |
    |

error[E0412]: cannot find type `StdinLock` in module `io`
    |
    |
165 | impl<'a> AsRawFd for io::StdinLock<'a> {
    |                          ^^^^^^^^^ not found in `io`

error[E0412]: cannot find type `StdoutLock` in module `io`
    |
    |
172 | impl<'a> AsRawFd for io::StdoutLock<'a> {
    |                          ^^^^^^^^^^ not found in `io`

error[E0412]: cannot find type `StderrLock` in module `io`
    |
    |
179 | impl<'a> AsRawFd for io::StderrLock<'a> {
    |                          ^^^^^^^^^^ not found in `io`

error[E0412]: cannot find type `Result` in module `io`
   |
   |
25 | pub(super) unsafe fn sockaddr_un(path: &Path) -> io::Result<(libc::sockaddr_un, libc::socklen_t)> {
   |                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   |
32 |         return Err(io::Error::new(
   |                        ^^^^^ not found in `io`
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   |
39 |         return Err(io::Error::new(
   |                        ^^^^^ not found in `io`
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
100 |     pub(super) fn new<F>(f: F) -> io::Result<SocketAddr>
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
115 |     ) -> io::Result<SocketAddr> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
121 |             return Err(io::Error::new(
    |                            ^^^^^ not found in `io`
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   |
   |
30 | ) -> io::Result<(usize, bool, io::Result<SocketAddr>)> {
   |          ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
30 | ) -> io::Result<(usize, bool, io::Result<SocketAddr>)> {
   |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
73 | ) -> io::Result<usize> {
   |          ^^^^^^ not found in `io`
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
104 |     pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixDatagram> {
    |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
131 |     pub fn unbound() -> io::Result<UnixDatagram> {
    |                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
154 |     pub fn pair() -> io::Result<(UnixDatagram, UnixDatagram)> {
    |                          ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
186 |     pub fn connect<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
    |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
213 |     pub fn try_clone(&self) -> io::Result<UnixDatagram> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
231 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
255 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
263 |     ) -> io::Result<(usize, SocketAddr)> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
305 |     pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
326 |     pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
380 |     ) -> io::Result<(usize, bool, SocketAddr)> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
437 |     ) -> io::Result<(usize, bool)> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
460 |     pub fn send_to<P: AsRef<Path>>(&self, buf: &[u8], path: P) -> io::Result<usize> {
    |                                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
496 |     pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
    |                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
544 |     ) -> io::Result<usize> {
    |              ^^^^^^ not found in `io`
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
591 |     ) -> io::Result<usize> {
    |              ^^^^^^ not found in `io`
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
635 |     pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
    |                                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
679 |     pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
    |                                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
700 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
721 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
739 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
762 |     pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
774 |     pub fn passcred(&self) -> io::Result<bool> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
794 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
794 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Error;
    |
---
11  | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
815 |     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
    |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
841 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
873 |     pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
72 |     pub fn bind<P: AsRef<Path>>(path: P) -> io::Result<UnixListener> {
   |                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
108 |     pub fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
134 |     pub fn try_clone(&self) -> io::Result<UnixListener> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
152 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
176 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
200 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
200 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
264 |     type Item = io::Result<UnixStream>;
    |                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
310 |     type Item = io::Result<UnixStream>;
    |                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
312 |     fn next(&mut self) -> Option<io::Result<UnixStream>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
97 |     pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<UnixStream> {
   |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
   |
11 | use alloc::fmt::Result;
   |
---
11 | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
125 |     pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
    |                          ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
149 |     pub fn try_clone(&self) -> io::Result<UnixStream> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
167 |     pub fn local_addr(&self) -> io::Result<SocketAddr> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
185 |     pub fn peer_addr(&self) -> io::Result<SocketAddr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
213 |     pub fn peer_cred(&self) -> io::Result<UCred> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
255 |     pub fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
    |                                                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
298 |     pub fn set_write_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
    |                                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
318 |     pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
339 |     pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
357 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
380 |     pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
392 |     pub fn passcred(&self) -> io::Result<bool> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
415 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
    |
    |
415 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Error;
    |
---
11  | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
438 |     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
    |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
464 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
518 |     ) -> io::Result<usize> {
    |              ^^^^^^ not found in `io`
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
567 |     ) -> io::Result<usize> {
    |              ^^^^^^ not found in `io`
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in module `io`
    |
    |
573 | impl io::Read for UnixStream {
    |          ^^^^ not found in `io`

error[E0412]: cannot find type `Result` in module `io`
    |
    |
574 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
578 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in module `io`
    |
    |
594 | impl<'a> io::Read for &'a UnixStream {
    |              ^^^^ not found in `io`

error[E0412]: cannot find type `Result` in module `io`
    |
    |
595 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
599 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Write` in module `io`
    |
615 | impl io::Write for UnixStream {
    |          ^^^^^ not found in `io`
    |
---
    |
11  | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
616 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Write` in `io`
    |
    |
617 |         io::Write::write(&mut &*self, buf)
    |             ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Write;
    |
    |
11  | use core::fmt::Write;
    |
11  | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
620 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Write` in `io`
    |
    |
621 |         io::Write::write_vectored(&mut &*self, bufs)
    |             ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Write;
    |
    |
11  | use core::fmt::Write;
    |
11  | use crate::fmt::Write;
    |

error[E0433]: failed to resolve: could not find `Write` in `io`
    |
    |
626 |         io::Write::is_write_vectored(&&*self)
    |             ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Write;
    |
    |
11  | use core::fmt::Write;
    |
11  | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
629 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Write` in `io`
    |
    |
630 |         io::Write::flush(&mut &*self)
    |             ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Write;
    |
    |
11  | use core::fmt::Write;
    |
11  | use crate::fmt::Write;
    |

error[E0405]: cannot find trait `Write` in module `io`
    |
    |
635 | impl<'a> io::Write for &'a UnixStream {
    |              ^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Write;
    |
    |
11  | use core::fmt::Write;
    |
11  | use crate::fmt::Write;
    |

error[E0412]: cannot find type `Result` in module `io`
    |
    |
636 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
640 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
649 |     fn flush(&mut self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
11  | use alloc::fmt::Result;
    |
---
11  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/ext/process.rs:74:27
74 |         F: FnMut() -> io::Result<()> + Send + Sync + 'static;
   |                           ^^^^^^ not found in `io`
   |
help: consider importing one of these items
---
5  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/ext/process.rs:87:27
87 |         F: FnMut() -> io::Result<()> + Send + Sync + 'static,
   |                           ^^^^^^ not found in `io`
   |
help: consider importing one of these items
---
5  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/ext/process.rs:120:31
    |
120 |     fn exec(&mut self) -> io::Error;
    |                               ^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Error;
    |
---
5   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/ext/process.rs:154:27
154 |         F: FnMut() -> io::Result<()> + Send + Sync + 'static,
    |                           ^^^^^^ not found in `io`
    |
help: consider importing one of these items
---
5   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/ext/process.rs:160:31
    |
160 |     fn exec(&mut self) -> io::Error {
    |                               ^^^^^ not found in `io`
help: consider importing one of these items
    |
5   | use alloc::fmt::Error;
    |
---
5   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/ext/ucred.rs:45:50
   |
45 |     pub fn peer_cred(socket: &UnixStream) -> io::Result<UCred> {
   |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
   |
39 |     use alloc::fmt::Result;
   |
---
39 |     use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
  --> library/std/src/sys/unix/ext/ucred.rs:67:25
   |
67 |                 Err(io::Error::last_os_error())
   |                         ^^^^^ not found in `io`
help: consider importing one of these items
   |
39 |     use alloc::fmt::Error;
   |
---
39 |     use crate::fmt::Error;
   |
     and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/fd.rs:87:47
   |
87 |     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
   |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
6  | use alloc::fmt::Result;
   |
---
6  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/fd.rs:94:69
   |
94 |     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
   |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
6  | use alloc::fmt::Result;
   |
---
6  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:110:57
    |
110 |     pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
    |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:115:63
    |
115 |     pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:125:18
125 |         ) -> io::Result<isize> {
    |                  ^^^^^^ not found in `io`
    |
help: consider importing one of these items
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:144:44
    |
144 |     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:151:63
    |
151 |     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:167:60
    |
167 |     pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
    |                                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:177:18
177 |         ) -> io::Result<isize> {
    |                  ^^^^^^ not found in `io`
    |
help: consider importing one of these items
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:197:38
    |
197 |     pub fn get_cloexec(&self) -> io::Result<bool> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:231:38
231 |     pub fn set_cloexec(&self) -> io::Result<()> {
    |                                      ^^^^^^ not found in `io`
    |
help: consider importing one of these items
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:243:61
    |
243 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:267:36
    |
267 |     pub fn duplicate(&self) -> io::Result<FileDesc> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/fd.rs:277:47
    |
277 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
6   | use alloc::fmt::Result;
    |
---
6   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   |
   |
98 |     ) -> Option<io::Result<FileAttr>> {
   |                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
301 |     pub fn modified(&self) -> io::Result<SystemTime> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
317 |     pub fn accessed(&self) -> io::Result<SystemTime> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
351 |     pub fn created(&self) -> io::Result<SystemTime> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
360 |                     Err(io::Error::new(
    |                             ^^^^^ not found in `io`
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
368 |         Err(io::Error::new(
    |                 ^^^^^ not found in `io`
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
433 |     type Item = io::Result<DirEntry>;
    |                     ^^^^^^ not found in `io`
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
484 |     fn next(&mut self) -> Option<io::Result<DirEntry>> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
531 |     pub fn metadata(&self) -> io::Result<FileAttr> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
572 |     pub fn file_type(&self) -> io::Result<FileType> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
694 |     fn get_access_mode(&self) -> io::Result<c_int> {
    |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
705 |     fn get_creation_mode(&self) -> io::Result<c_int> {
    |                                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
731 |     pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
    |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
736 |     pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
    |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
749 |     pub fn file_attr(&self) -> io::Result<FileAttr> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
768 |     pub fn fsync(&self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
782 |     pub fn datasync(&self) -> io::Result<()> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
814 |     pub fn truncate(&self, size: u64) -> io::Result<()> {
    |                                              ^^^^^^ not found in `io`
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
    |
    |
822 |                 size.try_into().map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    |                                                 ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
    |
    |
827 |     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
831 |     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
840 |     pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
844 |     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
848 |     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
857 |     pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
    |                                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
861 |     pub fn flush(&self) -> io::Result<()> {
    |                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
865 |     pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
    |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
877 |     pub fn duplicate(&self) -> io::Result<File> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
889 |     pub fn set_permissions(&self, perm: FilePermissions) -> io::Result<()> {
    |                                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
900 |     pub fn mkdir(&self, p: &Path) -> io::Result<()> {
    |                                          ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
911 | fn cstr(path: &Path) -> io::Result<CString> {
    |                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
    |
    |
999 | pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1022 | pub fn unlink(p: &Path) -> io::Result<()> {
     |                                ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1028 | pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
     |                                              ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1035 | pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
     |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1041 | pub fn rmdir(p: &Path) -> io::Result<()> {
     |                               ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1047 | pub fn readlink(p: &Path) -> io::Result<PathBuf> {
     |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1074 | pub fn symlink(original: &Path, link: &Path) -> io::Result<()> {
     |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1081 | pub fn link(original: &Path, link: &Path) -> io::Result<()> {
     |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1101 | pub fn stat(p: &Path) -> io::Result<FileAttr> {
     |                              ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1120 | pub fn lstat(p: &Path) -> io::Result<FileAttr> {
     |                               ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1139 | pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
     |                                      ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
     |
     |
1145 |             return Err(io::Error::last_os_error());
     |                            ^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Error;
     |
---
1    | use crate::fmt::Error;
     |
       and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1153 | fn open_from(from: &Path) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1170 | ) -> io::Result<(crate::fs::File, crate::fs::Metadata)> {
     |          ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
     |
     |
1206 | pub fn copy(from: &Path, to: &Path) -> io::Result<u64> {
     |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
     |
1    | use alloc::fmt::Result;
     |
---
1    | use crate::fmt::Result;
     |
       and 3 other candidates

error[E0412]: cannot find type `Result` in module `crate::io`
   --> library/std/src/sys/unix/kernel_copy.rs:153:42
    |
153 |         let mut flush = || -> crate::io::Result<u64> {
    |                                          ^^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
47  | use alloc::fmt::Result;
    |
---
47  | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/net.rs:25:35
   |
25 | pub fn cvt_gai(err: c_int) -> io::Result<()> {
   |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
  --> library/std/src/sys/unix/net.rs:34:24
   |
34 |         return Err(io::Error::last_os_error());
   |                        ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
  --> library/std/src/sys/unix/net.rs:40:13
40 |     Err(io::Error::new(
   |             ^^^^^ not found in `io`
   |
help: consider importing one of these items
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/net.rs:47:53
   |
47 |     pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> {
   |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/net.rs:55:50
   |
55 |     pub fn new_raw(fam: c_int, ty: c_int) -> io::Result<Socket> {
   |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/net.rs:90:51
   |
90 |     pub fn new_pair(fam: c_int, ty: c_int) -> io::Result<(Socket, Socket)> {
   |                                                   ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:124:80
    |
124 |     pub fn connect_timeout(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<()> {
    |                                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:142:28
142 |             return Err(io::Error::new(
    |                            ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:153:32
    |
153 |                 return Err(io::Error::new(io::ErrorKind::TimedOut, "connection timed out"));
    |                                ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:169:35
169 |                     let err = io::Error::last_os_error();
    |                                   ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:180:33
    |
180 | ...                   io::Error::new(io::ErrorKind::Other, "no error set after POLLHUP")
    |                           ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:191:78
    |
191 |     pub fn accept(&self, storage: *mut sockaddr, len: *mut socklen_t) -> io::Result<Socket> {
    |                                                                              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:225:36
    |
225 |     pub fn duplicate(&self) -> io::Result<Socket> {
    |                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:229:68
    |
229 |     fn recv_with_flags(&self, buf: &mut [u8], flags: c_int) -> io::Result<usize> {
    |                                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:236:47
    |
236 |     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:240:47
    |
240 |     pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
    |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:244:69
    |
244 |     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
    |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:257:14
    |
257 |     ) -> io::Result<(usize, SocketAddr)> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:274:52
    |
274 |     pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:287:59
    |
287 |     pub fn recv_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
    |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:292:52
    |
292 |     pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
    |                                                    ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:296:44
    |
296 |     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:300:63
    |
300 |     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
    |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:318:59
    |
318 |     pub fn send_msg(&self, msg: &mut libc::msghdr) -> io::Result<usize> {
    |                                                           ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:323:80
    |
323 |     pub fn set_timeout(&self, dur: Option<Duration>, kind: libc::c_int) -> io::Result<()> {
    |                                                                                ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:327:36
327 |                     return Err(io::Error::new(
    |                                    ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:352:53
    |
352 |     pub fn timeout(&self, kind: libc::c_int) -> io::Result<Option<Duration>> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:363:50
    |
363 |     pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
    |                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:373:53
    |
373 |     pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
    |                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:377:34
    |
377 |     pub fn nodelay(&self) -> io::Result<bool> {
    |                                  ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:383:55
    |
383 |     pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
    |                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:388:35
    |
388 |     pub fn passcred(&self) -> io::Result<bool> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:394:61
    |
394 |     pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    |                                                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/net.rs:406:37
    |
406 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/net.rs:406:55
    |
406 |     pub fn take_error(&self) -> io::Result<Option<io::Error>> {
    |                                                       ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/net.rs:408:53
    |
408 |         if raw == 0 { Ok(None) } else { Ok(Some(io::Error::from_raw_os_error(raw as i32))) }
    |                                                     ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:134:24
    |
134 | pub fn getcwd() -> io::Result<PathBuf> {
    |                        ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/os.rs:145:33
145 |                 let error = io::Error::last_os_error();
    |                                 ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
8   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:160:37
    |
160 | pub fn chdir(p: &path::Path) -> io::Result<()> {
    |                                     ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/os.rs:166:30
    |
166 |             false => Err(io::Error::last_os_error()),
    |                              ^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Error;
    |
---
8   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:343:29
    |
343 | pub fn current_exe() -> io::Result<PathBuf> {
    |                             ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/os.rs:345:70
    |
345 |         Err(ref e) if e.kind() == io::ErrorKind::NotFound => Err(io::Error::new(
    |                                                                      ^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Error;
    |
---
8   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:538:33
    |
538 | pub fn getenv(k: &OsStr) -> io::Result<Option<OsString>> {
    |                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:554:44
    |
554 | pub fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/os.rs:564:35
    |
564 | pub fn unsetenv(n: &OsStr) -> io::Result<()> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
8   | use alloc::fmt::Result;
    |
---
8   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:12:27
   |
12 | pub fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> {
   |                           ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:42:47
   |
42 |     pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
   |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:46:69
   |
46 |     pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
   |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:55:44
   |
55 |     pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
   |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:59:63
   |
59 |     pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
   |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/pipe.rs:76:85
   |
76 | pub fn read2(p1: AnonPipe, v1: &mut Vec<u8>, p2: AnonPipe, v2: &mut Vec<u8>) -> io::Result<()> {
   |                                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/pipe.rs:108:67
    |
108 |     fn read(fd: &FileDesc, dst: &mut Vec<u8>) -> Result<bool, io::Error> {
    |                                                                   ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/process/process_common.rs:89:42
   |
89 |     closures: Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>>,
   |                                          ^^^^^^ not found in `io`
help: consider importing one of these items
   |
4  | use alloc::fmt::Result;
   |
---
4  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_common.rs:230:71
    |
230 |     pub fn get_closures(&mut self) -> &mut Vec<Box<dyn FnMut() -> io::Result<()> + Send + Sync>> {
    |                                                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_common.rs:234:65
    |
234 |     pub unsafe fn pre_exec(&mut self, f: Box<dyn FnMut() -> io::Result<()> + Send + Sync>) {
    |                                                                 ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_common.rs:273:14
    |
273 |     ) -> io::Result<(StdioPipes, ChildPipes)> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_common.rs:341:57
    |
341 |     pub fn to_child_stdio(&self, readable: bool) -> io::Result<(ChildStdio, Option<AnonPipe>)> {
    |                                                         ^^^^^^ not found in `io`
help: consider importing one of these items
    |
4   | use alloc::fmt::Result;
    |
---
4   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/process/process_unix.rs:24:14
   |
24 |     ) -> io::Result<(Process, StdioPipes)> {
   |              ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
  --> library/std/src/sys/unix/process/process_unix.rs:30:28
   |
30 |             return Err(io::Error::new(ErrorKind::InvalidInput, "nul byte found in provided data"));
   |                            ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:114:51
    |
114 |     pub fn exec(&mut self, default: Stdio) -> io::Error {
    |                                                   ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/process/process_unix.rs:118:24
    |
118 |             return io::Error::new(ErrorKind::InvalidInput, "nul byte found in provided data");
    |                        ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Error` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:171:24
171 |     ) -> Result<!, io::Error> {
    |                        ^^^^^ not found in `io`
    |
help: consider importing one of these items
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/process/process_unix.rs:223:32
    |
223 |                 return Err(io::Error::last_os_error());
    |                                ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/process/process_unix.rs:253:17
    |
253 |         Err(io::Error::last_os_error())
    |                 ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:282:14
    |
282 |     ) -> io::Result<Option<Process>> {
    |              ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:420:35
    |
420 |     pub fn kill(&mut self) -> io::Result<()> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:434:35
    |
434 |     pub fn wait(&mut self) -> io::Result<ExitStatus> {
    |                                   ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
   --> library/std/src/sys/unix/process/process_unix.rs:445:39
    |
445 |     pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
    |                                       ^^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Result;
    |
---
1   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0405]: cannot find trait `Read` in module `io`
  --> library/std/src/sys/unix/stdio.rs:15:10
   |
15 | impl io::Read for Stdin {
   |          ^^^^ not found in `io`

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:16:47
   |
16 |     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
   |                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:20:69
   |
20 |     fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
   |                                                                     ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0405]: cannot find trait `Write` in module `io`
  --> library/std/src/sys/unix/stdio.rs:36:10
36 | impl io::Write for Stdout {
   |          ^^^^^ not found in `io`
   |
help: consider importing one of these items
---
   |
1  | use crate::fmt::Write;
   |

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:37:44
   |
37 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
   |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:41:63
   |
41 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
   |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:50:32
   |
50 |     fn flush(&mut self) -> io::Result<()> {
   |                                ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0405]: cannot find trait `Write` in module `io`
  --> library/std/src/sys/unix/stdio.rs:61:10
61 | impl io::Write for Stderr {
   |          ^^^^^ not found in `io`
   |
help: consider importing one of these items
---
   |
1  | use crate::fmt::Write;
   |

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:62:44
   |
62 |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
   |                                            ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:66:63
   |
66 |     fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
   |                                                               ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/stdio.rs:75:32
   |
75 |     fn flush(&mut self) -> io::Result<()> {
   |                                ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0412]: cannot find type `Error` in module `io`
  --> library/std/src/sys/unix/stdio.rs:80:27
   |
80 | pub fn is_ebadf(err: &io::Error) -> bool {
   |                           ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0405]: cannot find trait `Write` in module `io`
  --> library/std/src/sys/unix/stdio.rs:86:42
   |
86 | pub fn panic_output() -> Option<impl io::Write> {
   |                                          ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Write;
   |
   |
1  | use core::fmt::Write;
   |
1  | use crate::fmt::Write;
   |

error[E0412]: cannot find type `Result` in module `io`
  --> library/std/src/sys/unix/thread.rs:27:66
   |
27 |     pub unsafe fn new(stack: usize, p: Box<dyn FnOnce()>) -> io::Result<Thread> {
   |                                                                  ^^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Result;
   |
---
1  | use crate::fmt::Result;
   |
     and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
  --> library/std/src/sys/unix/thread.rs:60:21
   |
60 |             Err(io::Error::from_raw_os_error(ret))
   |                     ^^^^^ not found in `io`
help: consider importing one of these items
   |
1  | use alloc::fmt::Error;
   |
---
1  | use crate::fmt::Error;
   |
     and 1 other candidate

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/thread.rs:179:64
    |
179 |             assert!(ret == 0, "failed to join thread: {}", io::Error::from_raw_os_error(ret));
    |                                                                ^^^^^ not found in `io`
help: consider importing one of these items
    |
1   | use alloc::fmt::Error;
    |
---
1   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `crate::io`
   --> library/std/src/sys/unix/mod.rs:208:47
    |
208 | pub fn cvt<T: IsMinusOne>(t: T) -> crate::io::Result<T> {
    |                                               ^^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
3   | use alloc::fmt::Result;
    |
---
3   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/mod.rs:209:42
    |
209 |     if t.is_minus_one() { Err(crate::io::Error::last_os_error()) } else { Ok(t) }
    |                                          ^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
3   | use alloc::fmt::Error;
    |
---
3   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0412]: cannot find type `Result` in module `crate::io`
   --> library/std/src/sys/unix/mod.rs:212:44
    |
212 | pub fn cvt_r<T, F>(mut f: F) -> crate::io::Result<T>
    |                                            ^^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
3   | use alloc::fmt::Result;
    |
---
3   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0412]: cannot find type `Result` in module `crate::io`
   --> library/std/src/sys/unix/mod.rs:225:49
    |
225 | pub fn cvt_nz(error: libc::c_int) -> crate::io::Result<()> {
    |                                                 ^^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
3   | use alloc::fmt::Result;
    |
---
3   | use crate::fmt::Result;
    |
      and 3 other candidates

error[E0433]: failed to resolve: could not find `Error` in `io`
   --> library/std/src/sys/unix/mod.rs:226:52
    |
226 |     if error == 0 { Ok(()) } else { Err(crate::io::Error::from_raw_os_error(error)) }
    |                                                    ^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
3   | use alloc::fmt::Error;
    |
---
3   | use crate::fmt::Error;
    |
      and 1 other candidate

error[E0405]: cannot find trait `Write` in module `crate::io`
    |
    |
202 |     let write = |err: &mut dyn crate::io::Write| {
    |                                           ^^^^^ not found in `crate::io`
help: consider importing one of these items
    |
12  | use alloc::fmt::Write;
    |
