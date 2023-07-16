plain
   Compiling hashbrown v0.12.0
   Compiling miniz_oxide v0.4.0
   Compiling object v0.26.2
   Compiling addr2line v0.16.0
error[E0201]: duplicate definitions with name `set_quickack`:
    |
    |
412 | /     pub fn set_quickack(&self, quickack: bool) -> io::Result<()> {
413 | |         setsockopt(self, libc::IPPROTO_TCP, libc::TCP_QUICKACK, quickack as c_int)
414 | |     }
    | |_____- previous definition of `set_quickack` here
...
423 | /     pub fn set_quickack(&self, quickack: bool) -> io::Result<()> {
424 | |         setsockopt(self, libc::IPPROTO_TCP, libc::TCP_QUICKACK, quickack as c_int)
    | |_____^ duplicate definition


error[E0201]: duplicate definitions with name `quickack`:
    |
    |
417 | /     pub fn quickack(&self) -> io::Result<bool> {
418 | |         let raw: c_int = getsockopt(self, libc::IPPROTO_TCP, libc::TCP_QUICKACK)?;
419 | |         Ok(raw != 0)
420 | |     }
    | |_____- previous definition of `quickack` here
...
428 | /     pub fn quickack(&self) -> io::Result<bool> {
429 | |         let raw: c_int = getsockopt(self, libc::IPPROTO_TCP, libc::TCP_QUICKACK)?;
430 | |         Ok(raw != 0)
    | |_____^ duplicate definition

For more information about this error, try `rustc --explain E0201`.
error: could not compile `std` due to 2 previous errors
