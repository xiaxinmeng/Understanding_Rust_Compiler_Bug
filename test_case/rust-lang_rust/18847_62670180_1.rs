
SYNOPSIS
     #include <sys/socket.h>

     int
     listen(int socket, int backlog);

DESCRIPTION
     Creation of socket-based connections requires several operations.
     First, a socket is created with socket(2).  Next, a willingness to
     accept incoming connections and a queue limit for incoming connections
     are specified with listen().  Finally, the connections are accepted
     with accept(2).  The listen() call applies only to sockets of type
     SOCK_STREAM or SOCK_SEQPACKET.

     The backlog parameter defines the maximum length for the queue of pend-
     ing connections.  If a connection request arrives with the queue full,
     the client may receive an error with an indication of ECONNREFUSED.
     Alternatively, if the underlying protocol supports retransmission, the
     request may be ignored so that retries may succeed.
