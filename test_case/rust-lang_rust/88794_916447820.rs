plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking object v0.26.2
    Checking hashbrown v0.11.0
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking addr2line v0.16.0
error[E0433]: failed to resolve: use of undeclared crate or module `io`
   --> library/std/src/os/./windows/io/handle.rs:128:10
128 |     ) -> io::Result<Self> {
    |          ^^ use of undeclared crate or module `io`

error[E0433]: failed to resolve: use of undeclared crate or module `io`
error[E0433]: failed to resolve: use of undeclared crate or module `io`
  --> library/std/src/os/./windows/io/socket.rs:76:32
   |
76 |     pub fn try_clone(&self) -> io::Result<Self> {
   |                                ^^ use of undeclared crate or module `io`
error[E0433]: failed to resolve: use of undeclared crate or module `mem`
error[E0433]: failed to resolve: use of undeclared crate or module `mem`
  --> library/std/src/os/./windows/io/socket.rs:77:33
   |
77 |         let mut info = unsafe { mem::zeroed::<c::WSAPROTOCOL_INFO>() };
   |                                 ^^^ use of undeclared crate or module `mem`
error[E0412]: cannot find type `FileDesc` in this scope
error[E0412]: cannot find type `FileDesc` in this scope
   --> library/std/src/os/./windows/io/handle.rs:117:50
    |
114 | impl OwnedHandle {
    |     - help: you might be missing a type parameter: `<FileDesc>`
...
117 |     pub fn try_clone(&self) -> crate::io::Result<FileDesc> {

error[E0433]: failed to resolve: use of undeclared crate or module `io`
error[E0433]: failed to resolve: use of undeclared crate or module `io`
  --> library/std/src/os/./windows/io/socket.rs:99:32
   |
99 |                 return Err(io::Error::from_raw_os_error(error));
   |                                ^^^^^ not found in `io`
help: consider importing one of these items
   |
5  | use alloc::fmt::Error;
   |
---
5  | use crate::fmt::Error;
   |
     and 2 other candidates

error[E0425]: cannot find function `last_error` in this scope
   --> library/std/src/os/./windows/io/socket.rs:114:28
114 |                 return Err(last_error());
    |                            ^^^^^^^^^^ not found in this scope
    |
help: consider importing this function
help: consider importing this function
    |
5   | use crate::sys::net::last_error;
    |

error[E0599]: no function or associated item named `from_inner` found for struct `socket::OwnedSocket` in the current scope
  --> library/std/src/os/./windows/io/socket.rs:94:31
   |
53 | pub struct OwnedSocket {
   | ---------------------- function or associated item `from_inner` not found for this
...
94 |             unsafe { Ok(Self::from_inner(OwnedSocket::from_raw_socket(socket))) }
   |                               ^^^^^^^^^^ function or associated item not found in `socket::OwnedSocket`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `FromInner` defines an item `from_inner`, perhaps you need to implement it
   |
   |
75 | pub trait FromInner<Inner> {


error[E0599]: no function or associated item named `from_inner` found for struct `socket::OwnedSocket` in the current scope
   --> library/std/src/os/./windows/io/socket.rs:118:36
    |
53  | pub struct OwnedSocket {
    | ---------------------- function or associated item `from_inner` not found for this
...
118 |                 let socket = Self::from_inner(OwnedSocket::from_raw_socket(socket));
    |                                    ^^^^^^^^^^ function or associated item not found in `socket::OwnedSocket`
    = help: items from traits can only be used if the trait is implemented and in scope
    = help: items from traits can only be used if the trait is implemented and in scope
note: `FromInner` defines an item `from_inner`, perhaps you need to implement it
    |
    |
75  | pub trait FromInner<Inner> {


error: the `Self` constructor can only be used with tuple or unit structs
   --> library/std/src/sys/windows/fs.rs:458:12
    |
458 |         Ok(Self(self.0.try_clone()?))
    |            ^^^^ help: use curly brackets: `Self { /* fields */ }`
error[E0609]: no field `0` on type `&sys::windows::fs::File`
   --> library/std/src/sys/windows/fs.rs:458:22
    |
    |
458 |         Ok(Self(self.0.try_clone()?))
    |
    = note: available fields are: `handle`
help: one of the expressions' fields has a field of the same name
    |
    |
458 |         Ok(Self(self.handle.0.try_clone()?))


error[E0599]: no method named `duplicate` found for struct `socket::OwnedSocket` in the current scope
   --> library/std/src/sys/windows/net.rs:211:24
    |
211 |         Ok(Self(self.0.duplicate()?))
    |                        ^^^^^^^^^ method not found in `socket::OwnedSocket`
    |
   ::: library/std/src/os/./windows/io/socket.rs:53:1
    |
53  | pub struct OwnedSocket {
    | ---------------------- method `duplicate` not found for this
Some errors have detailed explanations: E0412, E0425, E0433, E0599, E0609.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `std` due to 11 previous errors
Build completed unsuccessfully in 0:00:20
