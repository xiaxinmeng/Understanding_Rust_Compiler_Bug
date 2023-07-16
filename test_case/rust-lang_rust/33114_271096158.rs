
$ telnet 127.0.0.1 1234
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
QEMU 2.8.0 monitor - type 'help' for more information
(qemu) info usernet
VLAN 0 (user.0):
  Protocol[State]    FD  Source Address  Port   Dest. Address  Port RecvQ SendQ
  TCP[SYN_SENT]      13       127.0.0.1 12345       127.0.0.1 12345     0     0
  TCP[HOST_FORWARD]  11               * 12345       127.0.0.1 12345     0     0
