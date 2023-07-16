
[01:01:24] error[E0609]: no field `dirp` on type `&mut sys::unix::fs::ReadDir`
[01:01:24]    --> libstd/sys/unix/fs.rs:232:52
[01:01:24]     |
[01:01:24] 232 |                 let entry_ptr = libc::readdir(self.dirp.0);
