
struct Ipv4Addr { address }
struct Ipv6Addr { address, scopeID }
enum IpAddr { Ipv4Addr, Ipv6Addr }
struct SockAddr { IpAddr, port }
