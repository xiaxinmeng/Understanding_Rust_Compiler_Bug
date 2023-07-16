plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking addr2line v0.16.0
error[E0432]: unresolved import `libc::iovec`
   --> library/std/src/io/readbuf.rs:311:20
    |
311 | use libc::{c_void, iovec};
    |                    ^^^^^ no `iovec` in the root
error[E0599]: no method named `read_buf_vectored` found for struct `sys::windows::fs::File` in the current scope
   --> library/std/src/fs.rs:742:20
    |
    |
742 |         self.inner.read_buf_vectored(cursor)
    |
   ::: library/std/src/sys/windows/fs.rs:21:1
    |
21  | pub struct File {
21  | pub struct File {
    | --------------- method `read_buf_vectored` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf_vectored`, perhaps you need to implement it
    |
572 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf_vectored` found for struct `sys::windows::fs::File` in the current scope
   --> library/std/src/fs.rs:802:20
    |
802 |         self.inner.read_buf_vectored(cursor)
    |
   ::: library/std/src/sys/windows/fs.rs:21:1
    |
21  | pub struct File {
21  | pub struct File {
    | --------------- method `read_buf_vectored` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf_vectored`, perhaps you need to implement it
    |
572 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf_vectored` found for struct `AnonPipe` in the current scope
    |
    |
362 |         self.inner.read_buf_vectored(cursor)
    |
   ::: library/std/src/sys/windows/pipe.rs:21:1
    |
21  | pub struct AnonPipe {
21  | pub struct AnonPipe {
    | ------------------- method `read_buf_vectored` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf_vectored`, perhaps you need to implement it
    |
572 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf_vectored` found for struct `AnonPipe` in the current scope
    |
    |
427 |         self.inner.read_buf_vectored(cursor)
    |
   ::: library/std/src/sys/windows/pipe.rs:21:1
    |
21  | pub struct AnonPipe {
21  | pub struct AnonPipe {
    | ------------------- method `read_buf_vectored` not found for this struct
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf_vectored`, perhaps you need to implement it
    |
572 | pub trait Read {
    | ^^^^^^^^^^^^^^


error[E0599]: no method named `read_buf_vectored` found for struct `Socket` in the current scope
   --> library/std/src/sys_common/net.rs:277:20
    |
277 |         self.inner.read_buf_vectored(cursor)
    |
   ::: library/std/src/io/mod.rs:866:8
    |
    |
866 |     fn read_buf_vectored(&mut self, bufs: BorrowedSliceCursor<'_>) -> Result<()> {
    |        ----------------- the method is available for `&Socket` here
   ::: library/std/src/sys/windows/net.rs:30:1
    |
    |
30  | pub struct Socket(OwnedSocket);
    | ----------------- method `read_buf_vectored` not found for this struct
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `io::Read` defines an item `read_buf_vectored`, perhaps you need to implement it
    |
572 | pub trait Read {
    | ^^^^^^^^^^^^^^

