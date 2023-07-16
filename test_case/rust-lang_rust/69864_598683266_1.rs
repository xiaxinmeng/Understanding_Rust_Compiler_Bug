
       If the space allocated for receiving incoming ancillary data is too
       small then the ancillary data is truncated to the number of headers
       that will fit in the supplied buffer (or, in the case of an
       SCM_RIGHTS file descriptor list, the list of file descriptors may be
       truncated).  If no buffer is provided for incoming ancillary data
       (i.e., the msg_control field of the msghdr structure supplied to
       recvmsg(2) is NULL), then the incoming ancillary data is discarded.
       In both of these cases, the MSG_CTRUNC flag will be set in the
       msg.msg_flags value returned by recvmsg(2).
