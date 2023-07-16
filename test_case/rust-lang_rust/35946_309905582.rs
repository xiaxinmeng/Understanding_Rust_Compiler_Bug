rust
struct NetworkData {
    cons: BTreeMap<Token, TcpStream>,
}
impl {
    pub fn get_client_ip(&self, token: &Token) -> io::Result<String> {
         match self.cons.get(token) {
             Option::None => Err(Error::new(ErrorKind::Other, "Client doesn't exist")),
             Option::Some(ref con) => match con.peer_addr() {
                    Ok(addr) => match addr {
                        SocketAddr::V4(v4) => Ok(format!("{:?}", v4)),
                        SocketAddr::V6(v6) => Ok(format!("{:?}", v6))
                     },
                     Err(e) => Err(e)
              }
          }
     }
}
