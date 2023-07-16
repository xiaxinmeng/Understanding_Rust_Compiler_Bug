rust
fn create_bindless_socket() -> std::io::Result<UdpSocket> {
    #[cfg(unix)]
    {
        use std::os::unix::io::FromRawFd;
        let fd = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_UDP) };
        if fd == -1 {
            return Err(std::io::Error::last_os_error());
        }
        let socket = unsafe { UdpSocket::from_raw_fd(fd) };
        Ok(socket)
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::FromRawSocket;
        use std::os::windows::io::RawSocket;
        use windows_sys::Win32::Networking::WinSock::SOCK_DGRAM;
        use windows_sys::Win32::Networking::WinSock::IPPROTO_UDP;
        use windows_sys::Win32::Networking::WinSock::AF_INET;
        use windows_sys::Win32::Networking::WinSock::socket;
        use windows_sys::Win32::Networking::WinSock::WSAGetLastError;
        use windows_sys::Win32::Networking::WinSock::INVALID_SOCKET;
        let sock = unsafe { socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP) };
        if sock == INVALID_SOCKET {
            return Err(std::io::Error::from_raw_os_error(unsafe { WSAGetLastError() }));
        }
        let socket = unsafe { UdpSocket::from_raw_socket(sock as RawSocket) };
        Ok(socket)
    }
}
