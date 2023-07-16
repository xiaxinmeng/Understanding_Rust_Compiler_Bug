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
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking unicode-width v0.1.8
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/library/test)
error[E0425]: cannot find function `getrandom` in crate `libc`
  --> library/test/src/helpers/shuffle.rs:10:28
   |
10 |             unsafe { libc::getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_NONBLOCK) };
   |                            ^^^^^^^^^ not found in `libc`

error[E0425]: cannot find value `GRND_NONBLOCK` in crate `libc`
  --> library/test/src/helpers/shuffle.rs:10:80
   |
10 |             unsafe { libc::getrandom(buf.as_mut_ptr().cast(), buf.len(), libc::GRND_NONBLOCK) };
   |                                                                                ^^^^^^^^^^^^^ not found in `libc`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `test` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
