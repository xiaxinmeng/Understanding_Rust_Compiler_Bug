rust
use std::mem;
use std::net::SocketAddr;

fn main() {
    unsafe {
        let bind_to: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        println!("Trying to bind to {}", bind_to);
        let sock_addr =
            nix::sys::socket::SockAddr::new_inet(nix::sys::socket::InetAddr::from_std(&bind_to));

        let fd = libc::socket(libc::AF_INET, libc::SOCK_STREAM | libc::SOCK_CLOEXEC, 0);
        let (addrp, len) = sock_addr.as_ffi_pair();
        assert_eq!(libc::bind(fd, addrp, len as _), 0);
        assert_eq!(libc::listen(fd, 128), 0);

        let mut storage: libc::sockaddr_storage = mem::zeroed();
        let mut len = mem::size_of_val(&storage) as libc::socklen_t;

        if libc::accept4(
            fd,
            &mut storage as *mut _ as *mut _,
            &mut len,
            libc::SOCK_CLOEXEC,
        ) < 0
        {
            println!("error on accept: {}", nix::errno::errno())
        } else {
            println!("read {} from socket", len);
        }
    }
}
