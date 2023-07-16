
class socket_handle {
    let sockfd: libc::c_int;
    new(x: libc::c_int) {self.sockfd = x;}
    drop {c::close(self.sockfd);}
}
