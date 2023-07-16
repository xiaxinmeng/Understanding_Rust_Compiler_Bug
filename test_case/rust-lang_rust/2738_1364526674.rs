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
    Checking tracing-error v0.2.0
error: lifetime parameter `'tcx` never used
  --> src/tools/miri/src/shims/unix/linux/fd/epoll.rs:39:12
   |
39 |     fn dup<'tcx>(&mut self) -> io::Result<Box<dyn FileDescriptor>> {
   |           -^^^^- help: elide the unused lifetime
   |
   = note: `-D unused-lifetimes` implied by `-D warnings`
error: lifetime parameter `'tcx` never used
  --> src/tools/miri/src/shims/unix/linux/fd/event.rs:24:12
   |
   |
24 |     fn dup<'tcx>(&mut self) -> io::Result<Box<dyn FileDescriptor>> {
   |           -^^^^- help: elide the unused lifetime
error: lifetime parameter `'tcx` never used
  --> src/tools/miri/src/shims/unix/linux/fd/socketpair.rs:18:12
   |
   |
18 |     fn dup<'tcx>(&mut self) -> io::Result<Box<dyn FileDescriptor>> {
   |           -^^^^- help: elide the unused lifetime
    Checking color-spantrace v0.2.0
    Checking addr2line v0.17.0
    Checking color-eyre v0.6.2
    Checking ui_test v0.5.0
