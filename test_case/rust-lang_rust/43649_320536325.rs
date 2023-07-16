
error[E0451]: field `__sparcv9_pad` of struct `libc::addrinfo` is private
   --> src/libstd/sys_common/net.rs:170:12
    |
170 |         .. unsafe { mem::zeroed() }
    |            ^^^^^^^^^^^^^^^^^^^^^^^^ field `__sparcv9_pad` is private

error: aborting due to previous error

error: Could not compile `std`.
