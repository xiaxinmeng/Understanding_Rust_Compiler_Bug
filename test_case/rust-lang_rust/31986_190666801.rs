
os/linux/fs.rs:19:5: 19:19 error: unresolved import `os::linux::raw`. Could not find `linux` in `os` [E0432]
os/linux/fs.rs:19 use os::linux::raw;
                      ^~~~~~~~~~~~~~
os/linux/fs.rs:19:5: 19:19 help: run `rustc --explain E0432` to see a detailed explanation
os/linux/fs.rs:76:16: 76:65 error: casting `&libc::unix::notbsd::linux::musl::b32::asmjs::stat` as `*const libc::unix::notbsd::linux::musl::b32::asmjs::stat64` is invalid
os/linux/fs.rs:76             &*(self.as_inner().as_inner() as *const libc::stat64
                                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:105:9: 105:44 error: attempted access of field `st_atime` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:105         self.as_inner().as_inner().st_atime as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:108:9: 108:49 error: attempted access of field `st_atime_nsec` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:108         self.as_inner().as_inner().st_atime_nsec as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:111:9: 111:44 error: attempted access of field `st_mtime` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:111         self.as_inner().as_inner().st_mtime as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:114:9: 114:49 error: attempted access of field `st_mtime_nsec` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:114         self.as_inner().as_inner().st_mtime_nsec as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:117:9: 117:44 error: attempted access of field `st_ctime` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:117         self.as_inner().as_inner().st_ctime as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
os/linux/fs.rs:120:9: 120:49 error: attempted access of field `st_ctime_nsec` on type `&libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
os/linux/fs.rs:120         self.as_inner().as_inner().st_ctime_nsec as i64
                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
sys/unix/fs.rs:153:21: 153:39 error: attempted access of field `st_mtime` on type `libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
sys/unix/fs.rs:153             tv_sec: self.stat.st_mtime as libc::time_t,
                                       ^~~~~~~~~~~~~~~~~~
sys/unix/fs.rs:153:31: 153:39 help: did you mean `st_mtim`?
sys/unix/fs.rs:153             tv_sec: self.stat.st_mtime as libc::time_t,
                                                 ^~~~~~~~
sys/unix/fs.rs:154:22: 154:45 error: attempted access of field `st_mtime_nsec` on type `libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
sys/unix/fs.rs:154             tv_nsec: self.stat.st_mtime_nsec as libc::c_long,
                                        ^~~~~~~~~~~~~~~~~~~~~~~
sys/unix/fs.rs:154:32: 154:45 help: did you mean `st_mtim`?
sys/unix/fs.rs:154             tv_nsec: self.stat.st_mtime_nsec as libc::c_long,
                                                  ^~~~~~~~~~~~~
sys/unix/fs.rs:160:21: 160:39 error: attempted access of field `st_atime` on type `libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
sys/unix/fs.rs:160             tv_sec: self.stat.st_atime as libc::time_t,
                                       ^~~~~~~~~~~~~~~~~~
sys/unix/fs.rs:160:31: 160:39 help: did you mean `st_atim`?
sys/unix/fs.rs:160             tv_sec: self.stat.st_atime as libc::time_t,
                                                 ^~~~~~~~
sys/unix/fs.rs:161:22: 161:45 error: attempted access of field `st_atime_nsec` on type `libc::unix::notbsd::linux::musl::b32::asmjs::stat`, but no field with that name was found
sys/unix/fs.rs:161             tv_nsec: self.stat.st_atime_nsec as libc::c_long,
                                        ^~~~~~~~~~~~~~~~~~~~~~~
sys/unix/fs.rs:161:32: 161:45 help: did you mean `st_atim`?
sys/unix/fs.rs:161             tv_nsec: self.stat.st_atime_nsec as libc::c_long,
                                                  ^~~~~~~~~~~~~
error: aborting due to 11 previous errors
Could not compile `std`.
