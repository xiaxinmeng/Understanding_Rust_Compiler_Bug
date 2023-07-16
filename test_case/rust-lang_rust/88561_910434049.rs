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
error[E0277]: the trait bound `sys::windows::process::Stdio: From<io::stdio::Stdin>` is not satisfied
     |
     |
1425 |         Stdio::from_inner(inherit.into())
     |                           ^^^^^^^^^^^^^^ the trait `From<io::stdio::Stdin>` is not implemented for `sys::windows::process::Stdio`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <sys::windows::process::Stdio as From<AnonPipe>>
               <sys::windows::process::Stdio as From<sys::windows::fs::File>>
     = note: required because of the requirements on the impl of `Into<sys::windows::process::Stdio>` for `io::stdio::Stdin`

error[E0277]: the trait bound `sys::windows::process::Stdio: From<io::stdio::Stdout>` is not satisfied
     |
     |
1454 |         Stdio::from_inner(inherit.into())
     |                           ^^^^^^^^^^^^^^ the trait `From<io::stdio::Stdout>` is not implemented for `sys::windows::process::Stdio`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <sys::windows::process::Stdio as From<AnonPipe>>
               <sys::windows::process::Stdio as From<sys::windows::fs::File>>
     = note: required because of the requirements on the impl of `Into<sys::windows::process::Stdio>` for `io::stdio::Stdout`

error[E0277]: the trait bound `sys::windows::process::Stdio: From<io::stdio::Stderr>` is not satisfied
     |
     |
1483 |         Stdio::from_inner(inherit.into())
     |                           ^^^^^^^^^^^^^^ the trait `From<io::stdio::Stderr>` is not implemented for `sys::windows::process::Stdio`
     = help: the following implementations were found:
     = help: the following implementations were found:
               <sys::windows::process::Stdio as From<AnonPipe>>
               <sys::windows::process::Stdio as From<sys::windows::fs::File>>
     = note: required because of the requirements on the impl of `Into<sys::windows::process::Stdio>` for `io::stdio::Stderr`
For more information about this error, try `rustc --explain E0277`.
error: could not compile `std` due to 3 previous errors
Build completed unsuccessfully in 0:00:17
