
 error: lifetime parameter `'tcx` never used
  --> src/tools/miri/src/shims/unix/linux/fd/socketpair.rs:18:12
   |
18 |     fn dup<'tcx>(&mut self) -> io::Result<Box<dyn FileDescriptor>> {
   |           -^^^^- help: elide the unused lifetime
