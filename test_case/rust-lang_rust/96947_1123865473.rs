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
    Checking rand v0.7.3
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
    Checking core v0.0.0 (/checkout/library/core)
error[E0252]: the name `OwnedSocket` is defined multiple times
 --> library/std/src/os/windows/io/tests.rs:5:68
  |
5 |         BorrowedSocket, FromRawSocket, IntoRawSocket, OwnedSocket, OwnedSocket, RawSocket,
  |                                                       -----------  ^^^^^^^^^^^--
  |                                                       |            |
  |                                                       |            `OwnedSocket` reimported here
  |                                                       |            help: remove unnecessary import
  |                                                       previous import of the type `OwnedSocket` here
  |
  = note: `OwnedSocket` must be defined only once in the type namespace of this block

error: unused import: `OwnedSocket`
 --> library/std/src/os/windows/io/tests.rs:5:68
  |
5 |         BorrowedSocket, FromRawSocket, IntoRawSocket, OwnedSocket, OwnedSocket, RawSocket,
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0252`.
error: could not compile `std` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:00:36
