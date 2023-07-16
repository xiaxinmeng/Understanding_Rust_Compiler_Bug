plain
[01:02:31] 
[01:02:32] error[E0432]: unresolved imports `libc::c_char`, `libc::c_void`
[01:02:32]  --> src/libstd/sys/wasi/fd.rs:7:18
[01:02:32]   |
[01:02:32] 7 | use libc::{self, c_char, c_void};
[01:02:32]   |                  ^^^^^^  ^^^^^^ no `c_void` in the root
[01:02:32]   |                  no `c_char` in the root
[01:02:32] 
[01:02:32] 
[01:02:32] error[E0432]: unresolved imports `libc::__wasi_ciovec_t`, `libc::__wasi_iovec_t`, `libc::c_void`
[01:02:32]  --> src/libstd/sys/wasi/io.rs:4:12
[01:02:32]   |
[01:02:32] 4 | use libc::{__wasi_ciovec_t, __wasi_iovec_t, c_void};
[01:02:32]   |            ^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^  ^^^^^^ no `c_void` in the root
[01:02:32]   |            |                |
[01:02:32]   |            |                no `__wasi_iovec_t` in the root
[01:02:32]   |            no `__wasi_ciovec_t` in the root
[01:02:32] 
[01:02:33] error[E0433]: failed to resolve: could not find `time_t` in `libc`
[01:02:33]   --> src/libstd/sys/wasi/thread.rs:37:44
[01:02:33]    |
[01:02:33] 37 |                     tv_sec: cmp::min(libc::time_t::max_value() as u64, secs) as libc::time_t,
[01:02:33]    |                                            ^^^^^^ could not find `time_t` in `libc`
[01:02:34] error[E0425]: cannot find function `malloc` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/alloc.rs:11:19
[01:02:34]    |
[01:02:34] 11 |             libc::malloc(layout.size()) as *mut u8
---
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `realloc` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/alloc.rs:38:19
[01:02:34]    |
[01:02:34] 38 |             libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
[01:02:34]    |                   ^^^^^^^ not found in `libc`
[01:02:34]    |
[01:02:34] 1  | use alloc::alloc::realloc;
[01:02:34]    |
[01:02:34] 
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `c_void` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/alloc.rs:38:45
[01:02:34]    |
[01:02:34] 38 |             libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
[01:02:34]    |                                             ^^^^^^ not found in `libc`
[01:02:34]    |
[01:02:34] 1  | use core::ffi::c_void;
[01:02:34]    |
[01:02:34] 1  | use core::ffi::c_void;
[01:02:34] 1  | use core::ffi::c_void;
[01:02:34]    |
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_args_sizes_get` in module `libc`
[01:02:34]    |
[01:02:34]    |
[01:02:34] 33 |         cvt_wasi(libc::__wasi_args_sizes_get(&mut argc, &mut argv_buf_size))?;
[01:02:34]    |                        ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] error[E0412]: cannot find type `c_char` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/args.rs:35:45
[01:02:34]    |
[01:02:34]    |
[01:02:34] 35 |         let mut argc = vec![0 as *mut libc::c_char; argc];
[01:02:34]    |                                             ^^^^^^ not found in `libc`
[01:02:34]    |
[01:02:34] 1  | use crate::os::raw::c_char;
[01:02:34]    |
[01:02:34] 
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_args_get` in module `libc`
[01:02:34]    |
[01:02:34]    |
[01:02:34] 37 |         cvt_wasi(libc::__wasi_args_get(argc.as_mut_ptr(), argv_buf.as_mut_ptr()))?;
[01:02:34]    |                        ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_fd_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:11:15
[01:02:34]    |
[01:02:34] 11 |     fd: libc::__wasi_fd_t,
[01:02:34]    |               ^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_iovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:27:51
[01:02:34]    |
[01:02:34] 27 | fn iovec(a: &mut [IoVecMut<'_>]) -> (*const libc::__wasi_iovec_t, usize) {
[01:02:34]    |                                                   ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_iovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:30:30
[01:02:34]    |
[01:02:34] 30 |         mem::size_of::<libc::__wasi_iovec_t>()
[01:02:34]    |                              ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_iovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:34:31
[01:02:34]    |
[01:02:34] 34 |         mem::align_of::<libc::__wasi_iovec_t>()
[01:02:34]    |                               ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_iovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:36:33
[01:02:34]    |
[01:02:34] 36 |     (a.as_ptr() as *const libc::__wasi_iovec_t, a.len())
[01:02:34]    |                                 ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_ciovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:39:45
[01:02:34]    |
[01:02:34] 39 | fn ciovec(a: &[IoVec<'_>]) -> (*const libc::__wasi_ciovec_t, usize) {
[01:02:34]    |                                             ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_ciovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:42:30
[01:02:34]    |
[01:02:34] 42 |         mem::size_of::<libc::__wasi_ciovec_t>()
[01:02:34]    |                              ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_ciovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:46:31
[01:02:34]    |
[01:02:34] 46 |         mem::align_of::<libc::__wasi_ciovec_t>()
[01:02:34]    |                               ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_ciovec_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:48:33
[01:02:34]    |
[01:02:34] 48 |     (a.as_ptr() as *const libc::__wasi_ciovec_t, a.len())
[01:02:34]    |                                 ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_fd_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:52:38
[01:02:34]    |
[01:02:34] 52 |     pub unsafe fn from_raw(fd: libc::__wasi_fd_t) -> WasiFd {
[01:02:34]    |                                      ^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_fd_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:56:36
[01:02:34]    |
[01:02:34] 56 |     pub fn into_raw(self) -> libc::__wasi_fd_t {
[01:02:34]    |                                    ^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_fd_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:62:35
[01:02:34]    |
[01:02:34] 62 |     pub fn as_raw(&self) -> libc::__wasi_fd_t {
[01:02:34]    |                                   ^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_datasync` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:67:33
[01:02:34]    |
[01:02:34] 67 |         cvt_wasi(unsafe { libc::__wasi_fd_datasync(self.fd) })
[01:02:34]    |                                 ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_pread` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:73:33
[01:02:34]    |
[01:02:34] 73 |         cvt_wasi(unsafe { libc::__wasi_fd_pread(self.fd, ptr, len, offset, &mut read) })?;
[01:02:34]    |                                 ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_pwrite` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:80:33
[01:02:34]    |
[01:02:34] 80 |         cvt_wasi(unsafe { libc::__wasi_fd_pwrite(self.fd, ptr, len, offset, &mut read) })?;
[01:02:34]    |                                 ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_read` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:87:33
[01:02:34]    |
[01:02:34] 87 |         cvt_wasi(unsafe { libc::__wasi_fd_read(self.fd, ptr, len, &mut read) })?;
[01:02:34]    |                                 ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_write` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fd.rs:94:33
[01:02:34]    |
[01:02:34] 94 |         cvt_wasi(unsafe { libc::__wasi_fd_write(self.fd, ptr, len, &mut read) })?;
[01:02:34]    |                                 ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_WHENCE_SET` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:100:44
[01:02:34]     |
[01:02:34] 100 |             SeekFrom::Start(pos) => (libc::__WASI_WHENCE_SET, pos as i64),
[01:02:34]     |                                            ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_WHENCE_END` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:101:42
[01:02:34]     |
[01:02:34] 101 |             SeekFrom::End(pos) => (libc::__WASI_WHENCE_END, pos),
[01:02:34]     |                                          ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_WHENCE_CUR` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:102:46
[01:02:34]     |
[01:02:34] 102 |             SeekFrom::Current(pos) => (libc::__WASI_WHENCE_CUR, pos),
[01:02:34]     |                                              ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_seek` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:105:33
[01:02:34]     |
[01:02:34] 105 |         cvt_wasi(unsafe { libc::__wasi_fd_seek(self.fd, offset, whence, &mut pos) })?;
[01:02:34]     |                                 ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_tell` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:111:33
[01:02:34]     |
[01:02:34] 111 |         cvt_wasi(unsafe { libc::__wasi_fd_tell(self.fd, &mut pos) })?;
[01:02:34]     |                                 ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_fdstat_set_flags` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:118:33
[01:02:34]     |
[01:02:34] 118 |         cvt_wasi(unsafe { libc::__wasi_fd_fdstat_set_flags(self.fd, flags) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_fdstat_set_rights` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:122:33
[01:02:34]     |
[01:02:34] 122 |         cvt_wasi(unsafe { libc::__wasi_fd_fdstat_set_rights(self.fd, base, inheriting) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_sync` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:126:33
[01:02:34]     |
[01:02:34] 126 |         cvt_wasi(unsafe { libc::__wasi_fd_sync(self.fd) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_advise` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:130:33
[01:02:34]     |
[01:02:34] 130 |         cvt_wasi(unsafe { libc::__wasi_fd_advise(self.fd, offset, len, advice as u8) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_allocate` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:134:33
[01:02:34]     |
[01:02:34] 134 |         cvt_wasi(unsafe { libc::__wasi_fd_allocate(self.fd, offset, len) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_create_directory` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:139:19
[01:02:34]     |
[01:02:34] 139 |             libc::__wasi_path_create_directory(self.fd, path.as_ptr() as *const c_char, path.len())
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_link` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:151:19
[01:02:34]     |
[01:02:34] 151 |             libc::__wasi_path_link(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_open` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:174:28
[01:02:34]     |
[01:02:34] 174 |             cvt_wasi(libc::__wasi_path_open(
[01:02:34]     |                            ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_readdir` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:192:19
[01:02:34]     |
[01:02:34] 192 |             libc::__wasi_fd_readdir(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_readlink` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:206:19
[01:02:34]     |
[01:02:34] 206 |             libc::__wasi_path_readlink(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_rename` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:220:19
[01:02:34]     |
[01:02:34] 220 |             libc::__wasi_path_rename(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filestat_t` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:231:48
[01:02:34]     |
[01:02:34] 231 |     pub fn filestat_get(&self, buf: *mut libc::__wasi_filestat_t) -> io::Result<()> {
[01:02:34]     |                                                ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_filestat_get` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:232:33
[01:02:34]     |
[01:02:34] 232 |         cvt_wasi(unsafe { libc::__wasi_fd_filestat_get(self.fd, buf) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_filestat_set_times` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:241:33
[01:02:34]     |
[01:02:34] 241 |         cvt_wasi(unsafe { libc::__wasi_fd_filestat_set_times(self.fd, atim, mtim, fstflags) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_filestat_set_size` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:245:33
[01:02:34]     |
[01:02:34] 245 |         cvt_wasi(unsafe { libc::__wasi_fd_filestat_set_size(self.fd, size) })
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filestat_t` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:252:25
[01:02:34]     |
[01:02:34] 252 |         buf: *mut libc::__wasi_filestat_t,
[01:02:34]     |                         ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_filestat_get` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:255:19
[01:02:34]     |
[01:02:34] 255 |             libc::__wasi_path_filestat_get(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_filestat_set_times` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:274:19
[01:02:34]     |
[01:02:34] 274 |             libc::__wasi_path_filestat_set_times(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_symlink` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:288:19
[01:02:34]     |
[01:02:34] 288 |             libc::__wasi_path_symlink(
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_unlink_file` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:300:19
[01:02:34]     |
[01:02:34] 300 |             libc::__wasi_path_unlink_file(self.fd, path.as_ptr() as *const c_char, path.len())
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_path_remove_directory` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:306:19
[01:02:34]     |
[01:02:34] 306 |             libc::__wasi_path_remove_directory(self.fd, path.as_ptr() as *const c_char, path.len())
[01:02:34]     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_sock_recv` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:319:19
[01:02:34]     |
[01:02:34] 319 |             libc::__wasi_sock_recv(self.fd, ptr, len, ri_flags, &mut ro_datalen, &mut ro_flags)
[01:02:34]     |                   ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_sock_send` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:327:33
[01:02:34]     |
[01:02:34] 327 |         cvt_wasi(unsafe { libc::__wasi_sock_send(self.fd, ptr, len, si_flags, &mut so_datalen) })?;
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_SHUT_RD` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:333:37
[01:02:34]     |
[01:02:34] 333 |             Shutdown::Read => libc::__WASI_SHUT_RD,
[01:02:34]     |                                     ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_SHUT_WR` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:334:38
[01:02:34]     |
[01:02:34] 334 |             Shutdown::Write => libc::__WASI_SHUT_WR,
[01:02:34]     |                                      ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_SHUT_WR` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:335:37
[01:02:34]     |
[01:02:34] 335 |             Shutdown::Both => libc::__WASI_SHUT_WR | libc::__WASI_SHUT_RD,
[01:02:34]     |                                     ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_SHUT_RD` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:335:60
[01:02:34]     |
[01:02:34] 335 |             Shutdown::Both => libc::__WASI_SHUT_WR | libc::__WASI_SHUT_RD,
[01:02:34]     |                                                            ^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_sock_shutdown` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:337:33
[01:02:34]     |
[01:02:34] 337 |         cvt_wasi(unsafe { libc::__wasi_sock_shutdown(self.fd, how) })?;
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find function `__wasi_fd_close` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fd.rs:347:19
[01:02:34]     |
[01:02:34] 347 |             libc::__wasi_fd_close(self.fd);
[01:02:34]     |                   ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filestat_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:24:17
[01:02:34]    |
[01:02:34] 24 |     meta: libc::__wasi_filestat_t,
[01:02:34]    |                 ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_dirent_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:41:17
[01:02:34]    |
[01:02:34] 41 |     meta: libc::__wasi_dirent_t,
[01:02:34]    |                 ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_lookupflags_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:50:21
[01:02:34]    |
[01:02:34] 50 |     dirflags: libc::__wasi_lookupflags_t,
[01:02:34]    |                     ^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_fdflags_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:51:20
[01:02:34]    |
[01:02:34] 51 |     fdflags: libc::__wasi_fdflags_t,
[01:02:34]    |                    ^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_oflags_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:52:19
[01:02:34]    |
[01:02:34] 52 |     oflags: libc::__wasi_oflags_t,
[01:02:34]    |                   ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_rights_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:53:31
[01:02:34]    |
[01:02:34] 53 |     rights_base: Option<libc::__wasi_rights_t>,
[01:02:34]    |                               ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_rights_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:54:37
[01:02:34]    |
[01:02:34] 54 |     rights_inheriting: Option<libc::__wasi_rights_t>,
[01:02:34]    |                                     ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filetype_t` in module `libc`
[01:02:34]   --> src/libstd/sys/wasi/fs.rs:64:17
[01:02:34]    |
[01:02:34] 64 |     bits: libc::__wasi_filetype_t,
[01:02:34]    |                 ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filestat_t` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:104:37
[01:02:34]     |
[01:02:34] 104 |     pub fn as_wasi(&self) -> &libc::__wasi_filestat_t {
[01:02:34]     |                                     ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_FILETYPE_DIRECTORY` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:121:28
[01:02:34]     |
[01:02:34] 121 |         self.bits == libc::__WASI_FILETYPE_DIRECTORY
[01:02:34]     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_FILETYPE_REGULAR_FILE` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:125:28
[01:02:34]     |
[01:02:34] 125 |         self.bits == libc::__WASI_FILETYPE_REGULAR_FILE
[01:02:34]     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0425]: cannot find value `__WASI_FILETYPE_SYMBOLIC_LINK` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:129:28
[01:02:34]     |
[01:02:34] 129 |         self.bits == libc::__WASI_FILETYPE_SYMBOLIC_LINK
[01:02:34]     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_filetype_t` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:132:33
[01:02:34]     |
[01:02:34] 132 |     pub fn bits(&self) -> libc::__wasi_filetype_t {
[01:02:34]     |                                 ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:34] error[E0412]: cannot find type `__wasi_dirent_t` in module `libc`
[01:02:34]    --> src/libstd/sys/wasi/fs.rs:176:52
[01:02:34]     |
[01:02:34] 176 |             let dirent_size = mem::size_of::<libc::__wasi_dirent_t>();
[01:02:34]     |                                                    ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:34] 
[01:02:35] error[E0412]: cannot find type `__wasi_dirent_t` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:185:78
[01:02:35]     |
[01:02:35] 185 |                 unsafe { ptr::read_unaligned(dirent.as_ptr() as *const libc::__wasi_dirent_t) };
[01:02:35]     |                                                                              ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `__wasi_inode_t` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:244:32
[01:02:35]     |
[01:02:35] 244 |     pub fn ino(&self) -> libc::__wasi_inode_t {
[01:02:35]     |                                ^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_LOOKUP_SYMLINK_FOLLOW` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:252:31
[01:02:35]     |
[01:02:35] 252 |         base.dirflags = libc::__WASI_LOOKUP_SYMLINK_FOLLOW;
[01:02:35]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_O_TRUNC` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:265:26
[01:02:35]     |
[01:02:35] 265 |         self.oflag(libc::__WASI_O_TRUNC, truncate);
[01:02:35]     |                          ^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_O_CREAT` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:269:26
[01:02:35]     |
[01:02:35] 269 |         self.oflag(libc::__WASI_O_CREAT, create);
[01:02:35]     |                          ^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_O_EXCL` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:273:26
[01:02:35]     |
[01:02:35] 273 |         self.oflag(libc::__WASI_O_EXCL, create_new);
[01:02:35]     |                          ^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_O_CREAT` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:274:26
[01:02:35]     |
[01:02:35] 274 |         self.oflag(libc::__WASI_O_CREAT, create_new);
[01:02:35]     |                          ^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_O_DIRECTORY` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:278:26
[01:02:35]     |
[01:02:35] 278 |         self.oflag(libc::__WASI_O_DIRECTORY, directory);
[01:02:35]     |                          ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `__wasi_oflags_t` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:281:36
[01:02:35]     |
[01:02:35] 281 |     fn oflag(&mut self, bit: libc::__wasi_oflags_t, set: bool) {
[01:02:35]     |                                    ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FDFLAG_APPEND` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:290:27
[01:02:35]     |
[01:02:35] 290 |         self.fdflag(libc::__WASI_FDFLAG_APPEND, set);
[01:02:35]     |                           ^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FDFLAG_DSYNC` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/fs.rs:294:27
[01:02:35]     |
[01:02:35] 294 |         self.fdflag(libc::__WASI_FDFLAG_DSYNC, set);
[01:02:35]     |                           ^^^^^^^^^^^^^^^^^^^ not found in `libc`
---
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_int` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:24:29
[01:02:35]    |
[01:02:35] 24 |         static errno: libc::c_int;
[01:02:35]    |                             ^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_int;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_int` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:32:37
[01:02:35]    |
[01:02:35] 32 |         fn strerror_r(errnum: libc::c_int, buf: *mut libc::c_char,
[01:02:35]    |                                     ^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_int;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_char` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:32:60
[01:02:35]    |
[01:02:35] 32 |         fn strerror_r(errnum: libc::c_int, buf: *mut libc::c_char,
[01:02:35]    |                                                            ^^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_char;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `size_t` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:33:37
[01:02:35]    |
[01:02:35] 33 |                       buflen: libc::size_t) -> libc::c_int;
[01:02:35]    |                                     ^^^^^^ not found in `libc`
[01:02:35] error[E0412]: cannot find type `c_int` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:33:54
[01:02:35]    |
[01:02:35]    |
[01:02:35] 33 |                       buflen: libc::size_t) -> libc::c_int;
[01:02:35]    |                                                      ^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_int;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_char` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:36:31
[01:02:35]    |
[01:02:35] 36 |     let mut buf = [0 as libc::c_char; 1024];
[01:02:35]    |                               ^^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_char;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_int` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/os.rs:40:38
[01:02:35]    |
[01:02:35] 40 |         if strerror_r(errno as libc::c_int, p, buf.len()) < 0 {
[01:02:35]    |                                      ^^^^^ not found in `libc`
[01:02:35]    |
[01:02:35] 1  | use crate::os::raw::c_int;
[01:02:35]    |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `environ` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:108:33
[01:02:35]     |
[01:02:35] 108 |         let mut environ = libc::environ;
[01:02:35]     |                                 ^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `getenv` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:139:23
[01:02:35]     |
[01:02:35] 139 |         let s = libc::getenv(k.as_ptr()) as *const libc::c_char;
[01:02:35]     |                       ^^^^^^ not found in `libc`
[01:02:35]     |
[01:02:35]     |
[01:02:35] 1   | use crate::sys::wasi::os::getenv;
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `c_char` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:139:58
[01:02:35]     |
[01:02:35]     |
[01:02:35] 139 |         let s = libc::getenv(k.as_ptr()) as *const libc::c_char;
[01:02:35]     |                                                          ^^^^^^ not found in `libc`
[01:02:35]     |
[01:02:35] 1   | use crate::os::raw::c_char;
[01:02:35]     |
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `setenv` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:155:19
[01:02:35]     |
[01:02:35] 155 |         cvt(libc::setenv(k.as_ptr(), v.as_ptr(), 1)).map(|_| ())
[01:02:35]     |                   ^^^^^^ not found in `libc`
[01:02:35]     |
[01:02:35]     |
[01:02:35] 1   | use crate::sys::wasi::os::setenv;
[01:02:35] 
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `unsetenv` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:164:19
[01:02:35]     |
[01:02:35] 164 |         cvt(libc::unsetenv(nbuf.as_ptr())).map(|_| ())
[01:02:35]     |                   ^^^^^^^^ not found in `libc`
[01:02:35]     |
[01:02:35]     |
[01:02:35] 1   | use crate::sys::wasi::os::unsetenv;
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `exit` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/os.rs:178:15
[01:02:35]     |
[01:02:35]     |
[01:02:35] 178 |         libc::exit(code)
[01:02:35]     |               ^^^^ not found in `libc`
[01:02:35]     |
[01:02:35] 1   | use crate::process::exit;
[01:02:35]     |
[01:02:35]     |
[01:02:35] 1   | use crate::sys::wasi::os::exit;
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `STDIN_FILENO` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/stdio.rs:20:59
[01:02:35]    |
[01:02:35]    |
[01:02:35] 20 |         ManuallyDrop::new(unsafe { WasiFd::from_raw(libc::STDIN_FILENO as u32) })
[01:02:35]    |                                                           ^^^^^^^^^^^^ not found in `libc`
[01:02:35] error[E0425]: cannot find value `STDOUT_FILENO` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/stdio.rs:35:59
[01:02:35]    |
[01:02:35]    |
[01:02:35] 35 |         ManuallyDrop::new(unsafe { WasiFd::from_raw(libc::STDOUT_FILENO as u32) })
[01:02:35]    |                                                           ^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `STDERR_FILENO` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/stdio.rs:54:59
[01:02:35]    |
[01:02:35] 54 |         ManuallyDrop::new(unsafe { WasiFd::from_raw(libc::STDERR_FILENO as u32) })
[01:02:35]    |                                                           ^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_EBADF` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/stdio.rs:76:38
[01:02:35]    |
[01:02:35] 76 |     err.raw_os_error() == Some(libc::__WASI_EBADF as i32)
[01:02:35]    |                                      ^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `__wasi_sched_yield` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/thread.rs:22:34
[01:02:35]    |
[01:02:35] 22 |         let ret = unsafe { libc::__wasi_sched_yield() };
[01:02:35]    |                                  ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] error[E0422]: cannot find struct, variant or union type `timespec` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/thread.rs:36:36
[01:02:35]    |
[01:02:35]    |
[01:02:35] 36 |                 let mut ts = libc::timespec {
[01:02:35]    |                                    ^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `time_t` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/thread.rs:37:87
[01:02:35]    |
[01:02:35] 37 |                     tv_sec: cmp::min(libc::time_t::max_value() as u64, secs) as libc::time_t,
[01:02:35]    |                                                                                       ^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `nanosleep` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/thread.rs:41:27
[01:02:35]    |
[01:02:35] 41 |                 cvt(libc::nanosleep(&ts, &mut ts)).unwrap();
[01:02:35]    |                           ^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find function `__wasi_clock_time_get` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/time.rs:17:24
[01:02:35]    |
[01:02:35] 17 |         cvt_wasi(libc::__wasi_clock_time_get(
[01:02:35]    |                        ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_CLOCK_MONOTONIC` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/time.rs:31:36
[01:02:35]    |
[01:02:35] 31 |         Instant(current_time(libc::__WASI_CLOCK_MONOTONIC))
[01:02:35]    |                                    ^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_CLOCK_REALTIME` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/time.rs:57:39
[01:02:35]    |
[01:02:35] 57 |         SystemTime(current_time(libc::__WASI_CLOCK_REALTIME))
[01:02:35]    |                                       ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0412]: cannot find type `__wasi_timestamp_t` in module `libc`
[01:02:35]   --> src/libstd/sys/wasi/time.rs:60:42
[01:02:35]    |
[01:02:35] 60 |     pub fn from_wasi_timestamp(ts: libc::__wasi_timestamp_t) -> SystemTime {
[01:02:35]    |                                          ^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FILETYPE_BLOCK_DEVICE` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/ext/fs.rs:339:41
[01:02:35]     |
[01:02:35] 339 |         self.as_inner().bits() == libc::__WASI_FILETYPE_BLOCK_DEVICE
[01:02:35]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FILETYPE_CHARACTER_DEVICE` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/ext/fs.rs:342:41
[01:02:35]     |
[01:02:35] 342 |         self.as_inner().bits() == libc::__WASI_FILETYPE_CHARACTER_DEVICE
[01:02:35]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FILETYPE_SOCKET_DGRAM` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/ext/fs.rs:345:41
[01:02:35]     |
[01:02:35] 345 |         self.as_inner().bits() == libc::__WASI_FILETYPE_SOCKET_DGRAM
[01:02:35]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:35] 
[01:02:35] error[E0425]: cannot find value `__WASI_FILETYPE_SOCKET_STREAM` in module `libc`
[01:02:35]    --> src/libstd/sys/wasi/ext/fs.rs:348:41
[01:02:35]     |
[01:02:35] 348 |         self.as_inner().bits() == libc::__WASI_FILETYPE_SOCKET_STREAM
[01:02:35]     |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:36] error[E0425]: cannot find value `STDIN_FILENO` in module `libc`
[01:02:36]    --> src/libstd/sys/wasi/ext/io.rs:128:15
[01:02:36]     |
[01:02:36] 128 |         libc::STDIN_FILENO as u32
---
[01:02:36]     |
[01:02:36] 134 |         libc::STDOUT_FILENO as u32
[01:02:36]     |               ^^^^^^^^^^^^^ not found in `libc`
[01:02:36] 
[01:02:36] error[E0425]: cannot find value `STDERR_FILENO` in module `libc`
[01:02:36]    --> src/libstd/sys/wasi/ext/io.rs:140:15
[01:02:36] 140 |         libc::STDERR_FILENO as u32
[01:02:36]     |               ^^^^^^^^^^^^^ not found in `libc`
[01:02:36] 
[01:02:36] 
[01:02:36] error[E0425]: cannot find function `abort` in module `libc`
[01:02:36]   --> src/libstd/sys/wasi/mod.rs:86:11
[01:02:36] 86 |     libc::abort()
[01:02:36]    |           ^^^^^ not found in `libc`
[01:02:36] help: possible candidates are found in other modules, you can import them into scope
[01:02:36]    |
---
[01:02:36] 
[01:02:36] error[E0412]: cannot find type `c_void` in module `libc`
[01:02:36]   --> src/libstd/sys/wasi/mod.rs:92:62
[01:02:36]    |
[01:02:36] 92 |         let base = &mut ret as *mut (u64, u64) as *mut libc::c_void;
[01:02:36]    |                                                              ^^^^^^ not found in `libc`
[01:02:36]    |
[01:02:36] 17 | use core::ffi::c_void;
[01:02:36]    |
[01:02:36] 17 | use core::ffi::c_void;
[01:02:36] 17 | use core::ffi::c_void;
[01:02:36]    |
[01:02:36] 
[01:02:36] error[E0425]: cannot find function `__wasi_random_get` in module `libc`
[01:02:36]   --> src/libstd/sys/wasi/mod.rs:94:24
[01:02:36]    |
[01:02:36] 94 |         cvt_wasi(libc::__wasi_random_get(base, len)).unwrap();
[01:02:36]    |                        ^^^^^^^^^^^^^^^^^ not found in `libc`
[01:02:36] 
[01:02:36] error[E0425]: cannot find value `__WASI_ESUCCESS` in module `libc`
[01:02:36]    --> src/libstd/sys/wasi/mod.rs:123:19
[01:02:36]     |
[01:02:36] 123 |     if r != libc::__WASI_ESUCCESS {
[01:02:36]     |                   ^^^^^^^^^^^^^^^ not found in `libc`
[01:02:39] error: aborting due to 181 previous errors
[01:02:39] 
[01:02:39] Some errors have detailed explanations: E0412, E0422, E0425, E0432, E0433.
[01:02:39] For more information about an error, try `rustc --explain E0412`.
---
travis_time:end:09432ac0:start=1555719040126785182,finish=1555719040132714541,duration=5929359
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3105b610
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01885a02
travis_time:start:01885a02
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ab45a80
$ dmesg | grep -i kill
