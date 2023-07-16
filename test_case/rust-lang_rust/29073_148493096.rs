 rust
extern "system" {
     fn getnameinfo(sa: *const libc::sockaddr, salen: socklen_t,
                    host: *mut c_char, hostlen: libc::size_t,
                    serv: *mut c_char, servlen: libc::size_t,
                    flags: c_int) -> c_int;
}
