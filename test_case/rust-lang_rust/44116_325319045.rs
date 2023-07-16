
[00:40:38] warning: dropping unsupported crate type `dylib` for target `asmjs-unknown-emscripten`
[00:40:38] 
[00:40:39] error[E0425]: cannot find function `pthread_sigmask` in module `libc`
[00:40:39]    --> /checkout/src/libstd/sys/unix/process/process_unix.rs:206:26
[00:40:39]     |
[00:40:39] 206 |             t!(cvt(libc::pthread_sigmask(libc::SIG_SETMASK, &set,
[00:40:39]     |                          ^^^^^^^^^^^^^^^ not found in `libc`
[00:40:39] 
[00:40:40] error[E0308]: mismatched types
[00:40:40]    --> /checkout/src/libstd/ffi/c_str.rs:313:32
[00:40:40]     |
[00:40:40] 313 |         let len = libc::strlen(ptr) + 1; // Including the NUL byte
[00:40:40]     |                                ^^^ expected i8, found u8
[00:40:40]     |
[00:40:40]     = note: expected type `*const i8`
[00:40:40]                found type `*mut u8`
[00:40:40] 
[00:40:40] error[E0308]: mismatched types
[00:40:40]    --> /checkout/src/libstd/ffi/c_str.rs:749:32
[00:40:40]     |
[00:40:40] 749 |         let len = libc::strlen(ptr);
[00:40:40]     |                                ^^^ expected i8, found u8
[00:40:40]     |
[00:40:40]     = note: expected type `*const i8`
[00:40:40]                found type `*const u8`
[00:40:40]     = help: here are some functions which might fulfill your needs:
[00:40:40]             - .offset(...)
[00:40:40]             - .wrapping_offset(...)
[00:40:40] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys_common/net.rs:172:38
[00:40:41]     |
[00:40:41] 172 |         match cvt_gai(c::getaddrinfo(c_host.as_ptr(), ptr::null(), &hints, &mut res)) {
[00:40:41]     |                                      ^^^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/weak.rs:78:37
[00:40:41]    |
[00:40:41] 78 |     libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) as usize
[00:40:41]    |                                     ^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]    |
[00:40:41]    = note: expected type `*const i8`
[00:40:41]               found type `*const u8`
[00:40:41]    = help: here are some functions which might fulfill your needs:
[00:40:41]            - .offset(...)
[00:40:41]            - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/args.rs:84:28
[00:40:41]    |
[00:40:41] 84 |             CStr::from_ptr(*argv.offset(i) as *const libc::c_char).to_bytes().to_vec()
[00:40:41]    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8
[00:40:41]    |
[00:40:41]    = note: expected type `*const u8`
[00:40:41]               found type `*const i8`
[00:40:41]    = help: here are some functions which might fulfill your needs:
[00:40:41]            - .offset(...)
[00:40:41]            - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/backtrace/printing/dladdr.rs:28:28
[00:40:41]    |
[00:40:41] 28 |             CStr::from_ptr(info.dli_sname).to_str().ok()
[00:40:41]    |                            ^^^^^^^^^^^^^^ expected u8, found i8
[00:40:41]    |
[00:40:41]    = note: expected type `*const u8`
[00:40:41]               found type `*const i8`
[00:40:41]    = help: here are some functions which might fulfill your needs:
[00:40:41]            - .offset(...)
[00:40:41]            - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/fd.rs:82:41
[00:40:41]    |
[00:40:41] 82 |             cvt(pread64(fd, buf, count, offset))
[00:40:41]    |                                         ^^^^^^ expected i32, found i64
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fd.rs:115:42
[00:40:41]     |
[00:40:41] 115 |             cvt(pwrite64(fd, buf, count, offset))
[00:40:41]     |                                          ^^^^^^ expected i32, found i64
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:352:28
[00:40:41]     |
[00:40:41] 352 |             CStr::from_ptr(self.entry.d_name.as_ptr()).to_bytes()
[00:40:41]     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8
[00:40:41]     |
[00:40:41]     = note: expected type `*const u8`
[00:40:41]                found type `*const i8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:434:20
[00:40:41]     |
[00:40:41] 434 |             open64(path.as_ptr(), flags, opts.mode as c_int)
[00:40:41]     |                    ^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:517:52
[00:40:41]     |
[00:40:41] 517 |         let n = cvt(unsafe { lseek64(self.0.raw(), pos, whence) })?;
[00:40:41]     |                                                    ^^^ expected i32, found i64
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:542:34
[00:40:41]     |
[00:40:41] 542 |         cvt(unsafe { libc::mkdir(p.as_ptr(), self.mode) })?;
[00:40:41]     |                                  ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:631:33
[00:40:41]     |
[00:40:41] 631 |         let ptr = libc::opendir(p.as_ptr());
[00:40:41]     |                                 ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:642:31
[00:40:41]     |
[00:40:41] 642 |     cvt(unsafe { libc::unlink(p.as_ptr()) })?;
[00:40:41]     |                               ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:649:31
[00:40:41]     |
[00:40:41] 649 |     cvt(unsafe { libc::rename(old.as_ptr(), new.as_ptr()) })?;
[00:40:41]     |                               ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:649:45
[00:40:41]     |
[00:40:41] 649 |     cvt(unsafe { libc::rename(old.as_ptr(), new.as_ptr()) })?;
[00:40:41]     |                                             ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:655:35
[00:40:41]     |
[00:40:41] 655 |     cvt_r(|| unsafe { libc::chmod(p.as_ptr(), perm.mode) })?;
[00:40:41]     |                                   ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:661:30
[00:40:41]     |
[00:40:41] 661 |     cvt(unsafe { libc::rmdir(p.as_ptr()) })?;
[00:40:41]     |                              ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:694:28
[00:40:41]     |
[00:40:41] 694 |             libc::readlink(p, buf.as_mut_ptr() as *mut _, buf.capacity())
[00:40:41]     |                            ^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:715:32
[00:40:41]     |
[00:40:41] 715 |     cvt(unsafe { libc::symlink(src.as_ptr(), dst.as_ptr()) })?;
[00:40:41]     |                                ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:715:46
[00:40:41]     |
[00:40:41] 715 |     cvt(unsafe { libc::symlink(src.as_ptr(), dst.as_ptr()) })?;
[00:40:41]     |                                              ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:722:29
[00:40:41]     |
[00:40:41] 722 |     cvt(unsafe { libc::link(src.as_ptr(), dst.as_ptr()) })?;
[00:40:41]     |                             ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:722:43
[00:40:41]     |
[00:40:41] 722 |     cvt(unsafe { libc::link(src.as_ptr(), dst.as_ptr()) })?;
[00:40:41]     |                                           ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:730:16
[00:40:41]     |
[00:40:41] 730 |         stat64(p.as_ptr(), &mut stat as *mut _ as *mut _)
[00:40:41]     |                ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:739:17
[00:40:41]     |
[00:40:41] 739 |         lstat64(p.as_ptr(), &mut stat as *mut _ as *mut _)
[00:40:41]     |                 ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:748:32
[00:40:41]     |
[00:40:41] 748 |         let r = libc::realpath(path.as_ptr(), ptr::null_mut());
[00:40:41]     |                                ^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/fs.rs:752:30
[00:40:41]     |
[00:40:41] 752 |         buf = CStr::from_ptr(r).to_bytes().to_vec();
[00:40:41]     |                              ^ expected u8, found i8
[00:40:41]     |
[00:40:41]     = note: expected type `*const u8`
[00:40:41]                found type `*mut i8`
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/net.rs:59:39
[00:40:41]    |
[00:40:41] 59 |         str::from_utf8(CStr::from_ptr(libc::gai_strerror(err)).to_bytes()).unwrap()
[00:40:41]    |                                       ^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8
[00:40:41]    |
[00:40:41]    = note: expected type `*const u8`
[00:40:41]               found type `*const i8`
[00:40:41]    = help: here are some functions which might fulfill your needs:
[00:40:41]            - .offset(...)
[00:40:41]            - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:112:42
[00:40:41]     |
[00:40:41] 112 |                 let len = CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_bytes().len();
[00:40:41]     |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected u8, found i8
[00:40:41]     |
[00:40:41]     = note: expected type `*const u8`
[00:40:41]                found type `*const i8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:136:27
[00:40:41]     |
[00:40:41] 136 |         match libc::chdir(p.as_ptr()) == (0 as c_int) {
[00:40:41]     |                           ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:391:59
[00:40:41]     |
[00:40:41] 391 |             if let Some(key_value) = parse(CStr::from_ptr(*environ).to_bytes()) {
[00:40:41]     |                                                           ^^^^^^^^ expected u8, found i8
[00:40:41]     |
[00:40:41]     = note: expected type `*const u8`
[00:40:41]                found type `*const i8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:426:30
[00:40:41]     |
[00:40:41] 426 |         let s = libc::getenv(k.as_ptr()) as *const _;
[00:40:41]     |                              ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:443:36
[00:40:41]     |
[00:40:41] 443 |         let ret = cvt(libc::setenv(k.as_ptr(), v.as_ptr(), 1)).map(|_| ());
[00:40:41]     |                                    ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:443:48
[00:40:41]     |
[00:40:41] 443 |         let ret = cvt(libc::setenv(k.as_ptr(), v.as_ptr(), 1)).map(|_| ());
[00:40:41]     |                                                ^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/os.rs:454:38
[00:40:41]     |
[00:40:41] 454 |         let ret = cvt(libc::unsetenv(nbuf.as_ptr())).map(|_| ());
[00:40:41]     |                                      ^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]   --> /checkout/src/libstd/sys/unix/process/process_common.rs:96:24
[00:40:41]    |
[00:40:41] 96 |             argv: vec![program.as_ptr(), ptr::null()],
[00:40:41]    |                        ^^^^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]    |
[00:40:41]    = note: expected type `*const i8`
[00:40:41]               found type `*const u8`
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/process/process_common.rs:116:42
[00:40:41]     |
[00:40:41] 116 |         self.argv[self.args.len() + 1] = arg.as_ptr();
[00:40:41]     |                                          ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/process/process_common.rs:136:30
[00:40:41]     |
[00:40:41] 136 |             self.envp = Some(envp);
[00:40:41]     |                              ^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `alloc::Vec<*const i8>`
[00:40:41]                found type `alloc::Vec<*const u8>`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .to_vec()
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/process/process_common.rs:152:27
[00:40:41]     |
[00:40:41] 152 |                 envp[i] = new_key.as_ptr();
[00:40:41]     |                           ^^^^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/process/process_common.rs:157:33
[00:40:41]     |
[00:40:41] 157 |                 envp[len - 1] = new_key.as_ptr();
[00:40:41]     |                                 ^^^^^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error[E0308]: mismatched types
[00:40:41]    --> /checkout/src/libstd/sys/unix/process/process_unix.rs:180:32
[00:40:41]     |
[00:40:41] 180 |             t!(cvt(libc::chdir(cwd.as_ptr())));
[00:40:41]     |                                ^^^^^^^^^^^^ expected i8, found u8
[00:40:41]     |
[00:40:41]     = note: expected type `*const i8`
[00:40:41]                found type `*const u8`
[00:40:41]     = help: here are some functions which might fulfill your needs:
[00:40:41]             - .offset(...)
[00:40:41]             - .wrapping_offset(...)
[00:40:41] 
[00:40:41] error: aborting due to 42 previous errors
[00:40:41] 
[00:40:41] error: Could not compile `std`.
