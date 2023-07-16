plain
   Compiling addr2line v0.16.0
error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:111:17
     |
111  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected `i32`, found `usize`
     |                          expected `i32`, found `usize`
     |
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {
     |        ^^^
help: you can convert a `usize` to an `i32` and panic if the converted value doesn't fit
     |
111  |                 cmp::min(buf.len().try_into().unwrap(), READ_LIMIT),
     |                                   ++++++++++++++++++++
help: you can convert a `usize` to an `i32` and panic if the converted value doesn't fit
     |
111  |                 cmp::min(buf.len(), READ_LIMIT.try_into().unwrap()),

error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:111:17
     |
     |
111  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected *-ptr, found `usize`
     |                          expected *-ptr, found `usize`
     |
     = note: expected raw pointer `*mut libc::c_void`
                       found type `usize`
                       found type `usize`
     = note: expected raw pointer `*mut libc::c_void`
                       found type `usize`
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {

error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:111:17
     |
     |
111  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected `i64`, found `usize`
     |                          expected `i64`, found `usize`
     |
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {
     |        ^^^
help: you can convert a `usize` to an `i64` and panic if the converted value doesn't fit
     |
111  |                 cmp::min(buf.len().try_into().unwrap(), READ_LIMIT),
     |                                   ++++++++++++++++++++
help: you can convert a `usize` to an `i64` and panic if the converted value doesn't fit
     |
111  |                 cmp::min(buf.len(), READ_LIMIT.try_into().unwrap()),

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fd.rs:112:17
     |
     |
108  |             cvt(pread64(
     |                 ------- arguments to this function are incorrect
112  |                 offset as libc::off_t,
     |                 ^^^^^^^^^^^^^^^^^^^^^ expected `i64`, found `i32`
     |
note: function defined here
note: function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1685:12
     |
1685 |     pub fn pread64(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off64_t) -> ::ssize_t;
help: you can convert an `i32` to an `i64`
     |
     |
112  |                 (offset as libc::off_t).into(),

error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:178:17
     |
     |
178  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected `i32`, found `usize`
     |                          expected `i32`, found `usize`
     |
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {
     |        ^^^
help: you can convert a `usize` to an `i32` and panic if the converted value doesn't fit
     |
178  |                 cmp::min(buf.len().try_into().unwrap(), READ_LIMIT),
     |                                   ++++++++++++++++++++
help: you can convert a `usize` to an `i32` and panic if the converted value doesn't fit
     |
178  |                 cmp::min(buf.len(), READ_LIMIT.try_into().unwrap()),

error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:178:17
     |
     |
178  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected *-ptr, found `usize`
     |                          expected *-ptr, found `usize`
     |
     = note: expected raw pointer `*const libc::c_void`
                       found type `usize`
                       found type `usize`
     = note: expected raw pointer `*const libc::c_void`
                       found type `usize`
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {

error[E0308]: arguments to this function are incorrect
    --> library/std/src/sys/unix/fd.rs:178:17
     |
     |
178  |                 cmp::min(buf.len(), READ_LIMIT),
     |                 ^^^^^^^^ ---------  ---------- expected `i64`, found `usize`
     |                          expected `i64`, found `usize`
     |
note: function defined here
    --> /checkout/library/core/src/cmp.rs:1194:8
    --> /checkout/library/core/src/cmp.rs:1194:8
     |
1194 | pub fn min<T: Ord>(v1: T, v2: T) -> T {
     |        ^^^
help: you can convert a `usize` to an `i64` and panic if the converted value doesn't fit
     |
178  |                 cmp::min(buf.len().try_into().unwrap(), READ_LIMIT),
     |                                   ++++++++++++++++++++
help: you can convert a `usize` to an `i64` and panic if the converted value doesn't fit
     |
178  |                 cmp::min(buf.len(), READ_LIMIT.try_into().unwrap()),

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fd.rs:179:17
     |
     |
175  |             cvt(pwrite64(
     |                 -------- arguments to this function are incorrect
179  |                 offset as libc::off_t,
     |                 ^^^^^^^^^^^^^^^^^^^^^ expected `i64`, found `i32`
     |
note: function defined here
note: function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1686:12
     |
1686 |     pub fn pwrite64(
     |            ^^^^^^^^
help: you can convert an `i32` to an `i64`
     |
179  |                 (offset as libc::off_t).into(),

error[E0308]: mismatched types
    --> library/std/src/sys/unix/fs.rs:969:56
     |
     |
969  |         let n = cvt(unsafe { lseek64(self.as_raw_fd(), pos as libc::off_t, whence) })?;
     |                              -------                   ^^^^^^^^^^^^^^^^^^ expected `i64`, found `i32`
     |                              arguments to this function are incorrect
     |
note: function defined here
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1673:12
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1673:12
     |
1673 |     pub fn lseek64(fd: ::c_int, offset: off64_t, whence: ::c_int) -> off64_t;
help: you can convert an `i32` to an `i64`
     |
     |
969  |         let n = cvt(unsafe { lseek64(self.as_raw_fd(), (pos as libc::off_t).into(), whence) })?;

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std` due to 9 previous errors
Build completed unsuccessfully in 0:00:44
