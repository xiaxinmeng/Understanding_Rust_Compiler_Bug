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
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
  --> library/std/src/rt.rs:78:9
   |
78 |         sys::init(argc, argv, ignore_sigpipe);
   |         ^^^^^^^^^             -------------- argument unexpected
note: function defined here
  --> library/std/src/sys/windows/mod.rs:50:15
   |
   |
50 | pub unsafe fn init(_argc: isize, _argv: *const *const u8) {
help: remove the extra argument
   |
   |
78 |         sys::init(argc, argv);

For more information about this error, try `rustc --explain E0061`.
error: could not compile `std` due to previous error
Build completed unsuccessfully in 0:00:16
