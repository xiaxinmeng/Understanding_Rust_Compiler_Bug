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
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error[E0308]: mismatched types
   --> library/std/src/os/./windows/io/handle.rs:121:50
    |
121 |         Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
    |                                                  ^^^^^^ expected *-ptr, found struct `os::imp::windows::io::handle::OwnedHandle`
    |
    = note: expected raw pointer `*mut libc::c_void`
                    found struct `os::imp::windows::io::handle::OwnedHandle`

error: the `Self` constructor can only be used with tuple or unit structs
  --> library/std/src/os/./windows/io/socket.rs:96:25
   |
96 |             unsafe { Ok(Self(OwnedSocket::from_raw_socket(socket))) }
   |                         ^^^^ help: use curly brackets: `Self { /* fields */ }`

error: the `Self` constructor can only be used with tuple or unit structs
   --> library/std/src/os/./windows/io/socket.rs:120:30
    |
120 |                 let socket = Self(OwnedSocket::from_raw_socket(socket));
    |                              ^^^^ help: use curly brackets: `Self { /* fields */ }`

error[E0599]: no method named `try_clone` found for struct `Handle` in the current scope
   --> library/std/src/sys/windows/fs.rs:458:39
    |
458 |         Ok(Self { handle: self.handle.try_clone()? })
    |                                       ^^^^^^^^^ method not found in `Handle`
   ::: library/std/src/sys/windows/handle.rs:17:1
    |
    |
17  | pub struct Handle(OwnedHandle);
    | ------------------------------- method `try_clone` not found for this
Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `std` due to 4 previous errors
Build completed unsuccessfully in 0:00:18
