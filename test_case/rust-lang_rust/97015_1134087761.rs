plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking addr2line v0.16.0
error[E0432]: unresolved import `crate::io::ReadBuf`
 --> library/std/src/sys/windows/fs.rs:5:51
  |
5 | use crate::io::{self, Error, IoSlice, IoSliceMut, ReadBuf, SeekFrom};
  |                                                   |
  |                                                   |
  |                                                   no `ReadBuf` in `io`
  |                                                   help: a similar name exists in the module: `readbuf`
error[E0432]: unresolved import `crate::io::ReadBuf`
 --> library/std/src/sys/windows/handle.rs:4:61
  |
  |
4 | use crate::io::{self, ErrorKind, IoSlice, IoSliceMut, Read, ReadBuf};
  |                                                             |
  |                                                             |
  |                                                             no `ReadBuf` in `io`
  |                                                             help: a similar name exists in the module: `readbuf`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:15
