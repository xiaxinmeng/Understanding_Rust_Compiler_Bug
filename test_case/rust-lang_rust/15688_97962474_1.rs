
                #[derive(Copy, Clone)] pub struct sockaddr_in6 {
                    pub sin6_family: sa_family_t,
                    pub sin6_port: in_port_t,
                    pub sin6_flowinfo: u32,
                    pub sin6_addr: in6_addr,
                    pub sin6_scope_id: u32,
                }
