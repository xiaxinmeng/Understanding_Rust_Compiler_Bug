
     let hints = c::addrinfo {
         ai_socktype: c::SOCK_STREAM,
         .. mem::zeroed()
    };
